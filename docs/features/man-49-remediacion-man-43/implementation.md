---
feature_name: man-49-remediacion-man-43
created: "2026-11-20"
updated: "2026-11-20"
process: feature
branch_name: feat/man-49-remediacion-man-43
persist_ref: docs/features/man-49-remediacion-man-43
pbi_ref: MAN-49
parent_ref: MAN-43
document_id: PBI-7e574af7
phase: ejecucion
status: implemented
norm_ref: features-documentation-pattern@1.3.1
execution_id: "f01f024a-812e-4de3-a80a-6b6f3f2e0635"
---

# Implementación — man-49-remediacion-man-43

Remediación de los 4 hallazgos (`A-01`..`A-04`) de [man-43-post-merge-audit-20261117.md](../../audits/man-43-post-merge-audit-20261117.md):

## 1. R-01 (A-01 — Cero creación de directorios de topología en raíz)
- [lib.rs](../../../SddIA/sddia-daemon-runtime/src/lib.rs), [eda_bus.rs](../../../SddIA/sddia-daemon-runtime/src/eda_bus.rs), [eda_bus_topology.rs](../../../SddIA/engine/execute-process/src/engine/eda_bus_topology.rs): `ensure_fractal_dirs`, `ensure_bus_dirs` y `ensure_event_bus_topology` convertidos en no-op (retornan la topología sin `fs::create_dir_all`).
- [thermodynamic.rs](../../../SddIA/engine/execute-process/src/engine/thermodynamic.rs), [cli_detach.rs](../../../SddIA/engine/execute-process/src/engine/cli_detach.rs), [eda_bus.rs](../../../SddIA/engine/execute-process/src/engine/eda_bus.rs): eliminadas las invocaciones residuales a `fs::create_dir_all` sobre `tele_dir` / `orch_dir` / `FRACTAL_FAMILIES` que recreaban `orchestration/` y `telemetry/` en la raíz al ejecutar `execute-process`.
- [eda_event_token_hygiene.rs](../../../SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs): añadido test `eda_event_token_hygiene_repo_root_no_bus_topology_dirs`.

## 2. R-02 (A-02 — `event-watcher` y `eda_sweep` exclusivamente vía `EventManagerPort`)
- [main.rs](../../../SddIA/daemons/event-watcher/src/main.rs): `prune_routed_ok`, `fractal_side_effect_committed`, `watcher_skip_reason` y `apply_route_outcome_fractal` migrados de `fs::read_dir` / `Path::is_file` / `fs::read_to_string` a `EventUri` + `EventManagerPort::get_event`.
- [eda_sweep.rs](../../../SddIA/sddia-daemon-runtime/src/eda_sweep.rs): eliminados los helpers FS muertos (`safe_remove_path`, `ensure_state_header`, `maybe_purge_processing_header`, fallback FS de `move_fractal_to_dead_letter`) e imports sin uso (`std::fs`, `std::thread`, `std::time::Duration`).
- [gate_event_manager_port.rs](../../../SddIA/tools/sddia-qa/src/gate_event_manager_port.rs): ampliado para vetar `fs::read_dir`, `fs::create_dir_all`, `fs::remove_file` y `fs::rename` en `event-watcher` y `sddia-daemon-runtime/src/eda_*.rs`.

## 3. R-04 (A-04 — Gates honestos y poda de `INTEGRATION_SRC`)
- [eda_event_token_hygiene.rs](../../../SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs) y [gate_eda_unit_no_real_backend.rs](../../../SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs): eliminado `concat!('.', "events/")`, restaurado `".events/"` y añadido `eda_event_token_hygiene.rs` a `path_allowlisted`.
- [gate_eda_unit_no_real_backend.rs](../../../SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs) y [eda-unit-backend.toml](../../../SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml): reducida `integration_src` de **34 entradas + 2 comodines** (`kalma2`, `email_`) a **3 entradas** (`route_domain_core.rs`, `handlers/system_vitality.rs`, `handlers/daemon_heartbeat.rs`), moviendo `tests/instance_creator.rs` a integración y limpiando 31 módulos unitarios.
- **Nota L-TRUTH (3 entradas restantes):** `route_domain_core.rs`, `system_vitality.rs` y `daemon_heartbeat.rs` dependen en `#[cfg(test)]` de `eda_port::test_lab` (compilado solo bajo `#[cfg(test)]`) y helpers privados de módulo; moverlos a `tests/` requiere refactorizar `test_lab` a un crate o feature de test dedicada fuera del alcance de este fix.

## 4. R-03 (A-03 — Convergencia documental y genoma)
- Actualizados [README.md](../../../README.md) y [tekton-fire-and-forget.md](../../../.antigravity/rules/tekton-fire-and-forget.md) a `event_uri` / `EventManagerPort`.
- Actualizadas entidades del genoma (`process`, `action`, `event`, `norm`, `tool`, `agent`, `daemon`) mediante `execute-process --process entity-manager` y recalculadas firmas (`recalc-process-hash-signatures --write`).
- Actualizados documentos sin frontmatter (`CONSTITUTION_CORE.md`, `events-contract.md`, `daemons-contract.md`, `tools-contract.md`, `*/index.md`, `progress-trace-contract.md`, `capsule-json-io.md`, `external-ai-constraints.md`, `obediencia-procesos.md`, `templates/eda-instance-events/README.md`).
- Activado `eda_event_token_hygiene_genome_and_readme_body` en [eda_event_token_hygiene.rs](../../../SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs) (excluyendo `SddIA/evolution/` histórico, `clean-project-ephemeral.md` y falsos positivos `directories.events/`).
