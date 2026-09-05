---
document_id: PBI-FIX-FRACTURE-7bc20a6b4dd6
uuid: "db46c34e-4c2d-42dd-b2e1-36230853f23c"
title: "[FIX] system-vitality-probe — fractura sistémica (sddia-qa ausente en sonda cumulo.tools_index)"
format: markdown
version: "1.2.0"
created: "2026-09-04"
updated: "2026-09-05"
status: cerrado
closed: "2026-09-05"
fix_ref: docs/fixes/system-vitality-probe-7bc20a6b4dd6
refinement_status: refinado
priority: alta
process: bug-fix
type: fix
dispatch: false
laudo: B
laudo_kind: deuda-documental-no-regresion
fracture_hash: 7bc20a6b4dd6
fracture_process: system-vitality-probe
incident_ref: "System_Fracture_Detected — 7bc20a6b4dd6"
friction_ids:
  - F-VITALIDAD-CUMULO-TOOLS_INDEX
physical_fix_commit: "ab272346cd77a9cb9fd320d177179086409ae6ce"
physical_fix_feature: docs/fixes/dcc-sddia-qa-lab
physical_fix_pr: "https://github.com/racso80es/SddIA/pull/251"
physical_fix_pr_title: "fix(ignition): cápsulas DCC, F4b lab y correlato evolution"
segregated_in: docs/fixes/centinelas-fracture-ola-20260901/spec.md
suggested_branch: fix/system-vitality-probe-7bc20a6b4dd6
persist_ref_suggested: docs/fixes/system-vitality-probe-7bc20a6b4dd6
parent_pbi: docs/todos/done/[OPERATIVO] Latido Ontológico (System Heartbeat).md
related_pbis:
  - docs/todos/done/[FIX] delivery-close-cycle — Ola 2 sddia-qa (ca3d901fdc9a).md
architectural_constraints:
  - A-VITALITY-NO-PROCESS-MUTATION
  - A-QA-BIN-SSOT-TARGET
  - A-IGNITION-PRODUCE-QA
  - A-NO-GENOME-MUTATION-THIS-CYCLE
execution_file_lock: []
gates_this_wave:
  - VITALITY-DOC-CA1
  - VITALITY-DOC-CA2
  - VITALITY-DOC-CA3
  - VITALITY-DOC-CA4
  - VITALITY-DOC-CA5
  - VITALITY-DOC-CA6
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/events/telemetry/system-vitality-probed.md
  - SddIA/process/system-vitality-probe.md
  - SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs
  - start-sddia.sh
  - docs/fixes/dcc-sddia-qa-lab/spec.md
  - docs/fixes/centinelas-fracture-ola-20260901/spec.md
  - docs/todos/done/[FIX] delivery-close-cycle — Ola 2 sddia-qa (ca3d901fdc9a).md
source_audit: "2026-09-05. Causa física (omisión cargo -p sddia-qa en start-sddia.sh) subsanada en ab27234, mergeada en PR #251 (fix/ignition-pre-push-guard), no en #248/#249. Sonda cumulo.tools_index verde en runtime local. Deuda de este ciclo: reconciliación documental + no-regresión + archivo PBI."
review_notes: "v1.1.0 (worktree, no commit) atribuyó el fix a PR #248/#249 y marcó CA APTO con PBI aún en pending. v1.2.0 corrige linaje GitHub, husos horarios y CAs de este ciclo."
---

# [FIX] system-vitality-probe — fractura sistémica (sddia-qa ausente en sonda cumulo.tools_index)

## 0. Auditoría de alucinaciones

Contraste stub Cúmulo **v1.0.0** (HEAD) y borrador **v1.1.0** (worktree, no commitido) contra git, GitHub y runtime local `2026-09-05`.

### 0.1 Stub v1.0.0 (Cúmulo / Mayeuta)

