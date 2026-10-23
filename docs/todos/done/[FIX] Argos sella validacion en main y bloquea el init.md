---
document_id: PBI-FIX-ARGOS-VALIDATION-ON-MAIN
uuid: "f126631d-3d98-43c4-b251-911410e06001"
title: "[FIX] Argos escribe validacion.md en main y el init rechaza el árbol"
format: markdown
version: "1.0.0"
created: "2026-10-23"
status: done
priority: alta
process: bug-fix
dispatch: true
suggested_branch: fix/argos-validation-on-main
persist_ref: docs/fixes/argos-validation-on-main
source_audit: docs/features/tracker-lifecycle-sink/validacion.md
related:
  - docs/features/tracker-lifecycle-sink/validacion.md
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - docs/todos/pending/[ARQUITECTURA] HU-003 PBI-05 — tracker lifecycle.md
---

# [FIX] Argos escribe validacion.md en main y el init rechaza el árbol

## 1. Hechos

Re-auditoría Argos `2026-10-23T08:14Z` en `docs/features/tracker-lifecycle-sink/validacion.md` (`global: NO_APTO`). El fichero está modificado en el trabajo y `HEAD` es `main`. A las 08:15 el `refactorization` `6959110a-3bf0-46f9-aa02-62d48ec9793a` murió en `workspace-init`:

```text
dirty-worktree: cambios fuera de persist_ref/pbi_ref: docs/features/tracker-lifecycle-sink/validacion.md
```

El gate está en `workspace_init.rs`: cualquier path fuera de `persist_ref` y `pbi_ref` aborta el checkout. El comportamiento del gate es el esperado. La fricción es el artefacto de verificación escrito en la rama equivocada.

Causas selladas por Argos, sin añadir otras:

| ID | Hecho | Qué no es |
|---|---|---|
| F1 | `process` / `phase` / `execution_id` / `correlation_id` / `branch` del handoff llegaron vacíos. Argos infirió el contexto y escribió `validacion.md` sobre el checkout actual (`main`). | No es un fallo del gate `dirty-worktree`. |
| F2 | Ref local `feat/tracker-lifecycle-sink` ausente. Remoto `origin/feat/tracker-lifecycle-sink` @ `91ef2e8cb3ad4682c1cac56180c76772eb1588c6`. | La rama no está borrada en origin. |
| F3 | `cargo test` y `git-manager` no tienen stdout: el shell de la sesión fue rechazado por Auto-review. T14 y T16–T19 quedan `NO_EVIDENCIA`. | No hay fallo de cápsula observado. No se declara un test rojo. |
| F4 | T20: `/home/racso/Proyectos/SddIA_AP` no existe en este disco. El PBI-05 prohíbe fabricar su manifiesto aquí y prohíbe cerrar la tarea sin ese checkout. | Este fix no crea `SddIA_AP` ni altera `.SddIA/project.md` del Core. |

## 2. Corte

1. Si el handoff de Verificación no trae `branch` o `execution_id`, Argos no escribe `validacion.md` en el árbol de la rama actual. Emite fallo de handoff y no deja un diff en `main`.
2. Con `branch` presente y ref local ausente, el sello ocurre tras checkout de esa rama (`skill:git-manager`), no sobre `main`.
3. `workspace-init` sigue rechazando paths ajenos a `persist_ref` / `pbi_ref`. No se amplía el allowlist para `docs/features/**`.
4. T14/T16–T19 se re-auditan solo con stdout de los tests ya nombrados en el blueprint de `validacion.md`. Sin stdout siguen `NO_EVIDENCIA`.
5. T20 queda fuera de este fix. Lo cierra un laudo sobre el AC o un checkout real de `SddIA_AP`.

## 3. Criterios de aceptación

| ID | Criterio |
|---|---|
| FX-CA1 | Una verificación con handoff vacío no modifica `docs/features/tracker-lifecycle-sink/validacion.md` en `main`. |
| FX-CA2 | Con `branch_name` resuelto y ref local ausente, el `validacion.md` queda en esa rama. `main` no muestra ese diff. |
| FX-CA3 | `refactorization` con otro `persist_ref` ya no falla por este fichero cuando `main` está limpio de ese sello. |
| FX-CA4 | El mensaje `dirty-worktree` no cambia de contrato. Un path ajeno sigue abortando. |
| FX-CA5 | T20 y el manifiesto de Paciente 0 no se tocan. |

## 4. Fuera de alcance

- Reescribir el veredicto `NO_APTO` de T20.
- Marcar T14/T16–T19 como APTO sin ejecución.
- Despachar `PBI-KAIZEN-TRACKER-PROCESS-DECOUPLE` en el mismo ciclo.
- Mutar genoma fuera de `docs/fixes/argos-validation-on-main/` + `entity-manager`.
