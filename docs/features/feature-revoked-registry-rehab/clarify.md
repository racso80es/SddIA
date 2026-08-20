---
feature_name: feature-revoked-registry-rehab
created: "2026-08-20"
purpose: Estabilización Mayeuta — PBI-FEATURE-185-REVOKED-REGISTRY (rehab process feature + fail-soft padre DCC + poda telemetría hueca)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/feature-revoked-registry-rehab
persist_ref: docs/features/feature-revoked-registry-rehab
pbi_ref: docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
document_id: PBI-FEATURE-185-REVOKED-REGISTRY
uuid: c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f
source_correlation_id: 17043d6d-c978-4245-b554-2c5edcf94422
source_pr_url: https://github.com/racso80es/SddIA/pull/185
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
olas:
  - A1
  - A2
  - A3
---

# Clarificación — feature-revoked-registry-rehab

Transcript Mayeuta. Estabiliza el **qué** y el **por qué**. Sin diseño de cápsulas, YAML de proceso ni mutación de genoma.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| PBI canónico | `docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md` (`document_id: PBI-FEATURE-185-REVOKED-REGISTRY`; `uuid: c8f4e2a1-…`; `status: pending`) |
| Ciclo | `refactorization` · rama `refactor/feature-revoked-registry-rehab` · un `persist_ref` · un PR |
| Semilla operador | Rehabilitar process `feature` tras `max_recovery_attempts_exceeded`; fail-soft padre DCC post-umbral físico; poda telemetría hueca del batch Radamanto |
| Padre | #174+#177 (`persist_ref`: `docs/features/radamanto-process-threshold-rehab`) — umbral `process: 0.70` + fail-soft PPR/DCC; **prohibió** tocar `feature` (`L-SCOPE-HARD`) |
| Check origen | `RBAC_PROCESS_SIGNER_REVOKED` (F4 Cerbero · alerta auditoría no bloqueante) |
| Sighting | PPR #185 · CID `17043d6d-c978-4245-b554-2c5edcf94422` — la ejecución **entregó**; Cerbero alerta porque el firmante ya estaba en `permanent` |
| `correlation_id` runtime de esta fase | vacío en inputs |

### Estado empírico (corte estabilización 2026-08-20)

| Clave | Cerbero | Radamanto | Nota |
|-------|---------|-----------|------|
| `feature` (raíz) | **`permanent.feature`** · `entity_type: process` · `reason: max_recovery_attempts_exceeded` · `since: 2026-08-19T07:59:05Z` | `status: deprecated` · `recovery_attempts: 4` · `degraded_at: 2026-08-13T06:08:38Z` · `structure_valid: true` · ventana 11 samples (8 OK / 3 KO · rate 0,727) | **Vector activo.** Ausente de `revoked` |
| `entities.feature` | — | `healthy` · `recovery_attempts: 0` | Fósil; no es el revocado |
| `process:feature` | — | `healthy` · `recovery_attempts: 0` | Prefijo `type:id`; no es el revocado |
| Laterales | `revoked.bug-fix`; `revoked.emit-pr-audited-event` | fuera de alcance | Prohibido rehabilitar este ciclo |

Dictamen PBI (vinculante): contrato de fases de `feature` íntegro. Muerte termodinámica: peaje binario de un macro-proceso de 7 fases + self-healing **sin** reset de `recovery_attempts` + umbral `max_recovery_attempts: 3` superado (4). Rate 0,727 **pasa** umbral `process` vigente; vector ≠ `success_rate_below_threshold`.

## D1 — Misión (qué / por qué)

