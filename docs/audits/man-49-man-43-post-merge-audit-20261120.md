# Informe de Auditoría Post-Merge — `MAN-49` y `MAN-43`

- **PBIs auditados:**
  - `MAN-49` (`PBI-7e574af7` / `PBI-2e55b337`, PR #455, padre `MAN-43`)
  - `MAN-43` (`PBI-0f4f0f5f`, PR #454, épica `MAN-40` / `MAN-32`)
- **Rama / Commit auditado:** `main` @ `ee17d9537a1b567505049bb1a2bf3245c7560d39`
- **Fecha de auditoría:** 2026-11-20
- **Dictamen de auditoría independiente:**
  - **`MAN-49`:** **`CUMPLIMIENTO PARCIAL ALTO (4/5 ACs cumplidos al 100%, 1 AC parcial con excepción documentada L-TRUTH)`**
  - **`MAN-43` (acumulado post-`MAN-49`):** **`CUMPLIMIENTO PARCIAL — 16/19 ACs cumplidos, 3 ACs con brechas residuales (AC-5, AC-11, AC-14) agrupadas en 3 hallazgos (B-01..B-03)`**

---

## 1. Contraste contra las Auditorías Previas

| Auditoría | Veredicto declarado entonces | Contraste contra la realidad física en `main` (`ee17d953`) |
|---|---|---|
| `docs/audits/man-32-lineage-man-43-remediation-20261111.md` | `NO_APTO_TOTAL` (`F-01`..`F-12`) en H1; re-etiquetado en frontmatter a `APTO_TOTAL` en O15 (`2026-11-17`) | **Exacta en su diagnóstico original (`F-01`..`F-12`).** Los hallazgos de núcleo (`F-01` igualdad exacta Redb, `F-02` schema v2, `F-04..F-08` suite contractual, `F-11` demolición de `sddia-event-store`) están cerrados en `main`. Sin embargo, `F-03` ("doble materialización puerto + `.events/`") sigue **parcialmente vivo** en los 6 escritores de fallback de dead-letter (`B-01`). |
| `docs/audits/man-43-o6-remediation-20261113.md` | `NO_APTO_TOTAL` en cuerpo (§ Dictamen L24), pero frontmatter y § Cierre L72 cambiados a `APTO_TOTAL` en O15 | **Inconsistencia interna:** el cuerpo del documento aún afirma *"El feature MAN-43 no alcanza APTO_TOTAL (AC-13)"* (L24) y la tabla L63 mantiene *"AC-13: Abierto — este laudo NO_APTO_TOTAL"*, mientras que el frontmatter (L12) y el párrafo final añadido en O15 (L72) declaran `APTO_TOTAL`. |
| `docs/audits/man-43-o14d-reaudit-ac13-20261116.md` | `APTO_TOTAL_PROPUESTO` (19/19 `PASS`, *"0 ficheros productivos contienen `.events/`"*) | **Falso negativo en O14d:** declaró `AC-4` y `AC-5` `PASS` basándose en un oráculo léxico que solo buscaba `".events/"` con barra final. Ignoró `bus_defaults_from_root(".events")` en `eda_bus_topology.rs` y la creación de `orchestration/` y `telemetry/` en la raíz del repo. |
| `docs/audits/man-43-post-merge-audit-20261117.md` | `CUMPLIMIENTO PARCIAL — NO APTO_TOTAL` (14/19 ACs; hallazgos `A-01`..`A-04`) | **Confirmada y remediada en un ~90% por `MAN-49`:** sus 4 hallazgos (`A-01`..`A-04`) guiaron `MAN-49`. Sin embargo, al centrarse `A-01` en `orchestration/` y `telemetry/` en la raíz y `A-02` en `event-watcher`/`eda_sweep`, quedó fuera del radar la escritura en `.events/dead-letter/` dentro de `execute-process` (`eda_bus_topology.rs`). |
| `docs/features/man-49-remediacion-man-43/validacion.md` | `global: APTO` (`PASS_F5_VERDICT` emitido por Argos en PR #455) | **Gobernanza correcta (`L-ARGOS-SEAL` cumplido):** Tekton dejó `PENDIENTE_ARGOS` y fue Argos quien evaluó y selló `APTO`. No obstante, Argos marcó `DOC_EVOLUTION: NO_APTO` porque evaluó `d619d307` sin re-evaluar la cicatriz `SddIA/evolution/dcb149cd-c795-4737-8446-e03ab907fe1d.md` ya presente en `e6b519da`. |

---

## 2. Verificación de Cierre de `A-01`..`A-04` (`MAN-49`)

| Hallazgo (`2026-11-17`) | Laudo `MAN-49` | Estado en `main` (`ee17d953`) | Evidencia verificada |
|---|---|---|---|
| **`A-01` (P0):** Creación de `orchestration/` y `telemetry/` vacíos en la raíz del repo | **R-01 / AC-1** | **CERRADO (100%)** | Eliminados los `fs::create_dir_all` en `SddIA/sddia-daemon-runtime/src/lib.rs:174-195`, `SddIA/sddia-daemon-runtime/src/eda_bus.rs:140`, `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs:164`, `SddIA/engine/execute-process/src/engine/thermodynamic.rs:180-181`, `SddIA/engine/execute-process/src/engine/cli_detach.rs:67` y `SddIA/engine/execute-process/src/engine/eda_bus.rs:170`. Raíz limpia (`ROOT_CLEAN`) y test `eda_event_token_hygiene_repo_root_no_bus_topology_dirs` en verde. |
| **`A-02` (P1):** Operaciones `std::fs` residuales en `event-watcher` y `eda_sweep` | **R-02 / AC-2** | **CERRADO (100%)** | `0` llamadas `std::fs` en `SddIA/daemons/event-watcher/src/main.rs` y `0` llamadas productivas `std::fs` en `SddIA/sddia-daemon-runtime/src/eda_sweep.rs` (solo en `#[cfg(test)]` L594-L620). Gate `SddIA/tools/sddia-qa/src/gate_event_manager_port.rs` activo y con `0` violaciones sobre `216` archivos. |
| **`A-03` (P1):** Deriva documental en `README.md`, reglas y 22 contratos del genoma | **R-03 / AC-3** | **CERRADO EN LOTE `MAN-49` (Parcial global)** | `README.md`, `.antigravity/rules/tekton-fire-and-forget.md` y los contratos del lote de `MAN-49` tienen `0` ocurrencias de `.events/` y `event_file_path` (verificado por `eda_event_token_hygiene_genome_and_readme_body`). Quedan 6 menciones menores fuera de ese lote (ver **`B-03b`**). |
| **`A-04` (P2):** `concat!` de evasión, allowlist `INTEGRATION_SRC` (34+2) y auto-sello de Tekton | **R-04 / AC-4 / AC-5** | **PARCIAL (`concat!` y sello cerrados; `INTEGRATION_SRC` = 3)** | `concat!` eliminado en `SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs:336`; sello emitido por Argos; `INTEGRATION_SRC` reducida un **91%** (de 34 entradas + 2 comodines a **3 entradas** en `SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml:33-37`). |

---

## 3. Brechas Residuales Abiertas en `main` (`B-01` .. `B-03`)

### Hallazgo `B-01` (Severidad P1 — Incumplimiento residual `MAN-43 AC-4 / AC-5`)
**`execute-process` todavía escribe ficheros JSON físicos en `.events/dead-letter/` y `.events/dead-letter/subscribers/` en tiempo de ejecución.**
- **Evidencia empírica en disco (`2026-11-20`):**
  - `.events/dead-letter/67a88418-a9b4-4e32-96dd-7eda49a5f3af.cerbero-di-rbac.json` (`08:02`, `cerbero_di_rbac`).
  - `.events/dead-letter/c98eab3f-98ed-4571-9f60-182ce7b4276f.capability-di-gate.json` (`08:02`, `capability_di_gate`).
  - `.events/dead-letter/subscribers/976783ef-88bc-4ad8-8080-c158bd81d716.argos.notify-humanized-pr-merged.json` (`09:13`, durante `accept-pr`).
- **Causa raíz en código productivo:**
  1. `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs:63-85` define `bus_defaults_from_root(".events")` (sin barra final, eludiendo el filtro léxico `".events/"` de `eda_event_token_hygiene`) y construye `format!("{event_bus}/dead-letter")` y `format!("{event_bus}/dead-letter/subscribers")`.
  2. `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs:206-214` (`write_json_atomic`) ejecuta `fs::create_dir_all(parent)` + `fs::write` + `fs::rename`.
  3. Seis puntos productivos de `execute-process` invocan `write_json_atomic` sobre `topo.dead_letter` / `bus.dead_letter_subscribers`:
     - `SddIA/engine/execute-process/src/engine/route_domain_core.rs:2277-2287` (`write_dead_letter_fallback`).
     - `SddIA/engine/execute-process/src/engine/capability_di_gate.rs:257-279` (`write_di_dead_letter`).
     - `SddIA/engine/execute-process/src/engine/capability_di_resolver.rs:279-304` (`write_resolve_dead_letter`).
     - `SddIA/engine/execute-process/src/engine/capability_di_output_validator.rs:68-95` (`write_output_dead_letter`).
     - `SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs:182-205` (`write_rbac_dead_letter`).
     - `SddIA/engine/execute-process/src/engine/cerbero_di_envelope.rs:207-231` (`write_envelope_dead_letter`).
- **Fix requerido:**
  - Migrar los 6 escritores `write_*_dead_letter` a `EventManagerPort` (`move_to_dlq` / `fail_delivery` o registro en store sin I/O en `.events/dead-letter/`).
  - Eliminar `bus_defaults_from_root(".events")` y `write_json_atomic` en `eda_bus_topology.rs`.
  - Ampliar `eda_event_token_hygiene` para vetar el literal `".events"` (con o sin barra final) en código productivo.

### Hallazgo `B-02` (Severidad P2 — Higiene operativa de binarios y workspace)
**Binarios precompilados antiguos en `SddIA/target/`, daemon `event-watcher` anterior a `MAN-49` y directorio `.events/` histórico no purgado.**
- **Evidencia:**
  1. `SddIA/target/release/sddia-workspace-server` (`28 sep`) y `SddIA/target/debug/sddia-workspace-server` (`7 nov`) no fueron recompilados tras migrar `SddIA/tools/sddia-workspace-server/src/telemetry.rs:63` a `publish_ecst`. Al ser invocado como servidor MCP durante `feature`, el binario antiguo escribió 7 ficheros en `.events/telemetry/*.json` el `2026-11-20` a las `07:45`.
  2. El daemon `event-watcher` en ejecución en segundo plano (PID `26551`) sigue corriendo una imagen de `SddIA/target/release/event-watcher` arrancada el `16 nov`.
  3. El directorio `.events/` en la raíz del workspace contiene artefactos históricos desde el `30 oct`.
- **Fix requerido:**
  - Recompilar `sddia-workspace-server` y `event-watcher` en `debug` y `release` (`cargo build --release -p sddia-workspace-server -p event-watcher -p execute-process`), reiniciar `event-watcher`, eliminar el árbol `.events/` del workspace y añadir aserción de ausencia de `.events` en `assert_repo_root_bus_dirs_absent`.

### Hallazgo `B-03` (Severidad P2 — Remanentes `MAN-43 AC-11 / AC-14` y `MAN-49 AC-4`)
**3 entradas restantes en `INTEGRATION_SRC` y 6 referencias documentales secundarias.**
- **Ubicación:**
  - **`B-03a`:** `SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml:33-37` y `SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs:64-68` mantienen `route_domain_core.rs`, `handlers/system_vitality.rs` y `handlers/daemon_heartbeat.rs` porque sus tests invocan `eda_port::test_lab` (compilado bajo `#[cfg(test)]` en `SddIA/engine/execute-process/src/engine/eda_port.rs:500-501`) y helpers privados.
  - **`B-03b`:** `.events/` en `.antigravity/rules/[ARQUITECTURA] constitution_core.md`; `SDDIA_EVENT_STORE` en `SddIA/events/events-contract.md:76` y en 4 procesos del códice (`SddIA/library/codexes/codex-software-engineering/process/{accept-pr,delivery-close-cycle,pull-request-review,refine-item}.md`); inconsistencia interna de texto en `docs/audits/man-43-o6-remediation-20261113.md:24,63`.
- **Fix requerido:**
  - Extraer o habilitar `test_lab` para tests de integración y migrar los tests con Redb real de `route_domain_core.rs`, `system_vitality.rs` y `daemon_heartbeat.rs` a `SddIA/engine/execute-process/tests/`, dejando `integration_src = []`.
  - Limpiar `.antigravity/rules/[ARQUITECTURA] constitution_core.md`, `SddIA/events/events-contract.md:76`, los 4 procesos del códice (vía `entity-manager`) y la nota de `man-43-o6-remediation-20261113.md`.
