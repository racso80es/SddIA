---
document_id: PBI-EMAIL-DIGEST-PREFERENCE-REPLY
uuid: "bba79183-b76b-4e05-aa02-5adf1d09b500"
title: "[OPERATIVO] Réplica del digest de ruido → preferencias"
format: markdown
version: "1.0.0"
created: "2026-09-06"
updated: "2026-09-06"
status: "propuesta"
refinement_status: unrefined
priority: baja
type: operativo
process: feature
dispatch: false
suggested_branch: feat/email-digest-preference-reply
persist_ref_suggested: docs/features/email-digest-preference-reply
depends_on:
  - PBI-EMAIL-NOISE-HEURISTIC-DIGEST
spawned_by: PBI-EMAIL-TRIAGE-HEURISTIC
related:
  - SddIA/events/domain/user-preference-change-requested.md
  - SddIA/process/user-preference-ingest.md
  - SddIA/actions/emit-user-preference-change-requested.md
  - SddIA/process/telegram-fallback-responder.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-quick-action-ingest.md
  - docs/todos/pending/[OPERATIVO] Soporte de Botonera Inline en Telegram (InlineKeyboardMarkup + callback_query).md
  - docs/features/memoria-preferencias-usuario/spec.md
---

### [OPERATIVO] Réplica del digest de ruido → preferencias

#### Origen

Laudo D2 / `PBI-EMAIL-TRIAGE-HEURISTIC` v1.4.0: el digest pregunta si inyectar `priority:max`; **este** PBI cierra el lazo: indicación humana → `User_Preference_Change_Requested` → `user-preference-ingest`. No implementa el informe batch (`PBI-EMAIL-NOISE-HEURISTIC-DIGEST`).

#### Alcance (borrador)

1. Correlacionar la respuesta del Vértice con el digest (ventana + remitente). `callback_data` Telegram ≤ 64 bytes si se usan botones; alternativa: comando/texto con id corto del digest, no utterance larga en ECST.
2. Emitir `User_Preference_Change_Requested` con `operation: activate`, `channel: telegram`, `predicate: priority`, `priority_level: max` (o `mute` si la indicación es ignorar), `subject_key` = **hash** del remitente (nunca addr en claro en el evento). Autoridad destino: `explicit_user`.
3. `user-preference-ingest` persiste. El padre (`P-EXEMPT-C`) solo honra `explicit_user`+`priority` max\|high: sin este ciclo el muro C no se mueve.

#### Fuera de alcance / trampas

- **No** `telegram-fallback-responder`: triaje inverso de texto libre (`TelegramMessage_Received`), sin FSM de digest.
- **No** `email-quick-action-ingest`: `archive|draft|delegate`; no preferencias; no IMAP igual, pero contrato distinto.
- **No** activar desde `inferred` / `proposed` sin confirmación (spec memoria-preferencias §3).
- Botonera: si el digest se responde con inline keyboard, gate `PBI-TELEGRAM-INLINE-KEYBOARD`. No es dependencia dura si el canal es comando de texto.
- No mutar IMAP. No reabrir Clasificacion en el momento del digest (el cambio aplica al **siguiente** `Email_Received`).

#### Criterios de aceptación (borrador)

- [ ] Una indicación explícita sobre un remitente del digest produce revisión `active` / `explicit_user` / `priority.max` con `subject_key` hasheado.
- [ ] ECST de preferencia sin `body` / `snippet` / addr en claro.
- [ ] Indicación ausente o «ignorar digest»: cero escritura, o `operation: ignore`.
- [ ] El siguiente correo de ese remitente `noreply@` ya no cierra por `C-NOREPLY` **solo si** Slice 1 del padre (`P-EXEMPT-C`) está implementado. Este PBI no parchea el gateway.
