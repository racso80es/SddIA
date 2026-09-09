---
feature_name: kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
created: "2026-09-09"
process: bug-fix
version_implementation: "1.0.0"
items:
  - resolver-release-first
  - launcher-preflight-warn
  - hermetic-tests
  - host-release-mayeuta-llm
---

# Implementación — prótesis `mayeuta-llm` (`64f37c7f7b34`)

## Cambios

| Archivo | Cambio |
|---------|--------|
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | `resolve_mayeuta_llm`: orden **release → debug**. Tests herméticos (`resolve_mayeuta_llm_*`). Traza literal intacta. Override env intacto. |
| `SddIA/scripts/daemons/kalma2-bridge.sh` | `warn_if_mayeuta_llm_missing` antes de `exec`. WARN + comando de compile. **No** `exit 1`. |
| `SddIA/target/release/mayeuta-llm` | Host CA1. Gitignored. |
| `interfaces/kalma2/app.js` | **Intacto** (fuera de sello; dirty local excluido). |
| Cubo Kaizen | **No** (CA5 diferido). |

## Detalle

1. **Resolver (CA2):** candidatos default `SddIA/target/release/mayeuta-llm` luego `debug`. Si `$SDDIA_MAYEUTA_LLM_BIN` set y no ELF → traza `SDDIA_MAYEUTA_LLM_BIN no es ELF nativo` (otro sello).
2. **Tests (CA3):** tempdir + magia `\x7fELF`; mutex + restore de env. Cuatro casos: override ELF, release>debug, ausente=traza literal, override no-ELF.
3. **Lanzador (CA4):** override no-ELF o ausencia en `target/{release,debug}` → stderr `[WARN] … Compilar: cd SddIA && cargo build --release -p mayeuta-llm`. El `exec` del puente sigue.
4. **Host (CA1):** `cargo build --release -p mayeuta-llm` con `CARGO_TARGET_DIR` del workspace.

## Sin cambios

- Skill `mayeuta-llm` (solo compile)
- `resolve_orchestrator`
- WUI / SSE / `/api/aiua/interact`
- Retry/backoff (DA-5)