| Decisión | Laudo |
|----------|--------|
| Objetivo | Rehabilitar el proceso `feature` en Cerbero **y** cortar el vector de re-muerte: fail-soft del padre cuando el DCC hijo ya cruzó umbral físico, y dejar de alimentar supervivencia Radamanto con runs de laboratorio huecos. |
| Por qué ahora | Rehab de registro **sin** A1 absoluto + A2 + A3 reabre el mismo contador. #174+#177 dejó `permanent.feature` a propósito; PPR #185 confirma firmante ya permanente. |
| Efecto observable | `feature` ∉ `permanent` ni `revoked`; stats raíz `healthy` con `recovery_attempts: 0`; padre `feature` no colapsa por cola secundaria DCC post-`pr_url`/`delivery_push`; batch de supervivencia ignora samples huecos; PEC sigue emitiéndose. |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un `persist_ref`, un PR. Prohibido despachar `bug-fix` satélite. |
| **L-WAVES** | Tres olas innegociables en el mismo ciclo: **A1** saneamiento instancia, **A2** fail-soft padre, **A3** poda telemetría hueca. Rehab Cerbero sola = reabrir vector. |
| **L-REHAB-INST** | A1 = instancia `.SddIA/` (no genoma). Evidencia en `execution.md`. Prohibido commitear `.SddIA/cerbero/` / `.SddIA/radamanto/` como cierre del PR (jurisprudencia #174+#177). |
| **L-CERBERO** | Borrar clave `permanent.feature`. Verificar `revoked.feature` ausente. No reescribir como `revoked` residual. Cerbero **no** tiene estado `healthy`. |
| **L-STATS** | Reset **solo** del bucket raíz `feature` (el deprecado). No mutar fósil `entities.feature` ni `process:feature`. |
| **L-RESET-ABS** | Absoluto: `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-FEATURE-185-REVOKED-REGISTRY`; `rehabilitated_at` ISO. Ventana `samples` vacía o ≤ últimos éxitos runtime (`duration_ms` real, `exit_code: 0`). Si `attempts` queda ≥ 3, el primer fallo re-emite `Domain_Entity_Deprecated`. |
| **L-ONTOLOGY** | Conservar `entity_type: process` en instancia. No regresionar a `tool`. |
| **L-AGGREGATOR** | `aggregate_execution_terminal` **intacto**. Ya trata `simulated` / `skipped` / `awaiting*` como neutrales. Prohibido «tolerar simulated» en el agregador. |
| **L-FAILSOFT-PADRE** | Kintsugi en el **runner de `feature`**, no en el agregador compartido. Si el DCC hijo cruzó umbral físico (`pr_url` o `delivery_push`) y el fallo es cola secundaria (higiene local / impacto SddIA / timeout `telemetry_receipt` no causal / `telemetry_io_failed`), el padre marca `fail_soft` y **no** propaga `Err` fatal. |
| **L-CAUSAL** | Siguen abortando `feature`: fase 1 git/workspace; snapshot/push/apertura PR del DCC hijo; Argos `block`; fases agente 2–5 cuando el runtime está vivo y el agente **falla** (`simulated` ya es neutral, no convertirlo en `fail_soft`); fase 6 archivo PBI en runtime. |
| **L-TELEMETRY** | Poda del **sample de supervivencia** (`Raw_Execution_Finished` → `radamanto-batch`). PEC (`Process_Execution_Completed`) **sigue**. Hueco = `cycle_phase` ∈ {`initialized`, `awaiting_agents`} **o** lab-skip de cierre **o** `lab_hollow: true`. No podar `completed` ni fallos reales (`exit_code: 1`). Una fase `skipped` dentro de un run `completed` no es hueco. |
| **L-FILTER-SCOPE** | Filtro A3 aplica a procesos lifecycle que comparten peaje (`feature`, y de paso `bug-fix`/`refactorization`). **No** reabre rehab de `bug-fix`. |
| **L-THRESH** | Umbrales Radamanto 1.1.0 (`process: 0.70`, tabla post-#174+#177) **intactos**. Redención **sin** reabrir `success_rate` ni `max_recovery_attempts`. |
| **L-OUT** | Fuera: residual Kalma2 Shell/`git-manager` (dedup #136); lab IMAP/Telegram vivo; rehab `bug-fix` / `emit-pr-audited-event`; troceo EDA de `feature` (faro Kaizen, Filtro C). |
| **L-DOC** | Cascada `features-documentation-pattern` + `validacion.md` APTO + `pbi_archived: true` + PBI en `docs/todos/done/` en la rama del PR. |

### Ajustes anti-alucinación (órdenes crudas → laudo)

| Orden cruda | Laudo |
|-------------|-------|
| «pasar a healthy en Cerbero» | Rehab Cerbero = **borrar** `permanent.feature`. `healthy` solo en `stats.json`. |
| «refactorizar el agregador» | No. Marcar `fail_soft` **antes** del agregador cuando el hijo DCC falle post-umbral físico. |
| «ignorar skipped en telemetría» | No cualquier `skipped`. Poda = runs huecos / lab-skip de cierre, no fase periférica en `completed`. |
| «ignorar el peaje» | PEC orquestación permanece. Se poda el sample de supervivencia. |

## D3 — Matriz de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-A1** | `feature` ∉ `permanent` ni `revoked`; stats raíz `healthy`; `recovery_attempts: 0`; `rehab_laudo: PBI-FEATURE-185-REVOKED-REGISTRY`; `rehabilitated_at`; ventana recortada; evidencia en `execution.md` (no en el diff de instancia). |
| **AC-ONTO** | `entity_type: process` conservado; no regresionar a `tool`. |
| **AC-A2** | DCC hijo post-`pr_url`/`delivery_push` + cola secundaria → padre `fail_soft` + éxito operativo global; git/snapshot/push/PR/Argos/agentes reales siguen causales; agregador intacto. |
| **AC-A3** | `Raw_Execution_Finished` porta `cycle_phase` (y `lab_hollow` si aplica); `radamanto-batch` no empuja samples huecos ni muta `recovery_attempts`/`success_rate` por ellos; PEC sigue emitiéndose. |
| **AC-THRESH** | Umbrales 1.1.0 intactos; redención sin reabrir `success_rate` ni `max_recovery_attempts`. |
| **AC-DOC** | Cascada bajo `persist_ref`; PBI en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## D4 — Handoff Dedalo

1. Consumir este transcript + cuerpo de `objectives.md` como `refined_requirements`.
2. `spec.md`: touchpoints A2 (`feature-delivery-close` / invocación DCC hijo / simetría de código con `bug-fix` **sin** rehab de `bug-fix`) y A3 (payload `Raw_Execution_Finished` + filtro `radamanto-batch`). Agregador fuera de mutación.
3. `plan.md`: un PR motor (A2+A3) + procedimiento A1 instancia evidenciado en `execution.md`.
4. Tests de producto (qué, no cómo): hijo con `pr_url` + higiene failed → padre `fail_soft`; hijo sin push/PR + snapshot failed → padre causal `exit_code: 1`; sample hueco no entra a `samples`.
5. Prohibido reabrir umbrales, rehab laterales, o castrar PEC.
