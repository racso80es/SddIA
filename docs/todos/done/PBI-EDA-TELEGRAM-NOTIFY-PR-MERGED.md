---
document_id: PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
uuid: "5880d6fc-99f3-4ecf-8c9e-a4885d45f117"
title: "[ARQUITECTURA] Notificación Telegram reactiva post-merge — suscripción PullRequest_Merged"
format: markdown
version: "1.1.0"
created: "2026-09-05"
updated: "2026-09-05T16:55:00+02:00"
status: cerrado
refinement_status: implemented
fix_ref: docs/features/eda-telegram-notify-pr-merged
priority: media
process: feature
executor_vehicle: feature
type: arquitectura
dispatch: false
suggested_branch: feat/eda-telegram-notify-pr-merged
persist_ref_suggested: docs/features/eda-telegram-notify-pr-merged
related:
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/events/domain/pull-request-merged.md
  - SddIA/events/domain/pull-request-presented.md
  - SddIA/events/events-contract.md
  - SddIA/actions/emit-pr-merged-event.md
  - SddIA/core/cumulo.paths.json
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/event-subscriptions.json
  - SddIA/process/route-domain-event.md
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/actions.rs
  - SddIA/engine/execute-process/src/engine/accept_pr.rs
  - SddIA/tools/send-telegram-notification.md
  - SddIA/daemons/event-watcher.md
  - SddIA/norms/pull-request-orchestration.md
---

# [ARQUITECTURA] Notificación Telegram reactiva post-merge — suscripción PullRequest_Merged

PBI **abierto**. No está implementado en HEAD. La ejecución transita por proceso `feature` (`executor_vehicle`). Prohibido declarar Done por mutación local no mergeada.

## 0. Filtro A — correcciones respecto a v1.0.0

La v1.0.0 mezclaba especificación con un informe de cierre falso (`status: done`, `refinement_status: implemented`, CA con ✅, «382 passed»). Hechos contrastados contra genoma + runtime:

| ID | Afirmación v1.0.0 | Verdad objetiva | Efecto en este PBI |
| :--- | :--- | :--- | :--- |
| FA-1 | PBI `done` / `implemented` en `docs/todos/pending/` | Ubicación = pendiente. HEAD no tiene el suscriptor Telegram. | `status: abierto`, `refinement_status: refinado`. |
| FA-2 | `emit-pr-merged-event` no propaga `traceability_anomaly` | `actions.rs` `emit_pr_merged` copia `traceability_anomaly` y `traceability_note` desde inputs al payload. `accept_pr.rs` Fase 3 los inyecta si `orphan_merge`. | DD-3 reescrito. Versionar la Clase **no** es prerrequisito para que el JSON los lleve. |
| FA-3 | `correlation_id` es dato de **payload** | Envelope ECST (`events-contract` §3). `emit_pr_merged` lo escribe en raíz; el compositor debe leer `event.correlation_id`. | Tabla de datos y CA corregidos. |
| FA-4 | `event-watcher` compone el mensaje / decide timezone | El centinela solo delega en `route-domain-event`. La composición vive en `build_telegram_message_from_event` (`route_domain_core.rs`). El compositor **no** incluye `timestamp`. | DD-4: omitir hora; no atribuir la decisión al watcher. |
| FA-5 | Paridad dual de JSON como si ambos fueran SSOT | `cumulo.paths.json` → `eda_bus.subscriptions` = `event-domain-subscriptions.json`. Ese es el fichero que lee `subscribers_for_event_type`. `event-subscriptions.json` es registro legado parcialmente solapado. | DD-5: SSOT = domain; paridad legado por patrón PR, no dual-SSOT. |
| FA-6 | `security_clearance` / `pr_url` / `correlation_id` al mismo nivel de «condicionales» | Clase v1.0.0: `security_clearance` **REQUIRED**; `pr_url` OPTIONAL; `correlation_id` envelope OPTIONAL. `accept-pr` **no** pasa `pr_url` a `emit-pr-merged-event`. | En flujo canónico el enlace GitHub **no** aparece. Ejemplo v1.0.0 con URL era atípico. |
| FA-7 | Simetría de mensaje con `PullRequest_Presented` | Simetría de **agente+tool** (Argos → `send-telegram-notification`). El mensaje Presented es plano (`PR presentado: {branch}` + URL). Merged será más rico a propósito. | No vender simetría de formato. |
| FA-8 | Intent JSON «auditor y anomalías» | Fase 1 no muestra anomalías. | Intent sin «anomalías». |
| FA-9 | Mutar `pull-request-merged.md` § Suscripciones sin tocar `hash_signature` | `events-contract` exige `hash_signature` del archivo Clase. El uuid `cfb8ce66-784e-4826-8a0a-a20c671e3a60` en `eda-coverage.json` replica ese hash. | Artefacto obligatorio: recalcular hash + alinear coverage. |
| FA-10 | CA2 «`cargo test` 382 passed» | Cifra no reproducible como criterio. No hay test unitario de la rama `PullRequest_Merged` en el compositor. | CA de test local, sin conteo global. |

