---
document_id: PBI-KAIZEN-PR-ADUANA-FRICTION-20261012
uuid: "e0685a1f-4f7f-48c6-9d52-dd03118b3892"
title: "[KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs, sellos post-merge y PEC Telegram"
format: markdown
version: "1.1.0"
created: "2026-10-13"
updated: "2026-10-13"
status: done
priority: alta
process: bug-fix
type: kaizen
dispatch: false
suggested_branch: fix/kaizen-pr-aduana-friction-20261012
incident_ref: "PR #344 docs/merge-thermo-hu-f2-close — PEC failed pull-request-review FsPzaGfcfLysuGC9pwM3gMATjvK4xQEVbHFkLQ7KVb9j (2026-10-12T10:11:30Z) y PEC success accept-pr e2e8519b-abab-47ae-8ee0-afb3b016da9f (2026-10-12T16:05:46Z)"
friction_ids:
  - F-PPR-LLM-AFTER-F2-BLOCK
  - F-PPR-PRESENTED-FANOUT
  - F-PERSIST-REF-DOCS-PREFIX
  - F-PRESENTED-SCAN-PENDING-ONLY
  - F-HYGIENE-REMOTE-ABSENT-REF
  - F-PEC-TELEGRAM-OPAQUE
related:
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
  - SddIA/engine/execute-process/src/engine/accept_pr.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - SddIA/evolution/c032d392-a586-4b8c-baaf-6cb831ebb943.md
source_audit: "Audición Telegram 2026-10-13. Evidencia: .SddIA/workspaces/pull-request-review/fb20748d-1a5a-4db0-b519-6b1966ec0915/execution_report.json; .SddIA/workspaces/accept-pr/467b26c6-6ccc-496a-a986-872be1ea5cb1/execution_report.json; .SddIA/proofs/pec-correlation/; árbol 6858d7f2 (sin cascada F2) vs e8fde8e4 (cascada F2, 18:05 +0200)."
out_of_scope:
  - "No debilitar el peaje FAIL_F2_DOC ni L-HYGIENE-SOFT (status_code 0 si merge y push salen y la higiene falla de verdad)."
  - "No reintroducir Argos LLM en Auditoría Genómica del camino merge_already_done (O4)."
---

# [KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs, sellos post-merge y PEC Telegram

## 1. Falla y contexto

Audición de dos PEC del PR #344 (`docs/merge-thermo-hu-f2-close`). El `failed` de `pull-request-review` y el `success` de `accept-pr` cumplen contrato. Dentro hay cinco fricciones que el motor puede dejar de producir sin mover esos controles.

### 1.1. Cronología

