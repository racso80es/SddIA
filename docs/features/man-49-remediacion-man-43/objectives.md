---
feature_name: man-49-remediacion-man-43
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Objetivos estabilizados — remediación A-01..A-04 de MAN-43 hasta APTO_TOTAL Argos (19/19 ACs parentales)
branch_name: feat/man-49-remediacion-man-43
persist_ref: docs/features/man-49-remediacion-man-43
pbi_ref: MAN-49
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-7e574af7
document_id_seed_alt: PBI-2e55b337
capability_id: "eda:event-manager"
contract: "eda.event_manager"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
audit_ref: docs/audits/man-43-post-merge-audit-20261117.md
parent_commit: ea20bd5bfc313a58168df213cadf02ba4e918320
parent_pr: 454
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-49.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/2e407038-fe9a-4bc6-8fd4-ca4cc19355f2"
execution_id: "f01f024a-812e-4de3-a80a-6b6f3f2e0635"
agents: mayeuta
---
# Objetivos — man-49-remediacion-man-43

## Misión

Cerrar las brechas abiertas de la auditoría post-merge de **MAN-43** (`PBI-0f4f0f5f`, `main` @ `ea20bd5bfc313a58168df213cadf02ba4e918320`, PR #454) hasta dictamen **`APTO_TOTAL`** emitido por Argos, alcanzando cumplimiento íntegro (**19/19 ACs**) del desacople a `EventManagerPort` sin efectos colaterales en sistema de ficheros ni deriva contractual.

Tracker **MAN-49** (padre **MAN-43**, documento canónico **PBI-7e574af7**).

**Por qué:** el desacople quedó parcialmente cumplido (`CUMPLIMIENTO PARCIAL — NO APTO_TOTAL`, 14/19). El runtime aún crea directorios de bus en la raíz del repo, conserva `std::fs` residual sobre rutas/`eda://`, documenta topología legacy, y evade/allowlistea gates de backend real (hallazgos **A-01..A-04**).

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-49** (canal `.SddIA/state/tracker-outbound/MAN-49.jsonl`); padre **MAN-43** |
| Fuente vinculante | `docs/audits/man-43-post-merge-audit-20261117.md` |
| Documento refine | **PBI-7e574af7** (`verdict: refined`; `open_questions: []`) |
| Semilla prompt | `PBI-2e55b337` — DoR alineado; canónico = refine (**L-DOC-CANON** en `clarify.md`) |
| Clarificaciones | `clarify.md` (laudos **R-01..R-04**; veredicto **ok**) |
| `execution_id` | `f01f024a-812e-4de3-a80a-6b6f3f2e0635` |

## Alcance (laudos R-01..R-04)

| Laudo | Objetivo |
|-------|----------|
| **R-01 (A-01 / P0)** | Erradicar creación de directorios de topología de bus en raíz (`ensure_fractal_dirs` / `ensure_bus_dirs` / `ensure_event_bus_topology` / `EdaBusTopology::ensure_dirs`); regresión en `eda_event_token_hygiene`. |
| **R-02 (A-02 / P1)** | `event-watcher` + `eda_sweep` 100 % vía `EventManagerPort`; eliminar helpers FS muertos; ampliar `gate_event_manager_port` (prohibir `fs::read_dir` / `create_dir_all` / `remove_file` / `rename`). |
| **R-03 (A-03 / P1)** | README + reglas + 22 genomas → `event_uri` / `EventManagerPort` **solo** vía `entity-manager`; `scan_genome_contract` sobre `SddIA/**/*.md` + `README.md`. |
| **R-04 (A-04 / P2)** | Quitar evasión `concat!`; `path_allowlisted` honesto; `INTEGRATION_SRC = []`; suite/clippy/gates verdes; sello **`APTO_TOTAL` solo Argos**. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Raíz limpia de topología FS | Cero dirs legacy de bus en raíz tras tests/daemons/CLI; test de regresión |
| **O2** | Centinelas solo puerto | Watcher/sweep sin `std::fs` de bus/`eda://`; gate FS ampliado verde |
| **O3** | Convergencia documental | README, reglas y 22 genomas sin `.events/` / `event_file_path` / `SDDIA_EVENT_STORE` legacy; escaneo higiene completo |
| **O4** | Gobernanza de gates | Sin `concat!` evasión; allowlist `INTEGRATION_SRC` vacía; higiene allowlisteada |
| **O5** | Cierre parental | Suite + clippy + `sddia-index-integrity --verify` verdes; Argos emite `APTO_TOTAL` |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1 (A-01)** | Cero creación de directorios `orchestration/`, `telemetry/`, `pending/`, `domain/`, `dead-letter/` o `progress/` en la raíz del repositorio al ejecutar tests, daemons o CLI; regresión cubierta en `eda_event_token_hygiene`. |
| **AC-2 (A-02)** | `event-watcher` y `eda_sweep` operan exclusivamente a través de `EventManagerPort`, sin llamadas `std::fs` sobre rutas de bus ni URIs `eda://`; verificado por `gate-event-manager-port` (prohibición de las operaciones FS de R-02). |
| **AC-3 (A-03)** | `README.md`, `.antigravity/rules/tekton-fire-and-forget.md` y los 22 contratos del genoma sincronizados a `event_uri` / `EventManagerPort` vía `entity-manager`; escaneo de higiene sobre `SddIA/**/*.md` + `README.md`. |
| **AC-4 (A-04)** | Sin `concat!` de evasión en `eda_event_token_hygiene.rs`; fichero registrado en `path_allowlisted` de `gate_eda_unit_no_real_backend`; `INTEGRATION_SRC` vacío (`entries = []`) en `eda-unit-backend.toml`. |
| **AC-5 (Cierre)** | `cargo test --workspace -- --test-threads=1`, `cargo clippy --workspace -- -D warnings` y `sddia-index-integrity --verify` en verde; dictamen `APTO_TOTAL` emitido por Argos tras `pull-request-review`. |

## Invariantes

- No reabrir el *qué* de MAN-43 / laudos L-1..L-7 salvo lo necesario para cerrar A-01..A-04.
- `SddIA/events/` y `.SddIA/events/` permanecen (genoma / personalización de instancia).
- Mutaciones genómicas solo vía `entity-manager`.
- `APTO_TOTAL` final solo por Argos (no Tekton).
- Git solo vía `skill:git-manager`.
- KM / `docs/todos/`: solo `agent:cumulo` o evento `Kaizen_Alert_Required`.

## Fuera de alcance

- Reabrir diseño EventManager / H1..H6 de MAN-43 más allá de A-01..A-04.
- Nuevas capacidades de EventManager distintas a la remediación de hallazgos.
- Eliminar `SddIA/events/` o `.SddIA/events/`.
- Semillas Kaizen bajo `docs/todos/` desde Tekton/Argos/Mayeuta.

## Dependencias

- Padre / origen: **MAN-43** (`PBI-0f4f0f5f`); informe `docs/audits/man-43-post-merge-audit-20261117.md`.
- Runtime: `sddia-daemon-runtime` (`lib.rs`, `eda_sweep.rs`, `eda_bus.rs`, `eda_bus_topology.rs`), `daemons/event-watcher`.
- Gates / higiene: `gate_event_manager_port`, `gate_eda_unit_no_real_backend`, `eda_event_token_hygiene`, allowlist `eda-unit-backend.toml`.
- Documentación: `README.md`, `.antigravity/rules/tekton-fire-and-forget.md`, 22 entidades genómicas vía `entity-manager`.
- Cierre: revisión independiente Argos (`pull-request-review`).
- Norma: `features-documentation-pattern` v1.3.1.
- Clarificaciones canónicas: `clarify.md`.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma: mutaciones solo vía `entity-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos R-*).
