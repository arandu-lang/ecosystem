# Roadmap Operacional do c2arandu

Este documento é a fila de execução e rastreamento de tarefas do projeto **c2arandu**. Todas as decisões técnicas e padrões arquiteturais seguem a [RFC 0001: Arquitetura do c2arandu e Elevação Semântica](docs/rfcs/0001-c2arandu-transpiler-architecture-and-lifting.md).

---

## 🚦 Visão Geral dos Marcos

- [x] **Marco 0: Fundação, Clang Index e Tipos Primitivos (v0.1.0)**
- [x] **Marco 1: Controle de Fluxo, Statements e Expressões de Função (v0.2.0)**
- [x] **Marco 2: Elevação Semântica OSSA (Lifting para `ref`, `mut ref` e `[]T`) (v0.3.0)**
- [ ] **Marco 3: Dual-Locus Storage e Interop com Bibliotecas Reais (v0.4.0)**
- [ ] **Marco 4: Integração de Tooling e CLI Oficial (`arandu c2arandu`) (v1.0.0)**

---

## 📦 Marco 0: Fundação, Clang Index e Tipos Primitivos (v0.1.0)
> **Objetivo:** Ingerir arquivos C via Clang AST e gerar tipos estruturais Arandu compatíveis com `arandu check`.

### Infraestrutura e Setup
- [x] Criar repositório estruturado em Rust com `Cargo.toml`.
- [x] Integrar com `clang-sys` e vincular à biblioteca nativa `/usr/lib/libclang.so`.
- [x] Implementar autodescoberta do diretório de recursos do Clang (`-resource-dir`) para carregar cabeçalhos padrão (`stdint.h`, `stdbool.h`).
- [x] Configurar tratamento de diagnósticos e erros de compilação do C.
- [x] Criar processo formal de RFCs em `docs/rfcs/`.
- [x] Publicar [RFC 0001](docs/rfcs/0001-c2arandu-transpiler-architecture-and-lifting.md) com o estudo comparativo de transpiladores.

### Mapeamento de Tipos e Declarações C11
- [x] Mapeamento de tipos primitivos escalares (`void`, `bool`, `int`, `uint`, `i8`..`i64`, `u8`..`u64`, `f32`, `f64`).
- [x] Mapeamento de inteiros de ponteiro (`size_t` $\to$ `usize`, `ptrdiff_t` $\to$ `isize`).
- [x] Mapeamento de ponteiros crus (`T*` $\to$ `ptr[T]`).
- [x] Mapeamento de arrays estáticos (`T[N]` $\to$ `[N]T`).
- [x] Extração de `typedef` e geração de aliases `public type X = Y`.
- [x] Extração de `enum` C e geração de `public enum Nome { ... }`.
- [x] Extração de `struct` C com `@Repr("C")` e campos públicos alinhados.
- [x] Extração de protótipos de funções externas C para bloco `extern "C"`.

### Validação e Gate 0
- [x] Criar caso de teste real (`examples/game_math.c`).
- [x] Validar que o código `.aru` gerado é aceito pelo compilador Arandu com `arandu check`.

---

## ⚙️ Marco 1: Controle de Fluxo, Statements e Expressões de Função (v0.2.0)
> **Objetivo:** Transpilar o corpo executável de funções C `{ ... }` para código Arandu nativo sem necessidade de `extern "C"`.

### Suporte a Modos de CLI
- [x] Adicionar flag `--mode=transpile` (padrão para `.c`) e `--mode=binding` (para `.h`).
- [x] Separar funções com corpo (geram `func nome(...) { ... }`) de funções sem corpo.

### Declarações Locais e Atribuições
- [x] Transpilar declaração de variáveis locais: `int x = 10;` $\to$ `let mut x: int = 10`.
- [x] Inferência e shadowing de parâmetros mutados por valor: `_param` e `let mut param = _param`.
- [x] Transpilar atribuições simples (`x = y;`) e compostas (`x += y;`, `x -= y;`, etc.).
- [x] Transpilar incremento e decremento (`x++`, `x--`) para `x = x + 1`.

### Estruturas de Controle de Fluxo
- [x] Transpilar blocos condicionais: `if (cond) { ... } else { ... }` $\to$ `if cond { ... } else { ... }`.
- [x] Adequar condições booleanas inteiras: `if (x)` em C $\to$ `if (x != 0)` em Arandu.
- [x] Transpilar laço `while (cond) { ... }`.
- [x] Transpilar laço `do { ... } while (cond);`.
- [x] Transpilar laço `for (int i = 0; i < n; i++)` para `for` idiomático ou `while`.
- [x] Transpilar comandos de salto `return`, `break` e `continue`.
- [x] Transpilar `switch (val) { case ... }` para `match val { ... }` com braço `_ => { ... }` exaustivo.

