---
feature_name: email-digest-preference-reply
created: "2026-09-12"
process: feature
branch_name: feat/email-digest-preference-reply
persist_ref: docs/features/email-digest-preference-reply
pbi_ref: docs/todos/pending/[OPERATIVO] Réplica del digest de ruido → preferencias.md
execution_id: "b6930240-b8a7-440a-ab0d-f9f853b1855a"
document_id: PBI-EMAIL-DIGEST-PREFERENCE-REPLY
pbi_uuid: "bba79183-b76b-4e05-aa02-5adf1d09b500"
pbi_version: "1.2.0"
status: in-progress
---

# Objetivos — email-digest-preference-reply

## Misión

Cerrar el lazo digest de ruido → pulsación Telegram → preferencia `explicit_user` → `P-EXEMPT-C` en el siguiente correo. Botonera por remitente listado. Token corto. Cero PII en `callback_data` y en los ECST de réplica.

## Alcance (manifiesto)

- Ciclo `feature` inicializado (`execution_id` `b6930240-…`). Corte L0: planificación. Código y genoma en L1–L5 del mismo PR.
- Extender handler `email-noise-digest`: tokens, `reply_markup`, pie CTA, persistencia `tokens`.
- Proceso nuevo `email-digest-preference-reply` en root kalma2 (no Core ni software-engineering).
- Handler nativo + dispatch + allowlist EDA + suscripción `TelegramCallback_Received`.
- Tests `email_noise_digest` + `email_digest_preference_reply` (incluye lazo P-EXEMPT-C).
- Cierre documental en rama + DCC + PR. `accept-pr` condicionado a CI verde.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`. Rama `feat/email-digest-preference-reply`.
- DA-2/DA-4: topología `objectives.md` en rama **antes** de mutar process de códice / `library_codexes`.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CA-CI verde (`run_id`).
- `CONSTITUTION_CORE` Filtro A: no Done sobre diffs locales.
- Matriz: `P-EXEMPT-C` = `explicit_user` + `priority` max\|high + `active`. `P-MUTE-SENDER` = `mute` ∧ `value.muted=true`.
- `process-creator` v1.2.0: alta domain nueva exige `process_jurisdiction: domain` + `process_domain_root` explícito.
- ECST: `TelegramCallback_Received` v1.0.0; `User_Preference_Change_Requested` sin campos `authority`/`status`.

## Criterios (PBI v1.2.0)

| ID | Criterio |
|----|----------|
| CA-1 | `callback_data` ≤ 64 bytes, `dpref:<max\|mute>:<token-hex>`. |
| CA-2 | Cero addr/asunto/cuerpo en botones y ECST de réplica. |
| CA-3 | `tokens` en estado; skip/vacío no borran el mapa. |
| CA-4 | Despacho EDA con `event_file_path`. |
| CA-5 | Prefijo ajeno → skipped, exit 0. |
| CA-6 | Token miss → skipped-expired, cero emit. |
| CA-7 | max → activate + `priority_level=max` → asiento `explicit_user`/`active`. |
| CA-8 | mute persiste `{muted:true}`; ign/no-tap cero escritura. |
| CA-9 | Tras max, `noreply@` no cierra `C-NOREPLY` (`P-EXEMPT-C`). |
| CA-10 | Process/códice solo vía `entity-manager` (domain / kalma2 / 1.4.0). |
| CA-11 | Tests locales verdes; CA-CI = `PENDIENTE-CI` hasta `run_id`. |
