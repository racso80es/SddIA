---
document_id: AUDIT-KALMA2-WUI-HU-B07-CORE-SELF-20261014
uuid: "45d94db5-219b-42c4-8131-4f97e262722f"
title: "Auditoría — prueba Kalma2 WUI del PBI HU-B 07 Core-self"
created: "2026-10-14"
updated: "2026-10-14"
process: audit
verdict: CICLO_PARCIAL_MUTACION_EN_RAMA_AJENA_UI_CONGELADA
pbi_ref: "docs/todos/pending/[ARQUITECTURA] Linear HU-B 07 — Manifiesto Core-self y activación done_gate linear.md"
document_id_pbi: PBI-LINEAR-B-07-CORE-SELF-GATE
correlation_id: "fd2db080-dbf3-44ab-b70f-63b90d748455"
execution_id: "5178e34f-8e17-4f93-a8d8-054496306053"
persist_ref: docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli
prior_audit: docs/audits/kalma2-pbi-telegram-pec-silence-20261013.md
prior_fix_pbi: PBI-FIX-KALMA2-BRANCH-AND-STATUS
prior_fix_ref: "docs/todos/done/[FIX] Kalma2 — rama antes de mutar y aviso de avance de agentes.md"
prior_fix_pr: "https://github.com/racso80es/SddIA/pull/347"
proposed_followup: bug-fix
proposed_followup_id: PBI-FIX-KALMA2-REINCIDENCE-STALE-RUNTIME-AND-FAILED-PROJECTION
proposed_followup_ref: "docs/todos/pending/[FIX] Kalma2 — reincidencia PR 347 runtime fósil e init failed en status.md"
proposed_kaizen_id: PBI-KAIZEN-RUNTIME-BINARY-PARITY-GATE
proposed_kaizen_ref: "docs/todos/pending/[KAIZEN] Aduana de paridad binario-fuente para órganos vivos.md"
related:
  - docs/audits/kalma2-pbi-telegram-pec-silence-20261013.md
  - docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/_agent_handoff.md
  - docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/clarify.md
  - docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/objectives.md
  - docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/spec.md
  - docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/plan.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - interfaces/kalma2/app.js
  - SddIA/engine/execute-process/src/engine/progress_trace.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
---

# Auditoría — prueba Kalma2 WUI del PBI HU-B 07 Core-self

Fecha de corte: 2026-10-14 19:51 CEST (17:51:24Z). HEAD git: `docs/cerrar-pbi-kalma2-branch-before-mutate` (1 commit detrás de `origin/main`). La rama `feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli` no existe.