### Operadores e Expressões
- [x] Operadores aritméticos: `+`, `-`, `*`, `/`, `%`.
- [x] Operadores relacionais: `==`, `!=`, `<`, `<=`, `>`, `>=`.
- [x] Operadores lógicos: `&&`, `||`, `!`.
- [x] Operadores bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`.
- [x] Casts explícitos: `(float)x` $\to$ `(x as f32)`.
- [x] Acesso a campos: `ptr->campo` e `val.campo` $\to$ `ptr.campo` / `val.campo`.
- [x] Desreferenciação e indexação: `*ptr` $\to$ `ptr[0]`, `arr[i]` $\to$ `arr[i]`.
- [x] Chamadas de função: `foo(a, b)` $\to$ `foo(a, b)`.

### Validação e Gate 1
- [x] Transpilar algoritmo recursivo/iterativo real (Fibonacci) e validar 100% aprovado no `arandu check`.
- [x] Teste de regressão diferencial executável: paridade de execução confirmada (binário C e JIT Arandu executaram com o mesmo código de saída: `204`).

---

## 🧠 Marco 2: Elevação Semântica OSSA (Semantic Lifting) (v0.3.0)
> **Objetivo:** Eliminar `ptr[T]` inseguros em favor de referências seguras `ref T`, `mut ref T`, fatias `[]T` e erros idiomáticos `Result`.

### Análise de Proveniência e Escape de Ponteiros
- [x] Publicar RFC dedicada: [RFC 0002: Elevação Semântica OSSA e Promoção Segura de Ponteiros C](docs/rfcs/0002-ossa-semantic-lifting-and-safe-pointer-promotion.md).
- [x] Análise de imutabilidade de parâmetros:
  - Se um argumento `const T*` ou `T*` for somente lido e não sofrer aritmética: elevar para `ref T`.
  - Se um argumento `T*` sofrer escrita direta em campo/endereço sem sofrer aritmética: elevar para `mut ref T`.
- [x] Fusão de buffers em fatias seguras:
  - Detectar parâmetros indexados com `[i]` e promover para fatia segura `[]T` (ou `mut []T` se sofrer escrita).
- [x] Análise de nulabilidade:
  - Se houver checagens `p == NULL` ou `p != NULL` / `!p`: elevar para opcional `ref T?` / `mut ref T?`.
- [x] Flag CLI `--no-lifting` para permitir desativar a elevação e manter ponteiros crus `ptr[T]`.

### Validação e Gate 2
- [x] Demonstrar utilitário geométrico com structs (`examples/geometry.c`) transpilado com zero ocorrências de `ptr[T]`: `ref Point` e `mut ref Point`.
- [x] Demonstrar utilitário de array (`sum_array`) transpilado com fatia segura `[]int`.
- [x] Todo código transpilado com lifting validado 100% aprovado no `arandu check`.

---

## 🏛️ Marco 3: Dual-Locus Storage e Interop com Bibliotecas Reais (v0.4.0)
> **Objetivo:** Resolver alocação dinâmica e estruturas de dados complexas conectando-se ao modelo Dual-Locus (RFC 0043 do Arandu).

- [x] RFC dedicada: [RFC 0003: Integração do c2arandu com TaskArena e GenRef](docs/rfcs/0003-dual-locus-task-arena-and-dynamic-memory.md).
- [x] Mapeamento de `malloc` / `calloc`:
  - Alocações que não escapam da tarefa são baixadas para a `TaskArena` linear de $O(1)$.
  - Verificação de segurança em chamadas a alocadores externos e suporte a unsafe block.
- [x] Mapeamento de nós encadeados e grafos cíclicos com deleção dinâmica para `gen ref T`.
- [x] Transpilar e rodar a suíte de testes de bibliotecas C reais da indústria:
  - [x] `miniz.h` e `miniz.c` (implementação pura de zlib/deflate/crc/adler).
    - [x] Transpilação completa de cabeçalhos (`miniz.h`) validada pelo `arandu check` (`miniz_h.aru`).
    - [x] Ingestão contextual de tipos e enums de headers locais com deduplicação de enums anônimos.
    - [x] Avaliação compile-time de `sizeof` via Clang evaluation engine.
    - [x] Verificação JIT nativa com paridade matemática bit-a-bit de `mz_adler32` e `mz_crc32` contra o zlib/C (`tests/miniz_parity.rs`).
  - [ ] `stb_image.h` (decodificador de imagens em arquivo único).
  - [ ] Módulo central de B-Tree ou VFS do `sqlite3.c`.

---

## 🚀 Marco 4: Integração de Tooling e CLI Oficial (v1.0.0)
> **Objetivo:** Disponibilizar o `c2arandu` como subcomando oficial e integrado da linguagem Arandu.

- [ ] Integrar comando oficial `arandu c2arandu` no `arandu_cli`.
- [ ] Suporte a projetos inteiros com múltiplos arquivos `.c` e diretórios de headers.
- [ ] Geração automática de `arandu.toml` para pacotes C transpilados.
- [ ] Integração com o LSP do Arandu para oferecer quick-fixes e refatorações no código transpilado.
