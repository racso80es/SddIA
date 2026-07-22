---
document_id: PBI-042-INYECCION-DEPENDENCIAS-CAPACIDADES
title: "[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos"
format: markdown
version: "1.2.0"
created: "2026-07-21"
promoted_from: docs/todos/kitchen/PBI_Inyeccion_Dependencias_Capacidades.md
promoted_at: "2026-07-21"
status: abierto
priority: alta
process: feature
mvp_status: entregado_en_rama
mvp_feature: docs/features/inyeccion-dependencias-capacidades
mvp_branch: feat/inyeccion-dependencias-capacidades
hito2_status: entregado_en_main
hito2_feature: docs/features/inyeccion-dependencias-resolucion-ciega
hito2_branch: feat/inyeccion-dependencias-resolucion-ciega
hito2_pr: https://github.com/racso80es/SddIA/pull/127
hito2_merge_commit: 60c4635b351ee78c4f5d1050cc09e4bda3f8c6af
hito3_status: entregado_en_main
hito3_feature: docs/features/inyeccion-dependencias-gobernanza-asincronia
hito3_branch: feat/inyeccion-dependencias-gobernanza-asincronia
hito3_pr: https://github.com/racso80es/SddIA/pull/128
hito3_merge_commit: 51fd4344ac07ddb27fe96ba4c25c9c27f87a20ca
hito4_status: apto_en_rama
hito4_feature: docs/features/inyeccion-dependencias-envelope-homologacion
hito4_branch: feat/inyeccion-dependencias-envelope-homologacion
hito4_execution_id: 0ec31f97-ad31-4ae5-8005-dc6220bad185
related:
  - docs/features/inyeccion-dependencias-envelope-homologacion/objectives.md
  - docs/features/inyeccion-dependencias-envelope-homologacion/spec.md
  - docs/features/inyeccion-dependencias-envelope-homologacion/validacion.md
  - docs/features/inyeccion-dependencias-capacidades/spec.md
  - docs/features/inyeccion-dependencias-capacidades/plan.md
  - docs/features/inyeccion-dependencias-resolucion-ciega/spec.md
  - docs/features/inyeccion-dependencias-resolucion-ciega/validacion.md
  - docs/features/inyeccion-dependencias-gobernanza-asincronia/objectives.md
  - docs/features/inyeccion-dependencias-gobernanza-asincronia/spec.md
  - docs/features/inyeccion-dependencias-gobernanza-asincronia/plan.md
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/core/capability-bindings.md
  - SddIA/engine/execute-process/src/engine/capability_di_gate.rs
  - SddIA/engine/execute-process/src/engine/capability_di_resolver.rs
  - docs/todos/pending/[Kaizen] delivery-close — snapshot vacío y pr_body newlines en shell-executor.md
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

> Ciclo `docs/features/inyeccion-dependencias-resolucion-ciega` · PR [#127](https://github.com/racso80es/SddIA/pull/127) merge `60c4635` (2026-07-22). R4 = piloto ampliado (no migración masiva).

| ID | Ítem | Notas |
|----|------|-------|
| **R1** | Injector que resuelve `requires_capability` → artefacto físico **sin** depender de `delegates_to` por identidad | **Hecho en main** — `capability_di_resolver` |
| **R2** | Wrapper CLI: inyectar rutas/contratos resueltos en el JSON de `stdin` de la cápsula | **Hecho en main** — `di_binding` envelope v2 |
| **R3** | Mapa capability→artefacto (binding table en Cúmulo o entidad dedicada) | **Hecho en main** — `capability-bindings.md` + `capability_di.bindings` |
| **R4** | Anotar `provides`/`requires_capability` (piloto ampliado) | **Hecho en main** — `feature` + `bug-fix` ciegos; migración masiva diferida |

### Hito 3 — Gobernanza y asincronía

> Ciclo `docs/features/inyeccion-dependencias-gobernanza-asincronia` · PR [#128](https://github.com/racso80es/SddIA/pull/128) merge `51fd434` (2026-07-22).

| ID | Ítem | Notas |
|----|------|-------|
| **R5** | Cerbero: cruce RBAC + (opcional) revalidación de schema DI en el payload empaquetado | **Hecho en main** — `cerbero_di_rbac` post-gate; revalidación schema diferida (Q2) |
| **R6** | Composición DI 100% EDA (PBI §2.6): sin hilo síncrono de fases; ECST post-cápsula en `./.events/` | **Hecho en main** — piloto `CapabilityDi_*` + `SDDIA_DI_EDA_PILOT=1`; sync H2 default |
| **R7** | Expansión del Códice de la Lengua más allá de `doc:closure` | **Hecho en main** — `proc:git-sync` → `git-manager` |
| **R8** | Validación JSON Schema runtime del **payload** de salida (no solo firma declarada de `outputs`) | **Hecho en main** — `capability_di_output_validator` |

### Hito 4 — Envelope Cerbero + homologación catálogo

> Ciclo `docs/features/inyeccion-dependencias-envelope-homologacion` · rama `feat/inyeccion-dependencias-envelope-homologacion` · execution `0ec31f97-…` (2026-07-22). Argos: **APTO** (desbloqueo runtime).

| ID | Ítem | Notas |
|----|------|-------|
| **R9** | Cerbero revalida schema del envelope `di_binding` empaquetado (Q2 Hito 3) | **APTO en rama** — `cerbero_di_envelope.rs` + tests 24/24 |
| **R10** | Homologación ampliada catálogo ED (≥8 total; ≥4 nuevas) | **APTO en rama** — 8 ED; hash_signature recalc; L-R10-SEAL |

### Ortogonal (no este PBI)

- Plan maestro GesFer / Paciente 0 (otro PBI kitchen).
- Fractura Core F1 ya en curso en otro `persist_ref`.

---

## 4. Criterios de aceptación — residual (ciclo Hito 4+)

| ID | Criterio |
|----|----------|
| **AC-R1** | Proceso declara solo `requires_capability`; runtime elige proveedor homologado sin `delegates_to` hardcodeado |
| **AC-R2** | Cápsula recibe en `stdin` el binding resuelto (paths/contrato) de forma ciega |
| **AC-R5** | Cerbero puede rechazar inject por RBAC aunque el gate DI haya pasado |
| **AC-R6** | Flujo piloto de DI vía evento de dominio + reacción asíncrona sin bloquear el orquestador de fases |
| **AC-R9** | Cerbero rechaza inject si `di_binding` empaquetado incumple schema aunque gate+RBAC pasen |
| **AC-R10** | ≥8 ED homologadas con `provides`/`requires_capability` + bindings coherentes |

---

## 5. Referencias

- Feature MVP: `docs/features/inyeccion-dependencias-capacidades/`
- Norma: `SddIA/library/norms/capability-taxonomy.md`
- Gate: `SddIA/engine/execute-process/src/engine/capability_di_gate.rs`
- Semilla kitchen original: promovida desde `docs/todos/kitchen/PBI_Inyeccion_Dependencias_Capacidades.md` (2026-07-21)