Continuación de `docs/audits/kalma2-pbi-telegram-pec-silence-20261013.md`. Aquella prueba dejó seis fricciones y el PBI `PBI-FIX-KALMA2-BRANCH-AND-STATUS` (PR #347, `status: done`, cerrado el 2026-10-14). Esta pasada usa el mismo gesto de operador (Forjar Proceso, Core sin proyecto) sobre otro PBI. El runtime que atendió el ciclo es `SddIA/target/debug/execute-process` (feature) y `SddIA/target/release/kalma2-bridge` (WUI, binario 18:00 CEST, pid 15196).

## 0. Veredicto

El despacho Kalma2 arrancó. Mayeuta y Dedalo escribieron el paquete documental. A las 17:48:44Z Tekton entró en Ejecución y ya había mutado `project_binding.rs` y `done_gate.rs`. La prueba no está cerrada: no hay rama de trabajo, la verificación no ha arrancado y el proof PEC sigue en `awaiting_agents` / `success` desde el segundo del acuse.

Comportamiento no esperado, el mismo que el 13 de octubre: la inicialización falló y los agentes siguieron. Las mutaciones no cayeron en `main`; cayeron en la rama que ya estaba en checkout (`docs/cerrar-pbi-kalma2-branch-before-mutate`), que es el cierre documental del propio fix.

La caja Respuesta de la WUI sigue en el texto inicial `awaiting_agents`. La consola de progreso quedó vacía. El puente no proyecta los fines de fase que ya están en disco.

## 1. Estímulo

| Campo | Valor |
|-------|--------|
| Superficie | Kalma2 WUI `http://127.0.0.1:8765/`, botón Forjar Proceso |
| Desplegable | Core (sin proyecto). `core` no está en la lista (`barcelonaxplorer`, `lab-done-gate`, `lab-done-gate-both`) |
| Perfil | `engineering`, `forge_allowed: true` |
| Prompt | `inicia feature docs/todos/pending/[ARQUITECTURA] Linear HU-B 07 — Manifiesto Core-self y activación done_gate linear.md` |
| Proceso hijo | `feature` |
| PBI | `PBI-LINEAR-B-07-CORE-SELF-GATE` |
| `pbi_ref` | `docs/todos/pending/[ARQUITECTURA] Linear HU-B 07 — Manifiesto Core-self y activación done_gate linear.md` |
| `correlation_id` | `fd2db080-dbf3-44ab-b70f-63b90d748455` |
| `execution_id` | `5178e34f-8e17-4f93-a8d8-054496306053` |
| `persist_ref` | `docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli` |
| Rama pedida | `feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli` |
| Acuse HTTP | `202` `accepted` en 3 ms. Cuerpo: `intención aceptada; consultar GET /api/status` |
| Texto que permanece | `[estado: awaiting_agents] Ciclo «feature» en espera de agentes IDE.` más `proceso=feature status=success cycle_phase=awaiting_agents` |

El evento de dominio `Kalma2_Process_Requested` lleva `process: feature` y el `pbi_ref` exacto, con corchetes y raya. El anclaje del path no falló.

El slug de 64 caracteres sale del título sin espacios ni acentos (`arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli`).

Antes del clic el worktree ya tenía cambios fuera de `persist_ref` y `pbi_ref` (cierre de `kalma2-branch-before-mutate`, `.SddIA/observability/ecosystem-health.json`, historias). La sonda de esta auditoría añadió trazas en `interfaces/kalma2/app.js`. Esa suciedad es ajena al PBI HU-B 07.

## 2. Línea de tiempo (UTC)

Fuentes: PTC en `.events/progress/fd2db080-dbf3-44ab-b70f-63b90d748455/` (7 hojas al corte), proof `.SddIA/proofs/pec-correlation/fd2db080-dbf3-44ab-b70f-63b90d748455.json`, `_agent_handoff.md`, `GET /api/status`.

| Hora Z | Hecho | Superficie |
|--------|--------|------------|
| 17:43:07 | `Kalma2_Process_Requested` en dominio | evento |
| 17:43:08 | Inicio y fin `failed` de Inicialización de Espacio de Trabajo. Mismo segundo: inicio Estabilización de Requisitos. Proof PEC sellado `cycle_phase=awaiting_agents`, `status=success`, `pec_event_id=f5317107-8718-4964-bb04-aa8d3090480c` | PTC + proof |
| 17:43:08+ | WUI: `pending` («consenso de suscriptores incompleto») y en el sondeo siguiente `awaiting_agents` | `GET /api/status` |
| 17:45:01 | Fin Estabilización `executed` (Mayeuta). Inicio Diseño de Blueprint (Dedalo) | handoff + PTC |
| 17:48:44 | Fin Diseño `executed` (Dedalo). Inicio Ejecución (Tekton) | handoff + PTC |
| 17:51:24 | Corte. Proceso `feature` pid 101739 vivo (8 min). Proof PEC sin reescritura. WUI sin cambio de frase. Consola de progreso: 0 líneas | proceso + WUI |

Una sola pasada sobre `fd2db080-…`. No hay reentrada del mismo `correlation_id`.

## 3. Qué materializaron los agentes

### 3.1. Mayeuta — `executed`

`clarify.md` y `objectives.md`. Laudo C-1…C-8: `project_root: .`, degradación sin `tracker_ref` hacia `Git`, D-I (a) sin migrar otros proyectos. Sin mutación de código en esa fase.

### 3.2. Dedalo — `executed`

`spec.md` (`status: dedalo_locked`, `branch_name` declarado y no creado) y `plan.md`. Orden T1 bind → T2 degrade → T3 manifiestos → T4 tests → T5 evolution/cierre. Handoff: «listo para Tekton». El mensaje PTC de fin de fase no lleva ese veredicto.

### 3.3. Tekton — en curso al corte

PTC: inicio de Ejecución, sin fin. Diff sin commit, sobre `docs/cerrar-pbi-kalma2-branch-before-mutate`:

| Archivo | Delta al corte |
|---------|----------------|
| `SddIA/engine/execute-process/src/engine/project_binding.rs` | modificado |
| `SddIA/engine/execute-process/src/engine/done_gate.rs` | modificado |
| `docs/features/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli/` | untracked (`clarify.md`, `objectives.md`, `spec.md`, `plan.md`, `_agent_handoff.md`) |
| `.SddIA/projects/core.md` / `.SddIA/project.md` | aún ausentes (T3 del plan no materializado) |

`validacion.md`, paso del PBI a `done/` y `delivery-close-cycle` no iniciados.

## 4. Mutación no esperada fuera de la rama de trabajo

HEAD durante el ciclo: `docs/cerrar-pbi-kalma2-branch-before-mutate`. La ref `feat/arquitecturalinearhu-b07manifiestocore-selfyactivacindone-gateli` no existe (`git show-ref` vacío). Aun así quedaron escritos, sin commit, el `persist_ref` y el diff de motor citados en §3.3.

No es `main`, a diferencia de la auditoría del 13. Es la misma regla incumplida: si el ciclo trabaja en rama, ninguna mutación de código o de `persist_ref` puede ocurrir en la rama que ya estaba en checkout cuando ese checkout no ha tenido éxito.

### 4.1. El checkout no llegó a ejecutarse

`workspace-init` evalúa `dirty_paths_outside_scope` antes del `checkout` con `create_if_not_exists`. La hoja PTC de fin es `status=failed` y no incluye `verdict` ni `error`. Esta pasada no deja en la traza la cadena literal `dirty-worktree:` que sí capturó la auditoría del 13. El worktree sucio fuera de alcance y la rama ausente coinciden con aquel gate. El binario que ejecutó el ciclo no volcó el `error` de la fase al PTC (el fuente actual de `progress_trace.rs` inserta `phase_name_raw` y `verdict`; estas siete hojas no traen `phase_name_raw`).

### 4.2. La fase fallida no cortó a los agentes

Tras el `failed` de inicialización, Mayeuta, Dedalo y Tekton corrieron. El barrier que la auditoría del 13 describió (`agent_phase_blocks_downstream` solo sobre fases de agente) sigue sin armarse ante una inicialización nativa `failed`, al menos en el `execute-process` debug que atendió este `correlation_id`.

## 5. Por qué la WUI no se movió

Misma caja que el 13 de octubre. `pollStatus` reescribe Respuesta en cada `GET /api/status` y solo sale del bucle con `completed` o `failed`. `awaiting_agents` alarga el plazo a 30 min.

`project_status` decide la frase con el PEC de correlación. El proof de las 17:43:08Z no se ha reescrito. A las 17:51Z, con Diseño `executed` y Ejecución ya iniciada, la API responde:

```text
status=awaiting_agents
message=Ciclo «feature» en espera de agentes IDE.
process_status=success
cycle_phase=awaiting_agents
```

El fuente actual, si el último fin de fase es `executed` o `blocked`, sustituye esa frase. El puente release en ejecución no lo hace. Además `phase_end_from_trace` descarta `phase_status=failed`, así que el fallo de inicialización no puede pintar la caja aunque el binario estuviera al día: el último fin reconocido sería Diseño `executed`, no el `failed` de las 17:43:08Z.

La consola de progreso (`EventSource` a `/api/progress/stream`) no recibió ni una línea. El timing de red del navegador registra `/api/execute` y decenas de `/api/status`, y no registra el stream. Un `curl` de 2 s al mismo path devolvió 0 bytes. Hay 7 trazas en disco. El operador no ve ni el `failed` de init ni el «listo para Tekton» del handoff.

## 6. Contraste con la auditoría del 13

| Fricción (2026-10-13) | Esta pasada (2026-10-14) |
|------------------------|---------------------------|
| F-BRANCH-NOT-FIRST | Sigue. Rama `feat/…` ausente. Checkout no ejecutado. |
| F-INIT-FAIL-CONTINUES | Sigue. Init `failed` y Mayeuta, Dedalo y Tekton continúan. |
| F-WORK-ON-MAIN | Misma clase, otra base. Diff sobre `docs/cerrar-pbi-kalma2-branch-before-mutate`, no sobre `main`. |
| F-KALMA2-STATUS-STALE | Sigue. PEC de las 17:43:08Z congela la caja. Diseño `executed` no se proyecta. |
| F-KALMA2-AGENT-VERDICT-MUTE | Sigue. PTC sin veredicto. El handoff sí lo tiene («listo para Tekton») y la WUI no. |
| F-REENTRY-SAME-CID | No reproducida. Una sola pasada. |
| Consola vacía | Nueva respecto al corte del 13, donde el canal existía y el déficit era el texto. Aquí el stream no entrega bytes a la epidermis. |

El PBI de cierre (`PBI-FIX-KALMA2-BRANCH-AND-STATUS`, PR #347, `done`) declara esas cinco primeras fricciones resueltas y deja fuera de alcance rediseñar la WUI y relajar el gate dirty. El runtime de esta prueba no se comporta como ese cierre: el ELF debug no emite `phase_name_raw`/`verdict`, y el ELF release del puente no sustituye `awaiting_agents` por el fin de fase.

## 7. Estado al corte

| Pieza | Estado |
|-------|--------|
| HEAD | `docs/cerrar-pbi-kalma2-branch-before-mutate` |
| Rama de trabajo | ausente |
| Inicialización | `failed` (sin error en PTC) |
| Estabilización (Mayeuta) | `executed` |
| Diseño (Dedalo) | `executed` |
| Ejecución (Tekton) | iniciada, sin fin; diff de motor ya en el worktree |
| Verificación / cierre | no iniciados |
| WUI Respuesta | congelada en el aviso inicial |
| Consola de progreso | vacía |
| Proof PEC | `awaiting_agents` / `success` desde 17:43:08Z |
| Proceso | pid 101739 vivo |
| `execution_id` | `5178e34f-8e17-4f93-a8d8-054496306053` |