**Árbol de trabajo sucio (no-HEAD):** hay diffs locales en suscripciones, `route_domain_core.rs` y `pull-request-merged.md` que anticipan este PBI, con intent «anomalías» y **sin** recálculo de `hash_signature`. Eso no es Done ni sustituye al proceso `feature`. `eda-coverage.json` sucio incluye entradas ajenas (`sha256:deadbeef`); **fuera de alcance** de este PBI.

## 1. Falla estructural (HEAD)

Secuencia canónica de PR (`pull-request-orchestration` + códice software-engineering):

```
delivery-close-cycle → PullRequest_Presented → pull-request-review → accept-pr → PullRequest_Merged
```

`accept-pr` (Fase 3) sella vía `emit-pr-merged-event` en `eda_bus.pending`. `event-watcher` dispara `route-domain-event`. Fan-out según `event-domain-subscriptions.json`.

En HEAD, `PullRequest_Presented` tiene tres suscriptores en el JSON de dominio: `pull-request-review` (Argos), `iota-immutable-publisher` (Cúmulo), `send-telegram-notification` (Argos). La Clase `pull-request-presented.md` **no** documenta Telegram (solo aduana + DLT); el JSON sí.

`PullRequest_Merged` en HEAD tiene **un** suscriptor: `iota-immutable-publisher`. El operador recibe aviso de presentación y no de fusión.

### Asimetría HEAD

| Evento | IOTA | Telegram | Proceso |
| :--- | :---: | :---: | :---: |
| `PullRequest_Presented` | sí | sí (JSON; Clase incompleta) | sí (`pull-request-review`) |
| `PullRequest_Merged` | sí | **no** | — |

## 2. Objetivo

Añadir suscripción `argos` → `send-telegram-notification` a `PullRequest_Merged` y componer un resumen ejecutivo en `build_telegram_message_from_event`. Tras fusión soberana, el Vértice Biológico recibe confirmación táctica por Telegram, desacoplada de `accept-pr` (coreografía EDA).

### Mapa de datos

| Origen real | Campo | Rol en el mensaje |
| :--- | :--- | :--- |
| payload REQUIRED | `source_branch` | Encabezado |
| payload REQUIRED | `target_branch` | Etiqueta de destino (canónico `"main"`; no hardcodear si el payload lo trae) |
| payload REQUIRED | `merge_commit_hash` (40 hex) | 7 primeros chars |
| payload REQUIRED | `author` | Integrador |
| payload REQUIRED | `security_clearance.auditor` + `policy_applied` | Cadena de confianza. Fallback `"?"` solo si instancia malformada. |
| payload OPTIONAL | `pr_url` | Línea extra si no vacío. En `accept-pr` canónico **ausente**. |
| envelope OPTIONAL | `correlation_id` | 8 primeros chars si `len >= 8`. `emit_pr_merged` siempre lo mintea si falta en inputs. |
| payload extra-contractual | `traceability_anomaly` / `traceability_note` | **No mostrar** (fase 1). Runtime ya puede persistirlos; la Clase v1.0.0 no los declara. |
| envelope | `timestamp` | **No mostrar**. |