| # | Tesis del stub | Hecho verificado | Dictamen |
|---|----------------|------------------|----------|
| **H1** | Causa raíz no clasificada; mandato `process_fix` sobre `system-vitality-probe`, acción homónima y `argos`. | Traza: `sonda cumulo.tools_index en rojo: sddia-qa ausente: …/SddIA/target/debug/sddia-qa`. Handler `qa_bin()` + `probe_tools_index` reportan ausencia de ELF. `start-sddia.sh` en `cf323cd` no incluye `-p sddia-qa`. | **Falso positivo de proceso.** Sonda íntegra. Causa ambiental: ignición no compilaba `sddia-qa`. Prohibido mutar `SddIA/process/system-vitality-probe.md` ni `system_vitality.rs`. |
| **H2** | Fractura viva; «corregir la causa raíz del colapso». | ELF `SddIA/target/{debug,release}/sddia-qa` ejecutables. `verify-tools-index: OK`. Estado local `cumulo.tools_index.verdict=green`. | **Causa física ya resuelta.** Este ciclo es Laudo B (documental / no-regresión). |
| **H3** | Sin genealogía. | `docs/fixes/centinelas-fracture-ola-20260901/spec.md`: 5 PBI lock huérfano consolidados; `7bc20a6b4dd6` **excluido** («Ciclo aparte»). Ola 2 `dcc-sddia-qa-lab` cerró el síntoma DCC (`ca3d901fdc9a-OLA2`) y no archivó este PBI. | **Linaje restituido.** No fusionar `document_id`. |

### 0.2 Borrador v1.1.0 (worktree) — inexactitudes propias

| # | Tesis v1.1.0 | Hecho verificado | Dictamen |
|---|--------------|------------------|----------|
| **H6** | `resolved_by_pr: …/pull/248`. | PR **#248** = `fix(dcc): pr_title argv preflight` mergeado `2026-09-01T13:12:39Z`. `ab27234` pertenece a PR **#251** (`fix/ignition-pre-push-guard`, merge `2026-09-04T09:19:16Z`). | **Alucinación de PR.** Atribución canónica: `#251`. |
| **H7** | CA5 cita «PR #248 / PR #249». | PR **#249** = `feat: telemetria CI Job Failed via github-bridge-watcher` (`2026-09-01`). Ajeno. | **Ruido.** Eliminado. |
| **H8** | Marcas `10:06Z` / `10:54Z` sobre `cf323cd` / `ab27234`. | Commits en `+0200`: `cf323cd` = `2026-09-04T08:06:32Z`; `ab27234` = `2026-09-04T08:54:48Z`. | **Huso mal etiquetado.** Usar UTC o conservar offset. |
| **H9** | CA1–CA5 `APTO` con `status: abierto` y archivo a `done/` sin marcar. | CAs de *este* ciclo aún no ejecutados. Hechos de runtime son evidencia de no-regresión, no cierre. | **Cierre prematuro.** CAs de ola = VITALITY-DOC-CA1…CA6. |
| **H10** | Enlaces `file:///home/racso/Proyectos/SddIA/…`. | Viola agnosticismo del Core. | **Rutas lógicas** vía Cúmulo. |
| **H11** | `execution_file_lock` incluye `system_vitality.rs`. | Laudo B: cero mutación. El handler no es superficie de cambio. | **Lock vacío** este ciclo. |
| **H12** | Timeline 10:10Z sweeper / 10:12Z Cúmulo / 11:15Z ola centinelas. | No hay testigo de esos instantes en git. PR #252 merge `2026-09-04T09:49:05Z`. | **Tiempos no certificados.** Omitidos. |

H1 sobre «fallback ciego de Mayeuta» se sostiene en el texto del stub (`process_fix` no clasificado). No se afirma aquí un defecto residual en `enrich_fracture_pbi_kaizen.rs` (cubo latido de centinelas es otro PBI, ya cerrado).

---

## 1. Incidente

| Campo | Valor |
|-------|--------|
| Document ID | `PBI-FIX-FRACTURE-7bc20a6b4dd6` |
| Proceso emisor | `system-vitality-probe` |
| Agente | `argos` |
| Friction ID | `F-VITALIDAD-CUMULO-TOOLS_INDEX` |
| Fracture hash | `7bc20a6b4dd6` |
| Creación PBI | `2026-09-04` (stub Cúmulo) |
| Ventana de causa | post-`cf323cd` (`2026-09-04T08:06:32Z`): ignición cubre centinelas y omite `sddia-qa` |

### Traza

```
sonda cumulo.tools_index en rojo: sddia-qa ausente: /home/racso/Proyectos/SddIA/SddIA/target/debug/sddia-qa
```

La ruta absoluta es la del host del incidente. El handler resuelve `SddIA/target/debug/sddia-qa` relativo al repo; si no hay ELF en debug ni release, el fallback del mensaje es la ruta debug.

---

## 2. Estado empírico (`2026-09-05`, pre-ciclo)

