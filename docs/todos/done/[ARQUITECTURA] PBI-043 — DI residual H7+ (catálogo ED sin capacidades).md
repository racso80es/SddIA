---
document_id: PBI-043-DI-CATALOGO-RESIDUAL-H7
title: "[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades)"
format: markdown
version: "1.2.0"
created: "2026-07-22"
uuid: 3d68a854-1e90-41fe-8205-f9e982866c4b
status: cerrado
closed_at: "2026-07-23"
close_feature: docs/features/inyeccion-dependencias-h-doc-readme
close_branch: docs/inyeccion-dependencias-h-doc-readme
close_execution_id: 5df409e5-c594-4e07-bc6f-857d69a433f1
spawned_pbi_045: docs/todos/pending/[ARQUITECTURA] PBI-045 — DI para Gobernanza, Lotes y Notificaciones (Hito 11).md
priority: media
process: feature
spawned_from: docs/todos/done/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
spawned_at: "2026-07-22"
spawn_reason: Residual H7+ diferido en Done global PBI-042 (R15); laudo Racso para documentar backlog accionable
baseline_taxonomy: "doc:closure | proc:git-sync | fs:persist | bus:route | qa:probe | audit:compliance | llm:interact"
baseline_bindings: SddIA/core/capability-bindings.md@1.3.0
inventory_count: 7
inventory_with_capability: 35
inventory_date: "2026-07-23"
hito7_status: entregado_en_main
hito7_pr: https://github.com/racso80es/SddIA/pull/144
hito8_pr: https://github.com/racso80es/SddIA/pull/147
hito9_pr: https://github.com/racso80es/SddIA/pull/149
hito10a_pr: https://github.com/racso80es/SddIA/pull/151
laudo_filtro_c: "2026-07-23 — H10-B rechazado; Done con integridad 35/7; spawn PBI-045"
related:
  - docs/features/inyeccion-dependencias-h-doc-readme/
  - docs/todos/pending/[ARQUITECTURA] PBI-045 — DI para Gobernanza, Lotes y Notificaciones (Hito 11).md
  - docs/todos/done/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/core/capability-bindings.md
---

# [ARQUITECTURA] PBI-043: DI residual H7+ — catálogo ED sin capacidades

## 1. Historia de Usuario

* **Como:** Arquitecto del Core / Nodo de Control.
* **Quiero:** Completar la homologación DI del catálogo de procesos que quedó fuera del Done de PBI-042.
* **Para:** Extender ceguera espacial y Aduana Temprana más allá del núcleo forja/creators ya sellado (MVP→H6).

**Precedente cerrado:** PBI-042 Done global. **Cierre:** Laudo Estructural Filtro C (2026-07-23) — Done con integridad 35 with / 7 without; miscelánea diferida a PBI-045.

**Corrección ontológica:** SSOT ED = `{name}.md` + frontmatter. Prohibido `spec.json`. Mutación genoma vía `entity-manager` + `Domain_Entity_Updated` + evolution.

---

## 2. Estado de cierre (integridad laudoada)

| Vector | Estado |
|--------|--------|
| Runtime DI | **Entregado** — no reescrito |
| Taxonomía | 7 términos v1.0.4 |
| Bindings | v1.3.0 |
| Process **con** `requires_capability` | **35** |
| Process **sin** `requires_capability` | **7** → PBI-045 |
| README DI (Norte Magnético) | **Hecho** — H-DOC |

---

## 3. Inventario — cierre

### 3.1–3.3 Homologados (H7–H9)

§3.1 FS ×8 · §3.2 routes ×3 (`bus:route`) · §3.3 auditorías ×5 (`qa:probe` / `audit:compliance`) — **DONE**.

### 3.4 Interactores

| ED | Capacidad | Estado |
|----|-----------|--------|
| `kalma2-interact` | `llm:interact` | **DONE** H10-A |
| 7 ED restantes | — | **Diferidas** → [PBI-045](../pending/[ARQUITECTURA]%20PBI-045%20—%20DI%20para%20Gobernanza,%20Lotes%20y%20Notificaciones%20(Hito%2011).md) |

Residual encapsulado: `cerbero-governance-react`, `radamanto-batch`, `memory-evolution-ingest`, `execute-suite`, `telegram-gateway`, `telegram-fallback-responder`, `capsule-invoke-smoke`.

---

## 4. Plan de olas — veredicto

| Hito | Resultado |
|------|-----------|
| H7 FS | APTO · PR #144 |
| H8 route | APTO · PR #147 |
| H9 auditorías | APTO · PR #149 |
| H10-A llm | APTO · PR #151 |
| H10-B miscelánea | **RECHAZADO** materializar (Filtro C) → PBI-045 |
| H-DOC README | APTO · ciclo de cierre |
| R10 EDA-only | Fuera (no bloquea Done) |

**Laudo Estructural 2026-07-23:** Done global con integridad 35/7; prohibido empujar 5 altas ortogonales en una ola.

---

## 5. Preguntas — cerradas o trasplantadas

Q1–Q3 resueltas en H7–H9. Capacidad telegram/gobernanza/lotes → PBI-045.

---

## 6. Criterios de aceptación — cierre

| ID | Veredicto |
|----|-----------|
| **AC-DONE-PBI** | **APTO** por laudo — H7–H10-A + H-DOC; H10-B fuera del perímetro |
| **AC-NO-INVENT** | Preservado |
| **AC-ORPHAN** / **AC-REG** | Verdes en hitos entregados |

---

## 7. Fuera de alcance (histórico + spawn)

| Ítem | Destino |
|------|---------|
| 7 ED H10-B | **PBI-045** (Hito 11) |
| R10 EDA-only | Solo laudo |
| PPR #136 | PBI distinto |

---

## 8. Done global

```text
Done(PBI-043) = H7–H10-A APTO + H-DOC README
 + validacion.md APTO pbi_archived: true
 + PBI en docs/todos/done/
 + residual 7 ED en PBI-045 pending
```

---

## 9. Referencias

- PBI padre: `docs/todos/done/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md`
- Spawn: `docs/todos/pending/[ARQUITECTURA] PBI-045 — DI para Gobernanza, Lotes y Notificaciones (Hito 11).md`
- H-DOC: `docs/features/inyeccion-dependencias-h-doc-readme/`
- Norma / bindings: Códice DI vigente