### Ejemplo canónico (sin `pr_url`)

```
✅ PR Fusionado — feat/accept-pr-telegram-notify
━━━━━━━━━━━━━━━━━━━━━━━━
📦 Commit: a1b2c3d (main)
👤 Integrador: integration-operator
🔐 Auditor: Argos · pr-acceptance-protocol
🔗 Correlación: 7f3a9c2e…
```

Si `payload.pr_url` está poblado, una línea adicional con la URL cruda (sin markup). No es el caso del sello que emite `accept-pr` hoy.

## 3. Decisiones de diseño

- **DD-1 · Agente = Argos.** Misma titularidad que el Telegram de `PullRequest_Presented`. La tool permanece ciega: solo recibe `message`.
- **DD-2 · Composición en `route_domain_core.rs`.** Extender `build_telegram_message_from_event` (ya ramifica `PullRequest_Presented`, `System_Fracture_Detected`, `Email_Triaged`, y `Process_Execution_Completed` de familia **orchestration**). Añadir `"PullRequest_Merged"`. No acoplar `accept_pr.rs`.
- **DD-3 · Anomalías de trazabilidad fuera de fase 1.** El runtime **sí** puede escribir `traceability_anomaly`/`traceability_note` en el payload (extra permitido: `ecst_validation` solo exige REQUIRED y rechaza FORBIDDEN; no rechaza claves no listadas). La Clase no los declara OPTIONAL. El mensaje no los consume. Hacerlos contractuales exige Clase `1.1.0` (evolución futura), no es bloqueo de esta entrega.
- **DD-4 · Sin reloj en el mensaje.** No convertir timezone. No imprimir `timestamp`. `user-preference-store` / `user-preference-core` existen; inyectar hora local queda fuera (evolución §8.2).
- **DD-5 · Registro de suscripciones.** Escribir el suscriptor en `event-domain-subscriptions.json` (**SSOT** `cumulo.paths.json`). Replicar en `event-subscriptions.json` por paridad con las claves PR ya duplicadas. No consolidar los dos ficheros.
- **DD-6 · `pr_url` OPTIONAL.** Incluir solo si string no vacío. No backfill desde `PullRequest_Presented` en fase 1 (`accept-pr` no propaga la URL).
- **DD-7 · `parse_mode`.** Sin cambios en `send-telegram-notification`. La Táctica del Refugio (reintento único sin `parse_mode` ante HTTP 400 de parsing) ya cubre emojis y caja Unicode. El mensaje Merged es más agresivo que Presented (emojis); el fallback es la mitigación.
- **DD-8 · Test unitario obligatorio.** Un test de `build_telegram_message_from_event` para Merged: required + envelope `correlation_id`; ausencia de `pr_url`; presencia de `pr_url`; no filtrar `traceability_anomaly` al texto.
- **DD-9 · Hash de Clase.** Cualquier edición de `pull-request-merged.md` recalcula `hash_signature` y alinea `eda-coverage.json` → uuid `cfb8ce66-784e-4826-8a0a-a20c671e3a60`. `index.md` de dominio: bump de versión de Clase si se versiona el frontmatter (patch `1.0.1` si solo cambia § Suscripciones; **no** `1.1.0`).

## 4. Alcance

### Dentro

- Suscriptor `argos` → `send-telegram-notification` bajo `PullRequest_Merged` en ambos JSON (SSOT domain + paridad legado). Intent **sin** la palabra «anomalías».
- Rama de composición + tests en `route_domain_core.rs`.
- § Suscripciones de `pull-request-merged.md` (tabla IOTA + Telegram) + `hash_signature` + coverage de ese uuid.
- Patch de versión de Clase `1.0.1` si se toca el cuerpo/frontmatter de la Clase.

### Fuera

- Clase `1.1.0` / mostrar merges huérfanos.
- Inyectar `pr_url` desde `accept-pr` o desde el scan de Presented.
- Timezone / `timestamp` en Telegram.
- Consolidar los dos JSON de suscripciones.
- Corregir `pull-request-presented.md` (Telegram no documentado) — higiene ajena.
- Mutar `send-telegram-notification`.
- Entradas ajenas de `eda-coverage.json`.

