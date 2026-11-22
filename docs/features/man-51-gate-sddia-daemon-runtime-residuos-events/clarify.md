---


feature_name: man-51-gate-sddia-daemon-runtime-residuos-events
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Clarificaciones canónicas — cierre de punto ciego de gates y residuos .events
branch_name: feat/man-51-gate-sddia-daemon-runtime-residuos-events
persist_ref: docs/features/man-51-gate-sddia-daemon-runtime-residuos-events
pbi_ref: MAN-51
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-1377d0ec
document_uuid: "c3aff1f0-495b-40ab-a042-4420dd241400"
phase: mayeuta-stabilization
capability_id: "eda:event-manager"
contract: "eda.event_manager"
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-51.jsonl"
audit_ref: docs/audits/informe_de_diagnostico_desacople_eventos_sddia.md
agents: mayeuta
execution_id: "d577eabc-9947-4781-b087-7d0475963e29"
---
# Clarificación — man-51-gate-sddia-daemon-runtime-residuos-events

## Laudos de clarificación

- **L-1 (Descubrimiento de crates en gates `sddia-qa`):** `gate_event_manager_port.rs` y `gate_eda_unit_no_real_backend.rs` deben leer `[workspace].members` desde `SddIA/Cargo.toml` (expandiendo globs `*` como `engine/*`, `tools/*`, `daemons/*`, `skills/*`, `interfaces/*` y rutas explícitas como `sddia-daemon-runtime`, `sddia-core`, `sddia-runtime-parity`, `ecosystem-health`), manteniendo las exclusiones explícitas de host/adaptadores (`sddia-event-manager-host`, `infrastructure/`).
- **L-2 (Resolución de repo en `fractal_bus::stamp_fractal_delivery_state`):** En lugar de buscar un ancestro llamado `.events`, `repo_root_from_event` debe buscar hacia arriba el marcador SSOT `SddIA/core/cumulo.paths.json` y, si no aparece (p. ej. URI `eda://...`), usar `crate::core::repo::find_repo_root()`.
- **L-3 (`transit-event-payload` y `hook_common.sh`):** `transit-event-payload` no debe tener defaults `.events/*` ni invocar `fs::create_dir_all` sobre topología de bus; si se invoca sin topología FS explícita, falla con error explícito indicando que el bus opera vía `EventManagerPort`. En `hook_common.sh`, `should_skip_pre_push_present` delega en `_gh_pr_state_for_branch` eliminando `eda_bus_dirs()` sobre `.events/*`.
- **L-4 (Contrato `events-contract.md`):** `SddIA/events/events-contract.md` (§§ 3–4) se alinea exclusivamente a `EventManagerPort` v2 + `EventUri`, preservando intacta la sección § 5 y siguientes.
