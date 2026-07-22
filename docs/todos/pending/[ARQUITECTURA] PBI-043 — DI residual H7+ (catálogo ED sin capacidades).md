---
document_id: PBI-043-DI-CATALOGO-RESIDUAL-H7
title: "[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades)"
format: markdown
version: "1.1.0"
created: "2026-07-22"
uuid: 3d68a854-1e90-41fe-8205-f9e982866c4b
status: abierto
priority: media
process: feature
spawned_from: docs/todos/done/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
spawned_at: "2026-07-22"
spawn_reason: Residual H7+ diferido en Done global PBI-042 (R15); laudo Racso para documentar backlog accionable
baseline_taxonomy: "doc:closure | proc:git-sync | fs:persist"
baseline_bindings: SddIA/core/capability-bindings.md@1.1.0
inventory_count: 16
inventory_date: "2026-07-22"
hito7_status: entregado_en_main
hito7_feature: docs/features/inyeccion-dependencias-h7-nucleo-fs
hito7_branch: feat/inyeccion-dependencias-h7-nucleo-fs
hito7_execution_id: b8e2a471-5c9d-4f3a-8e1b-6d0c9f2a4b7e
hito7_pr: https://github.com/racso80es/SddIA/pull/144
hito7_snapshot_commit: 67f7e8dce98f71268c130f06e8ae42a2f2f3d542
hito7_merge_commit: 8f882b82c74660e0ec5be8c0ed2931bfab454290
related:
  - docs/features/inyeccion-dependencias-h7-nucleo-fs/finalize-process.md
  - docs/todos/done/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
  - docs/features/inyeccion-dependencias-cierre-pbi/finalize-process.md
  - docs/features/inyeccion-dependencias-barrido-creators/finalize-process.md
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/core/capability-bindings.md
  - SddIA/engine/execute-process/src/engine/capability_di_gate.rs
  - SddIA/engine/execute-process/src/engine/capability_di_resolver.rs
---

# [ARQUITECTURA] PBI-043: DI residual H7+ — catálogo ED sin capacidades

## 1. Historia de Usuario

* **Como:** Arquitecto del Core / Nodo de Control.
* **Quiero:** Completar la homologación DI del catálogo de procesos que quedó fuera del Done de PBI-042.
* **Para:** Extender ceguera espacial y Aduana Temprana más allá del núcleo forja/creators ya sellado (MVP→H6).

