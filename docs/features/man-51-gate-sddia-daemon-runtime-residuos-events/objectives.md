---
feature_name: man-51-gate-sddia-daemon-runtime-residuos-events
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Objetivos estabilizados — cerrar punto ciego de gate-event-manager-port sobre sddia-daemon-runtime y erradicar residuos .events en código, legacy y events-contract.md
branch_name: feat/man-51-gate-sddia-daemon-runtime-residuos-events
persist_ref: docs/features/man-51-gate-sddia-daemon-runtime-residuos-events
pbi_ref: MAN-51
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-1377d0ec
document_uuid: "c3aff1f0-495b-40ab-a042-4420dd241400"
capability_id: "eda:event-manager"
contract: "eda.event_manager"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
audit_ref: docs/audits/informe_de_diagnostico_desacople_eventos_sddia.md
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-51.jsonl"
agents: mayeuta
execution_id: "e8cd02b1-e76c-4a3c-a44c-1cef29bc4704"
---
# Objetivos — man-51-gate-sddia-daemon-runtime-residuos-events

## Misión

Cerrar el punto ciego de las aduanas estáticas (`gate-event-manager-port` y `gate-eda-unit-no-real-backend`) sobre crates del workspace (especialmente `SddIA/sddia-daemon-runtime`) y erradicar los focos residuales de `.events` / protocolo `filesystem/dual` en el motor (`instance_creator.rs`, `fractal_bus.rs`), herramientas/hooks legacy (`transit-event-payload`, `hook_common.sh`, binario fósil `event-sweeper`) y el contrato maestro `SddIA/events/events-contract.md`.

Tracker **MAN-51** (padre **MAN-43**, documento **PBI-1377d0ec**, `uuid`: `c3aff1f0-495b-40ab-a042-4420dd241400`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1** | `gate_event_manager_port.rs` y `gate_eda_unit_no_real_backend.rs` escanean todos los crates miembros del workspace Cargo bajo `SddIA/` (incluyendo `sddia-daemon-runtime`, `sddia-core`, `sddia-runtime-parity`, `ecosystem-health`) y fallan ante evasiones con `concat!` que reconstruyan rutas o tokens vetados. |
| **AC-2** | `instance_creator.rs` y `fractal_bus.rs` no crean ni referencian `.events`; ambos quedan fuera de `VIRTUAL_PATH_ALLOW` en `eda_event_token_hygiene.rs`. |
| **AC-3** | `transit-event-payload` y `hook_common.sh` no tienen defaults ni creación sobre `.events/*`; no existe binario `event-sweeper` bajo `SddIA/target/`. |
| **AC-4** | `SddIA/events/events-contract.md` (§§ 3–4) describe exclusivamente el flujo `EventManagerPort` v2 + `EventUri` (sin diagrama/reglas `filesystem/dual`, `pending/` JSON, `processing/subscribers/`, `EventStore::replay` ni `event-sweeper` como protocolo vigente); `BODY_FORBIDDEN` en `eda_event_token_hygiene.rs` veta `EventStore::`, `StoreMode::`, `filesystem/dual` y `processing/subscribers`. |

## Fuera de alcance

- `MAN-52` (ampliación de `sddia-runtime-parity` y frescura de cápsulas).
- `MAN-53` (endurecimiento de `apto_coherence` / matrices AC parciales).
- `MAN-54` (blindaje de `entity-manager update` y `recalc-process-hash-signatures` en códices).
- `MAN-55` (ratchet monótono de allowlists y estandarización de prompts).
