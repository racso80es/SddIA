---
document_id: AUDIT-KALMA2-PBI-TELEGRAM-PEC-SILENCE-20261013
uuid: "1c0bfdc5-6213-47ac-b13f-517ff9b65b87"
title: "Auditoría — prueba Kalma2 del PBI Telegram (silencio PEC y síntesis)"
created: "2026-10-13"
updated: "2026-10-13"
process: audit
verdict: CICLO_PARCIAL_MUTACION_EN_MAIN_UI_CONGELADA
pbi_ref: "docs/todos/pending/[KAIZEN] Telegram — silencio PEC accept-pr y PPR, síntesis de valor en castellano.md"
document_id_pbi: PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES
correlation_id: "09a61ea3-b2d9-43c3-b65d-b815385e9a22"
execution_id_handoff: "3c792bba-cb4e-41d5-9d9e-02201cf6ebd5"
execution_id_tekton_docs: "766e1932-9d7c-4d72-a0bf-fdcd4110a19f"
persist_ref: docs/fixes/kaizen-telegram-pec-silence-value-es
proposed_followup: bug-fix
proposed_followup_id: PBI-FIX-KALMA2-BRANCH-AND-STATUS
proposed_followup_ref: "docs/todos/pending/[FIX] Kalma2 — rama antes de mutar y aviso de avance de agentes.md"
related:
  - docs/fixes/kaizen-telegram-pec-silence-value-es/_agent_handoff.md
  - docs/fixes/kaizen-telegram-pec-silence-value-es/spec.md
  - docs/fixes/kaizen-telegram-pec-silence-value-es/plan.md
  - docs/fixes/kaizen-telegram-pec-silence-value-es/implementation.md
  - docs/fixes/kaizen-telegram-pec-silence-value-es/execution.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - interfaces/kalma2/app.js
  - SddIA/engine/execute-process/src/engine/progress_trace.rs
  - SddIA/library/norms/progress-trace-contract.md
---

# Auditoría — prueba Kalma2 del PBI Telegram

Fecha de corte: 2026-10-13 17:20 CEST (relectura tras el aviso de que la WUI sigue en la notificación inicial). HEAD git: `main` (`36414ec6`, `origin/main`). La rama `fix/kaizen-telegram-pec-silence-value-es` no existe.

## 0. Veredicto

El despacho Kalma2 arrancó y los agentes escribieron diseño y código. La prueba **no** quedó cerrada en el ciclo automático: Ejecución terminó `blocked`, la verificación no arrancó y el proof final es `failed`.

Comportamiento no esperado: las mutaciones cayeron sobre `main`. El primer paso de `feature`, `bug-fix` y kaizen que trabaja en rama tiene que ser crear esa rama. Aquí el checkout no se ejecutó y los agentes siguieron.

La WUI siguió en el texto inicial `awaiting_agents` mientras el ciclo ya había avanzado. Es el mismo `bug-fix`, no una refactorización.

## 1. Estímulo

| Campo | Valor |
|-------|--------|
| Superficie | Kalma2 WUI, botón Forjar Proceso, desplegable Core (sin proyecto) |
| Proceso hijo | `bug-fix` |
| PBI | `PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES` |
| `pbi_ref` | `docs/todos/pending/[KAIZEN] Telegram — silencio PEC accept-pr y PPR, síntesis de valor en castellano.md` |
| `correlation_id` | `09a61ea3-b2d9-43c3-b65d-b815385e9a22` |
| `execution_id` (handoff) | `3c792bba-cb4e-41d5-9d9e-02201cf6ebd5` |
| `execution_id` (frontmatter Tekton) | `766e1932-9d7c-4d72-a0bf-fdcd4110a19f` |
| `persist_ref` | `docs/fixes/kaizen-telegram-pec-silence-value-es` |
| Acuse WUI | `intención aceptada; consultar GET /api/status` |
| Texto que permanece | `[estado: awaiting_agents] Ciclo «bug-fix» en espera de agentes IDE.` más `proceso=bug-fix status=success cycle_phase=awaiting_agents` |

## 2. Línea de tiempo (UTC)

Fuentes: `_agent_handoff.md`, PTC en `.events/progress/09a61ea3-b2d9-43c3-b65d-b815385e9a22/` (18 hojas al cierre), proof `.SddIA/proofs/pec-correlation/09a61ea3-b2d9-43c3-b65d-b815385e9a22.json`, `execution_report.json` de `3c792bba-…` y `f0d9a570-…`.

