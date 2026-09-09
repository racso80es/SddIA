---
feature_name: kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
created: "2026-09-09"
process: bug-fix
branch: fix/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
execution_id: "aee88988-ea8f-434d-a0bd-8bc67e5748b9"
items_applied:
  - host-release-mayeuta-llm
  - resolver-release-first
  - launcher-preflight-warn
  - hermetic-tests
  - chat-no-regression
---

# Execution — kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-64f37c7f7b34-init.json
```

`execution_id`: `aee88988-ea8f-434d-a0bd-8bc67e5748b9`. workspace-init **executed**. Diseño `simulated`. Planificación commit `be2e5ed`.

## CA1 host

`unset CARGO_TARGET_DIR && cd SddIA && cargo build --release -p mayeuta-llm`

`SddIA/target/release/mayeuta-llm`: ELF 64-bit LSB pie, x86-64. Gitignored.

## CA3 tests

```text
cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge -- resolve_mayeuta_llm -- --test-threads=1
# 4 passed; 0 failed (resolve_mayeuta_llm_prefers_env_override_elf, prefers_release_over_debug, missing_emits_literal_trace, override_rejects_non_elf)
```

## CA4 lanzador

```bash
SDDIA_KALMA2_BRIDGE_BIN=/bin/true SDDIA_MAYEUTA_LLM_BIN=/tmp/not-elf \
  SddIA/scripts/daemons/kalma2-bridge.sh
```

exit 0. stderr: `[WARN] SDDIA_MAYEUTA_LLM_BIN no es ELF nativo … Compilar: cd SddIA && cargo build --release -p mayeuta-llm`. Órgano no tumba.

## CA6 probe

`POST http://127.0.0.1:8765/api/chat` `{"prompt":"sello 64f37c7f7b34"}` `-m 5`: curl 28, 0 bytes, **no** HTTP 500, **no** traza literal. El 500 de este sello es inmediato (pre-spawn); timeout implica prótesis resuelta y spawn. No se exigió combustión LLM.

## Fuera del diff

`interfaces/kalma2/app.js`, `.SddIA/observability/ecosystem-health.json`, PBI operativo 503 Gemini.
