---
feature_name: execute-process-phase-failure-propagation
created: "2026-08-13"
process: bug-fix
branch_name: fix/execute-process-phase-failure-propagation
persist_ref: docs/fixes/execute-process-phase-failure-propagation
pbi_ref: docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
document_id: 04f8f435-450b-477a-970a-4a05dd0224cb
finding: EV-AUD-005
correlation_id: dcb9efed-2268-4298-8108-7a55cf4db323
phase: Estabilización
agent: mayeuta
---

# Clarify — auditoría del resultado Kalma2 (NO_APTO)

## Pregunta

¿Por qué el ciclo forjado vía Kalma2 (`correlation_id` `dcb9efed-2268-4298-8108-7a55cf4db323`) terminó en **`global: NO_APTO`** pese a materializar cascada documental e implementación EV-AUD-005?

## Hechos de runtime (sesión 2026-08-11 → relectura 2026-08-13)

| Hecho | Evidencia |
|-------|-----------|
| Emisión | `Kalma2_Process_Requested` → `bug-fix` + `pbi_ref` EV-AUD-005 |
| Doble despacho | Carrera: slug legado `execute-processfallo…` **y** canónico `execute-process-phase-failure-propagation` |
| Ciclo legado | Argos `NO_APTO` en árbol slug |
| Ciclo canónico | Argos `NO_APTO` en `docs/fixes/execute-process-phase-failure-propagation/validacion.md` |
| Terminal orquestación | Procesos `bug-fix`/TQM **muertos**; evento domain/PEC purgados → API Kalma2 `evento no encontrado` |
| PBI | Sigue en `docs/todos/pending/` |
| Post-facto (2026-08-13) | `cargo test -p execute-process --lib phase_terminal` → **13 passed** (incluye `t9_regression_62b201cf_*`) |

## Motivos del NO_APTO (Argos) — clasificación

### Bloqueantes (certificación de entrega)

1. **`CARGO_TEST_PHASE_TERMINAL` / `CA7`**  
   En el momento de la Verificación, Shell IDE **Rejected** `cargo test … phase_terminal`. Sin stdout físico, Argos no pudo certificar la regresión `62b201cf`.  
   *Nota 2026-08-13:* el verde físico ya es reproducible en host; falta **re-auditoría Argos** que lo cite (no basta esta nota).

2. **`SCOPE_WIP_CONTAMINATION`**  
   El working tree mezclaba diffs sin causalidad EV-AUD-005: instrumentación/debug `kalma2.rs`, TQM `suggested_branch`, `event-watcher` async, `kalma2-bridge`/`app.js` (poll awaiting_agents), docs `evolution-contract-index-v11`, `.cursor/debug-*.log`.  
   Spec/plan prohíben esa mezcla → no se puede sellar un PR “puro” EV-AUD-005.

3. **`PERSIST_REF_DUAL_TREE`**  
   Mismo `document_id` / `correlation_id` en dos árboles:
   - canónico: `docs/fixes/execute-process-phase-failure-propagation/`
   - legado: `docs/fixes/execute-processfallodefasedebefallarejecucinglobalev-aud-005/`  
   Causa: heurística/slug TQM previa a `suggested_branch` + re-despacho del mismo evento.

4. **`PBI_ARCHIVED`**  
   `pbi_archived: false` correcto (Argos no miente). El PBI no se movió a `done/` porque el veredicto no es APTO y la fase de cierre documental no corre sobre rechazo.

### No bloqueantes (no bastan solos para APTO)

5. **`GIT_EVIDENCE_SESSION_SHELL`** — Shell Rejected sobre `git-manager`; R2 canónico vía Evidence Bridge (`prosthesis_subprocess`) sí APTO.  
6. **`CA3b_failsoft_phase_field`** — helper honra `fail_soft:true` en report (forward-compat) mientras L1.5 del spec exige contrato genómico; desviación documentada, no bloquea la lógica CA1–CA5 estática.

## Motivos de fricción Kalma2 (orquestación, no lógica EV-AUD-005)

Estos **no** son el hallazgo EV-AUD-005, pero explican por qué la UX y el cierre salieron turbios:

| Fricción | Efecto | Estado remedio |
|----------|--------|----------------|
| Poll UI cortaba en `initialized` | Operador veía “inicializado” y el sondeo paraba | Fix local `interfaces/kalma2/app.js` + early PEC `awaiting_agents` (**WIP ajeno** al scope puro) |
| Watcher síncrono | Evento domain huérfano tras saturación pending | Fix async `event-watcher` (**WIP ajeno**) |
| Slug sin `suggested_branch` | Dual persist_ref | Fix TQM (**WIP ajeno**) |
| Doble `route-domain` | Dos `bug-fix` concurrentes mismo `correlation_id` | Deuda: idempotencia / single-flight TQM |

## Conclusión Mayeuta

El **NO_APTO no niega** el arreglo de agregación terminal (CA1–CA6 estáticos APTO en `validacion.md`). Niega la **certificabilidad de la entrega** en esa sesión: falta evidencia de tests en el acto de Argos, hay contaminación de alcance y dualidad documental.

**Pendientes normativos** → ver `spec.md` § Pendientes post-Verificación (CA7-R, CA8–CA11).

## Decisiones para la siguiente ola

1. Separar PR/commit: **solo** touchpoints EV-AUD-005 (`phase_terminal` + consumers) vs ola Kalma2 (poll/watcher/TQM).  
2. Borrar o archivar árbol slug legado.  
3. Re-ejecutar Argos con stdout de `cargo test -p execute-process --lib phase_terminal`.  
4. Cierre documental en rama solo tras `global: APTO`.

## Tareas Kalma2 registradas

PBI operativo: `docs/todos/pending/[OPERATIVO] Kalma2 — ola mejora post-auditoría EV-AUD-005 (K1–K5).md` (`document_id: b2e4c891-3f7a-4d2e-9c1b-8a5f6e0d2c47`).

Procedimiento de retoma (vías A/B): `procedimiento-retoma.md` en este `persist_ref`.
