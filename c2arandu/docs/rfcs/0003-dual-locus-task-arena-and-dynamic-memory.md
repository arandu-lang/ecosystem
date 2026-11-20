# RFC 0003: Integração do c2arandu com TaskArena e GenRef (Dual-Locus Storage)

- **Número da RFC:** 0003
- **Título:** Integração do c2arandu com TaskArena e GenRef (Dual-Locus Storage & Dynamic Memory)
- **Autor(es):** Bruno Bispo dos Santos & Equipe Arandu
- **Data de Início:** 2026-11-08
- **Status:** `Proposed`
- **Área Principal:** `Memory Model` / `Lifting` / `Dual-Locus` / `Runtime Interop`
- **Evolui e Estende:** RFC 0001 (Arquitetura c2arandu), RFC 0002 (OSSA Semantic Lifting) e RFC 0043 do Arandu (Dual-Locus Semantic Memory Model)

---

## 1. Resumo Executivo

O maior desafio histórico na transpilação de C para linguagens com segurança de memória é o gerenciamento de alocação dinâmica (`malloc`, `calloc`, `realloc`, `free`). Ferramentas como `c2rust` ou `corrosion` simplesmente mapeiam essas chamadas para `libc::malloc` e envelopam tudo em ponteiros crus e blocos `unsafe`.

Com a conclusão dos Passos 1 a 4 da **RFC 0043 (Dual-Locus Semantic Memory Model)** no compilador Arandu, a linguagem agora possui:
1. **Desacoplamento ortogonal entre Storage Locus ($\lambda_{\text{store}}$) e Reference Locus ($\lambda_{\text{ref}}$)**: Uma estrutura ou variável pode ter seu armazenamento alocado em uma arena linear com rebobinamento em $O(1)$ (`TaskArena`) enquanto suas referências permanecem ponteiros nativos diretos (`DirectPtr`).
2. **Ciclo de vida por Escopo e Laço**: `ArenaCheckpoint` e `ArenaRewind` automáticos para alocações locais a laços.
3. **GenRef**: Referências geracionais para estruturas cíclicas complexas e grafos com descarte dinâmico não aninhado.

Esta RFC estabelece a estratégia de mapeamento e elevação de memória dinâmica no **`c2arandu`**, definindo como padrões idiomáticos de C (`malloc`, `free`, buffers temporários e estruturas persistentes) são convertidos em construções Arandu seguras, sem vazamentos de memória e sem dependência de coletor de lixo (GC).

---

## 2. Taxonomia de Alocações C e Mapeamento no Arandu

Em C, toda memória dinâmica é alocada pela mesma primitiva (`malloc(size)`). No entanto, semanticamente, o código C divide-se em três padrões fundamentais:

```
                          Alocação C (malloc / calloc)
                                       │
                      O ponteiro escapa da função atual?
                                     /   \
                         [ NÃO ]    /     \    [ SIM ]
                                   v       v
                        Padrão 1:       O tempo de vida é delimitado
                       Scratch Buffer   por uma sessão/tarefa estruturada?
                       (TaskArena)                /   \
                                      [ SIM ]    /     \    [ NÃO ]
                                                v       v
                                           Padrão 2:     Padrão 3:
                                          Task Arena     Grafo Dinâmico / Nó Cíclico
                                          Carrier        (GenRef / GenArena)
```

### Padrão 1: Buffers Temporários Locais (*Scratch Storage*)
- **Padrão em C:** Uma função aloca um buffer de trabalho com `malloc`, processa dados e invoca `free` antes de retornar.
  ```c
  void processar_bloco(const uint8_t *in, size_t len) {
      uint8_t *tmp = (uint8_t *)malloc(len * 2);
      // ... cálculos ...
      free(tmp);
  }
  ```
- **Elevação c2arandu:**
  - O `c2arandu` detecta que `tmp` não escapa do escopo léxico da função.
  - Eleva `tmp` para um array dinâmico alocado na **`TaskArena`** ou `std.alloc.TaskArena`.
  - O `free(tmp)` é elidido pelo compilador Arandu, pois a arena é linear e rebobinada automaticamente no término do escopo ou iteração do laço.

