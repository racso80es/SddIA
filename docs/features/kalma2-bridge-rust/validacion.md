---
feature_name: kalma2-bridge-rust
branch: feat/kalma2-bridge-rust
global: APTO
pbi_archived: true
created: "2026-06-19"
process: feature
checks:
  O1_crate: "APTO — cargo build -p kalma2-bridge"
  O2_static: "APTO — GET / HTTP 200; assets interfaces/kalma2/"
  O3_interact: "APTO — POST /api/interact passthrough envelope orquestador"
  O4_ceguera: "APTO — sin lógica de negocio en crate"
  O5_start_sddia: "APTO — start-sddia.sh arranca kalma2-bridge"
  O6_poda_py: "APTO — sddia-client-bridge.py eliminado"
  traversal: "APTO — GET /../Cargo.toml → 404"
git_changes:
  - SddIA/Cargo.toml
  - SddIA/Cargo.lock
  - SddIA/interfaces/kalma2-bridge/
  - start-sddia.sh
  - start-sddia.md
  - sddia-run.sh
  - interfaces/kalma2/app.js
  - interfaces/kalma2/README.MD
  - .SddIA/client/sddia-client-bridge.py
  - docs/features/kalma2-bridge-rust/
  - docs/todos/done/[FEATURE] kalma2-bridge — puente HTTP nativo Rust.md
---

# Validación — kalma2-bridge (Rust)

**Veredicto global: APTO**

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| O1 | Crate workspace | ✅ | `cargo build -p kalma2-bridge` |
| O2 | Servidor estático | ✅ | `curl GET /` → 200 |
| O3 | POST /api/interact | ✅ | envelope `kalma2-interact` + `duration_ms` |
| O4 | Ceguera espacial | ✅ | passthrough stdout; sin semántica prompt |
| O5 | start-sddia | ✅ | `_resolve_bridge_bin` + health check |
| O6 | Poda Python | ✅ | `.SddIA/client/sddia-client-bridge.py` eliminado |
| T1 | Path traversal | ✅ | `GET /../Cargo.toml` → 404 |
| T2 | Unit test | ✅ | `cargo test -p kalma2-bridge` |

## Comandos (2026-06-19)

```bash
cd SddIA && cargo build -p kalma2-bridge && cargo test -p kalma2-bridge
SDDIA_REPO_ROOT=$PWD/.. SddIA/target/debug/kalma2-bridge &
curl -sf -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8765/
curl -sf -XPOST http://127.0.0.1:8765/api/interact \
  -H 'Content-Type: application/json' -d '{"prompt":"hola"}'
```

## Deuda aceptada

- Centinelas EDA: routing pendiente post-migración Rust (`execute-process.py` ausente en backlog eventos).
- Bridge debug profile (~11 MB); optimización release fuera de alcance PoC.

## Operador post-merge

```bash
cd SddIA && cargo build -p kalma2-bridge -p execute-process
./start-sddia.sh
# → http://127.0.0.1:8765/
```