| Hora Z | Hecho | Superficie |
|--------|--------|------------|
| 15:09:25 | Inicio y fin `failed` de Inicialización de Espacio de Trabajo. Mismo segundo: inicio Diseño del fix | PTC |
| 15:11:58 | Fin Diseño `executed` (Dedalo). Inicio Ejecución (Tekton) | handoff + PTC |
| 15:19:31 | Fin Ejecución `blocked` (Tekton) | handoff + PTC `severity=error` |
| 15:19:35 | Segunda ráfaga sobre el mismo `correlation_id`: inicio Inicialización, fin `failed`, inicio Diseño. Proof PEC reescrito a `cycle_phase=awaiting_agents`, `status=success`, `pec_event_id=d2d6cfea-e844-4de2-862d-6d2986649d30` | PTC + proof |
| 15:21:58 | Tercera inicialización: inicio y fin `failed` (`execution_id` `f0d9a570-…`) | PTC + `execution_report.json` |
| 15:22:57 | Fin Diseño `executed` (reasello) | handoff + PTC |
| 15:23:52 | Fin Ejecución `blocked`. Proof PEC final `status=failed`, `cycle_phase=failed`, `pec_event_id=85ee1f7d-09c8-4ab9-9600-c61c6ba94618` | handoff + proof |

Los directorios `.SddIA/workspaces/bug-fix/3c792bba-…` y `…/766e1932-…` están vacíos: no hay `execution_report.json` de este ciclo en disco.

## 3. Qué materializaron los agentes

### 3.1. Dedalo — `executed`

`spec.md` y `plan.md` sellados. `objectives.md` no existía; semilla = cuerpo del PBI.

- L1: silencio en `build_telegram_message_from_event` para `(accept-pr, success)` y `(pull-request-review, failed)`.
- L2: prompt en castellano y omisión explícita tipada.
- TG-CA7: input de idioma **descartado**. Sin mutar `gemini-http-infer`.

### 3.2. Tekton — `blocked`

Diff sin commit, sobre `main`:

| Archivo | Delta |
|---------|--------|
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | `pec_telegram_silent` + tests `tg_ca1_*` / `tg_ca2_*` / `tg_ca3_*` |
| `SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs` | prompt «en castellano», `SynthesisOutcome`, líneas `(omitida — inferencia no disponible)` y `(omitida — texto vacío)` |
| `docs/fixes/kaizen-telegram-pec-silence-value-es/` | `spec.md`, `plan.md`, `implementation.md`, `execution.md`, `_agent_handoff.md` (untracked) |
| PBI pending | untracked |
| `.SddIA/observability/ecosystem-health.json` | modificado; ajeno a este PBI |

Causa del bloqueo (handoff y `execution.md`): Shell IDE `Rejected`. No hubo `cargo test` ni `git-manager checkout` de `fix/kaizen-telegram-pec-silence-value-es`. Genoma de la acción no alineado (fase opcional, sin `entity-manager`). `validacion.md`, paso del PBI a `done/` y `delivery-close-cycle` no iniciados.

La lectura del código confirma los símbolos. No sustituye la batería: los tests no se han ejecutado en esta auditoría.

## 4. Mutación no esperada en `main`

HEAD durante todo el ciclo automático: `main` (`36414ec6`). La ref `fix/kaizen-telegram-pec-silence-value-es` no existía. Aun así quedaron modificados, sin commit:

- `SddIA/engine/execute-process/src/engine/route_domain_core.rs`
- `SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs`
- `docs/fixes/kaizen-telegram-pec-silence-value-es/` (untracked)
- el PBI en `docs/todos/pending/` (untracked)

`.SddIA/observability/ecosystem-health.json` ya estaba sucio y es ajeno al PBI.

### 4.1. El checkout nunca llegó a ejecutarse

`workspace-init` (`workspace_init.rs`) hace el gate `dirty_paths_outside_scope` **antes** del `checkout` con `create_if_not_exists: true`. Si hay suciedad fuera de `persist_ref` y `pbi_ref`, retorna error y no crea la rama.

Primera pasada (`3c792bba-…`), error literal:

```text
dirty-worktree: cambios fuera de persist_ref/pbi_ref: .SddIA/observability/ecosystem-health.json
```

Tercera pasada (`f0d9a570-…`), el propio código ya escrito en `main` entra en el gate:

