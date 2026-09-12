---
feature_name: intercepcion-habitos-kalma2
created: "2026-09-12"
process: feature
purpose: Estabilización Filtro A PBI v1.2.0; tendón delegar_habito
version_clarify: "1.0.0"
execution_id: "54b02c30-b579-4d0b-a7e0-272d3c3c6af0"
pbi_ref: docs/todos/pending/[NÚCLEO] Intercepción Ontológica y Delegación de Hábitos desde Kalma2.md
document_id: PBI-NUCLEO-INTERCEPCION-HABITOS-KALMA2
pbi_uuid: "8f3d1b22-6b9c-4e89-a512-9c3e4f7a1102"
pbi_version: "1.2.0"
---

# Clarificación — intercepcion-habitos-kalma2

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `54b02c30-b579-4d0b-a7e0-272d3c3c6af0`. Rama `feat/intercepcion-habitos-kalma2`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

## Decisiones

| ID | Laudo |
|----|-------|
| **L-EMIT** | `delegar_habito` reutiliza `user_preference_change_requested::run`. No escribir el ECST con `emitter_agent: aiua-stimulus-processing` (esa identidad es de `Aiua_Process_Requested`). Clase vigente: emisores `kalma2-bridge` y `emit-user-preference-change-requested`. |
| **L-FDM** | Filtro de Materialización ≠ matriz borrar→mute. FdM = Aiúa no ejecuta. La matriz vive en el despachador (Filtro A / no-destrucción) y se aplica aunque el LLM emita `delete`. |
| **L-MAYEUTA** | `user-preference-ingest` no invoca Mayeuta. Destilación nativa. |
| **L-KEY** | `canonical_subject_key_from_hint` es **nueva**. Si `payload.subject_key` existe, se conserva (fixtures `hash-juan-smoke`, digest SHA-256). Si solo hay `subject_hint`, se hashea. Prohibido copiar el hint en claro a `subject_key`. `subject_kind` no cae al hint; default `person`. |
| **L-MUTE-VAL** | Predicado mute sin `value.muted` → `{"muted": true}`. El default histórico `{level: high}` rompe `p_mute_sender`. |
| **L-KIND** | Email-mute desde NL: `subject_kind: person` (P-MUTE-SENDER es de remitente). `topic` no es el default del ejemplo computrabajo. |
| **L-MATCH** | Triaje: unión de `query` por `canonical_subject_key_from_addr(from)` y hashes de candidatos (display From, local-part, labels de dominio salvo TLD 2–3 letras, tokens alfanuméricos del asunto ≥3). No KNN para CA-8. |
| **L-ORDER** | L-ORDER intacto. C concluyente gana a mute. Fixture CA-8: From sin `noreply`/List-Id/asunto C-SUBJECT-NOISE. |
| **L-UTTER** | `raw_utterance` no entra al ECST. Opcional `utterance_ref` = SHA-256 hex. |
| **L-DUAL** | Persistencia = `put_revision_durable`. No reimplementar LanceDB. |
| **L-FORGE** | Update `dispatch-aiua-intent` vía `entity-manager`. `aiua_core.md` edición directa post-topología. No mutar la clase del evento. |
| **L-CI** | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo entonces. |

## Filtro A (no reintroducir)

- EXPUNGE / STORE Deleted / APPEND IMAP.
- Emisor ECST `aiua-stimulus-processing` para esta clase.
- Mayeuta en ingest.
- `subject_hint` como `subject_kind` o como `subject_key` plaintext.
- Dual-run paralelo a `put_revision_durable`.
- Join latido → ingest → triage.
- PR #285 como anatomía motora (ese PR es otro fix; tendones = #286).
