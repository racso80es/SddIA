---
feature_name: p16-pyyaml-poda
created: "2026-07-10"
process: bug-fix
items_applied:
  - route-bridge-deleted
  - frontmatter-rust-cli
  - qa-pyyaml-purged
  - requirements-txt-deleted
---

# Ejecución — P16 poda PyYAML

## Cambios aplicados

| Artefacto | Acción |
|-----------|--------|
| `_execute_process_route_bridge.py` | Eliminado |
| `engine/handlers/route_domain.rs` | Entry nativo vía `python_core::invoke_route_domain_event` |
| `engine/python_core.rs` | `invoke_route_domain_event` (paridad bridge) |
| `execute-process --parse-frontmatter` | CLI utilitaria (serde_yaml → JSON) |
| `frontmatter_rust.py` | Wrapper Python sobre parser Rust |
| `execute_process_core.py` + 6 scripts QA | Sin `import yaml` |
| `requirements.txt` | Eliminado (sin consumidores activos) |
| `.github/workflows/sddia-index-qa.yml` | Sin `pip install pyyaml`; build `execute-process` donde aplica |

## Gate P16

| Gate | Estado |
|------|--------|
| Capsules bridge | ✅ (PR #102) |
| Route bridge | ✅ eliminado — core EDA sigue en `route_domain_event_core.py` vía `python_core` |
| `grep` limpio touchpoints QA productivos | ✅ 0 `import yaml` en `SddIA/scripts/qa/` |

## Deuda residual

- Porte full nativo de `route_domain_event_core.py` → Rust ([FIX] Porte route-domain-event core a Rust).
- Cores Python residuales en `python_core.rs` (route fractal, radamanto, telemetry).

## Verificación

```bash
cd SddIA && cargo build -p execute-process
cargo test -p execute-process --lib
python3 SddIA/scripts/qa/verify-process-integrity.py
rg 'import yaml|PyYAML' SddIA/scripts/qa/
```
