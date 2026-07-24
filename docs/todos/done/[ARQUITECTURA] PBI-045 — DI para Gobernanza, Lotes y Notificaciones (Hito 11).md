---
document_id: PBI-045-DI-GOBERNANZA-LOTES-NOTIFICACIONES
title: "[ARQUITECTURA] PBI-045 — DI para Gobernanza, Lotes y Notificaciones (Hito 11)"
format: markdown
version: "1.0.0"
created: "2026-07-23"
closed_at: "2026-07-23"
close_feature: docs/features/inyeccion-dependencias-h11-gobernanza-lotes-notif
close_branch: feat/inyeccion-dependencias-h11-gobernanza-lotes-notif
racso_countersign: "2026-07-23T14:49:00Z"
uuid: 0de372fb-c559-43e4-9238-8c91c45e606c
status: cerrado
priority: media
process: feature
spawned_from: docs/todos/done/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
spawned_at: "2026-07-23"
spawn_reason: "Laudo Estructural Racso Filtro C — H10-B rechazado en ciclo PBI-043; 7 ED residuales encapsuladas en PBI independiente"
inventory_count: 7
inventory_date: "2026-07-23"
baseline_taxonomy: "doc:closure | proc:git-sync | fs:persist | bus:route | qa:probe | audit:compliance | llm:interact"
baseline_bindings: SddIA/core/capability-bindings.md@1.3.0
related:
  - docs/todos/done/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
  - docs/todos/done/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/core/capability-bindings.md
---

# [ARQUITECTURA] PBI-045: DI para Gobernanza, Lotes y Notificaciones (Hito 11)

## 1. Historia de Usuario

* **Como:** Arquitecto del Core / Nodo de Control.
* **Quiero:** Homologar con DI las 7 ED process residuales de gobernanza, lotes, suites, memoria y canales.
* **Para:** Completar ceguera espacial sin atentar contra la economía termodinámica de una sola ola (Laudo Filtro C 2026-07-23).

**Precedente:** PBI-043 Done con integridad 35 with / 7 without. H10-B **no** materializado en PBI-043.

## 2. Inventario residual (N_ola = 7)

| ED | Delegados actuales | Notas |
|----|-------------------|-------|
| `cerbero-governance-react` | `agent:cerbero` | Gobernanza RBAC |
| `radamanto-batch` | `agent:radamanto` | Lote telemetría |
| `memory-evolution-ingest` | `agent:cumulo` | Ingesta vectorial |
| `execute-suite` | multi (cumulo/tekton/argos/radamanto + `action:execute-process`) | Orquestación Suite |
| `telegram-gateway` | `tool:telegram-gateway` | Canal |
| `telegram-fallback-responder` | `agent:mayeuta` + `tool:send-telegram-notification` | Canal + LLM |
| `capsule-invoke-smoke` | `tool:io-choke` | Smoke cápsula |

## 3. Restricciones

| ID | Norma |
|----|-------|
| **AC-NO-INVENT** | Altas Códice solo con laudo Racso; K altas por sub-ola (no 5 ortogonales en un ciclo) |
| **AC-THERMO** | Partir en sub-olas Dedalo si blast-radius |
| **AC-SEAL** | Mutación vía `entity-manager` + `Domain_Entity_Updated` + evolution |
| **AC-ORPHAN** | `orphan_count == 0` |
| **AC-REG** | Regresión DI MVP→H10-A |

## 4. Fuera de alcance

- Reescritura runtime DI (salvo bug).
- Reabrir PBI-043.
- GesFer / F1 / PPR #136.

## 5. Done

Done = homologación umbral Dedalo ≥ piso + PR merge + `validacion` APTO + PBI en `docs/todos/done/` + `pbi_archived: true`.
