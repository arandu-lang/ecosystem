# RFC 0001: Arquitetura do c2arandu: Estudo de Transpiladores, Lowering C11 e Elevação Semântica Dual-Locus

- **Número da RFC:** 0001
- **Título:** Arquitetura do c2arandu: Estudo de Transpiladores, Lowering C11 e Elevação Semântica Dual-Locus
- **Autor(es):** Bruno Bispo dos Santos & Equipe Arandu
- **Data de Início:** 2026-11-08
- **Status:** `Accepted`
- **Área Principal:** `Frontend` / `Transpiler` / `Semantics` / `OSSA`

---

## 1. Resumo Executivo

O **c2arandu** é um transpilador de código C (C99/C11) para Arandu seguro e idiomático. Seu objetivo é viabilizar a modernização gradual e segura de bases de código legadas em C (sistemas operacionais, bancos de dados como SQLite, bibliotecas multimídia, engines de jogos e infraestrutura) sem perda de desempenho bare-metal e sem a necessidade de reescrita manual do zero.

Esta RFC estabelece:
1. O **estudo comparativo profundo** dos transpiladores existentes na indústria (`c2rust`, `zig translate-c`, `c2go`, `rust-bindgen`), identificando o que copiar e o que rejeitar.
2. A política de distinção entre **Declarações sem Corpo (Protótipos de Cabeçalho)** e **Definições de Funções (Implementações)**.
3. O pipeline em 4 estágios de transpilação e a estratégia de **Elevação Semântica (*Semantic Lifting*)** alavancando o Modelo Dual-Locus ([RFC 0043 do Arandu](file:///home/bruno/Documentos/Desenvolvimento/Arandu-Lang/docs/rfcs/0043-dual-locus-semantic-memory-model.md)).

---

## 2. Estudo Comparativo de Transpiladores Existentes

Para que o `c2arandu` seja uma ferramenta de classe mundial, analisamos os sucessos e os fracassos das ferramentas que tentaram resolver esse problema nas últimas décadas.

| Ferramenta | Origem / Alvo | O que Funciona (Copiar) | Onde Falha (Rejeitar) |
| :--- | :--- | :--- | :--- |
| **`c2rust`** | C $\to$ Rust | • Uso da AST canônica do Clang (`libclang`).<br>• Preservação de macros constantes e tipos básicos.<br>• Suíte extensa de testes diferenciais. | • **Gera 99% de código `unsafe`**.<br>• Força a topologia flexível do C no modelo mono-locus de lifetimes do Rust, resultando em aberrações cheias de `*mut c_void` e casts crus impossíveis de manter. |
| **`zig translate-c`** | C $\to$ Zig | • Embutido diretamente no toolchain.<br>• Mapeamento de tipos C para fatias e tipos primitivos limpos.<br>• Alta velocidade de tradução. | • **Não introduz segurança de memória**.<br>• Mantém alocações manuais com `free()`; os bugs de Use-After-Free, Double Free e vazamento de memória do C continuam idênticos no código Zig gerado. |
| **`c2go`** | C $\to$ Go | • Gerou código funcional suficiente para migrar o compilador do Go 1.5 para Go puro.<br>• Código final conciso e legível. | • **Dependência de Garbage Collector**.<br>• Destruiu a previsibilidade de latência e alocação de baixo nível, impossibilitando uso em sistemas bare-metal, drivers ou games. |
| **`rust-bindgen`** | C $\to$ Rust (FFI) | • Geração limpa de `@Repr("C")` e alinhamentos exatos.<br>• Heurísticas de comentários e documentação Doxygen. | • Focado apenas em declarações de interfaces externas (`extern "C"`), sem capacidade de transpilar corpos ou converter lógica interna para a nova linguagem. |
| **Apple Obj-C $\to$ Swift** | Obj-C $\to$ Swift | • Inferência de nulabilidade (`_Nonnull`, `_Nullable` $\to$ `T?`).<br>• Migração incremental módulo por módulo. | • Específico de ecossistema com runtime gerenciado por ARC e dependente de Foundation/Cocoa. |

### Decisões Fundamentais Derivadas do Estudo:
1. **Copiar do `c2rust` e `bindgen`**: O pipeline baseado na AST completa do Clang (`clang-sys`), com preservação de alinhamentos e layout `@Repr("C")`.
2. **Rejeitar do `c2rust`**: A emissão passiva de blocos `unsafe` infinitos. No Arandu, ponteiros C devem ser elevados para o modelo semântico seguro OSSA.
3. **Copiar do `zig translate-c`**: A simplicidade sintática e a preservação de operadores diretos de máquina.
4. **Rejeitar do `zig translate-c`**: O conformismo com a insegurança de memória manual. O Arandu deve proteger o ciclo de vida com Dual-Locus.

---

## 3. O Problema das Declarações sem Corpo (`extern "C"` vs Funções Nativas)

Na primeira versão do `c2arandu`, funções declaradas como:
```c
void spawn_player(Player* p, float x, float y);
```
foram emitidas dentro de um bloco `extern "C" { ... }`. 

### Por que isso aconteceu?
No modelo de compilação do C, existe uma separação estrita entre:
1. **Declaração de Protótipo (`CXCursor_FunctionDecl` sem corpo)**: Apenas informa a assinatura ao compilador para que outros arquivos possam chamá-la. A implementação real reside em outro arquivo `.c` ou biblioteca compilada (`.a`/`.so`).
2. **Definição de Função (`CXCursor_FunctionDecl` com corpo `{ ... }`)**: Contém os statements executáveis.

Quando o `c2arandu` lê um arquivo `.h` (cabeçalho), as funções **não possuem corpo**. Tratá-las como `extern "C"` é a escolha correta quando o objetivo é **gerar bindings de interoperabilidade** para chamar código C já existente.

### A Nova Política Arquitetural do c2arandu:
O `c2arandu` passará a operar em dois modos explícitos via flag da CLI:

#### Modo A: `--mode=transpile` (Padrão para arquivos `.c`)
* Toda função com corpo é **totalmente transpilada** para uma função Arandu nativa:
  ```arandu
  public func spawn_player(p: ptr[Player], x: f32, y: f32): void {
      // Corpo transpilado para statements Arandu
  }
  ```
* Se uma função for apenas um protótipo sem corpo no mesmo arquivo, o compilador emite um alerta informativo e gera uma assinatura Arandu pura com corpo stub `todo` (ou vincula à unidade de tradução irmã correspondente).

#### Modo B: `--mode=binding` (Padrão para arquivos `.h` puros)
* Focado em gerar cabeçalhos de interoperabilidade FFI com bibliotecas C já compiladas, emitindo `extern "C" { ... }`.

---

## 4. O Pipeline em 4 Estágios do c2arandu

```
[ Arquivo C (.c / .h) ]
         │
         ▼ (Estágio 1)
┌─────────────────────────────────┐
│ Clang AST Ingestion & Normalizer│ ───► Extrai Structs, Enums, Typedefs, Decls e AST de Expressões
└─────────────────────────────────┘
         │
         ▼ (Estágio 2)
┌─────────────────────────────────┐
│ Structural C11 Translation      │ ───► Tradução 1:1 de Statements, Loops, Operadores e ptr[T]
└─────────────────────────────────┘
         │
         ▼ (Estágio 3)
┌─────────────────────────────────┐
│ OSSA Semantic Lifting           │ ───► Eleva ptr[T] para ref T / mut ref T / fatias []T
└─────────────────────────────────┘
         │
         ▼ (Estágio 4)
┌─────────────────────────────────┐
│ Dual-Locus Storage Placement    │ ───► Mapeia malloc/escape para TaskArena (RFC 0043)
└─────────────────────────────────┘
         │
         ▼
[ Código Arandu (.aru) Puro, Seguro e Idiomático ]
```

### Estágio 1: Ingestão e Normalização (Clang AST)
Utiliza a `libclang` para pré-processamento integral (expansão de `#define`, macros condicionais `#ifdef`, inclusão de headers do sistema).

### Estágio 2: Tradução Estrutural C11
Traduz fielmente cada instrução C para seu equivalente direto em Arandu:
- `while (cond) { ... }` $\to$ `while cond { ... }`
- `for (int i = 0; i < n; i++)` $\to$ `for i in 0..n { ... }`
- `if (x) { ... } else { ... }` $\to$ `if x != 0 { ... } else { ... }`
- `ptr->field` $\to$ `ptr.field`
- `*ptr = val` $\to$ `ptr[0] = val` ou atribuição de referência.

### Estágio 3: Elevação Semântica (*Semantic Lifting*)
O diferencial revolucionário do `c2arandu`. O analisador estático inspeciona os acessos a ponteiros:
- **Ponteiro de Leitura Pura**: Um argumento `const T* p` cujos campos são apenas lidos e nunca sofrem aritmética de ponteiro é elevado de `ptr[T]` para `ref T`.
- **Ponteiro de Escrita Exclusiva**: Um argumento `T* p` que só sofre escritas locais e não escapa é elevado para `mut ref T`.
- **Fatias Seguras**: Um padrão comum `(const T* buffer, size_t len)` é detectado e fundido em uma fatia `[]T`.
- **Tratamento de Nulabilidade**: Se um ponteiro nunca é verificado contra `NULL`, o compilador presume `ref T` não-nulo. Se houver checagem `if (p != NULL)`, o tipo é elevado para o opcional `ref T?`.

### Estágio 4: Alocação Dual-Locus e Grafos
- Alocações do tipo `malloc(sizeof(Node))` que escapam do frame léxico são vinculadas ao $\lambda_{\text{store}}$ da **TaskArena**.
- Estruturas de dados com ciclos ou deleções arbitrárias recebem a anotação `gen ref Node` (GenRef), garantindo que a memória física continue sendo um ponteiro nativo em registradores, mas protegida contra Use-After-Free em tempo de execução.

---

## 5. Próximos Passos e Critérios de Aceite

1. **Gate Fase 1**: Toda struct C `@Repr("C")` e enum transpilados devem compilar sem erros com `arandu check`. *(Concluído)*
2. **Gate Fase 2**: Funções C com corpos contendo `if`, `while`, `for`, `+`, `-`, `*`, `/` devem ser transpiladas para código Arandu executável e produzir os mesmos resultados numéricos do binário C nativo.
3. **Gate Fase 3**: Testes de lifting provando que `const Vec2* a` vira `ref Vec2` no código final sem emitir `ptr[Vec2]`.
