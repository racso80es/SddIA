---
feature_name: event-bus-dead-letter-remediation
created: "2026-07-11"
process: bug-fix
branch_name: fix/event-bus-dead-letter-remediation
---

# Ejecución — Remediación dead-letters bus de eventos

## Comandos ejecutados

```bash
# Rama
git checkout -b fix/event-bus-dead-letter-remediation

# Compilación orquestador (debug — usado por sddia-run.sh)
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build -p execute-process

# Tests
python3 -m unittest test_eda_bus_v3plus.TestPullRequestLifecycle -v
cargo test -p execute-process pull_request_audited_forbidden -- --nocapture

# Limpieza ops: retirar testigos ecst-gate obsoletos (5 UUID PullRequest_Audited)
rm .events/dead-letter/subscribers/{5c69f54c,...,b7f4404f}.ecst-gate.json

# Re-enrutado manual 9 pending estancados
./sddia-run.sh --process route-domain-event --inputs '{"event_file_path":".events/pending/<uuid>.json"}'

# Verificación
./sddia-run.sh --process event-bus-audit --inputs '{"emit_kaizen_alert":false}'
```

## Resultados

| Paso | Resultado |
|------|-----------|
| Compilación `execute-process` | ✅ debug actualizado (`target/debug/execute-process`) |
| Tests Python/Rust | ✅ 7 + 1 tests OK |
| Daemons activos | ✅ `event-watcher`, `event-sweeper`, `telegram-watcher`, `github-bridge-watcher` (locks 2026-07-11) |
| Re-enrutado 5× `PullRequest_Audited` | ✅ `sweep.status=purged` |
| Re-enrutado 3× `PullRequest_Presented` + 1× `Fracture` (previo) | ✅ `kaizen-finalized` / purged |
| Re-auditoría | ✅ `stale_pending_count: 0` — informe `72af29f1-397f-429b-b570-d530265e7cc6` |

## Notas operativas

- **Node/npm:** no disponibles en host (`which node` vacío). `npm install` en `SddIA/scripts/limbo/tools/iota-immutable-publisher/` omitido; remediación depende de cápsula Rust nativa.
- **Dead-letters históricos:** conservados como registro Kaizen (90 cabeceras); no purgados.
- **Nuevos DL por Telegram/IOTA en re-ruta:** corregidos con `prefer_wasm=false` en router Rust; históricos WASI permanecen en DL.
