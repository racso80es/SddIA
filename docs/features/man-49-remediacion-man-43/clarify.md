---
feature_name: man-49-remediacion-man-43
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Estabilización de requisitos — remediación post-merge MAN-43 (A-01..A-04) hasta APTO_TOTAL Argos
branch_name: feat/man-49-remediacion-man-43
persist_ref: docs/features/man-49-remediacion-man-43
pbi_ref: MAN-49
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-7e574af7
document_id_seed_alt: PBI-2e55b337
phase: mayeuta-stabilization
capability_id: "eda:event-manager"
contract: "eda.event_manager"
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-49.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/2e407038-fe9a-4bc6-8fd4-ca4cc19355f2"
audit_ref: docs/audits/man-43-post-merge-audit-20261117.md
parent_commit: ea20bd5bfc313a58168df213cadf02ba4e918320
parent_pr: 454
execution_id: "f01f024a-812e-4de3-a80a-6b6f3f2e0635"
agents: mayeuta
---
# Clarificación — man-49-remediacion-man-43

Transcript Mayeuta (2026-11-20). Fase **Estabilización de Requisitos** del proceso `feature`.
Canal tracker: **MAN-49** (padre **MAN-43**). Remediación de hallazgos abiertos de la auditoría post-merge; **no** reabre L-1..L-7 de MAN-43 salvo lo necesario para cerrar A-01..A-04.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-49** (provider Linear). Canal: `.SddIA/state/tracker-outbound/MAN-49.jsonl` (`refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `feature`). |
| Padre / origen | **MAN-43** (`PBI-0f4f0f5f`, épica `MAN-40` / `MAN-32`); `main` @ `ea20bd5bfc313a58168df213cadf02ba4e918320`, PR **#454**. |
| Fuente vinculante | `docs/audits/man-43-post-merge-audit-20261117.md` — **`CUMPLIMIENTO PARCIAL — NO APTO_TOTAL`** (14/19 ACs; hallazgos **A-01..A-04**). Materializado en disco (MCP). |
| Documento refine | **PBI-7e574af7** (`uuid` `7e574af7-ce14-4e69-9367-b2952387e34b`). Workspace: `.SddIA/workspaces/refine-item/2e407038-fe9a-4bc6-8fd4-ca4cc19355f2/` (`verdict: refined`, `open_questions: []`). |
| Semilla ciclo feature | Prompt operador cita `document_id: PBI-2e55b337` (`uuid` `2e55b337-0fe6-486a-ab5f-b197a18f7e34`). Contenido DoR alineado a R-01..R-04. **L-DOC-CANON**. |
| Problema estabilizado | Desacople a `EventManagerPort` parcialmente cumplido: creación FS de topología en raíz, `std::fs` residual sobre bus/`eda://`, deriva documental legacy, evasión/allowlist de gates. |
| Modo | `branch_pr` / `feature` — remediación focal A-01..A-04; hijos de forja = laudos R-01..R-04 (no H1..H6 de MAN-43). |
| `execution_id` | **f01f024a-812e-4de3-a80a-6b6f3f2e0635** |

## D1 — Matriz hallazgo × laudo de remediación

| Hallazgo (auditoría) | Severidad | ACs parentales afectados | Laudo remediación |
|----------------------|-----------|--------------------------|-------------------|
| **A-01** — `ensure_*` / `ensure_dirs` crean `orchestration/`, `telemetry/`, etc. en raíz | P0 | AC-5, AC-13 (MAN-43) | **R-01** |
| **A-02** — `std::fs` residual en `event-watcher` / `eda_sweep` sobre rutas/`eda://` | P1 | AC-5, AC-6 | **R-02** |
| **A-03** — Deriva `event_file_path` / `.events/` / `SDDIA_EVENT_STORE` en README, reglas, 22 genomas | P1 | AC-3, AC-14 | **R-03** |
| **A-04** — `concat!` evasión, `INTEGRATION_SRC` (38), auto-sello Tekton | P2 | AC-11, AC-13 + gobernanza | **R-04** |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **R-01 (A-01)** | `ensure_fractal_dirs`, `ensure_bus_dirs` y `ensure_event_bus_topology` **no** crean directorios (no-op o eliminación de llamadas). `EdaBusTopology::ensure_dirs` **no** ejecuta `fs::create_dir_all` (salvo tests explícitos en `tempdir` si Dedalo lo acota). Tras tests/daemons/CLI: cero `orchestration/`, `telemetry/`, `pending/`, `domain/`, `dead-letter/` o `progress/` (ni equivalentes de topología legacy) en la raíz del repo. Regresión en `eda_event_token_hygiene.rs`. Borrar residuos ya creados en workspace. |
| **R-02 (A-02)** | `event-watcher` opera solo vía `EventManagerPort` (`get_event` / `list_pending`); sin `fs::read_dir` / `is_file` / `fs::read_to_string` sobre rutas de bus ni URIs `eda://`. Eliminar helpers FS muertos de `eda_sweep.rs` (`safe_remove_path`, `ensure_state_header`, `maybe_purge_processing_header`, fallback FS de `move_to_dead_letter`). Ampliar `gate_event_manager_port` para prohibir `fs::read_dir`, `fs::create_dir_all`, `fs::remove_file` y `fs::rename` en `event-watcher` y `eda_*.rs` del daemon-runtime. |
| **R-03 (A-03)** | `README.md` y `.antigravity/rules/tekton-fire-and-forget.md` alineados a `event_uri` (`eda://{family}/{event_id}`) / `EventManagerPort`. Los **22** contratos del genoma citados en la auditoría se actualizan **exclusivamente** vía `execute-process --process entity-manager`. `eda_event_token_hygiene::scan_genome_contract` cubre el contenido **completo** de `SddIA/**/*.md` y `README.md` (no solo claves YAML de frontmatter). |
| **R-04 (A-04)** | Sin evasión `concat!` en `eda_event_token_hygiene.rs` (literal `".events/"` restaurado; fichero en `path_allowlisted` de `gate_eda_unit_no_real_backend`). `INTEGRATION_SRC` vacío/migrado (`entries = []`) en `eda-unit-backend.toml`. Suite + clippy + gates en verde. Sello final **`APTO_TOTAL` solo por Argos** tras `pull-request-review` (prohibido auto-sello Tekton). |
| **L-DOMAIN** | Frontera productiva = `EventManagerPort` / `EventUri`. Este ciclo **no** añade capacidades nuevas del puerto; solo remedia A-01..A-04. |
| **L-SUPERSEDE-SCOPE** | Laudos L-1..L-7 / H1..H6 de MAN-43 **permanecen**; no se reabre el *qué* parental salvo el cierre de brechas A-01..A-04. |
| **L-NON-GOALS** | No eliminar `SddIA/events/` ni `.SddIA/events/`. No nuevas ops EventManager fuera de remediación. No mutar `docs/todos/` desde Tekton/Argos/Mayeuta (solo `agent:cumulo` / `Kaizen_Alert_Required`). |
| **L-DOC-CANON** | Canónico de refine/Linear = **PBI-7e574af7**. Semilla prompt feature = **PBI-2e55b337** (mismo `tracker_ref` MAN-49, DoR alineado). Unión de ACs: se adopta la formulación **más estricta** del prompt (dirs `dead-letter`/`progress`, CLI, `path_allowlisted`, `--test-threads=1`) por ser compatible con el plan de fix de la auditoría. |
| **L-ARGOS-SEAL** | `APTO_TOTAL` de cierre = dictamen Argos. Tekton no escribe el sello final en `validacion.md` / auditorías parentales. |
| **L-GENOME** | Mutaciones de entidades del genoma **solo** vía `entity-manager`. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. Evidencia MCP `git_status` preferida; no bypass raw destructivo. |
| **L-TRUTH** | No inventar éxito: si gates/tests no materializan verde, el check permanece NO_APTO. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`. |

## D3 — Criterios de aceptación del ciclo (producto)

| ID | Criterio |
|----|----------|
| **AC-1 (A-01)** | Cero creación de directorios `orchestration/`, `telemetry/`, `pending/`, `domain/`, `dead-letter/` o `progress/` en la raíz al ejecutar tests, daemons o CLI; regresión en `eda_event_token_hygiene`. |
| **AC-2 (A-02)** | `event-watcher` y `eda_sweep` operan exclusivamente vía `EventManagerPort`, sin `std::fs` sobre rutas de bus ni URIs `eda://`; `gate-event-manager-port` prohíbe las ops FS de R-02. |
| **AC-3 (A-03)** | `README.md`, `.antigravity/rules/tekton-fire-and-forget.md` y los 22 contratos del genoma sincronizados a `event_uri` / `EventManagerPort` vía `entity-manager`; escaneo de higiene sobre `SddIA/**/*.md` + `README.md`. |
| **AC-4 (A-04)** | Sin `concat!` de evasión en `eda_event_token_hygiene.rs`; fichero en `path_allowlisted` de `gate_eda_unit_no_real_backend`; `INTEGRATION_SRC` vacío (`entries = []`) en `eda-unit-backend.toml`. |
| **AC-5 (Cierre)** | `cargo test --workspace -- --test-threads=1`, `cargo clippy --workspace -- -D warnings` y `sddia-index-integrity --verify` en verde; dictamen `APTO_TOTAL` emitido por **Argos** tras `pull-request-review`. |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Reabrir L-1..L-7 / H1..H6 de MAN-43? | **No** — solo remediación A-01..A-04 (**L-SUPERSEDE-SCOPE**). |
| 2 | ¿`document_id` PBI-2e55b337 vs PBI-7e574af7? | Canónico refine = **PBI-7e574af7**; semilla prompt = alt metadato; DoR unificado (**L-DOC-CANON**). |
| 3 | ¿Quién sella `APTO_TOTAL`? | **Solo Argos** (**L-ARGOS-SEAL** / **R-04**). |
| 4 | ¿Eliminar `SddIA/events/` o `.SddIA/events/`? | **No** (**L-NON-GOALS**). |
| 5 | ¿Forjar genoma a mano? | **No** — solo `entity-manager` (**L-GENOME** / **R-03**). |
| 6 | ¿Vaciar también `event-manager-port.toml`? | Ya vacío según auditoría; foco = `eda-unit-backend.toml` / `INTEGRATION_SRC` (**R-04**). |

## D5 — Riesgos (vigilancia, no bloqueo Mayeuta)

| Riesgo | Tratamiento |
|--------|-------------|
| Directorios vacíos invisibles a `git status --porcelain` | Aserción de higiene + limpieza explícita de residuos (**R-01**). |
| Gate léxico insuficiente (solo literales) | Ampliar `gate_event_manager_port` a ops FS concretas (**R-02**). |
| Escaneo genoma solo frontmatter | Ampliar `scan_genome_contract` a cuerpo completo + README (**R-03**). |
| Presión CI → `concat!` / allowlist | Allowlist honesta + `INTEGRATION_SRC=[]` + sello Argos (**R-04**). |
| Colapso Tekton/Argos en cierre | Prohibir auto-sello; `pull-request-review` emite dictamen (**L-ARGOS-SEAL**). |

## D6 — Handoff Dedalo / Tekton

1. Consumir `clarify.md` + `objectives.md` en solo lectura; **no** reabrir R-01..R-04.
2. Blueprint (`spec.md` / `plan.md`) acota touchpoints: `sddia-daemon-runtime`, `event-watcher`, gates/higiene, README/reglas, 22 genomas vía `entity-manager`.
3. Orden sugerido de forja: R-01 → R-02 → R-04 (gates) → R-03 (docs/genoma) → verificación AC-5 → Argos.
4. Mayeuta **no** diseña implementación ni emite `delegates_to` de código.

## D7 — Veredicto

**ok** — Requisitos estabilizados. El *qué* queda cerrado en laudos **R-01..R-04** / ACs **1..5**; fuente vinculante auditoría post-merge leída; refine `verdict: refined`; sin preguntas abiertas bloqueantes. Tracker **MAN-49** / documento canónico **PBI-7e574af7** / padre **MAN-43** / `execution_id` **f01f024a-812e-4de3-a80a-6b6f3f2e0635**.
