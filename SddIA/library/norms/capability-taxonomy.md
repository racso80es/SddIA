---
uuid: "e9c66ec6-5b59-4aae-b9f2-91cc313fe295"
name: "capability-taxonomy"
version: "1.0.0"
nature: "tactical-norm"
author: "tekton"
scope: "agnostic"
category: "architecture"
hash_signature: "sha256:fdb04b61b8abcf8910b5a1bef27578a89d162e31dd55a26ebb98feac70c437a8"
catalog:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
    description: "Cierre documental en rama (archivo PBI / validacion pbi_archived)."
  - id: "proc:git-sync"
    contract: "proc.git_sync"
    version: "1.0.0"
    description: "Operaciones Git gobernadas (sync, branch, commit) vía cápsula git-manager."
  - id: "fs:persist"
    contract: "fs.persist"
    version: "1.0.0"
    description: "Persistencia de artefactos vía filesystem-manager (READ/WRITE/LIST/DELETE/CREATE/MOVE) en forja/índice/workspace."
  - id: "bus:route"
    contract: "bus.route"
    version: "1.0.0"
    description: "Enrutado / fan-out del bus EDA fractal (domain, orchestration, telemetry) vía cápsula bus-operator."
  - id: "qa:probe"
    contract: "qa.probe"
    version: "1.0.0"
    description: "Sonda de caos / auditoría empírica vía tools (sandbox-breacher, schema-corruptor, io-choke, event-bus-audit)."
  - id: "audit:compliance"
    contract: "audit.compliance"
    version: "1.0.0"
    description: "Auditoría de cumplimiento termodinámico/gobernanza (exclusiva telemetry-compliance-audit); distinta de qa:probe (Caos)."
  - id: "llm:interact"
    contract: "llm.interact"
    version: "1.0.0"
    description: "Interacción LLM gobernada vía skill mayeuta-llm (síntesis / clasificación de intención)."
  - id: "llm:infer"
    contract: "llm.infer"
    version: "1.0.0"
    description: "Inferencia cruda de proveedor LLM vía adaptador (tool:/skill:); error_code tipado; consumida por tool:llm-router. Ortogonal a llm:interact."
  - id: "gov:rbac"
    contract: "gov.rbac"
    version: "1.0.0"
    description: "Gobernanza RBAC / reacción Self-Healing Cerbero (revocación/rehabilitación); ortogonal a audit:compliance."
  - id: "channel:ingest"
    contract: "channel.ingest"
    version: "1.0.0"
    description: "Ingesta aferente de canal externo (Telegram) hacia eventos domain vía tool telegram-gateway."
  - id: "agenda:persist"
    contract: "agenda.persist"
    version: "1.0.0"
    description: "Asiento de agenda local de instancia vía skill agenda-manager."
  - id: "memory:pref-write"
    contract: "memory.pref_write"
    version: "1.0.0"
    description: "Escritura de preferencias del usuario en store local de instancia vía user-preference-store."
  - id: "memory:pref-query"
    contract: "memory.pref_query"
    version: "1.0.0"
    description: "Consulta opt-in de preferencias activas del usuario vía user-preference-store."
  - id: "memory:evolution-store"
    contract: "memory.evolution_store"
    version: "1.0.0"
    description: "Persistencia y consulta vectorial de EvolutionEvent en LanceDB vía proveedor DI."
  - id: "memory:pref-vector"
    contract: "memory.pref_vector"
    version: "1.0.0"
    description: "Réplica y consulta vectorial de preferencias en LanceDB vía proveedor DI."
  - id: "memory:thought-graph"
    contract: "memory.thought_graph"
    version: "1.0.0"
    description: "Búsqueda y persistencia de ThoughtNode en LanceDB vía proveedor DI."
  - id: "tracker:lifecycle"
    contract: "tracker.lifecycle"
    version: "1.0.0"
    description: "Ciclo de vida de un ítem de gestión. La salida auditada es el stdout de emit_v2 (success, exitCode, message, result)."
  - id: "eda:event-manager"
    contract: "eda.event_manager"
    version: "2.0.0"
    description: "EventManagerPort 2.0 — ciclo de vida, consultas, DLT/admin y verify ECST; store vía host Redb (MAN-43)."
  - id: "agent:runtime"
    contract: "agent.runtime"
    version: "1.0.0"
    description: "AgentRuntimePort 1.0 — ejecución cognitiva de fases de agentes del Core por thinking_profile y proveedor DI sin auto-failover (AUT-122)."
---

## Propósito

**Códice de la Lengua** — Taxonomía Universal de Capacidades SddIA.

Ninguna Entidad de Dominio puede declarar `provides` o `requires_capability` con un `id` que no figure en el catálogo machine-readable de esta norma (`catalog` en frontmatter). La invención libre de términos (`doc:close` vs `document:closure`, etc.) es **Entropía Semántica** y falla el Filtro A.