### Padrão 2: Agregados que Carregam Borrows Locais (*Carrier Return*)
- **Padrão em C:** Uma função cria um descritor ou par de ponteiros e retorna por valor:
  ```c
  typedef struct { const int *x; const int *y; } Par;
  Par criar_par(int a, int b) {
      Par p;
      p.x = &a; // Erro clássico em C (dangling pointer)!
      p.y = &b;
      return p;
  }
  ```
- **Elevação c2arandu (Aproveitando a RFC 0043 do Arandu):**
  - O `c2arandu` emite:
    ```arandu
    struct Par { x: &int, y: &int }
    func criar_par(a: int, b: int): Par {
        return Par { x: &a, y: &b }
    }
    ```
  - Graças ao modelo **Dual-Locus**, o compilador Arandu promove automaticamente `a` e `b` para $\lambda_{\text{store}} = \text{TaskArena}$ com $\lambda_{\text{ref}} = \text{DirectPtr}$, tornando o código 100% seguro contra dangling pointers!

### Padrão 3: Buffers Redimensionáveis e Streams (Estilo `miniz`)
- **Padrão em C:** Estruturas de contexto como `z_stream` ou buffers de descompressão:
  ```c
  void *state = malloc(sizeof(tinfl_decompressor));
  // uso contínuo em chamadas repetidas
  free(state);
  ```
- **Elevação c2arandu:**
  - Se a alocação tiver tamanho conhecido estaticamente (`sizeof(T)`), eleva diretamente para uma struct Arandu por valor ou `Box[T]` / `GenRef[T]`.
  - Se for um buffer de bytes redimensionável, eleva para `std.alloc.Vec<u8>`.

---

## 3. Estratégia de Aritmética de Ponteiro para Fatias (`[]T`)

O `miniz.c` realiza intensa leitura sequencial em buffers de bits e tabelas de Huffman:
```c
const mz_uint8 *pSrc = pIn_buf;
mz_uint32 b = *pSrc++;
```

O `c2arandu` implementa a elevação de ponteiros cursor:
1. **Detecção de Cursor Contíguo:** Quando uma variável ponteiro avança apenas incrementalmente dentro dos limites de uma fatia (`pSrc++` ou `pSrc += n` com checagem de bound).
2. **Reescrita Idiomática:**
   ```arandu
   // Em vez de ponteiro cru mutável:
   let mut src_idx: usize = 0
   let b = in_buf[src_idx]
   set src_idx = src_idx + 1
   ```
   Isso preserva as garantias de bounds checking e elimina completamente o uso de `unsafe` em loops de descompressão.

---

## 4. Plano de Implementação (Marco 3)

1. **Passo 3.1: Mapeamento de Primitivas `stdlib.h` (`malloc`, `calloc`, `free`)**
   - Transpilação de chamadas para inicializadores Arandu nativos ou APIs da stdlib (`TaskArena` / `alloc`).
2. **Passo 3.2: Suporte a Casts e Punning Estático de Structs**
   - Suporte no parser do `c2arandu` para Clang AST nodes `CStyleCastExpr`, `UnaryExprOrTypeTraitExpr` (`sizeof`).
3. **Passo 3.3: Benchmark e Validação com `miniz`**
   - Transpilar primeiro funções isoladas:
     - `mz_adler32` (cálculo de checksum, laços contíguos de memória).
     - `mz_crc32` (tabela estática e manipulação bitwise).
     - `tinfl_decompress` (máquina de estados do descompressor DEFLATE).
   - Validar 100% dos arquivos gerados com `arandu check` e teste diferencial executável.

---

## 5. Critérios de Sucesso

- O `c2arandu` processa funções com alocação e buffers sem abortar por nós AST não reconhecidos.
- O código gerado compila com `arandu check` sem exigir anotações `@NoFallback` ou hacks inseguros.
- Teste de paridade com `miniz`: compressão/descompressão produz o mesmo hash SHA-256 no binário C e no Arandu JIT.
