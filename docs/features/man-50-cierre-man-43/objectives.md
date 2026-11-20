---
feature_name: man-50-cierre-man-43
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Objetivos estabilizados — cierre B-01..B-03 residuales post-merge MAN-49/MAN-43 hasta APTO Argos
branch_name: feat/man-50-cierre-man-43
persist_ref: docs/features/man-50-cierre-man-43
pbi_ref: MAN-50
parent_ref: MAN-43
related_ref: MAN-49
historia_ref: MAN-43
document_id: PBI-91b7ec84
document_uuid: "91b7ec84-a164-48ce-ba51-40e872ef3be1"
capability_id: "eda:event-manager"
contract: "eda.event_manager"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
audit_ref: docs/audits/man-49-man-43-post-merge-audit-20261120.md
parent_commit: ee17d9537a1b567505049bb1a2bf3245c7560d39
related_pr: 455
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-50.jsonl"
agents: mayeuta
execution_id: "6793b598-8a33-43d8-87db-3c3012d5712d"
---
# Objetivos — man-50-cierre-man-43

## Misión

Cerrar las **3 brechas residuales** (`B-01`..`B-03`) de la auditoría post-merge de **MAN-49** y **MAN-43** hasta dictamen **`APTO`** emitido por Argos, sin reabrir el *qué* parental salvo lo necesario para esas brechas.

Tracker **MAN-50** (padre **MAN-43**, relacionado **MAN-49** / PR #455, documento **PBI-91b7ec84**).

**Por qué:** tras el merge de MAN-49/MAN-43 (`main` @ `ee17d9537a1b567505049bb1a2bf3245c7560d39`) el desacople a `EventManagerPort` sigue parcial: writers DLQ con I/O físico a `.events/dead-letter/`, binarios locales stale con residuos de bus en raíz, `INTEGRATION_SRC` con 3 entradas, y referencias documentales secundarias a `SDDIA_EVENT_STORE` / `.events/`.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-50** (canal `.SddIA/state/tracker-outbound/MAN-50.jsonl`); padre **MAN-43**; relacionado **MAN-49** |
| Fuente vinculante | `docs/audits/man-49-man-43-post-merge-audit-20261120.md` @ `ee17d953` |
| Documento refine | **PBI-91b7ec84** (`uuid` `91b7ec84-a164-48ce-ba51-40e872ef3be1`) |
| Clarificaciones | `clarify.md` (laudos **R-01**, **R-02**, **R-03a**, **R-03b**; veredicto **ok**) |
| `execution_id` | `6793b598-8a33-43d8-87db-3c3012d5712d` |

## Alcance (laudos R-01..R-03b)

| Laudo | Objetivo |
|-------|----------|
| **R-01 (B-01 / P1)** | 6 writers `write_*_dead_letter` solo vía `EventManagerPort`; eliminar `bus_defaults_from_root(".events")` + `write_json_atomic` en `eda_bus_topology`; `eda_event_token_hygiene` veta `".events"` (± barra). |
| **R-02 (B-02 / P2)** | Recompilar `sddia-workspace-server`, `event-watcher`, `execute-process` (debug+release); `.events/` ausente en raíz; `assert_repo_root_bus_dirs_absent` cubre `".events"`. |
| **R-03a (B-03a / P2)** | `integration_src = []` + `INTEGRATION_SRC = &[]`; migrar tests Redb fuera del árbol unitario; gate `gate-eda-unit-no-real-backend` verde. |
| **R-03b (B-03b / P2)** | Cero `.events/` / `SDDIA_EVENT_STORE` en reglas antigravity, `events-contract.md` y 4 procesos del códice (vía `entity-manager`); nota O6 alineada. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | DLQ solo puerto | 6 writers sin I/O `.events/dead-letter/`; higiene veta `".events"` |
| **O2** | Paridad binaria + raíz limpia | 3 crates rebuild; `.events/` ausente; aserción raíz |
| **O3** | Allowlist unitaria vacía | `integration_src`/`INTEGRATION_SRC` vacíos; gate verde |
| **O4** | Convergencia documental residual | Cero tokens legacy en reglas/contrato/códex citados; O6 coherente |
| **O5** | Cierre independiente | Suite + clippy + `verify-process-integrity` + gates QA verdes; Argos emite `APTO` |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1 (B-01)** | Los 6 escritores `write_*_dead_letter` operan solo vía `EventManagerPort`; `bus_defaults_from_root(".events")` eliminado; `eda_event_token_hygiene` veta `".events"` (con o sin barra) en código productivo. |
| **AC-2 (B-02)** | Binarios `sddia-workspace-server`, `event-watcher` y `execute-process` recompilados; `.events/` ausente en la raíz del repo y verificado por `assert_repo_root_bus_dirs_absent`. |
| **AC-3 (B-03a)** | `integration_src = []` en `SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml` y `CONST INTEGRATION_SRC: &[&str] = &[]` en `gate_eda_unit_no_real_backend.rs`; gate `gate-eda-unit-no-real-backend` en verde. |
| **AC-4 (B-03b)** | Cero ocurrencias de `.events/` y `SDDIA_EVENT_STORE` en `.antigravity/rules/*.md`, `SddIA/events/events-contract.md` y `SddIA/library/codexes/codex-software-engineering/process/*.md` (vía `entity-manager`); nota `docs/audits/man-43-o6-remediation-20261113.md` alineada. |
| **AC-5 (Cierre)** | `cargo test --workspace -- --test-threads=1`, `cargo clippy --workspace -- -D warnings`, `sddia-qa verify-process-integrity` y gates QA en verde; dictamen `APTO` emitido por Argos (no auto-sello Tekton). |

## Invariantes

- No reabrir el *qué* de MAN-43 / MAN-49 salvo lo necesario para cerrar `B-01`..`B-03`.
- `SddIA/events/` y `.SddIA/events/` permanecen (genoma / personalización de instancia).
- Mutaciones genómicas solo vía `entity-manager`.
- `APTO` final solo por Argos (no Tekton).
- Git solo vía `skill:git-manager`.
- KM / `docs/todos/`: solo `agent:cumulo` o evento `Kaizen_Alert_Required`.
- Artefactos de fase bajo `persist_ref` solo vía MCP `sddia-workspace-server`.

## Fuera de alcance

- Reabrir el *qué* de MAN-43 / MAN-49 más allá de `B-01`..`B-03`.
- Nuevas capacidades de EventManager distintas a la remediación residual.
- Eliminar `SddIA/events/` o `.SddIA/events/`.
- Semillas Kaizen bajo `docs/todos/` desde Tekton/Argos/Mayeuta.

## Dependencias

- Padre / origen: **MAN-43** (relacionado **MAN-49**, PR #455).
- Informe fuente: `docs/audits/man-49-man-43-post-merge-audit-20261120.md`.
- Runtime: `execute-process` (`eda_bus_topology`, writers DLQ, handlers citados), daemons / `sddia-workspace-server`.
- Gates / higiene: `eda_event_token_hygiene`, `gate_eda_unit_no_real_backend`, allowlist `eda-unit-backend.toml`.
- Genoma: mutaciones de procesos del códice solo vía `entity-manager`.
- Cierre: revisión independiente Argos.
- Norma: `features-documentation-pattern` v1.3.1.
- Clarificaciones canónicas: `clarify.md`.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma: mutaciones solo vía `entity-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos R-*).
