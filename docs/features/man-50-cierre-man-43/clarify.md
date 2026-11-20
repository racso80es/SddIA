---

feature_name: man-50-cierre-man-43
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Estabilización de requisitos — cierre brechas residuales B-01..B-03 post-merge MAN-49/MAN-43 hasta APTO Argos
branch_name: feat/man-50-cierre-man-43
persist_ref: docs/features/man-50-cierre-man-43
pbi_ref: MAN-50
parent_ref: MAN-43
related_ref: MAN-49
historia_ref: MAN-43
document_id: PBI-91b7ec84
document_uuid: "91b7ec84-a164-48ce-ba51-40e872ef3be1"
phase: mayeuta-stabilization
capability_id: "eda:event-manager"
contract: "eda.event_manager"
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-50.jsonl"
audit_ref: docs/audits/man-49-man-43-post-merge-audit-20261120.md
parent_commit: ee17d9537a1b567505049bb1a2bf3245c7560d39
related_pr: 455
agents: mayeuta
execution_id: "ad89137d-966b-4076-8d75-e299e3f3b137"
---
# Clarificación — man-50-cierre-man-43

Transcript Mayeuta (2026-11-20). Fase **Estabilización de Requisitos** del proceso `feature`.
Canal tracker: **MAN-50** (padre **MAN-43**, relacionado **MAN-49** / PR #455). Cierre de brechas residuales post-merge; **no** reabre el *qué* de MAN-43 / MAN-49 salvo lo necesario para cerrar `B-01`..`B-03`.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-50** (provider Linear). Canal: `.SddIA/state/tracker-outbound/MAN-50.jsonl` (`refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `feature`). |
| Padre / origen | **MAN-43**; relacionado **MAN-49** (PR **#455**). |
| Fuente vinculante | `docs/audits/man-49-man-43-post-merge-audit-20261120.md` — `main` @ `ee17d9537a1b567505049bb1a2bf3245c7560d39`. Dictamen: MAN-49 cumplimiento parcial alto; MAN-43 **16/19 ACs** con **3 hallazgos residuales `B-01`..`B-03`**. Materializado en disco (MCP). |
| Documento refine | **PBI-91b7ec84** (`uuid` `91b7ec84-a164-48ce-ba51-40e872ef3be1`; `refined_by: mayeuta`; `priority: high`; `parent: MAN-43`). |
| Problema estabilizado | Tras merge MAN-49/MAN-43: escrituras físicas a `.events/dead-letter/`, binarios/daemons locales desactualizados con residuos de bus en raíz, `INTEGRATION_SRC` con 3 entradas, y referencias documentales secundarias a `SDDIA_EVENT_STORE` / `.events/`. |
| Modo | `branch_pr` / `feature` — remediación focal `B-01`..`B-03`; laudos `R-01`..`R-03b` (no reabrir H1..H6 / A-01..A-04 ya cerrados salvo residual). |
| `execution_id` | **6793b598-8a33-43d8-87db-3c3012d5712d** |

## D1 — Matriz hallazgo × laudo de remediación

| Hallazgo (auditoría) | Severidad | ACs parentales / origen | Laudo remediación |
|----------------------|-----------|-------------------------|-------------------|
| **B-01** — 6 writers `write_*_dead_letter` aún I/O físico en `.events/dead-letter/`; `bus_defaults_from_root(".events")` elude higiene léxica | P1 | MAN-43 AC-4 / AC-5 | **R-01** |
| **B-02** — Binarios `sddia-workspace-server` / `event-watcher` / `execute-process` desalineados; `.events/` histórico en raíz | P2 | Higiene operativa post-MAN-49 | **R-02** |
| **B-03a** — 3 entradas restantes en `integration_src` / `INTEGRATION_SRC` (Redb en unitarios) | P2 | MAN-43 AC-11 / AC-14 | **R-03a** |
| **B-03b** — `.events/` / `SDDIA_EVENT_STORE` en reglas, `events-contract.md`, 4 procesos del códice; nota O6 inconsistente | P2 | MAN-49 AC-4 | **R-03b** |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **R-01 (B-01)** | Los 6 escritores productivos `write_*_dead_letter` operan **exclusivamente** vía `EventManagerPort` (`move_to_dlq` / `fail_delivery` o registro en store **sin** I/O en `.events/dead-letter/`). Callers en alcance: `route_domain_core`, `capability_di_gate`, `capability_di_resolver`, `capability_di_output_validator`, `cerbero_di_rbac`, `cerbero_di_envelope`. Eliminar `bus_defaults_from_root(".events")` y `write_json_atomic` asociados en `eda_bus_topology`. `eda_event_token_hygiene` veta el literal `".events"` (con o sin barra final) en código productivo. |
| **R-02 (B-02)** | Recompilar `sddia-workspace-server`, `event-watcher` y `execute-process` (`debug` y `release`) alineados al puerto; directorio `.events/` **ausente** en la raíz del repositorio; `assert_repo_root_bus_dirs_absent` cubre `".events"`. Reinicio operativo de daemons fuera del laudo de código queda como verificación de higiene. |
| **R-03a (B-03a)** | `integration_src = []` en `SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml` y `CONST INTEGRATION_SRC: &[&str] = &[]` en `gate_eda_unit_no_real_backend.rs`. Tests de integración Redb migrados fuera del árbol unitario (p. ej. `execute-process/tests/` / lab de puerto habilitado). Gate `gate-eda-unit-no-real-backend` en verde. |
| **R-03b (B-03b)** | Cero ocurrencias de `.events/` y `SDDIA_EVENT_STORE` en `.antigravity/rules/*.md`, `SddIA/events/events-contract.md` y procesos del códice `SddIA/library/codexes/codex-software-engineering/process/{accept-pr,delivery-close-cycle,pull-request-review,refine-item}.md`. Mutación genómica **solo** vía `entity-manager`. Nota `docs/audits/man-43-o6-remediation-20261113.md` alineada (sin inconsistencia APTO/NO_APTO). |
| **L-DOMAIN** | Frontera productiva = `EventManagerPort` / `EventUri`. Este ciclo **no** añade capacidades nuevas del puerto; solo remedia `B-01`..`B-03`. |
| **L-SUPERSEDE-SCOPE** | Laudos de MAN-43 / MAN-49 **permanecen**; no se reabre el *qué* parental salvo el cierre de brechas residuales. |
| **L-NON-GOALS** | No eliminar `SddIA/events/` ni `.SddIA/events/`. No nuevas ops EventManager fuera de remediación. No mutar `docs/todos/` desde Tekton/Argos/Mayeuta (solo `agent:cumulo` / `Kaizen_Alert_Required`). |
| **L-DOC-CANON** | Canónico de refine/Linear = **PBI-91b7ec84** / tracker **MAN-50**. Fuente vinculante = auditoría `20261120` @ `ee17d953`. |
| **L-ARGOS-SEAL** | Dictamen final **`APTO`** = emisión **Argos** tras revisión. Prohibido auto-sello Tekton en `validacion.md`. |
| **L-GENOME** | Mutaciones de `SddIA/library/codexes/.../process/*.md` y entidades indexadas **solo** vía `execute-process --process entity-manager`. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. Evidencia MCP `git_status` preferida; no bypass raw destructivo. |
| **L-TRUTH** | No inventar éxito: si gates/tests/binarios no materializan verde, el check permanece NO_APTO. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`. |
| **L-MCP-ADUANA** | Materialización bajo `docs/features/man-50-cierre-man-43` **solo** vía MCP `sddia-workspace-server`. |

## D3 — Criterios de aceptación del ciclo (producto)

| ID | Criterio |
|----|----------|
| **AC-1 (B-01)** | Los 6 escritores `write_*_dead_letter` operan solo vía `EventManagerPort`; `bus_defaults_from_root(".events")` eliminado; `eda_event_token_hygiene` veta `".events"` (con o sin barra) en código productivo. |
| **AC-2 (B-02)** | Binarios `sddia-workspace-server`, `event-watcher` y `execute-process` recompilados; `.events/` ausente en la raíz del repo y verificado por `assert_repo_root_bus_dirs_absent`. |
| **AC-3 (B-03a)** | `integration_src = []` en `eda-unit-backend.toml` y `CONST INTEGRATION_SRC: &[&str] = &[]` en `gate_eda_unit_no_real_backend.rs`; gate `gate-eda-unit-no-real-backend` en verde. |
| **AC-4 (B-03b)** | Cero ocurrencias de `.events/` y `SDDIA_EVENT_STORE` en `.antigravity/rules/*.md`, `SddIA/events/events-contract.md` y `SddIA/library/codexes/codex-software-engineering/process/*.md` (vía `entity-manager`); nota O6 alineada. |
| **AC-5 (Cierre)** | `cargo test --workspace -- --test-threads=1`, `cargo clippy --workspace -- -D warnings`, `sddia-qa verify-process-integrity` y gates QA en verde; dictamen `APTO` emitido por **Argos** (no auto-sello Tekton). |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Reabrir diseño EventManager / A-01..A-04 de MAN-49? | **No** — solo residual `B-01`..`B-03` (**L-SUPERSEDE-SCOPE**). |
| 2 | ¿Eliminar `SddIA/events/` o `.SddIA/events/`? | **No** (**L-NON-GOALS**). |
| 3 | ¿Quién sella `APTO`? | **Solo Argos** (**L-ARGOS-SEAL**). |
| 4 | ¿Forjar códice/procesos a mano? | **No** — solo `entity-manager` (**L-GENOME** / **R-03b**). |
| 5 | ¿API concreta del puerto para DLQ? | Mayeuta fija el *qué*: vía `EventManagerPort` sin I/O `.events/dead-letter/`. Dedalo elige `move_to_dlq` / `fail_delivery` / registro store (**R-01**). |
| 6 | ¿Semillas Kaizen desde este agente? | **No** — solo `agent:cumulo` / `Kaizen_Alert_Required` (**L-NON-GOALS**). |

## D5 — Riesgos (vigilancia, no bloqueo Mayeuta)

| Riesgo | Tratamiento |
|--------|-------------|
| Higiene léxica solo `".events/"` (falso negativo O14d) | Veto explícito de `".events"` con/sin barra (**R-01**). |
| Binarios stale en `target/` siguen escribiendo telemetría FS | Rebuild debug+release de los 3 crates + aserción raíz (**R-02**). |
| Tests Redb privados difíciles de extraer | Migrar a `execute-process/tests/` / lab puerto; allowlist vacía (**R-03a**). |
| Mutación genómica sin `entity-manager` → huérfana EDA | Forzar `entity-manager` (**L-GENOME** / **R-03b**). |
| Auto-sello Tekton | Prohibido; Argos emite dictamen (**L-ARGOS-SEAL**). |

## D6 — Handoff Dedalo / Tekton

1. Consumir `clarify.md` + `objectives.md` en solo lectura; **no** reabrir R-01..R-03b.
2. Blueprint (`spec.md` / `plan.md`) acota touchpoints: `eda_bus_topology`, 6 writers DLQ, `eda_event_token_hygiene`, rebuild binarios, allowlist/gate `INTEGRATION_SRC`, docs/genoma vía `entity-manager`, nota O6.
3. Orden sugerido de forja: **R-01** → **R-03a** (gates) → **R-02** (higiene binarios/raíz) → **R-03b** (docs/genoma) → verificación AC-5 → Argos.
4. Mayeuta **no** diseña implementación ni emite `delegates_to` de código.

## D7 — Evidencia de estabilización (MCP)

| Evidencia | Estado verificado |
|-----------|-------------------|
| Auditoría `docs/audits/man-49-man-43-post-merge-audit-20261120.md` | Leída; B-01..B-03 alineados al alcance |
| Canal tracker `MAN-50.jsonl` | Eventos refine/stamp/lifecycle/feature presentes |
| `eda-unit-backend.toml` `integration_src` | 3 entradas residuales (`route_domain_core`, `system_vitality`, `daemon_heartbeat`) |
| `gate_eda_unit_no_real_backend.rs` `INTEGRATION_SRC` | Misma ternaria residual |
| `eda_bus_topology.rs` | Residual B-01 vivo (`bus_defaults_from_root` / DLQ FS) según auditoría @ `ee17d953` |
| `git_status` MCP | Untracked: auditoría + `docs/features/man-50-cierre-man-43/` |

## D8 — Veredicto

**ok** — Requisitos estabilizados. El *qué* queda cerrado en laudos **R-01**, **R-02**, **R-03a**, **R-03b** / ACs **1..5**; fuente vinculante auditoría post-merge leída; documento **PBI-91b7ec84**; sin preguntas abiertas bloqueantes. Tracker **MAN-50** / padre **MAN-43** / relacionado **MAN-49** / `execution_id` **6793b598-8a33-43d8-87db-3c3012d5712d**.
