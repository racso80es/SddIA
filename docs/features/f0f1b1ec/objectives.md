---
feature_name: f0f1b1ec
created: "2026-07-20"
process: feature
branch_name: feat/f0f1b1ec
persist_ref: docs/features/f0f1b1ec
document_id: PBI-KALMA2-LLM-LIVE-V2
uuid: f0f1b1ec-4b79-47c6-85e2-a0ac2ca3164b
canonical_feature_name: kalma2-llm-live
canonical_persist_ref: docs/features/kalma2-llm-live
pbi_ref: docs/todos/done/[FEATURE] kalma2-llm-live — ejecución real Cursor desde Kalma2 (f0f1b1ec).md
pbi_status: done
correlation_id: 10c3fdf2-70d5-48b4-ab76-2833e97d2a46
depends_on:
  - docs/features/kalma2-full-cycle
status: stabilized_no_new_scope
verdict: blocked
---

# Objetivos — f0f1b1ec

## Misión (requisito termodinámico)

**No hay feature nueva que forjar.** El PBI `PBI-KALMA2-LLM-LIVE-V2` (`f0f1b1ec-4b79-47c6-85e2-a0ac2ca3164b`) está en `docs/todos/done/` con alcance S+ Grade ya entregado y validado (`docs/features/kalma2-llm-live/validacion.md` → `global: APTO`, `pbi_archived: true`).

Este ciclo lab (`feat/f0f1b1ec` · `persist_ref: docs/features/f0f1b1ec`) es un **re-init sobre UUID archivado**. El requisito estable para Dedalo/Tekton es **no-op de producto**.

## Por qué (semilla)

La intención cruda «inicia feature … (f0f1b1ec).md» apuntaba a `docs/todos/pending/…`, ruta inexistente: el artefacto vive en `done/` (v2.3.3). Reabrir el mismo UUID como feature nueva viola L-CLOSED (ver `clarify.md`).

## Alcance permitido en este ciclo

1. Documentar estabilización Mayeuta (`clarify.md` + este `objectives.md`) bajo el `persist_ref` del init.
2. Remitir trabajo residual:
   - **Merge** PR #123 → operador.
   - **Regresión/fractura** SSE bridge → proceso `bug-fix` / PBI `cbe0c30b3695` (no este feature).

## Alcance prohibido

- Re-diseñar o re-forjar SSE, `mayeuta-llm`, prótesis Cursor, UI Chat/Forjar, ECST/TQM, secretos.
- Tratar `docs/features/f0f1b1ec` como sustituto canónico de `docs/features/kalma2-llm-live`.
- Inventar criterios de aceptación nuevos sin PBI abierto.

## Criterios de aceptación (heredados — ya APTOs)

AC1–AC9 + host live + deuda §11: vigentes en cascada canónica `docs/features/kalma2-llm-live/`. No se re-evalúan aquí.

## Restricciones

- Git vía `skill:git-manager`.
- Cascada `features-documentation-pattern`.
- Core agnóstico; Foso Python desechable.
- Veredicto fase Mayeuta: **blocked** (sin alcance de forja). Handoff a Dedalo solo si Racso emite nueva semilla con PBI abierto distinto.
