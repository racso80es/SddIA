---
feature_name: email-digest-preference-reply
created: "2026-09-12"
process: feature
purpose: Estabilización Filtro A del PBI v1.2.0; corte planificación (L0)
version_clarify: "1.0.0"
execution_id: "b6930240-b8a7-440a-ab0d-f9f853b1855a"
pbi_ref: docs/todos/pending/[OPERATIVO] Réplica del digest de ruido → preferencias.md
document_id: PBI-EMAIL-DIGEST-PREFERENCE-REPLY
pbi_uuid: "bba79183-b76b-4e05-aa02-5adf1d09b500"
pbi_version: "1.2.0"
slice: all
---

# Clarificación — email-digest-preference-reply

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `b6930240-b8a7-440a-ab0d-f9f853b1855a`. Rama `feat/email-digest-preference-reply`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

Semilla: PBI v1.2.0 (Filtro A residual sobre v1.1.0). Corte L0 = clarify + objectives + spec + plan + commit. Sin mutar handler ni genoma en esta parada.

## Decisiones

| ID | Laudo |
|----|-------|
| L-CHANNEL | Canal canónico = botonera. XOR del gateway intacto. Cero comando de texto. Cero diff en `telegram-watcher` (ACK + `answerCallbackQuery` ya existen). |
| L-TOKEN | Token = prefijo hex de `canonical_subject_key_from_addr(row.key)`: 8 chars; colisión en el lote → 12. Nunca 64 hex ni addr en `callback_data`. `callback_data_ok` mide bytes (`str.len()`). |
| L-KEYS | Teclado = 2 botones por **línea listada** (K del truncado). `dpref:max:<token>` / `dpref:mute:<token>`. `2×K ≤ 100`. Líneas numeradas `N.` correlacionan con texto del botón `⭐ N` / `🔇 N`. Omitidos del truncado → sin botón. |
| L-IGN | Teclado v1 no emite `ign`. Parser: `ign` → skip sin emitir ECST. Otro action → fail-closed. No-tap = cero I/O. |
| L-EMIT | `user_preference_change_requested::run` (acción nativa). ECST **no** lleva `authority`/`status`. `activate` destila `Active`+`ExplicitUser` en ingest. max: `predicate=priority`, `priority_level=max`. mute: `predicate=mute`, `value={muted:true}`. `scope_type=channel`, `scope_id=email`. |
| L-STATE | Clave `tokens` en `{daemons_instance.state}/email-noise-digest.json`. Skip y ventana vacía **preservan** tokens. Notify OK reemplaza el mapa del lote notificado. |
| L-GATE | `callback_data` sin prefijo `dpref:` → skipped exit 0 (coexistencia). Token miss → `skipped-expired`, cero emit. |
| L-EDA | Suscripción `TelegramCallback_Received` → process `email-digest-preference-reply`. Allowlist `event_file_path` en `route_domain_core.rs`. Cero Clase ECST nueva. |
| L-FORGE | EM create process: `process_jurisdiction: domain`, `process_domain_root: SddIA/library/codexes/codex-kalma2-assistant/process`, `process_contract_version: 1.4.0`. EM update códice membership += nombre. EM update `email-noise-digest` (fase notificación menciona markup). Prohibido default `[0]`. Handler/tests ≠ DA-2. No «arreglar» membership de `email-quick-action-ingest`. |
| L-EXEMPT | `P-EXEMPT-C` ya en `email_triage.rs` (max\|high). Este PBI solo emite `max`. CA-9 = siguiente `Email_Received` no cierra `C-NOREPLY`. |
| L-NUCLEO | `NUCLEO-INTERCEPCION-HABITOS-KALMA2` = NL. Fuera de alcance. |
| L-IMAP | Cero STORE. Preferencia aplica al siguiente correo. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. |

## Fuera

Nuevo daemon; cron Core; utterance/texto libre; hash completo en callback; suscribir ingest a Telegram; nested `execute-process`; reabrir lote `noise`; membership de quick-action; acuse táctico Telegram como CA.
