---
feature_name: kaizen-runtime-binary-parity-gate
created: "2026-10-29"
process: feature
branch_name: feat/kaizen-runtime-binary-parity-gate
persist_ref: docs/features/kaizen-runtime-binary-parity-gate
pbi_ref: MAN-25
execution_id: "b4bc0b99-532a-4faa-8e57-dcd957d39b45"
---

# Especificación — paridad binario/fuente (MAN-25)

## Componentes

| ID | Ubicación | Comportamiento |
|----|-----------|----------------|
| KZ-CA1 | `start-sddia.sh` | `--seal-capsules` release sin silenciar stderr/exit |
| KZ-CA2 | `kalma2-bridge` `accept_execute` | `409` + `STALE_RUNTIME_BINARY` si testigo ≠ fuente/ELF; bypass `SDDIA_ALLOW_STALE_RUNTIME=1` |
| KZ-CA3 | `sddia-ecosystem-health` | Campo `runtime_binary_parity`; `map_status: DEGRADED` si `ok: false` |
| KZ-CA4 | `execute-process` `runtime_recycle` | Tras merge en `accept-pr`, rebuild+seal+try-restart si diff toca órganos |
| KZ-CA5 | `pull-request-review` triaje técnico + `validacion_runtime_parity.rs` |

## Crate compartido

`sddia-runtime-parity`: lectura de testigo `.sha256`, digest de fuente alineado a `capsule_digest`, informe `stale[]`.