```text
dirty-worktree: cambios fuera de persist_ref/pbi_ref: .SddIA/observability/ecosystem-health.json, SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs, SddIA/engine/execute-process/src/engine/route_domain_core.rs
```

### 4.2. La fase fallida no cortó a los agentes

El barrier de `executor.rs` (`agent_phase_blocks_downstream`) solo mira fases de agente (`failed` / `blocked` / `awaiting_agents`). Una inicialización nativa `failed` no lo arma. Dedalo y Tekton corrieron con HEAD en `main` y escribieron ahí.

Regla que el ciclo no cumplió: en `feature`, `bug-fix` y kaizen, si el trabajo va en rama, el primer paso es crearla. Ninguna mutación de código o de `persist_ref` puede ocurrir en la rama base cuando ese checkout no ha tenido éxito.

## 5. Por qué la WUI no se movió

El operador ve la caja **Respuesta**. `pollStatus` en `interfaces/kalma2/app.js` la reescribe en cada sondeo con `GET /api/status`. Solo sale del bucle si el status es `completed` o `failed`. `awaiting_agents` alarga el plazo a 30 min y vuelve a pintar la misma frase.

`project_status` en `kalma2-bridge` (`main.rs`) decide esa frase solo con el PEC de correlación. Si `status=success` y `cycle_phase=awaiting_agents`, el mensaje es fijo: `Ciclo «bug-fix» en espera de agentes IDE.` No lee las hojas PTC.

Entre medias, el proof de las 15:19:35Z (PEC temprano de reentrada, `awaiting_agents` / `success`) pisó el `blocked` de las 15:19:31Z. La caja no pudo mostrar «Diseño executed» ni «Ejecución blocked». El proof final, a las 15:23:52Z, ya es `failed` / `cycle_phase=failed`; la WUI que el operador miraba seguía anclada al texto inicial.

La consola de progreso es otro canal (`GET /api/progress/stream`, replay de `./.events/progress/{correlation_id}/`). Al corte de las 15:23:52Z hay 18 trazas. Su texto, armado en `progress_trace.rs` `build_message`, es solo `Inicio fase: …` / `Fin fase: … (status=…)`. No incluye el veredicto del handoff («Listo para Tekton», «Shell IDE Rejected», mutación en `main`). Aunque la consola pinte las trazas, el operador no recibe el avance útil.

## 6. Deuda — un bug-fix para todas las fricciones

No es refactorización. El canal PTC y el checkout de `workspace-init` ya existen; fallan el orden, el corte y la proyección.

Artefacto: `docs/todos/pending/[FIX] Kalma2 — rama antes de mutar y aviso de avance de agentes.md` (`PBI-FIX-KALMA2-BRANCH-AND-STATUS`).

| ID | Fricción |
|----|----------|
| F-BRANCH-NOT-FIRST | El checkout de la rama de trabajo va después del gate dirty y no llega a ejecutarse. El primer paso de `feature` / `bug-fix` / kaizen con rama tiene que ser crearla. |
| F-INIT-FAIL-CONTINUES | Inicialización `failed` no arma el barrier. Los agentes mutan la rama base. |
| F-WORK-ON-MAIN | Diff de motor y `persist_ref` quedaron sobre `main`. |
| F-KALMA2-STATUS-STALE | `GET /api/status` se queda en el PEC `awaiting_agents` o lo restaura un PEC temprano posterior. |
| F-KALMA2-AGENT-VERDICT-MUTE | El mensaje PTC no lleva el veredicto del agente. La caja Respuesta no lo muestra. |
| F-REENTRY-SAME-CID | Tres pasadas sobre `09a61ea3-…` (`3c792bba`, reentrada 15:19:35Z, `f0d9a570`) sin rama. |

## 7. Estado al corte de la prueba automática

| Pieza | Estado |
|-------|--------|
| HEAD del ciclo automático | `main` |
| Rama de trabajo | ausente durante el ciclo |
| Diseño | `executed` (dos veces) |
| Ejecución | `blocked` |
| Verificación / cierre | `skipped` (barrier por `blocked`) |
| WUI Respuesta | congelada en el aviso inicial durante la prueba |
| Proof PEC final | `failed` / `failed` a las 15:23:52Z |
| `execution_id` | `3c792bba-…`, `766e1932-…` (docs Tekton), `f0d9a570-…` |