Alta y mutación del `catalog` (incluido bump de versión por capacidad): únicamente vía `./sddia-run.sh --process entity-manager` con `entity_class: norm`, `lifecycle_operation: update` y `semantic_seed` (`artifact_yaml_replacements` / `artifact_body_replacements` según el corte). Prohibida mutación manual del `.md` (DA-2 / `external-ai-constraints`). Cierre: sello EDA `Domain_Entity_Updated` + registro en `SddIA/evolution/` cuando el corte lo exija (R11).

## Catálogo vigente (legible)

| id | contract | version | Descripción |
|----|----------|---------|-------------|
| `doc:closure` | `doc.closure` | 1.0.0 | Cierre documental en rama |
| `proc:git-sync` | `proc.git_sync` | 1.0.0 | Operaciones Git gobernadas vía git-manager |
| `fs:persist` | `fs.persist` | 1.0.0 | Persistencia artefactos vía filesystem-manager |
| `bus:route` | `bus.route` | 1.0.0 | Fan-out bus EDA fractal vía bus-operator |
| `qa:probe` | `qa.probe` | 1.0.0 | Sonda Caos / auditoría empírica vía tools |
| `audit:compliance` | `audit.compliance` | 1.0.0 | Cumplimiento termodinámico (Gobernanza; ≠ Caos) |
| `llm:interact` | `llm.interact` | 1.0.0 | Interacción LLM vía mayeuta-llm |
| `llm:infer` | `llm.infer` | 1.0.0 | Inferencia cruda de proveedor vía adaptador (≠ interact) |
| `gov:rbac` | `gov.rbac` | 1.0.0 | Gobernanza RBAC Cerbero Self-Healing |
| `channel:ingest` | `channel.ingest` | 1.0.0 | Ingesta canal Telegram → domain |
| `agenda:persist` | `agenda.persist` | 1.0.0 | Agenda local de instancia vía agenda-manager |
| `memory:pref-write` | `memory.pref_write` | 1.0.0 | Escritura preferencias usuario (store local) |
| `memory:pref-query` | `memory.pref_query` | 1.0.0 | Consulta opt-in preferencias activas |
| `memory:evolution-store` | `memory.evolution_store` | 1.0.0 | Persistencia vectorial EvolutionEvent (LanceDB) |
| `memory:pref-vector` | `memory.pref_vector` | 1.0.0 | Réplica y consulta vectorial de preferencias (LanceDB) |
| `memory:thought-graph` | `memory.thought_graph` | 1.0.0 | Búsqueda y persistencia de ThoughtNode (LanceDB) |
| `tracker:lifecycle` | `tracker.lifecycle` | 1.0.0 | Ciclo de vida de un ítem de gestión; stdout `emit_v2` |
| `eda:event-manager` | `eda.event_manager` | 2.0.0 | EventManagerPort 2.0 / host Redb (MAN-43) |
| `agent:runtime` | `agent.runtime` | 1.0.0 | AgentRuntimePort 1.0 / proveedor LLM activo sin auto-failover (AUT-122) |

## Obligaciones de aduana

- Prohibido declarar capacidades no indexadas en `catalog`.
- Prohibido usar `spec.json` como soporte de `provides`/`requires_capability`; SSOT = `{name}.md`.
- El contrato I/O de cada capacidad vive bajo el path Cúmulo `capability_contracts` (`{contract}.schema.json`).
- **Rigor taxonómico:** `qa:probe` (Caos/sonda) ≠ `audit:compliance` (Gobernanza/cumplimiento). Prohibido reuso cruzado.
- **Rigor H11:** `gov:rbac` ≠ `audit:compliance`; `channel:ingest` ≠ `bus:route`.
- **Rigor L-TERM:** `llm:infer` (adaptador / inferencia cruda) ≠ `llm:interact` (Mayeuta / síntesis e intención). Prohibido `provides llm:interact` en adaptadores de proveedor.
- **Rigor tracker:** el id es `tracker:lifecycle` (`dominio:verbo`). El contrato es `tracker.lifecycle`. Prohibido el id `tracker.lifecycle`.
- **Rigor eda:** el id es `eda:event-manager` (`dominio:verbo`). El contrato es `eda.event_manager`. Prohibido el id `eda.event_manager` en `catalog`.
- **Rigor eda 2.0:** la versión de catálogo de `eda:event-manager` es **2.0.0** y debe alinearse con `contract_version` en `capability-bindings` y `EventManagerPort` en core (MAN-43); el corte documental se sella por `entity-manager`, no por edit IDE.
- Referencia: PBI-042/043/045 features DI; H11 `inyeccion-dependencias-h11-gobernanza-lotes-notif`; PBI-MULTI-LLM-ROUTER `llm:infer`; PBI-EDA-001 / MAN-32 `eda:event-manager`; MAN-35 cutover 1.2 (histórico); MAN-43 contrato 2.0.0.
