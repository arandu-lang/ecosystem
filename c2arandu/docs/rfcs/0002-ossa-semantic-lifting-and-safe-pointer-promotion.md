# RFC 0002: Elevação Semântica OSSA e Promoção Segura de Ponteiros C

- **Número da RFC:** 0002
- **Título:** Elevação Semântica OSSA e Promoção Segura de Ponteiros C (Semantic Lifting)
- **Autor(es):** Bruno Bispo dos Santos & Equipe Arandu
- **Data de Início:** 2026-11-08
- **Status:** `Accepted`
- **Área Principal:** `Frontend` / `Semantic Analysis` / `Lifting` / `OSSA`
- **Evolui e Estende:** RFC 0001 (c2arandu) e RFC 0043 do Arandu (Dual-Locus)

---

## 1. Resumo Executivo

No C, a notação `T*` é semanticamente ambígua: ela é usada indistintamente para representar referências imutáveis, parâmetros de saída (*out-parameters*), arrays dinâmicos, ponteiros nulos opcionais e handles opacos. 

Enquanto ferramentas como o `c2rust` desistem da análise e rebaixam todos os ponteiros para `*mut T` em blocos `unsafe`, o **c2arandu** introduz a **Elevação Semântica (*Semantic Lifting*) baseada em OSSA**: um pass de análise estática de fluxo e proveniência que inspeciona o uso dos ponteiros no corpo das funções e os eleva para os tipos seguros e idiomáticos de primeira classe do Arandu:
1. `ref T` (referência imutável não-nula).
2. `mut ref T` (referência mutável exclusiva).
3. `[]T` e `mut []T` (fatias seguras com bounds checking).
4. `ref T?` / `mut ref T?` (opcionais com verificação de nulidade em tempo de compilação).

---

## 2. As 5 Regras Canônicas de Elevação Semântica

```
                           Ponteiro C (T* / const T*)
                                       │
            Sofre aritmética de ponteiro arbitrária não contígua?
                                      / \
                           [ SIM ]   /   \   [ NÃO ]
                                    /     \
                                   v       v
                             ptr[T] crú   É indexado com [i] e associado a um tamanho 'len'?
                             (Fallback)            / \
                                        [ SIM ]   /   \   [ NÃO ]
                                                 /     \
                                                v       v
                                           Fatia []T   Sofre escrita em *p ou p->field?
                                          / mut []T             / \
                                                     [ SIM ]   /   \   [ NÃO ]
                                                              /     \
                                                             v       v
                                                         mut ref T  ref T
                                                        (ou mut ref T?) (ou ref T?)
```

### Regra 1: Leitura Pura de Objeto Único $\to$ `ref T`
* **Condição:** Um parâmetro `p: const T*` ou `p: T*` é apenas lido no corpo da função (através de `*p` ou `p->campo`), nunca sofre escrita e nunca sofre aritmética de ponteiro.
* **Ação:** O tipo é promovido de `ptr[T]` para `ref T`.
* **Benefício:** Elimina a necessidade de ponteiros crus. Na máquina física, `ref T` é um ponteiro cru de 64 bits (zero custo!), mas o compilador agora garante que ele não pode ser corrompido.

### Regra 2: Parâmetro de Saída / Mutação de Objeto Único $\to$ `mut ref T`
* **Condição:** Um parâmetro `p: T*` sofre escrita (`*p = val` ou `p->campo = val`), mas nunca sofre aritmética de ponteiros nem é indexado como array.
* **Ação:** O tipo é promovido de `ptr[T]` para `mut ref T`.
* **Benefício:** Mapeia exatamente a semântica de *out-parameter* do C para a semântica de referência mutável e exclusiva do Arandu.

### Regra 3: Fusão de Buffer Contíguo $\to$ Fatias Seguras `[]T` / `mut []T`
* **Condição:** A função recebe um par de argumentos `(p: const T*, len: int)` ou `(p: T*, len: size_t)` e acessa `p[i]`.
* **Ação:** O par é fundido em um único parâmetro: `p: []T` (ou `mut []T` se houver mutação em `p[i]`).
* **Transformação no corpo:** Chamadas ou comparações com `len` passam a utilizar o método intrínseco de fatia `p.len()`.
* **Benefício:** Elimina a vulnerabilidade número 1 do C: o estouro de buffer (*Buffer Overflow*). O Arandu aplica verificação de limites (*Bounds Check Elimination* via AMIR) com custo zero em loops bem formados.

### Regra 4: Inferência de Nulabilidade $\to$ `T?`
* **Condição:** Se o código C contém checagens explícitas como `if (p == NULL)`, `if (p != NULL)` ou `if (!p)`.
* **Ação:** O tipo é promovido para o opcional `ref T?` ou `mut ref T?`.
* **Transformação no corpo:** As checagens `p != NULL` são normalizadas para desempacotamento seguro de `Option`/Nullable do Arandu (`if p is Some(val)` ou `if p != nil`).
* Se o código **nunca checar nulidade** e desreferenciar diretamente, ele é classificado como garantidamente não-nulo (`ref T`).

### Regra 5: Fallback Conservador e Seguro $\to$ `ptr[T]`
* **Condição:** Se o ponteiro sofrer aritméticas incomuns (casts de bytes com offsets não alinhados, XOR de ponteiros em listas ligadas de baixo nível).
* **Ação:** O compilador preserva `ptr[T]`, emitindo um aviso amigável explicando o motivo da preservação do ponteiro cru.

---

## 3. Impacto na Paridade de Código

Considere a função C clássica de distância vetorial:
```c
float distance(const Vec2* a, const Vec2* b) {
    float dx = a->x - b->x;
    float dy = a->y - b->y;
    return sqrtf(dx * dx + dy * dy);
}
```

* **Sem Lifting (Fase 1/2):**
  ```arandu
  public func distance(a: ptr[Vec2], b: ptr[Vec2]): f32 { ... }
  ```
* **Com Lifting OSSA (Fase 3):**
  ```arandu
  public func distance(a: ref Vec2, b: ref Vec2): f32 {
      let dx = a.x - b.x
      let dy = a.y - b.y
      return sqrtf(dx * dx + dy * dy)
  }
  ```
O código transpilado é **100% Arandu seguro, legível e idiomático**, mantendo **exatamente o mesmo código de máquina assembly emitido pelo GCC/Clang**.

---

## 4. Critérios de Aceite

1. **Gate 2.1**: Funções que recebem `const T*` e apenas lêem campos devem ser elevadas para `ref T`.
2. **Gate 2.2**: Funções que escrevem em `T*` devem ser elevadas para `mut ref T`.
3. **Gate 2.3**: Funções que indexam buffers com `len` devem ser elevadas para `[]T`.
4. **Gate 2.4**: Todo código gerado com lifting deve ser aprovado sem erros pelo `arandu check`.
