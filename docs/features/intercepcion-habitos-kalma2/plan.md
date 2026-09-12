---
feature_name: intercepcion-habitos-kalma2
created: "2026-09-12"
process: feature
phases:
  - forge-action-em
  - genome-conscience
  - handler-tendon-dispatch
  - destilar-hint-hash
  - triage-key-candidates
  - tests-lab
  - evolution-register
branch_name: feat/intercepcion-habitos-kalma2
persist_ref: docs/features/intercepcion-habitos-kalma2
execution_id: "54b02c30-b579-4d0b-a7e0-272d3c3c6af0"
document_id: PBI-NUCLEO-INTERCEPCION-HABITOS-KALMA2
---

# Plan — intercepcion-habitos-kalma2

## Orden

1. **Forge acción** — `entity-manager` update `dispatch-aiua-intent` 1.1.0 (tendón hábito + restauración de cuerpo).
2. **Conscience** — fila `delegar_habito` en `aiua_core.md` §6.
3. **Handler despacho** — `aiua_intent.rs`: `HABIT_TENDON`, matriz, emit reutilizado, tests.
4. **Destilación** — `canonical_subject_key_from_hint` + `preference_from_event_payload` + tests core/ingest.
5. **Triaje** — candidatos de clave + test CA-8; overlay latido CA-9.
6. **Evolution** — `sddia-qa evolution-register` + log.
7. **CA-10** — `cargo test -p execute-process -p user-preference-core`.

## Forja DA-2

Prefijo RAW KERNEL antes de EM. Semilla en `.tmp/` (gitignore). `SDDIA_AGENT_RELAY_IDE=1`.

No editar a mano `SddIA/actions/`. No mutar `SddIA/events/`.

## Commit

1. Planificación (este árbol + PBI v1.2.0).
2. Implementación (código + genoma EM + tests + evolution).
3. Cierre documental en rama tras CI verde (`validacion.md` APTO + PBI `done/`) antes de `accept-pr`.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Hashear `subject_key` existente rompe `hash-juan-smoke` | L-KEY: hash solo fallback de hint |
| Default `{level:high}` en mute | Default `muted:true` solo si predicado mute y falta `muted` |
| CA-8 con `noreply@` | Fixture `alertas@computrabajo.com` |
| EM update trunca I/O de la acción | Enviar `action_inputs`/`action_outputs`/`orchestration_logic` completos; UUID inmutable |
| `validate_ecst_event` en tmp sin clase | Copiar `user-preference-change-requested.md` al fixture como el test SDLC |
| Dual LanceDB en CI sin tabla | `put_revision_durable` no-op réplica si tabla ausente; JSON SSOT basta |
