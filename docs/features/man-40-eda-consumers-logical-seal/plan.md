---













feature_name: man-40-eda-consumers-logical-seal
created: "2026-11-09"
updated: "2026-11-09"
process: feature
branch_name: feat/man-40-eda-consumers-logical-seal
persist_ref: docs/features/man-40-eda-consumers-logical-seal
pbi_ref: MAN-40
parent_ref: MAN-36
historia_ref: MAN-36
document_id: PBI-1b004e99
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
version_plan: "1.0.0"
phases:
  - T0-sello-logico
  - T1-desacoplo-eda-port-topologia
  - T2-fallbacks-deteccion-modo
  - T3-event-watcher-puerto
  - T4-scripts-installer-lab
  - T5-gate-arquitectonico
  - T6-tests-validacion
rbac_ok: true
scope: "Sello lógico PublishSeal/event_id en consumidores EDA; cero path Redb ni FS `.events/*` como SSOT fuera del host"
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-40.jsonl"
forge_key: "bd5f4db27f549584d61e515fe85571b8fb2d5f3279161d9aa982ff2851471498"
execution_id: "87147804-e6a3-4063-a53e-63cf55e29b90"
---
# Plan / Blueprint — man-40-eda-consumers-logical-seal

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` **v1.0.0** (ids **MAN-40** / **PBI-1b004e99** / padre **MAN-36**).

`target_executor_rbac.allowed_policies` alineadas a Tekton: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `chaos-engineering`, `tracker-operations`.

**Lectura de estado:** host/bóveda MAN-36 vivos; contraste `linear_bridge` sano; **deuda confirmada** en inventarios L-SEAL-INVENTORY + L-FS-INVENTORY (evidencia §0 de `spec.md`). Este blueprint pide forja del delta, no reabrir MAN-36.

## Viabilidad RBAC (cruzada mecánicamente)

| Cápsula | `context` skill | ¿En Tekton? |
|---------|-----------------|-------------|
| `skill:filesystem-manager` | `filesystem-ops` | sí |
| `skill:shell-executor` | `system-operations` | sí (`cargo test` / clippy / gate) |
| `skill:git-manager` | `source-control` | sí — **única** vía Git (**L-GIT**) |
| `action:execute-process` → entity-manager | `ecosystem-evolution` | sí si muta Genoma/docs normativos (**L-GENOME**) |

**Prohibido:** TODOs bajo `docs/todos/` (solo `agent:cumulo` / `Kaizen_Alert_Required`).
**Prohibido:** reabrir bóveda/precedencia MAN-36; tocar ops EventManagerPort 1.2; rehabilitar `SDDIA_EVENT_STORE` productivo.
**rbac_ok:** `true` — ninguna fase requiere contexto fuera de Tekton.

---

## Fases declarativas

```yaml
phases:
  - name: "T0 Sello lógico"
    intent: "materialize_to_bus y análogos: return/log = PublishSeal/event_id; cero resolved_redb_path como sello."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T1 Desacoplo eda_port / topología / route"
    intent: "Retirar runtime_db_path como API/sello; witnesses/headers lógicos; terminal_witness_exists solo puerto."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T2 Fallbacks / detección / modo fichero"
    intent: "Sin layout cableado, sin file_name events.redb, sin productive_store_present como modo."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T3 event-watcher por puerto"
    intent: "Claim pending+familias por EventManagerPort; cero SSOT read_dir/is_file de colas/testigos."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T4 Scripts / installer / lab"
    intent: "Retirar SDDIA_EVENT_STORE y asunción events.redb; alinear lab forge-doc-drift."
    delegates_to: ["skill:filesystem-manager"]
  - name: "T5 Gate arquitectónico"
    intent: "Veto resolved_redb_path / runtime_db_path / DEFAULT_REDB_REL fuera host+allowlist."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T6 Tests y cierre forja"
    intent: "No-regresión + gates; implementation/execution; evidencia CA-1…7 sin inventar APTO."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager", "skill:git-manager"]
