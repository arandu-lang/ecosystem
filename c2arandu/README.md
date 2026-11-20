# c2arandu

> **Transpilador de código C para Arandu seguro e idiomático com suporte a Dual-Locus.**

O **c2arandu** é uma ferramenta projetada para resolver um dos maiores gargalos da engenharia de software contemporânea: a migração de bases de código legadas em C para uma linguagem moderna de alto desempenho e segurança de memória sem Garbage Collector.

---

## 💡 Por que o c2arandu existe?

Até hoje, a indústria esteve presa em um dilema:
1. **`c2rust`**: Converte C para Rust, mas produz um código ilegível repleto de blocos `unsafe`, ponteiros crus e casts manuais. O modelo estrito de lifetimes mono-locus do Rust rejeita as estruturas de dados fundamentais do C (grafos, nós encadeados, referências cruzadas).
2. **`zig translate-c`**: Transpila C para Zig com facilidade, mas como o Zig tem gerenciamento puramente manual, todos os bugs de *Use-After-Free* e vazamento de memória continuam existindo.
3. **`c2go`**: Converte C para Go, mas introduz um Garbage Collector pesado, matando o desempenho bare-metal para jogos, servidores de baixa latência e kernels.

### A Vantagem do Arandu
O Arandu resolve essa equação graças a duas inovações arquiteturais:
* **Dual-Locus Semantic Memory Model (RFC 0043)**: A representação física de uma referência (`ref T`) é exatamente um ponteiro de máquina nativo (`DirectPtr`), enquanto a promoção de escape do dado é direcionada para a `TaskArena` linear de $O(1)$ sem alterar o tipo da struct.
* **Preservação de ABI C**: Uma struct Arandu `@Repr("C")` possui alinhamento e offsets idênticos à struct C correspondente.

---

## 🚀 Uso Rápido

```bash
# Transpilar um arquivo C para Arandu
c2arandu input.c -o output.aru

# Transpilar com flags de include do C
c2arandu input.c -I/usr/include -o output.aru

# Inspecionar código Arandu gerado diretamente na saída padrão
c2arandu math_utils.c
```

---

## 🛠️ Arquitetura

O projeto é dividido nos seguintes módulos:
* `src/parser.rs`: Inicialização do Clang Index via `libclang` e varredura da AST C.
* `src/types.rs`: Tradutor canônico de tipos C para tipos de primeira classe do Arandu.
* `src/codegen.rs`: Emissor de código `.aru` formatado e idiomático.
* `src/lifting.rs` *(Fase 3)*: Pass de análise de fluxo para elevação de ponteiros crus (`ptr[T]`) para `ref T`, `mut ref T` e fatias `[]T`.

---

## 🗺️ Roadmap
Veja o documento completo em [ROADMAP.md](ROADMAP.md).
