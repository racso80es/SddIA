---
feature_name: f0f1b1ec
created: "2026-07-20"
process: feature
base: main
scope: no-op-reinit-closed-pbi
version_spec: "1.0.0"
uuid: f0f1b1ec-4b79-47c6-85e2-a0ac2ca3164b
canonical_feature_name: kalma2-llm-live
canonical_persist_ref: docs/features/kalma2-llm-live
canonical_spec: docs/features/kalma2-llm-live/spec.md
branch_name: feat/f0f1b1ec
persist_ref: docs/features/f0f1b1ec
correlation_id: 10c3fdf2-70d5-48b4-ab76-2833e97d2a46
status: dedalo_locked_noop
blueprint_required: false
verdict: blocked
---

# Especificación — f0f1b1ec (ciclo lab / re-init)

## 1. Laudo Dedalo

Consumido `objectives.md` como `refined_requirements`.

| Decisión | Valor |
|----------|--------|
| D-NOOP | No hay especificación de producto nueva que forjar |
| D-CANON | SSOT técnico permanece en `docs/features/kalma2-llm-live/spec.md` (PBI Done, `validacion.md` APTO) |
| D-BLUEPRINT | `blueprint_required: false` — no se instancia plan de implementación de feature |
| D-TEKTON | Prohibido mutar genoma / interfaces / UI / prótesis en este `persist_ref` |
| D-VERDICT | **blocked** (sin alcance de forja; no es fallo de diseño ambiguo) |

## 2. Topología (solo remisión)

No se redefine. Cascada canónica vigente:

```text
docs/features/kalma2-llm-live/
  clarify.md · objectives.md · spec.md · plan.md
  implementation.md · execution.md · validacion.md (global: APTO)
```

Ciclo lab `docs/features/f0f1b1ec/` = estabilización Mayeuta + este laudo Dedalo. **No** sustituye el alias canónico `kalma2-llm-live` (L-ALIAS).

## 3. Contratos / touchpoints

| Ámbito | Cambio en este ciclo |
|--------|----------------------|
| `kalma2-bridge` SSE / `/api/chat` | Ninguno |
| `mayeuta-llm` | Ninguno |
| Prótesis Cursor Python | Ninguno |
| UI Kalma2 Chat/Forjar | Ninguno |
| Core `SddIA/` genoma | Ninguno |
| Documental lab `persist_ref` | `clarify.md` + `objectives.md` + este `spec.md` + `plan.md` (omisión de forja) |

## 4. Criterios de aceptación

AC1–AC9 + host live + deuda §11: **heredados y APTOs** en cascada canónica. No se re-evalúan ni se inventan AC nuevos sin PBI abierto.

## 5. Remisiones fuera de jurisdicción

| Ítem | Destino |
|------|---------|
| Merge PR #123 | Operador |
| Fractura SSE bridge / prótesis exit 1 | `bug-fix` PBI `cbe0c30b3695` |
| Nueva capacidad producto | Semilla Racso + PBI abierto distinto (UUID nuevo) |

## 6. Vacío / ambigüedad

Ninguno sobre requisitos de producto. El pack Mayeuta es suficiente: el requisito termodinámico es explícitamente **no-op**. No se escala a Mayeuta por ambigüedad; se cierra diseño con veredicto **blocked**.