1. `test -x SddIA/target/debug/sddia-qa` y `release/` → presentes.
2. `SddIA/target/debug/sddia-qa verify-tools-index` → `verify-tools-index: OK` (exit 0).
3. `.SddIA/daemons/state/vitality-probe.json`: `cumulo.tools_index.verdict = green` (también `bus.topology`, `cerbero.config`, `kalma2.http`).
4. `start-sddia.sh`: `sddia-qa` en `release_pkgs` y en el lote debug (`ab27234`; lote debug ampliado en `e548071` del mismo PR #251).

Re-verificar CA1 en ejecución; no dar el ciclo por cerrado con este snapshot.

---

## 3. Causa raíz y atribución

`qa_bin` consulta `SddIA/target/debug/sddia-qa` luego `release/`. Ausencia total → sonda roja. Correcto.

`ab27234` (`2026-09-04T08:54:48Z`, mensaje `delivery-close: snapshot final consolidado`) añade `sddia-qa` a ignición. Feature documental: `docs/fixes/dcc-sddia-qa-lab` (Ola 2 DCC). **PR canónico: [#251](https://github.com/racso80es/SddIA/pull/251)**.

Este PBI no implementa de nuevo ese diff. Archiva la fractura `7bc20a6b4dd6` y deja constancia de no-regresión.

---

## 4. Laudos

- **L-VITALITY-NO-MUTATION:** no mutar proceso, handler ni `start-sddia.sh` en este ciclo.
- **L-PREV-FIX-ATTRIBUTION:** causa física = `ab27234` / PR #251 / `dcc-sddia-qa-lab`.
- **L-LAUDO-B:** deuda documental + no-regresión.
- **L-NO-FUSION:** no absorber este `document_id` en la ola centinelas ni en Ola 2 DCC.

---

## 5. Alcance de este ciclo

| Hacer | No hacer |
|-------|----------|
| `persist_ref` `docs/fixes/system-vitality-probe-7bc20a6b4dd6/` (`spec.md`, `implementation.md`, `execution.md`, `validacion.md`) | Mutar genoma (`SddIA/process/`, handler, eventos, `start-sddia.sh`) |
| Archivar este PBI a `docs/todos/done/` con `fix_ref` | Fusionar con `ca3d901fdc9a` ni con ola `20260901` |
| Evolution `{uuid}.md` vía `sddia-qa evolution-register` | Marcar CA de CI `APTO` sin `run_id` verde |
| `plan.md` no se emite (sin blueprint de proceso) | Bypass raw `gh`/`git` para apertura de PR: usar `delivery-close-cycle` |

---

## 6. Criterios de aceptación (ola documental)

| ID | Criterio | Verificación |
|----|----------|--------------|
| **VITALITY-DOC-CA1** | No-regresión: ELF `sddia-qa` debug y release ejecutables; `verify-tools-index` OK; `system-vitality-probe` `verdict: ok` y `fractures_emitted: []`; estado `cumulo.tools_index` green. | `test -x`; CLI QA; `execute-process --process system-vitality-probe`; parseo de `.SddIA/daemons/state/vitality-probe.json` |
| **VITALITY-DOC-CA2** | Linaje: PR #251 + `ab27234` + `dcc-sddia-qa-lab`; cero mención de #248/#249 como resolución. | Frontmatter + spec |
| **VITALITY-DOC-CA3** | PBI en `docs/todos/done/` con `status: cerrado` y `fix_ref: docs/fixes/system-vitality-probe-7bc20a6b4dd6`. Conservar `document_id`. | Path + frontmatter |
| **VITALITY-DOC-CA4** | `validacion.md` `global: APTO`, `pbi_archived: true`, `branch` coherente, en el mismo PR que el código/docs. | Patrón v1.2.1 |
| **VITALITY-DOC-CA5** | Diff sin mutación genómica ni `start-sddia.sh`. | `git diff` vs base |
| **VITALITY-DOC-CA6** | Checks GitHub del PR verdes. | `run_id` o URL de run; hasta entonces `PENDIENTE-CI` (no `APTO`) |

---

## 7. Criterio de cierre

- [x] CA1–CA5 en rama (evidencia en `docs/fixes/system-vitality-probe-7bc20a6b4dd6/`).
- [x] PR vía `delivery-close-cycle`: https://github.com/racso80es/SddIA/pull/260
- [x] CA6 verde post-PR (`run_id` `33969304706`).
- [x] PBI en `done/` + `validacion.md` APTO en ese PR.
