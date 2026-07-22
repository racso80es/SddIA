---
document_id: PBI-042-INYECCION-DEPENDENCIAS-CAPACIDADES
title: "[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos"
format: markdown
version: "1.1.0"
created: "2026-07-21"
promoted_from: docs/todos/kitchen/PBI_Inyeccion_Dependencias_Capacidades.md
promoted_at: "2026-07-21"
status: abierto
priority: alta
process: feature
mvp_status: entregado_en_rama
mvp_feature: docs/features/inyeccion-dependencias-capacidades
mvp_branch: feat/inyeccion-dependencias-capacidades
related:
  - docs/features/inyeccion-dependencias-capacidades/spec.md
  - docs/features/inyeccion-dependencias-capacidades/plan.md
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/engine/execute-process/src/engine/capability_di_gate.rs
---

# [ARQUITECTURA] PBI-042: DI por Capacidades y Validación de Contratos Semánticos

## 1. Historia de Usuario

* **Como:** Arquitecto del Core de SddIA (Nodo de Control / Orquestador Central).
* **Quiero:** Sustituir acoplamientos rígidos entre Entidades de Dominio por DI gobernada por capacidades semánticas y contratos JSON Schema.
* **Para:** Ceguera espacial, intercambiabilidad de artefactos y Filtro A contra alucinación estructural.

**Corrección ontológica (Mayeuta):** SSOT de ED = `{name}.md` + frontmatter. Prohibido `spec.json`.

---

## 2. Estado del MVP (ciclo `inyeccion-dependencias-capacidades`)

| Vector | Estado |
|--------|--------|
| Metadatos Activos (`provides` / `requires_capability`) | **Hecho** — contratos + piloto `feature` ↔ `filesystem-manager` |
| Códice de la Lengua (`capability-taxonomy`, alta `doc:closure`) | **Hecho** — Library_Norm vía `entity-manager` |
| Aduana Temprana (`capability_di_gate` pre-ignición en `execute-process`) | **Hecho** — path síncrono; tests AC-P1/P2/P3 |
| Contrato piloto `doc.closure.schema.json` + Cúmulo 1.5.2 | **Hecho** |
| DLQ ante fallo contractual | **Hecho** (escritura en `./.events/dead-letter`) |

DoD escenarios 1–3 del PBI original: **cubiertos en MVP síncrono** (sin resolución ciega por Library_Codex; `delegates_to` sigue anclando el artefacto físico).

---

## 3. Pendiente fuera del MVP (backlog post-merge)

> Esta sección es el residual explícito: **no** bloquea el Done del feature MVP actual.

### Hito 2 — Resolución ciega e inyección

| ID | Ítem | Notas |
|----|------|-------|
| **R1** | Injector que resuelve `requires_capability` → artefacto físico **sin** depender de `delegates_to` por identidad | Hoy el gate valida coherencia provides↔requires; no sustituye el binding |
| **R2** | Wrapper CLI: inyectar rutas/contratos resueltos en el JSON de `stdin` de la cápsula | Matriz original §4.2 |
| **R3** | Mapa capability→artefacto (binding table en Cúmulo o entidad dedicada) | **No** sobrecargar `Library_Codex` de normas (L-CODEX-ROLE); decidir entidad nueva vs ampliación explícita de contrato de códice |
| **R4** | Migración masiva: anotar `provides`/`requires_capability` en el catálogo ED (process/action/skill/tool/agent) | MVP solo piloto cierre documental |

### Hito 3 — Gobernanza y asincronía

| ID | Ítem | Notas |
|----|------|-------|
| **R5** | Cerbero: cruce RBAC + (opcional) revalidación de schema DI en el payload empaquetado | MVP: aduana DI = gate en `execute-process`; Cerbero permanece RBAC |
| **R6** | Composición DI 100% EDA (PBI §2.6): sin hilo síncrono de fases; ECST post-cápsula en `./.events/` | Bus Core = `./.events/` (no `.SddIA/events/`) |
| **R7** | Expansión del Códice de la Lengua más allá de `doc:closure` | Altas vía `entity-manager` update + evolution |
| **R8** | Validación JSON Schema runtime del **payload** de salida (no solo firma declarada de `outputs`) | MVP: contraste `required` del schema vs outputs declarados del proveedor |

### Ortogonal (no este PBI)

- Plan maestro GesFer / Paciente 0 (otro PBI kitchen).
- Fractura Core F1 ya en curso en otro `persist_ref`.

---

## 4. Criterios de aceptación — residual (futuros ciclos)

| ID | Criterio |
|----|----------|
| **AC-R1** | Proceso declara solo `requires_capability`; runtime elige proveedor homologado sin `delegates_to` hardcodeado |
| **AC-R2** | Cápsula recibe en `stdin` el binding resuelto (paths/contrato) de forma ciega |
| **AC-R5** | Cerbero puede rechazar inject por RBAC aunque el gate DI haya pasado |
| **AC-R6** | Flujo piloto de DI vía evento de dominio + reacción asíncrona sin bloquear el orquestador de fases |

---

## 5. Referencias

- Feature MVP: `docs/features/inyeccion-dependencias-capacidades/`
- Norma: `SddIA/library/norms/capability-taxonomy.md`
- Gate: `SddIA/engine/execute-process/src/engine/capability_di_gate.rs`
- Semilla kitchen original: promovida desde `docs/todos/kitchen/PBI_Inyeccion_Dependencias_Capacidades.md` (2026-07-21)