```

---

## T0 — Sello lógico (prioridad ancla)

| # | Entregable | Detalle |
|---|------------|---------|
| T0.1 | `github_bridge::materialize_to_bus` | Tras `record_ecst_on_port`, retornar token lógico (patrón `linear_bridge`) o `event_id`; **no** `resolved_redb_path` |
| T0.2 | Log `Materializado` | Imprime sello lógico / id, nunca path `.redb` |
| T0.3 | Call sites | Actualizar firmas/consumidores del `Option<PathBuf>` si cambia semántica |
| T0.4 | Dead-letter local | **No tocar** `write_fallback_dead_letter` |

**Commit:** `fix(eda): sello lógico PublishSeal en github_bridge (MAN-40)`.

---

## T1 — Desacoplo eda_port / topología / route

| # | Entregable | Detalle |
|---|------------|---------|
| T1.1 | `eda_port::runtime_db_path` | Retirar de API pública productiva o crate-private; sin sello físico |
| T1.2 | Helpers path | `path_is_resolved_redb` / `store_present` / `port_arc`: dejar de ser superficie de modo/sello |
| T1.3 | `write_processing_witness` | Retorno lógico (no `runtime_db_path`) |
| T1.4 | `route_domain_core` | `processing_header` sin path Redb |
| T1.5 | `terminal_witness_exists` | Solo puerto; sin OR FS en build productivo |

**Commit:** `refactor(eda): sello lógico en eda_port/topología/route (MAN-40)`.

---

## T2 — Fallbacks, detección y modo fichero

| # | Entregable | Detalle |
|---|------------|---------|
| T2.1 | `is_runtime_db_event_path` | Solo API host; quitar `file_name == "events.redb"` |
| T2.2 | `sddia-event-store` gate | Sin layout `.SddIA/runtime/events.redb` productivo; allowlist tests |
| T2.3 | `event-bus-audit` | Sin fallback/`productive_store_present` como SSOT de anomalía |
| T2.4 | purger + kalma2 `eda_productive` | Sin ramificar modo por existencia de fichero |

**Commit:** `refactor(eda): retirar fallbacks layout/nombre/modo fichero (MAN-40)`.

---

## T3 — event-watcher por puerto

| # | Entregable | Detalle |
|---|------------|---------|
| T3.1 | Claim pending | Conservar/reforzar vía puerto |
| T3.2 | `claim_store_fractal` + familias | Equivalente real por puerto (telemetría/dominio/orquestación) |
| T3.3 | Cierre de tick | Sin `is_file`/`read_dir` de colas/testigos como SSOT |
| T3.4 | `has_dead_letter_witnesses` | Sin `read_dir` FS como SSOT |
| T3.5 | Telemetría claim | Sin etiqueta store `"redb"` como SSOT |

**Commit:** `refactor(eda): event-watcher 100% EventManagerPort (MAN-40)`.

---

## T4 — Scripts / installer / lab

| # | Entregable | Detalle |
|---|------------|---------|
| T4.1 | `hook_common.sh` | Sin asunción `events.redb` único |
| T4.2 | `clean-project-ephemeral.sh` | Sin gobernanza `SDDIA_EVENT_STORE`; protección alineada a host |
| T4.3 | `sddia-installer.sh` | Sin default `SDDIA_EVENT_STORE` productivo |
| T4.4 | Lab forge-doc-drift | Contrato puerto/sello lógico |

**Commit:** `chore(eda): scripts/installer/lab sin SDDIA_EVENT_STORE (MAN-40)`.

---

## T5 — Gate arquitectónico

| # | Entregable | Detalle |
|---|------------|---------|
| T5.1 | Extender `gate_event_manager_port` o gate hermano | Veto `resolved_redb_path`, `runtime_db_path`, `DEFAULT_REDB_REL` fuera host |
| T5.2 | Allowlist | Host + tests/legado explícitos; cero consumidores productivos |
| T5.3 | Layout histórico | Veto construcción `repo.join(".SddIA/runtime/events.redb")` en productivo |

**Commit:** `test(eda): gate veto path Redb fuera del host (MAN-40)`.

---

## T6 — Tests y cierre forja

| # | Hecho | Verificación |
|---|--------|----------------|
| T6.1 | Sello lógico | Bridge/tests: sin `.redb` en return/log |
| T6.2 | Terminal / header | Sin FS fallback productivo |
| T6.3 | Watcher | Claim familias por puerto; cierre sin SSOT FS |
| T6.4 | `cargo test` paquetes tocados | Verde |
| T6.5 | Gates QA | gate Event Manager + gate MAN-40 |
| T6.6 | Docs forja | `implementation.md` + `execution.md` |

**Commit(s):** verificación; Git solo `skill:git-manager`.

---

## Orden de ejecución Tekton

1. **T0** sello ancla (`github_bridge`) — desbloquea CA-1 visible.
2. **T1** eda_port/topología/route + terminal.
3. **T2** fallbacks/detección/modo.
4. **T3** event-watcher (puede paralelizarse tras T1 si no comparte tipos).
5. **T4** scripts/lab (independiente tras T0 conceptual).
6. **T5** gate (tras T0–T4 para no fallar en deuda aún no migrada; o gate incremental con allowlist temporal **documentada** y retirada en el mismo PR).
7. **T6** tests + artefactos forja → handoff Argos/`validacion.md`.

No declarar CA-1…CA-7 sin evidencia. No abrir EventManagerPort 1.2. No seeds Kaizen desde Tekton.

## Criterios de salida del blueprint

- CA-1…CA-7 según `spec.md` / `objectives.md` (O1…O7; O8 = validación Argos).
- Laudos L-DOMAIN, L-CA4-RESIDUE, L-LOGICAL-SEAL, L-HOST-ONLY, L-NO-LAYOUT-FALLBACK, L-NO-NAME-DETECT, L-NO-FILE-MODE, L-WATCHER-PORT, L-TERMINAL-PORT, L-SCRIPTS, L-LAB-DRIFT, L-GATE, L-HEALTHY, L-NON-GOALS, L-PROCESS, L-GIT, L-GENOME, L-TRACKER respetados.
- Inventarios L-SEAL-INVENTORY + L-FS-INVENTORY **cerrados** o acotados con allowlist explícita de tests.
- Tracker: **MAN-40** / documento **PBI-1b004e99** / padre **MAN-36** / `execution_id` **d5f40be9-1866-4813-a5b2-28a26faed56c**.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Romper callers de `Option<PathBuf>` al cambiar sello | Seguir contraste `linear_bridge`; adaptar call sites en el mismo tajo |
| Gate T5 falla antes de migrar todo | Allowlist temporal documentada + retirada en el mismo PR; o T5 al final |
| Watcher sin API puerto para fractal | Extender helpers host/core mínimos **sin** reabrir bóveda |
| Reabrir MAN-36 por accidente | Diff review: cero cambios a `config.rs` precedencia/vars salvo API faltante |
| Contaminar dead-letter local del bridge | L-HEALTHY — fuera de alcance |
| Inventar APTO | Argos declara; Tekton solo evidencia |
