---
feature_name: kalma2-bridge-aiua-interact-stale-elf
created: "2026-09-09"
process: bug-fix
branch: fix/kalma2-bridge-aiua-interact-stale-elf
persist_ref: docs/fixes/kalma2-bridge-aiua-interact-stale-elf
pbi_ref: docs/todos/done/[FIX] kalma2-bridge ELF release fósil — POST api-aiua-interact 404.md
document_id: PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF
uuid: "bd611423-eea8-446a-a405-79cc08685e38"
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/280
ci_run_id: "34362468515"
checks:
  KALMA-STALE-CA1: APTO
  KALMA-STALE-CA2: APTO
  KALMA-STALE-CA3: APTO
  KALMA-STALE-CA4: APTO
  KALMA-STALE-CA5: APTO
  KALMA-STALE-CA6: APTO
  KALMA-STALE-CA-CI: APTO
git_changes:
  - docs/fixes/kalma2-bridge-aiua-interact-stale-elf/
  - docs/todos/done/[FIX] kalma2-bridge ELF release fósil — POST api-aiua-interact 404.md
  - docs/audits/kalma2-wui-tormentosa-chat-20260909.md
  - SddIA/evolution/51a53297-7e8d-469e-8bb2-8b413b28a366.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — kalma2-bridge-aiua-interact-stale-elf

**Veredicto global: APTO.** CA-CI sellado con run `34362468515` (PR #280, `headSha` `39dc52c`).

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| KALMA-STALE-CA1 | Símbolo ELF vivo | APTO | PID `1244544`; mangled `handle_aiua_interact`; substring `aiua/intera` |
| KALMA-STALE-CA2 | `POST /api/aiua/interact` ≠404 | APTO | Control 404 en `/api/no-existe`; aiua timeout 0 bytes (handler, no dispatcher) |
| KALMA-STALE-CA3 | `#aiua-pulse` sin `ruta desconocida` | APTO | `GET /` 200 con `#aiua-pulse`; ruta HTTP despachada |
| KALMA-STALE-CA4 | No-regresión `/api/chat` enrutamiento | APTO | ≠404 inmediato; combustión Mayeuta deslindada |
| KALMA-STALE-CA5 | Filename plano + enlaces lógicos | APTO | Basename sin `/`; PBI sin `file://` |
| KALMA-STALE-CA6 | PBI en `done/`; `pbi_archived: true`; sin diffs puente | APTO | Este archivo + PBI archivado; crate/WUI fuera del sello |
| KALMA-STALE-CA-CI | Checks GitHub Actions verdes | APTO | [run 34362468515](https://github.com/racso80es/SddIA/actions/runs/34362468515): `sddia-index-integrity`, `wasi-runtime-smoke`, `eda-iota-smoke-simulate`, `eda-bus-e2e-smoke`, `eda-iota-physical` pass. Duplicado push `34362461303`: mismos pass; skip e2e/physical del evento push no son fallo |

## Causa raíz cerrada (física + documental)

Residual de entrega: PID 6151 ejecutaba ELF 2026-09-06 sin la ruta de PR #276. Reciclo 15:17–15:18 alinea runtime con genoma. Este PR sella el archivo.

## Cierre documental

| Paso | Estado |
|------|--------|
| PBI → `docs/todos/done/` | ✅ |
| `pbi_archived: true` | ✅ |
| PR único pre-merge | ✅ https://github.com/racso80es/SddIA/pull/280 |
| CA-CI / `accept-pr` | ✅ run `34362468515`; `accept-pr` a continuación |