## 5. Artefactos a modificar (vía `feature`)

| Fichero | Cambio |
| :--- | :--- |
| `SddIA/core/event-domain-subscriptions.json` | +suscriptor Argos/Telegram en `PullRequest_Merged` |
| `SddIA/core/event-subscriptions.json` | Paridad legado |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | Rama compositor + tests DD-8 |
| `SddIA/events/domain/pull-request-merged.md` | § Suscripciones; `version: 1.0.1`; `hash_signature` nuevo |
| `SddIA/events/domain/index.md` | Versión de fila `pull-request-merged` si sube a 1.0.1 |
| `SddIA/core/eda-coverage.json` | Solo `last_hash` del uuid de esta Clase |

## 6. Criterios de aceptación

| ID | Criterio | Verificación |
| :--- | :--- | :--- |
| TG-MERGED-CA1 | `event-domain-subscriptions.json` contiene `argos` → `send-telegram-notification` bajo `PullRequest_Merged`. `event-subscriptions.json` en paridad. Intent sin «anomalías». | Inspección JSON. |
| TG-MERGED-CA2 | `build_telegram_message_from_event` maneja `"PullRequest_Merged"` y retorna `Some(String)` con branch, hash 7 chars, `target_branch`, autor y auditor/política. | Test unitario DD-8. |
| TG-MERGED-CA3 | `pr_url` solo si payload lo trae no vacío. `correlation_id` desde envelope, 8 chars, si `len >= 8`. `traceability_*` no aparece en el texto. | Mismos tests (casos con/sin URL; payload con anomalía ignorada). |
| TG-MERGED-CA4 | Orquestación idéntica a Presented: suscripción declarativa → `route-domain-event` → `invoke_send_telegram_notification`. Cero llamadas a la tool desde `accept_pr.rs`. | Auditoría de código. |
| TG-MERGED-CA5 | Entrada `iota-immutable-publisher` de `PullRequest_Merged` intacta. Fallo Telegram no debe exigir cambio del suscriptor DLT (fan-out independiente). | Diff JSON + lectura del despacho por `tool`. |
| TG-MERGED-CA6 | Clase documenta ambos suscriptores; `hash_signature` coincide con el archivo; coverage de `cfb8ce66-784e-4826-8a0a-a20c671e3a60` alineado. | Hash + JSON coverage. |

## 7. Dependencias y riesgos

| Riesgo | Probabilidad | Mitigación |
| :--- | :---: | :--- |
| MarkdownV2 + emojis/caja → HTTP 400 | Media (mensaje más ornamentado que Presented) | Táctica del Refugio ya en la tool (DD-7) |
| Doble aviso Presented + Merged percibido como ruido | Baja | Intenciones distintas: revisión vs fusión |
| Sello Fase 3 **antes** de push Fase 4: el remoto puede no tener el merge aún | Media (contrato real de `accept-pr.md`) | El mensaje afirma merge **local soberano**, no publicación remota |
| `pr_url` ausente en el flujo canónico | Alta | OPTIONAL; no bloquear; no backfill en fase 1 |
| Instancia huérfana con `traceability_*` extra | Media (solo si no hubo Presented en `pending/`) | Fase 1 ignora esos campos; ECST gate no los prohíbe |
| Declarar Done sobre diffs locales | Alta (ya ocurrió en v1.0.0) | Done = PR único mergeado + PBI en `done/` + `validacion.md` APTO |

## 8. Evoluciones futuras (fuera)

1. Clase `PullRequest_Merged` `1.1.0`: `traceability_anomaly` y `traceability_note` OPTIONAL; línea de aviso en Telegram para merge huérfano.
2. Hora local vía `user-preference-store` (`memory:pref-query`).
3. Unificar SSOT: deprecar `event-subscriptions.json` o fusionarlo.
4. Propagar `pr_url` en Fase 3 de `accept-pr` (inputs o scan de Presented) para que el enlace deje de ser excepcional.