| Hora (Z) | Hecho |
|----------|--------|
| 10:02 | `Local_QA_Requested` `712e3df1-…` → PPR `failed` (`FAIL_F2_DOC`) |
| 10:07 | `PullRequest_Presented` DCC `28d8d0df-…` y bridge `FsPzaGfc…` (PR #344) → dos PPR más, mismo fallo |
| 10:07–10:11 | Ejecución `fb20748d-…`: Triaje documental Argos `blocked` (~119 s) y Veredicto Argos `blocked` (~108 s). Handoff `skipped` (`verdict_not_aprobado`) |
| 10:07:16 | `6858d7f2`: el sink solo tiene `_agent_handoff.md` y `validacion.md` |
| 16:05:29 | `e8fde8e4`: aparecen `objectives.md`, `spec.md`, `plan.md`, `implementation.md` |
| 16:05:40 | GitHub merge `435319c2` |
| 16:05:46 | `accept-pr` `e2e8519b-…` / ejecución `467b26c6-…`: `merge_already_done`, `orphan_merge: true`, `presented_found: false`, delete remoto `ref remoto no existe`, PEC `success` |

## 2. Impacto

- Dos o tres sesiones Argos de pago por el mismo árbol sin cascada F2 (~4 min la ejecución `fb20748d` sola; la segunda llamada no cambia el PEC).
- `persist_ref` vacío en ramas `docs/…`: Triaje técnico corre con `persist_ref: null`. En este incidente el veredicto coincidió con el disco; con la cascada ya presente, el mismo hueco puede marcar `FAIL_F2_DOC` en falso.
- El sello `PullRequest_Merged` (`f3295cfa-…`) lleva `traceability_anomaly: merge_huérfano` aunque hubo `PullRequest_Presented` horas antes. La anomalía miente.
- La higiene anota fallo cuando la rama remota ya no existe tras el merge de GitHub. El estado deseado está cumplido.
- El aviso Telegram (`build_telegram_message_from_event`) es `PEC {status}/{cycle_phase}: {process}`. En el incidente salió `failed/failed` y `success/completed`: el ciclo repite el estado y no dice fase, motivo, rama ni aviso de sello. Esos datos ya están en el payload (`failed_phase`, `failed_phase_error`, `persist_ref`) o en el state de `accept-pr`, y no llegan al texto.

## 3. Causas (solo lo que hay que cambiar)

### 3.1. F-PPR-LLM-AFTER-F2-BLOCK

`pull-request-review` corre en `residual_runner::run_generic`, que no corta fases. `executor::agent_phase_blocks_downstream` solo aplica a `feature`, `bug-fix` y `refactorization`. Tras Triaje documental `blocked`, Veredicto vuelve a invocar el runtime.

El peaje F2 y el `status_code: 1` se quedan. Lo que sobra es la segunda sesión LLM. Veredicto, en este caso, debe materializarse en determinista a partir del bloqueo ya registrado (handoff sigue `skipped` / `verdict_not_aprobado`).

### 3.2. F-PPR-PRESENTED-FANOUT

O1 (`c032d392-…`) idempotiza el pre-push. No colapsa dos `PullRequest_Presented` del mismo commit (DCC + github-bridge, segundos aparte), ni el `Local_QA_Requested` solapado con ellos.

La clave de colapso es **rama + commit** (o PPR ya en vuelo sobre ese par). Un commit nuevo en la misma rama tiene que poder pasar aduana otra vez: si no, el push de la cascada F2 quedaría mudo.

### 3.3. F-PERSIST-REF-DOCS-PREFIX

`infer_persist_ref_from_branch` solo acepta `feat/`, `fix/` y `refactor/`. `docs/merge-thermo-hu-f2-close` no resuelve, aunque exista `docs/features/merge-thermo-hu-f2-close/`. O2 documenta el mismo límite. Hay que ampliar la heurística: prefijo `docs/{slug}` → `docs/features/{slug}` si el directorio existe; si no, `docs/fixes/{slug}`. Sin directorio, `persist_ref` sigue vacío (no inventar sink).

### 3.4. F-PRESENTED-SCAN-PENDING-ONLY

`scan_presented_for_branch` solo lee `.events/pending`. A las 16:05 los Presented de la rama ya estaban en `processed` y `dead-letter`. O4 pide `merge_huérfano` cuando **no hubo** Presented, no cuando el bus ya lo consumió. El escáner debe considerar `pending`, `processed` y `dead-letter`.

### 3.5. F-HYGIENE-REMOTE-ABSENT-REF

`delete_branch` remoto con `ref remoto no existe` entra en `hygiene_failure` (`ok: false`). Ausencia del ref es el estado objetivo de la fase 4. Debe contarse como éxito idempotente. Un rechazo distinto (auth, red, ref protegida) sigue siendo `hygiene_failure` y no tumba el `status_code` (L-HYGIENE-SOFT).

Con ref ausente = éxito, local y remoto quedan en éxito y `closed_branch` coincide con el markdown. No reabrir el caso en que el remoto falla por otra causa: el código y `t_a2_local_ok_closed_branch` publican `closed_branch` solo con delete local; el markdown exige ambos. Alinear código y test al markdown: `closed_branch` solo si las dos operaciones confirman éxito; el parcial permanece en `hygiene_failure`.

### 3.6. F-PEC-TELEGRAM-OPAQUE

`thermodynamic.rs` ya escribe en el PEC de fallo `failed_phase`, `failed_phase_code` y `failed_phase_error`. El humanizador los ignora. Rama, `pr_url`, higiene y `orphan_merge` ni siquiera entran al payload: el suscriptor Telegram no debe leer el `execution_report`.

El texto tiene que bastar para decidir si abrir el informe. Contrato de líneas, en este orden, omitiendo las vacías:

1. `PEC {status}: {process_name}`. Si `cycle_phase` no es `completed`, `failed` ni igual a `status`, sufijo ` ({cycle_phase})` — así sigue visible `awaiting_agents` y desaparece el `failed/failed`.
2. `fase: {failed_phase}` si existe.
3. `motivo: {failed_phase_error}` recortado a 120 caracteres.
4. Una sola línea de contexto: `rama: {branch}` si hay rama; si no, `persist_ref: {persist_ref}`.
5. `pr_url` en línea propia si existe.
6. `aviso: {notice}` recortado a 120 caracteres, si `notice` no está vacío.
7. `correlation_id={correlation_id}` siempre al final. Si falta, `sin-correlation_id` como ahora.

`notice` lo proyecta el emisor del PEC (no el humanizador), una sola cadena, con esta prioridad: higiene incompleta (`survived_branch` + causa corta) > `merge_huérfano` > `merge ya materializado`. Sin ninguno de los tres, no se escribe `notice`.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| ADU-CA1 | Triaje documental `blocked` no dispara una segunda sesión de runtime en Veredicto. Veredicto queda `blocked` determinista, handoff `skipped` (`verdict_not_aprobado`), `status_code: 1` |
| ADU-CA2 | Si Triaje documental no está `blocked`, Veredicto sigue delegando en Argos |
| ADU-CA3 | Dos `PullRequest_Presented` (o uno más un `Local_QA_Requested` solapado) con la misma rama y el mismo commit producen un solo `pull-request-review`. El duplicado queda descartado con causa explícita y rastro durable |
| ADU-CA4 | Un commit distinto en la misma rama no queda suprimido por ADU-CA3 |
| ADU-CA5 | Rama `docs/{slug}` con directorio `docs/features/{slug}` o, en su defecto, `docs/fixes/{slug}`, inyecta ese `persist_ref`. Sin directorio, el campo sigue ausente |
| ADU-CA6 | `feat/`, `fix/` y `refactor/` conservan la resolución actual |
| ADU-CA7 | `presented_found` es true si hay `PullRequest_Presented` de esa rama en `pending`, `processed` o `dead-letter`. Solo la ausencia en los tres deja `orphan_merge` y `traceability_anomaly: merge_huérfano` |
| ADU-CA8 | Delete remoto cuya única causa es ref ya ausente cuenta `ok: true` y no abre `hygiene_failure` |
| ADU-CA9 | Delete remoto fallido por causa distinta sigue en `hygiene_failure`. `closed_branch` solo si local y remoto están `ok`. `status_code` permanece 0 si merge y push salieron (L-HYGIENE-SOFT) |
| ADU-CA10 | Tests de regresión en `accept_pr` y en la inferencia de `persist_ref` cubren CA5–CA9. Test del corte de fase cubre CA1–CA2 |
| ADU-CA11 | Un PEC `failed` con `failed_phase` y `failed_phase_error` produce Telegram con `PEC failed: {process}`, `fase:`, `motivo:` (≤120) y `correlation_id`. No aparece el par redundante `failed/failed` |
| ADU-CA12 | Un PEC `success` con `cycle_phase=completed` no añade ciclo al titular. Un PEC con `cycle_phase=awaiting_agents` sí lo muestra entre paréntesis |
| ADU-CA13 | El emisor copia al payload `branch` (desde `source_branch`, `pr_branch` o `branch`), `pr_url` y como máximo un `notice` según la prioridad de §3.6. El humanizador solo lee el payload |
| ADU-CA14 | `telegram_message_for_pec_includes_correlation` sigue en verde. Tests nuevos cubren CA11–CA13, incluido el caso sin `notice` (línea `aviso` ausente) |

## 5. Notas de implementación

- Trabajar sobre `residual_runner` (o un barrier compartido con `executor`) solo para la fase Veredicto de `pull-request-review` cuando el state ya tiene bloqueo documental. No mover PPR entero al executor de `feature`/`bug-fix`: eso saltaría también Cerbero, Cosecha y el handoff, que hoy son baratos y dejan el `skipped` explícito.
- La deduplicación vive en el despacho (`route_domain_core` o guardián equivalente), no en un segundo PPR que arranque y aborte tarde. El rastro del descarte no puede evaporarse con `purge_after: true` sin suscriptor (mismo riesgo que TQM-CA8 en `PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI`).
- `infer_persist_ref_from_branch` es el único punto. No duplicar la heurística en el prompt de Argos.
- En higiene, clasificar el stderr de git (`remote ref does not exist` / `ref remoto no existe`) dentro de `record_hygiene_op` o justo después. No tragar otros errores.
- Genoma (`SddIA/process/`, normas, eventos nuevos) solo si ADU-CA3 exige una clase ECST que no exista. Preferir proyección ya suscrita. Cualquier entidad nueva sale por `entity-manager`, no por edición manual del genoma.
- El texto Telegram se arma solo en `build_telegram_message_from_event`. La proyección de `branch`, `pr_url` y `notice` va en `thermodynamic.rs` (y en el PEC temprano de TQM si ese payload ya conoce la rama). Prohibido que el humanizador abra `.SddIA/workspaces/` o `.events/`.
- `notice` no sustituye a `hygiene_failure` ni a `traceability_anomaly` en el sello. Es un resumen de una línea para el operador.

## 6. Fuera de alcance

- El PEC `failed` por cascada F2 ausente en `6858d7f2` fue correcto. No suavizarlo.
- `merge_already_done` y el atajo nativo de Auditoría Genómica en post-merge se quedan. El PEC solo los resume en `notice` cuando aplican.
