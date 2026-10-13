---
document_id: PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES
uuid: "ffe6a260-ffb7-450d-890e-f8c65acc9373"
title: "[KAIZEN] Telegram — silencio PEC accept-pr y PPR, síntesis de valor en castellano"
format: markdown
version: "1.0.0"
created: "2026-10-13"
updated: "2026-10-13"
status: done
priority: alta
process: bug-fix
type: kaizen
dispatch: false
suggested_branch: fix/kaizen-telegram-pec-silence-value-es
related:
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs
  - SddIA/actions/notify-humanized-pr-merged.md
  - SddIA/core/event-orchestration-subscriptions.json
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/events/domain/pull-request-merged.md
  - docs/todos/done/[KAIZEN] Aduana PR — corte LLM, dedupe Presented, persist_ref docs, sellos post-merge y PEC Telegram.md
antecesor_document_id: PBI-KAIZEN-PR-ADUANA-FRICTION-20261012
out_of_scope:
  - "No retirar la suscripción genérica de Process_Execution_Completed → send-telegram-notification."
  - "No silenciar PEC de otros procesos, ni el PEC success de pull-request-review, ni el PEC failed de accept-pr."
  - "No sustituir el bloque estático de PullRequest_Merged ni el fail-soft que evita colapsar el suscriptor si Gemini o git fallan."
  - "No versionar el ECST de PullRequest_Merged con diffs ni añadir un segundo envío Telegram."
---

# [KAIZEN] Telegram — silencio PEC accept-pr y PPR, síntesis de valor en castellano

## 1. Objetivo

Refinar la comunicación a Telegram en dos frentes: dejar de notificar dos cierres de ciclo que no aportan al operador, y auditar el aviso de PR fusionado para que el apartado **Síntesis de Valor** informe en castellano cuando hay inferencia.

## 2. Silencio de PEC

La suscripción de orquestación (`event-orchestration-subscriptions.json`) despacha todo `Process_Execution_Completed` a `send-telegram-notification`. El humanizador (`build_telegram_message_from_event`) arma el texto. Tras `PBI-KAIZEN-PR-ADUANA-FRICTION-20261012` el titular ya no repite `failed/failed` ni `success/completed`: es `PEC {status}: {process}`. El operador sigue recibiendo esos dos cierres y pide que no se notifiquen.

| Señal citada | Equivalencia en el humanizador actual | Acción |
|---|---|---|
| `PEC success/completed: accept-pr` | `process_name=accept-pr` y `status=success` (ciclo `completed` o ausente) | No enviar |
| `PEC failed/failed: pull-request-review` | `process_name=pull-request-review` y `status=failed` | No enviar |

El filtro vive en el humanizador: devolver ausencia de mensaje para esos dos pares. El resto de PEC conserva el contrato ADU-CA11–CA14 (fase, motivo, rama, aviso, correlación).

Motivo operativo: el `success` de `accept-pr` queda cubierto por el aviso `PullRequest_Merged`. El `failed` de `pull-request-review` es ruido de aduana (peaje F2 y reintentos) que el operador no quiere en el canal.

## 3. Síntesis de valor del PR fusionado

Suscriptor: `notify-humanized-pr-merged` sobre `PullRequest_Merged`. Ensambla el bloque estático y, si `gemini-http-infer` devuelve texto, añade:

```text
🧠 Síntesis de Valor: {hasta 2 líneas, ≤400 caracteres}
```

Defectos observados:

1. **Ausencia.** El paso es fail-soft: fallo, timeout o cuerpo vacío de Gemini omiten el apartado y el Telegram sale solo con el estático (`assemble_humanized_message`). El operador no distingue «no había valor que contar» de «la inferencia no corrió».
2. **Idioma.** El prompt de síntesis está en inglés (`Return only business value of this merge…`). El modelo responde en inglés. El apartado tiene que ir en castellano.

El bloque estático (rama, commit, integrador, auditor, URL) se queda. No se inventan ficheros ni intención ausentes del contexto (`commit_summary` fail-soft + CONTEXT).

### 3.1. Idioma vehicular

Valorar, en el diseño del fix, un input de la cápsula de inferencia (o de la acción) que fije el idioma vehicular, con defecto `es`.

- Si un campo ya existente o una cláusula estable del prompt basta para castellano y no hay otro llamador que lo necesite, no se abre entidad nueva.
- Si el modelo sigue desviándose con solo el prompt, el input entra en el contrato de `gemini-http-infer` (o en el de `notify-humanized-pr-merged` si la cápsula no debe conocer idioma) vía `entity-manager`. Defecto documental: castellano. Otros idiomas quedan fuera de este PBI.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| TG-CA1 | Un PEC `success` de `accept-pr` no produce `send-telegram-notification` |
| TG-CA2 | Un PEC `failed` de `pull-request-review` no produce `send-telegram-notification` |
| TG-CA3 | Un PEC `failed` de `accept-pr`, un PEC `success` de `pull-request-review` y un PEC de otro proceso siguen el texto ADU-CA11–CA14 |
| TG-CA4 | Con inferencia válida, el aviso de `PullRequest_Merged` incluye `Síntesis de Valor` en castellano, ≤2 líneas, sin repetir hash, rama, auditor ni correlación |
| TG-CA5 | La auditoría deja escrito por qué el apartado falta (Gemini, modelo, texto vacío) y el comportamiento resultante: o el apartado aparece en castellano, o la omisión queda explícita en el mensaje sin inventar valor |
| TG-CA6 | Tests del humanizador cubren CA1–CA3. Tests de `notify_humanized_pr_merged` cubren el castellano del prompt o del input de idioma, y el ensamblaje de CA4–CA5 |
| TG-CA7 | Decisión registrada sobre el input de idioma vehicular: adoptado (contrato + defecto `es`) o descartado (el prompt en castellano basta) |

## 5. Notas de implementación

- El silencio no se hace borrando la suscripción de orquestación: otros PEC siguen notificando.
- Prohibido que el humanizador lea `.SddIA/workspaces/` o `.events/` para decidir el silencio. Solo `process_name` y `status` del payload.
- La síntesis sigue fail-soft respecto al colapso del suscriptor: un fallo de Gemini no es `System_Fracture_Detected`.
- Genoma (`actions/`, `events/`, contratos de cápsula) solo si TG-CA7 adopta el input. Cualquier entidad nueva o mutación de contrato sale por `entity-manager`.
