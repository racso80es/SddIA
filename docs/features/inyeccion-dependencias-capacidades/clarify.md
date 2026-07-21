---
feature_name: inyeccion-dependencias-capacidades
created: "2026-07-21"
process: feature
purpose: Validación de contenido PBI-042 vs estructura actual del Core
branch_name: feat/inyeccion-dependencias-capacidades
persist_ref: docs/features/inyeccion-dependencias-capacidades
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-INYECCION-DEPENDENCIAS-CAPACIDADES
execution_id: 9120e3da-6ba9-4a93-9735-34486383c7de
phase: mayeuta-stabilization
---

# Clarificación — PBI-042 DI por capacidades

## D0 — Semilla

PBI kitchen `PBI_Inyeccion_Dependencias_Capacidades.md` (PBI-042). Init lab: `execute-process feature` → `workspace-init` **executed**; agent-runtime CLI falló (EPROTO SSL). Esta fase Mayeuta se materializa en IDE (relay Tekton) como primera validación de contenido vs topología real.

## D1 — Matriz de validación (PBI × estructura actual)

| Afirmación del PBI | Estado en repo | Evidencia SSOT |
|--------------------|----------------|----------------|
| Contrato ED en `spec.json` con `provides` / `requires_capability` | **Desalineado** | Estándar atómico: `{name}.md` + YAML frontmatter. `spec.json` = fósil prohibido (`.cursorrules`, `entidades-dominio-ecosistema-sddia.md`, `features-documentation-pattern`). |
| Capacidades declaradas en metadatos de ED | **Parcial** | Acciones/procesos ya tienen `capabilities: []` (strings libres). No hay `provides` ni `requires_capability`. Binding de fases = `delegates_to: ["tipo:nombre"]` (identidad, no abstracción). |
| Diccionario Universal de Capacidades en `SddIA/norms/` | **Ausente** | No existe norma/códex de taxonomía de capacidades. Forja usa `capability_name(name)` = slug del nombre, no glosario homologado. |
| Contratos I/O por capacidad (JSON Schema) | **Ausente como índice** | I/O vive en frontmatter/`inputs`/`outputs` de cada `{name}.md` + `capsule-json-io.md`. No hay archivo de contrato por capacidad. |
| Library_Codex resuelve `capability → artefacto físico` | **Rol distinto** | `library/codexes/` = domain-codex (empaquetado de normas por entorno). No es mapa DI. |
| Cúmulo como plano de enrutamiento DI | **Parcial / otra semántica** | `cumulo.paths.json` = topología de paths/contratos/EDA. No inyecta dependencias por capacidad. |
| Cerbero valida esquema + RBAC antes del inject | **Parcial** | Cerbero = RBAC / `revoked_entities` / `cerbero-governance-react`. No es aduana de contratos DI en runtime de inject. |
| Bus de eventos en `.SddIA/events/` | **Incorrecto para Core** | SSOT: `event_bus` = `./.events/`. `.SddIA/events` = `eda_instance.customization` (instancia). |
| DI sin llamadas síncronas (solo EDA) | **Contradice runtime actual** | `execute-process` orquesta fases de forma secuencial vía `delegates_to` / `phase_invocations`. EDA coexistente, no sustituye el hilo de fases. |
| Capacidad ejemplo `doc:closure` | **No indexada** | Análogo funcional: proceso `delivery-close-cycle` + fase «Cierre documental en rama» (`skill:filesystem-manager`). Binding por nombre de proceso/skill. |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| L-ARTIFACT | Sustituir en requisitos toda mención a `spec.json` por `{name}.md` + frontmatter. Extender esquema de metadatos existentes (`capabilities`) hacia `provides` / `requires_capability` — no inventar paralelo JSON. |
| L-GLOSSARY | El Diccionario Universal de Capacidades es **prerrequisito normativo** (nueva norma o códice bajo `directories.norms` / `library_*` vía `entity-manager`). Sin él, Escenario 3 del DoD no es implementable. |
| L-CODEX-ROLE | No sobrecargar `Library_Codex` con routing DI. Dedalo debe proponer entidad/mapa distinto (p. ej. binding table en Cúmulo o códice dedicado `capability-bindings`) o ampliar contrato de códice **explícitamente**. |
| L-BUS | Requisitos EDA deben citar `./.events/` (`cumulo.event_bus` / `eda_bus`), no `.SddIA/events/`. |
| L-SYNC | El DoD asíncrono puro (§2.6) se rebana: MVP puede validar inject + contrato en el path síncrono de `execute-process`; EDA ECST como notificación post-fase, no como único mecanismo de composición. |
| L-SCOPE-SLICE | Este ciclo feature **no** implementa el PBI entero. Fase documental actual = estabilización. Primera rebanada técnica propuesta a Dedalo: schema metadatos + glosario mínimo + validación pre-invoke de un caso piloto (`requires_capability` ↔ `provides` sobre cierre documental). |
| L-GESFER | Ortogonal al plan maestro GesFer (F1–F4). No absorber Fractura Core / inyección Paciente 0 en este `persist_ref`. |
| L-PBI-LOC | PBI promovido kitchen→`docs/todos/pending/` (2026-07-21) con residual R1–R8 fuera del MVP; **no** archivar a `done/` hasta cerrar residual o rebanar PBI hijo. |

## D3 — Resueltas (laudo Racso 2026-07-21 + Dedalo O1–O4)

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | Familias ED | **Metadatos Activos:** `provides` / `requires_capability` en `{name}.md`. MVP: process ↔ action\|skill (`spec.md` O1). |
| 2 | Locus glosario | **Códice de la Lengua:** Library_Norm `capability-taxonomy` (taxonomía universal; sin invención libre). |
| 3 | Aduana | **Aduana Temprana:** gate síncrono en `execute-process` pre-ignición (`capability_di_gate`). No Cerbero. |
| 4 | MVP sync | **Confirmado.** |

Detalle técnico: `spec.md` §3–§4 · blueprint: `plan.md`.

## D4 — Veredicto

**ok** — Handoff a Tekton con alcance MVP (M1+M2+M3) cerrado.