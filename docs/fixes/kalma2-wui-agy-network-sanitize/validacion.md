---
feature_name: kalma2-wui-agy-network-sanitize
created: "2026-09-10"
process: bug-fix
phase: validate
agents: argos
branch: fix/kalma2-wui-agy-network-sanitize
branch_name: fix/kalma2-wui-agy-network-sanitize
persist_ref: docs/fixes/kalma2-wui-agy-network-sanitize
pbi_ref: docs/todos/done/[FIX] Kalma2 WUI — red agy sanitizada en epidermis (sin retry).md
document_id: PBI-FIX-KALMA2-AGY-NETWORK-SANITIZE
uuid: "a8634b4b-49c8-4905-a824-ae0c251ca900"
global: PENDIENTE-CI
pbi_archived: true
checks:
  AGY-NET-CA1: APTO
  AGY-NET-CA2: APTO
  AGY-NET-CA3: APTO
  AGY-NET-CA4: APTO
  AGY-NET-CA5: APTO
  AGY-NET-CA6: APTO
  AGY-NET-CA-CI: PENDIENTE-CI
git_changes:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - docs/fixes/kalma2-wui-agy-network-sanitize/
  - docs/todos/done/[FIX] Kalma2 WUI — red agy sanitizada en epidermis (sin retry).md
  - SddIA/evolution/89661fec-bed6-4c3d-8169-3922ddbe7a2f.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — kalma2-wui-agy-network-sanitize

**Veredicto global: PENDIENTE-CI.** CA locales APTO.

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA1 | blob empírico → canónico | APTO | `sanitize_maps_agy_network_issue` |
| CA2 | `dial tcp` → canónico | APTO | mismo test, fixture elegibilidad |
| CA3 | no enmascara auth/503/timeout | APTO | `assert_ne!` + auth aserto |
| CA4 | cero retry | APTO | clasificador síncrono |
| CA5 | tests herméticos | APTO | `sanitize_maps_agy*` 2 passed |
| CA6 | PBI `done/` | APTO | este PR |
| CA-CI | GitHub verde | PENDIENTE-CI | — |