**Precedente cerrado:** PBI-042 Done global (PR [#142](https://github.com/racso80es/SddIA/pull/142) merge `90424f4`). Residual explícito: *«Ola H7+ ED residuales»* — este PBI lo materializa como backlog.

**Corrección ontológica:** SSOT ED = `{name}.md` + frontmatter. Prohibido `spec.json`. Mutación genoma vía `entity-manager` + `Domain_Entity_Updated` + evolution.

---

## 2. Estado de partida (innegociable)

| Vector | Estado |
|--------|--------|
| Runtime DI (gate / resolver / Cerbero RBAC+envelope / output validator / piloto EDA) | **Entregado** — no reescribir salvo bug |
| Taxonomía | `doc:closure`, `proc:git-sync`, `fs:persist` |
| Bindings | `capability-bindings.md` v1.1.0 |
| Process homologados con `requires_capability` | **26** post-H7 (baseline 18 + 8 §3.1) |
| Process **sin** `requires_capability` | **16** residual H8–H10 (post-H7; recontar al abrir hito) |
| Hito 1 H7 núcleo FS | **Hecho en main** — PR [#144](https://github.com/racso80es/SddIA/pull/144) merge `8f882b8` |

---

## 3. Inventario residual (24 ED)

> Snapshot 2026-07-22 sobre `SddIA/process/*.md` (excl. `process-contract`, `index`). Recontar al abrir cada hito.

### 3.1 Consumidores FS (`filesystem-manager` en fases) — **cerrado H7**

| ED | Capacidad | Estado |
|----|-----------|--------|
| `entity-manager` | `fs:persist` | **Hecho** H7 |
| `route-domain-event` | `fs:persist` | **Hecho** H7 |
| `daemon-kill-switch` | `fs:persist` | **Hecho** H7 |
| `governance-daemon-manager` | `fs:persist` | **Hecho** H7 |
| `daemon-heartbeat-audit` | `fs:persist` | **Hecho** H7 |
| `fix-tool-process` | `fs:persist` | **Hecho** H7 |
| `telemetry-batch-stub` | `fs:persist` | **Hecho** H7 |
| `workspace-smoke` | `fs:persist` | **Hecho** H7 |

### 3.2 Familia route (sin FS declarado)

| ED | Capacidad candidata | Notas |
|----|---------------------|-------|
| `route-domain` | *TBD* (`bus:route` o equivalente) | Hoy `agent:cumulo` |
| `route-orchestration` | *TBD* | Idem |
| `route-telemetry` | *TBD* | Idem |

### 3.3 Auditorías / telemetría

| ED | Capacidad candidata | Notas |
|----|---------------------|-------|
| `audit-sandbox-isolation-rbac` | *TBD* / tool-bound | `tool:sandbox-breacher` |
| `audit-telemetry-compliance-breach` | *TBD* | `tool:schema-corruptor` |
| `audit-thermodynamic-toll-failsoft` | *TBD* | `tool:io-choke` |
| `event-bus-audit` | *TBD* | `tool:event-bus-audit` |
| `telemetry-compliance-audit` | *TBD* / Argos | |

### 3.4 Gobernanza, interactores, smokes

| ED | Capacidad candidata | Notas |
|----|---------------------|-------|
| `cerbero-governance-react` | *TBD* | `agent:cerbero` |
| `radamanto-batch` | *TBD* | `agent:radamanto` |
| `kalma2-interact` | *TBD* (`llm:interact`?) | `skill:mayeuta-llm` |
| `memory-evolution-ingest` | *TBD* | `agent:cumulo` |
| `execute-suite` | *TBD* / multi | Orquesta execute-process |
| `capsule-invoke-smoke` | *TBD* | `tool:io-choke` |
| `telegram-gateway` | *TBD* | `tool:telegram-gateway` |
| `telegram-fallback-responder` | *TBD* | tool + Mayeuta |

---

## 4. Plan de olas (propuesta)

> Umbral exacto `N_ola` y lista final = Dedalo por ciclo. Pisos Mayeuta abajo. Sin bajar piso sin laudo Racso.

### Hito 1 — H7 · Núcleo FS (`fs:persist`)

> Ciclo `docs/features/inyeccion-dependencias-h7-nucleo-fs` · PR [#144](https://github.com/racso80es/SddIA/pull/144) merge `8f882b8` (2026-07-22).

| ID | Ítem | Notas |
|----|------|-------|
| **R1** | Homologar §3.1 (8 ED) con `requires_capability` → `fs:persist` (path ciego preferente) | **Hecho en main** — `N_ola=8` |
| **R2** | Mutación vía `entity-manager` + `Domain_Entity_Updated` + evolution; `orphan_count == 0` | **Hecho en main** |
| **R3** | Regresión suites DI MVP→H6 (capability_di / cerbero_di) | **Hecho en main** — 24/24 |

**AC-H7:** **APTO** — 8/8 §3.1; sellos ×8; orphan 0; runtime preservado.

### Hito 2 — H8 · Familia route

| ID | Ítem | Piso |
|----|------|------|
| **R4** | Decidir capacidad route (§5 Q1); alta taxonomía+binding **solo si** laudo Racso | K altas ≤ laudo |
| **R5** | Homologar `route-domain`, `route-orchestration`, `route-telemetry` (+ revalidar `route-domain-event` si drift) | `N_ola ≥ 3` |

**AC-H8:** 3 routes con DI o laudo explícito de *defer* documentado; orphan 0.

### Hito 3 — H9 · Auditorías

| ID | Ítem | Piso |
|----|------|------|
| **R6** | Modelo DI para ED tool-bound (auditorías §3.3): capacidad nueva vs `delegates_to` tool + `requires_capability` de orquestación | |
| **R7** | Homologar 5 ED §3.3 | `N_ola ≥ 5` |

**AC-H9:** 5/5 auditorías homologadas bajo modelo R6; orphan 0.

### Hito 4 — H10 · Gobernanza e interactores

| ID | Ítem | Piso |
|----|------|------|
| **R8** | Homologar 8 ED §3.4 (o partición Dedalo si blast-radius) | `N_ola ≥ 8` o 2 sub-olas |
| **R9** | Altas controladas al Códice solo con laudo (p. ej. `llm:interact`, `bus:ops`) | Sin inventar términos libres |

**AC-H10:** §3.4 homologado (umbral Dedalo ≥ piso); orphan 0; taxonomía coherente.

### Hito 5 (opcional) — EDA-only DI

| ID | Ítem | Notas |
|----|------|-------|
| **R10** | Sustitución path sync H2 por composición DI 100% EDA | **Fuera** salvo laudo Racso (mismo residual PBI-042) |

**AC-EDA:** Solo si laudo; no bloquea Done de H7–H10.

---

## 5. Preguntas abiertas (Dedalo / laudo)

| # | Pregunta | Default provisional |
|---|----------|---------------------|
| **Q1** | ¿Nueva capacidad `bus:route` para familia route? | Diferir a H8; no inventar en H7 |
| **Q2** | ¿Capacidad para tools de auditoría / telegram / smoke? | Preferir binding tool existente + `requires_capability` de orquestación; altas solo con laudo |
| **Q3** | ¿`entity-manager` es piloto ciego obligatorio en H7 o mixto Forja? | Preferir ciego `fs:persist`; mixto solo si forge lo exige (patrón daemon-creator H6) |
| **Q4** | ¿Partir H10 en dos ciclos si riesgo? | Sí, permitido; no bajar piso total |
| **Q5** | ¿Recontar inventario al start de cada hito? | **Obligatorio** — regenerar tabla §3 |

---

## 6. Criterios de aceptación globales

| ID | Criterio |
|----|----------|
| **AC-INV** | Inventario recontado al inicio de cada hito; drift documentado |
| **AC-SEAL** | Toda mutación ED vía `entity-manager` + `Domain_Entity_Updated` + evolution |
| **AC-ORPHAN** | `orphan_count == 0` tras cada ola |
| **AC-REG** | Regresión DI runtime (gate/resolver/Cerbero/envelope) verde por hito |
| **AC-NO-INVENT** | Prohibido inventar `capability_id` fuera de taxonomía sin laudo Racso |
| **AC-DONE-PBI** | Done = H7–H10 APTO (R10 opcional) + PBI en `docs/todos/done/` + `pbi_archived: true` en un PR de cierre |

---

## 7. Fuera de alcance

| Ítem | Destino |
|------|---------|
| Reescritura runtime DI (salvo bug) | Fuera |
| GesFer / Paciente 0 | Otro PBI kitchen |
| Fractura Core F1 | Otro `persist_ref` |
| EDA-only total (R10) | Solo con laudo |
| Skills/actions/tools sin proceso process | Fuera salvo que un hito lo declare |
| Deuda PPR #136 (`delivery-close-cycle` revoked/signer) | PBI distinto ya en pending |

---

## 8. Definición de Done por ciclo feature

Mismo estándar `features-documentation-pattern` v1.2.0 + `task-closure-documental`:

```text
Done(hito) = un PR mergeado en main
 + validacion.md APTO
 + pbi_archived: false mientras queden hitos
 + PBI padre permanece en pending/ hasta Done global
```

Done global PBI-043: mover a `docs/todos/done/` + `pbi_archived: true` cuando H7–H10 cerrados (R10 no obligatorio).

---

## 9. Referencias

- PBI padre cerrado: `docs/todos/done/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md`
- Finalize residual: `docs/features/inyeccion-dependencias-cierre-pbi/finalize-process.md`
- Norma: `SddIA/library/norms/capability-taxonomy.md`
- Bindings: `SddIA/core/capability-bindings.md`
- Gate / resolver: `capability_di_gate.rs` / `capability_di_resolver.rs`
