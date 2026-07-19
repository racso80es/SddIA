---
feature_name: kalma2-event-bus-integration
branch: feat/kalma2-event-bus-integration
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/119
created: "2026-07-19"
process: feature
checks:
  AC1_degraded: "APTO — data.degraded:true sin CLI; UI marca [degradado]"
  AC2_correlation: "APTO — emitted + event_id == correlation_id"
  AC3_status_404: "APTO — UUID desconocido → HTTP 404"
  AC4_routed: "APTO — project_status unit + delivery_state OK → routed"
  AC5_pec: "APTO — workspace-smoke + correlation_id → PEC; status completed"
  AC6_bridge_readonly: "APTO — /api/status solo lectura; sin write al bus"
  AC7_lazo: "APTO — pending (dominio) → completed (PEC); UI poll 404=pending"
  O1_aduana: "APTO — mock = fallback Mayeuta, no app.js"
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/kalma2.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/interfaces/kalma2-bridge/Cargo.toml
  - interfaces/kalma2/app.js
  - interfaces/kalma2/index.html
  - interfaces/kalma2/style.css
  - interfaces/kalma2/README.MD
  - docs/features/kalma2-event-bus-integration/
  - docs/todos/done/[OPERATIVO] PBI: Integración Real de Kalma2 con el Motor de Eventos SddIA.md
  - SddIA/evolution/835f4679-f143-4483-8fa4-346f7140348e.md
---

# Validación — kalma2-event-bus-integration

**Veredicto global: APTO**

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| AC1 | Chat degradado etiquetado | ✅ | `sddia-run` sin CLI → `degraded:true`; `app.js` prefijo `[degradado]` |
| AC2 | Acuse correlacionable | ✅ | emit fix → `event_id == correlation_id` |
| AC3 | Status 404 | ✅ | curl UUID desconocido → 404 |
| AC4 | Proyección routed | ✅ | `cargo test -p kalma2-bridge project_status_routed` |
| AC5 | PEC + completed | ✅ | smoke correlation → PEC; status con PEC → `completed` |
| AC6 | Bridge ciego (no write) | ✅ | solo lectura FS en `/api/status` |
| AC7 | Lazo UI/API | ✅ | pending→completed; poll tolera 404 post-purge |
| O1 | Diagnóstico aduana | ✅ | clarify D1; sin mock en `app.js` |
| T | Tests nativos | ✅ | `kalma2` 6 ok · `kalma2-bridge` 5 ok |

## Comandos (2026-07-19)

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo build -p execute-process -p kalma2-bridge
CARGO_TARGET_DIR=target cargo test -p execute-process kalma2
CARGO_TARGET_DIR=target cargo test -p kalma2-bridge

unset SDDIA_LLM_CLI_COMMAND
./sddia-run.sh --process kalma2-interact --inputs '{"prompt":"argos chat"}'
./sddia-run.sh --process kalma2-interact --inputs \
  '{"prompt":"inicia fix docs/todos/pending/[FIX] x.md"}'

# Status (bridge en puerto lab)
SDDIA_CLIENT_PORT=18765 SDDIA_REPO_ROOT=$PWD SddIA/target/debug/kalma2-bridge &
curl -sS "http://127.0.0.1:18765/api/status?event_id=<uuid>"
```

## Deudas / matices (no bloquean APTO)

| ID | Nota |
|----|------|
| N1 | Tras purge fractal del dominio, status puede 404 hasta aparece PEC; UI trata 404 como pending |
| N2 | En lab, suscriptores Kalma2 (TQM/IOTA) pueden fallar → dead-letter; PEC correlacionado sigue proyectando `completed` si existe |
| N3 | Deudas heredadas D1/D3/D5 del router: fuera de alcance (clarify D6) |

## Cierre documental

- PBI archivado en `docs/todos/done/` en esta rama (`pbi_archived: true`).
- Listo para `delivery-close-cycle` / PR único.
