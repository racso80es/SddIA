---
uuid: 6d59f23b-df29-4be5-9bb9-29cede3474b9
name: pull-request-review
version: 2.1.0
contract: process-contract v1.3.0
context:
- quality-assurance
- source-control
- pr-lifecycle
hash_signature: sha256:c0d8d748e7260e13f1303220a76c3b46fb3cc011d8525c3c293e553f09e04833
inputs:
- pr_id_or_path: Identificador o ruta lógica del PR
- pr_branch: Rama asociada al PR
- correlation_id: UUID v4 de correlación causal (event_id ECST Presented)
- pr_url: URL del PR en forja (opcional v1.1 ECST)
- code_diff: Diff o referencia al cambio bajo revisión
- persist_ref: Ruta documental de la feature bajo docs/features/
- tasks_path: Ruta de tareas semillas resuelta vía Cumulo
- document_context: Contexto documental opcional (normas, ADRs)
- cumulo_topology: Topología SSOT inyectada (paths, contratos, repository_path)
outputs:
- validacion.md: Informe estructurado de revisión
- verdict: aprobado | requiere_cambios | rechazado
- delivery_state: success | failed
- kaizen_seeds: Semillas Kaizen persistidas bajo docs/todos/
- accept_pr_handoff: true si se delegó handoff a accept-pr
phases:
- name: Preparación de rama
  intent: Alinear checkout, fetch y estado limpio para inspección reproducible de pr_branch.
  delegates_to:
  - skill:git-manager
- name: Triaje documental
  intent: Validar frontmatter YAML y presencia de spec.md, plan.md, implementation.md y objectives.md en persist_ref.
  delegates_to:
  - agent:argos
- name: Triaje técnico
  intent: Ejecutar tests, auditoría estática y sensor DIA (audit-doc-parity.py) vía cápsulas autorizadas; alerta documental no bloqueante; respeto de capsule-json-io.
  delegates_to:
  - action:execute-process
- name: Certificación RBAC
  intent: Cerbero certifica permisos del firmante sobre el área del genoma afectada.
  delegates_to:
  - agent:cerbero
- name: Veredicto y bloqueo
  intent: Argos sintetiza dictamen; aborta con delivery_state failed ante violación F2–F4.
  delegates_to:
  - agent:argos
- name: Cosecha Kaizen
  intent: Cúmulo persiste deuda no bloqueante en docs/todos/ sin interrumpir al operador.
  delegates_to:
  - agent:cumulo
- name: Handoff materialización
  intent: Si verdict aprobado, delegar fusión soberana en accept-pr (sin merge directo en aduana).
  delegates_to:
  - action:execute-process
minteo_maximo: null
porcentaje_de_exito: null
---

# pull-request-review

Proceso V5 **Aduana de Fricción** reactiva al estímulo **`PullRequest_Presented`**. Transmuta el legacy `validate-pull-requests` sin recrear agentes fósiles (`architect`, `qa-judge`, `security-engineer`): escrutinio absorbido por **Argos** (documental, técnico, veredicto), **Cerbero** (RBAC) y **Cúmulo** (Kaizen async).

## Disparo EDA

1. `delivery-close-cycle` emite **`PullRequest_Presented`** en `eda_bus.pending`.
2. Proceso **`route-domain-event`** / watcher despacha suscriptor **`pull-request-review`** (agente **Argos**).
3. La aduana evalúa; **`delivery_state: failed`** bloquea materialización downstream.

## Fases de triaje (Fase 1 TODO)

| Dimensión | Delegado | Criterio |
|-----------|----------|----------|
| Documental | Argos | Frontmatter + artefactos base en `persist_ref` |
| Técnica | execute-process → cápsulas QA | Tests/SAST; contratos JSON; sensor DIA |
| RBAC | Cerbero | Token firmante vs área genoma |

### Paridad documental (DIA) — Triaje técnico

Reglas **no bloqueantes** (fricción suave); el sensor no invoca agentes.

| ID | Regla |
|----|-------|
| **DIA-1** | Invocar `SddIA/scripts/qa/audit-doc-parity.py` con `persist_ref`, refs git y salida JSON |
| **DIA-2** | Si `alert_required: true`, propagar deuda a fase **Cosecha Kaizen**; **prohibido** `delivery_state: failed` por DIA |
| **DIA-3** | El sensor no delega a Cúmulo; persistencia Kaizen vía fase **Cosecha Kaizen** (async) o evento `Kaizen_Alert_Required` (deuda EDA v2) |

Prefijos monitorizados por defecto: `SddIA/core/`, `SddIA/process/`, `SddIA/scripts/qa/`, `README.md`.

## Bloqueo y Kaizen (Fases 2–3)

* **Filtro A / Cerbero fallido:** `verdict: rechazado`, `delivery_state: failed`; Argos mapea diff ↔ normas.
* **Deuda DIA (paridad documental):** alerta Kaizen `PENDING_AUDIT_DOC_*` en `docs/todos/pending/`; flujo continúa (`delivery_state: success`).
* **Deuda menor:** Cúmulo inyecta TODO en `docs/todos/` (`[ARQUITECTURA]` o `[OPERATIVO]`); flujo continúa.

## Materialización (Fase 4 TODO)

**Prohibido** merge directo vía `git-manager` en esta aduana. Tras `verdict: aprobado`, la fase **Handoff materialización** invoca **`accept-pr`** con `source_branch` = `pr_branch` y `correlation_id` heredado.

## Notas

* `context: pr-lifecycle` alinea jurisdicción con `accept-pr` sin duplicar auditoría genómica final.
* Principios `blocking_for_pr` enrutan escrutinio a Argos según `knowledge-contract.md`.
