---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
---

# Execution — kalma2-full-cycle

## Init

| Campo | Valor |
|-------|--------|
| `execution_id` | `956100c7-c03f-488b-af1e-2624f84bd0b0` |
| Rama | `feat/kalma2-full-cycle` |
| Skips lab | `SDDIA_LAB_SKIP_PBI_ARCHIVE` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE` |

## Verificación Slice A

| Comando | Resultado |
|---------|-----------|
| `cargo test -p execute-process derive_` | 5/5 OK (4 derive + 1 slug filter noise ok) |
| `cargo test -p kalma2-bridge project_status` | 4/4 OK |
| `cargo build -p execute-process -p kalma2-bridge` | OK |

## Deudas abiertas

| ID | Nota |
|----|------|
| B | Runtime Dedalo/Tekton/Argos post-init |
| C | Consumo cuerpo `pbi_ref` → `pbi_body` |
| Validacion Argos formal | Pendiente cuando se priorice cierre de A o A+B |
