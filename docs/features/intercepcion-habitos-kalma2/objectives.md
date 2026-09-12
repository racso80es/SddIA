---
feature_name: intercepcion-habitos-kalma2
created: "2026-09-12"
process: feature
branch_name: feat/intercepcion-habitos-kalma2
persist_ref: docs/features/intercepcion-habitos-kalma2
pbi_ref: docs/todos/pending/[NÚCLEO] Intercepción Ontológica y Delegación de Hábitos desde Kalma2.md
document_id: PBI-NUCLEO-INTERCEPCION-HABITOS-KALMA2
execution_id: "54b02c30-b579-4d0b-a7e0-272d3c3c6af0"
---

# Objetivos — intercepcion-habitos-kalma2

## Misión

Cerrar el vacío aferente de lenguaje natural: un tendón motor `delegar_habito` en el latido `aiua-stimulus-processing` que deposite `User_Preference_Change_Requested` vía la acción canónica `emit-user-preference-change-requested`, destile `subject_hint` a `subject_key` SHA-256 y permita a `email-triage-gateway` aplicar `P-MUTE-SENDER` por candidatos de From/asunto. Cero mutación IMAP. Cero RPC síncrono (DA-5). Cero emisor ECST inventado.

## Alcance

- Catálogo §6 en `aiua_core.md`: tendón `delegar_habito`.
- `is_motor_tendon` + rama de despacho en `aiua_intent.rs` (matriz defensiva borrar→mute).
- Reuso de `user_preference_change_requested::run`; `channel: kalma2`.
- `canonical_subject_key_from_hint` + destilación en `preference_from_event_payload` (hash solo si falta `subject_key`).
- Default mute `{muted: true}` si el predicado es mute y `value` no trae `muted`.
- Candidatos de clave en `email_triage.rs` (addr + tokens From/asunto).
- Forja DA-2: `entity-manager` update `dispatch-aiua-intent`.
- Tests CA-1…CA-10. Overlay lab `SDDIA_LAB_MOCK_AIUA_INTENT`.

## Fuera

Kalma2 WUI. IMAP/`email-watcher`. Nuevo evento. Mayeuta en ingest. Reescritura LanceDB. `kalma2-bridge` HTTP de preferencias. Gemini `functionDeclarations`. Join a ingest/traje en el latido.

## Ley aplicada

- `CONSTITUTION_CORE.md` Filtros C/A/B. Filtro de Materialización = Intención ≠ Ejecución (`aiua_core.md` §4).
- DA-2: `actions/` vía `entity-manager`. Conscience fuera de la tabla DA-2, post-topología feature.
- DA-5: éxito del latido = ECST sellado, no = ingest terminado.
- IMAP read-only. `events-contract` v1.1.0: sin `raw_utterance`/`body`/`snippet` en payload.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CI verde.

## Criterios (DoD PBI v1.2.0)

| ID | Objetivo |
|----|----------|
| CA-1 | Tendón documentado en §6; destino = emit canónico. |
| CA-2 | Parser extrae `delegar_habito`; `is_motor_tendon` true. |
| CA-3 | borra/elimina/limpia/delete → mute + `muted: true`; cero rama destructiva. |
| CA-4 | ECST válido; `channel=kalma2`; `emitter_agent=emit-user-preference-change-requested`; sin utterance. |
| CA-5 | Hint → SHA-256 de 64 chars; asiento sin hint en claro como key. |
| CA-6 | `activate` → `active` + `explicit_user` vía `put_revision_durable`. |
| CA-7 | Cero EXPUNGE/STORE Deleted. |
| CA-8 | Hint `computrabajo` + From `alertas@computrabajo.com` sin C-* → noise/P-MUTE-SENDER. |
| CA-9 | Latido no llama ingest ni triage. |
| CA-10 | `cargo test -p execute-process -p user-preference-core` verde. |
