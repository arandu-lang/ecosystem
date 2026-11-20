# Processo de RFCs do c2arandu

Todas as decisões arquiteturais, semânticas e de design do transpilador **c2arandu** devem ser propostas, debatidas e registradas na forma de RFCs (*Request for Comments*), mantendo o mesmo rigor de engenharia e solidez científica do compilador Arandu.

---

## 🏛️ Estados de Governança

```
┌─────────┐      ┌────────────┐      ┌───────────────┐      ┌─────────────┐
│  Draft  │ ───► │  Accepted  │ ───► │  Implemented │ ───► │  Frozen-R0  │
└─────────┘      └────────────┘      └───────────────┘      └─────────────┘
```

1. **`Draft`**: Proposta em discussão técnica aberta. O design detalhado, trade-offs e impactos ainda estão sendo refinados.
2. **`Accepted`**: Decisão aprovada pela arquitetura. Pronta para implementação.
3. **`Implemented`**: Escopo implementado no código, coberto por testes de regressão e validado pelo `arandu check`.
4. **`Frozen-R0`**: Baseline congelada e protegida contra quebras.

---

## 📚 Índice de RFCs

| Número | Título | Área Principal | Status | Data |
| :--- | :--- | :--- | :--- | :--- |
| [0001](0001-c2arandu-transpiler-architecture-and-lifting.md) | Arquitetura do c2arandu: Estudo de Transpiladores, Lowering C11 e Elevação Semântica Dual-Locus | Frontend / Transpiler / OSSA | `Accepted` | 2026-11-08 |
| [0002](0002-ossa-semantic-lifting-and-safe-pointer-promotion.md) | Elevação Semântica OSSA e Promoção Segura de Ponteiros C (Semantic Lifting) | Frontend / Semantic Analysis / Lifting / OSSA | `Accepted` | 2026-11-08 |
| [0003](0003-dual-locus-task-arena-and-dynamic-memory.md) | Integração do c2arandu com TaskArena e GenRef (Dual-Locus Storage) | Memory Model / Lifting / Dual-Locus / Runtime Interop | `Accepted` | 2026-11-08 |
