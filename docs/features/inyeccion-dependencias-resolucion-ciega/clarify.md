---
feature_name: inyeccion-dependencias-resolucion-ciega
created: "2026-07-22"
purpose: Estabilización Hito 2 PBI-042 — resolución ciega e inyección (R1–R4)
branch_name: feat/inyeccion-dependencias-resolucion-ciega
persist_ref: docs/features/inyeccion-dependencias-resolucion-ciega
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-RESOLUCION-CIEGA-INYECCION
execution_id: a7e3c9f2-4b1d-4e8a-9c5f-2d6b8e1a0f47
phase: mayeuta-stabilization
---

# Clarificación — PBI-042 Hito 2 (resolución ciega e inyección)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md` (v1.1.0; MVP entregado PR #126).
- **Ciclo:** feature `inyeccion-dependencias-resolucion-ciega` · rama `feat/inyeccion-dependencias-resolucion-ciega`.
- **Alcance declarado:** Hito 2 — R1, R2, R3, R4 (piloto ampliado). Criterios producto: **AC-R1**, **AC-R2**.
- **Precedente cerrado:** `docs/features/inyeccion-dependencias-capacidades` (Metadatos + Códice + Aduana). Gate vivo: `capability_di_gate.rs` (valida; **no** sustituye binding).
- **Norma:** `SddIA/library/norms/capability-taxonomy.md` (hoy solo `doc:closure`).
- **Fuera de alcance:** Hito 3 (R5–R8); GesFer; Fractura Core F1.

## D1 — Matriz de validación (residual × estado post-MVP)

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| Gate valida `requires` ↔ `provides` vía `delegates_to` | **Hecho (MVP)** | `capability_di_gate.rs` paso 3 lee identidad de `delegates_to` |
| Runtime elige proveedor sin identidad hardcodeada | **Ausente** | Fase piloto en `feature.md` aún declara `delegates_to: skill:filesystem-manager` |
| Binding table capability→artefacto | **Ausente** | Cúmulo tiene `capability_contracts`; no mapa DI. L-CODEX-ROLE prohíbe sobrecargar Library_Codex de normas |
| Inject en stdin JSON de cápsula (paths/contrato) | **Ausente** | Invocación actual no empaqueta binding resuelto como payload DI |
| Anotación ED más allá de piloto cierre | **Parcial** | Solo `feature` ↔ `filesystem-manager` + `doc:closure` |
| Taxonomía multi-término | **No este ciclo como R7** | Altas nuevas = Hito 3/R7; Hito 2 puede reutilizar `doc:closure` y/o anotar EDs ya homologables |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT2-SCOPE** | Este ciclo = **solo** R1–R4 + AC-R1/AC-R2. R5–R8, GesFer y F1 quedan explícitamente fuera. |
| **L-GATE-PRESERVE** | `capability_di_gate` **permanece** como Aduana Temprana. El injector **no** la reemplaza: resuelve binding y alimenta/adapta la validación. Fallos contractuales siguen DLQ. |
| **L-BLIND-RESOLVE** | AC-R1 es innegociable: el consumidor piloto debe poder declarar **solo** `requires_capability` (sin `delegates_to` por identidad). Si coexisten ambos en una fase, Dedalo define precedencia; el DoD del ciclo exige al menos un path piloto **ciego**. |
| **L-CODEX-ROLE** | Reafirmado: **prohibido** usar Library_Codex de normas como router DI. Binding = entidad/mapa dedicado o ampliación explícita de topología Cúmulo — **no** empaquetado de normas. |
| **L-R4-PILOT** | R4 en este ciclo = **piloto ampliado** (anotar un conjunto acotado de EDs process/action/skill adicionales), **no** migración masiva del catálogo entero (esa lectura del PBI §3 R4 se rebaná). |
| **L-TAX-BASE** | Capacidad ancla del piloto ciego = `doc:closure` (ya homologada). Altas nuevas al Códice = fuera (R7) salvo que Dedalo demuestre necesidad mínima para un segundo binding; en ese caso alta vía `entity-manager` + evolution. |
| **L-SYNC-PATH** | Composición sigue en path síncrono de `execute-process` (L-SYNC del MVP). EDA-only = R6 / Hito 3. |
| **L-PBI-LOC** | PBI-042 permanece en `docs/todos/pending/` hasta cerrar residual o archivar tras Hitos restantes; este feature no archiva el PBI padre solo por cerrar Hito 2 (salvo laudo Racso distinto al cierre). |
| **L-GESFER** | Ortogonal; no absorber Paciente 0 / Fractura Core en este `persist_ref`. |

## D3 — Ambigüedades acotadas (handoff Dedalo — no diseño Mayeuta)

| # | Pregunta | Opciones admisibles | Criterio de cierre |
|---|----------|---------------------|--------------------|
| **Q1** | Locus del mapa capability→artefacto (R3) | (A) clave/tabla en `cumulo.paths.json` / topología Cúmulo; (B) entidad dedicada `{name}.md` (p. ej. bajo library/core) indexada y referenciada desde Cúmulo | Debe ser SSOT machine-readable, forjable vía procesos canónicos, y **no** Library_Codex de normas |
| **Q2** | Ambigüedad N proveedores `provides` la misma `id` | (A) error duro `CAPABILITY_PROVIDER_AMBIGUOUS`; (B) política de preferencia explícita en el mapa (única fila canónica por capability) | Preferencia Mayeuta: **una fila canónica por capability** en el mapa; ambigüedad sin fila = abort limpio |
| **Q3** | Relación injector ↔ gate | (A) resolve → gate sobre artefacto resuelto → ignición; (B) gate genérico + resolve solo para empaquetado stdin | Debe preservar AC-P1–P3 del MVP sobre el proveedor elegido |
| **Q4** | Forma del payload inject (R2) | Campos mínimos en stdin JSON: identidad resuelta, path lógico/contrato, refs Cúmulo necesarias a la cápsula | Compatible con `capsule-json-io`; sin hardcode de rutas fuera de topología |
| **Q5** | Conjunto piloto R4 | Lista explícita de EDs a anotar en `spec.md`/`plan.md` | ≥1 consumidor ciego + ≥1 proveedor; expansión limitada documentada (no catálogo completo) |

## D4 — Criterios producto estabilizados (este ciclo)

| ID | Criterio | Verificación esperada (Argos) |
|----|----------|-------------------------------|
| **AC-R1** | Proceso/fase piloto declara solo `requires_capability`; runtime elige proveedor homologado **sin** `delegates_to` hardcodeado | Genoma piloto + traza de resolución / test engine |
| **AC-R2** | Cápsula recibe en `stdin` el binding resuelto (paths/contrato) de forma ciega | Contrato I/O + test/smoke de invocación |
| **AC-R3** *(derivado)* | Existe mapa SSOT capability→artefacto distinto de Library_Codex normas | Artefacto en topología + lectura runtime |
| **AC-R4** *(derivado)* | Piloto ampliado: EDs adicionales anotadas `provides`/`requires_capability` más allá del par MVP original | Diff genoma acotado |

Regresión: AC-P1, AC-P2, AC-P3 del MVP **siguen verdes**.

## D5 — Veredicto

**ok** — Requisitos Hito 2 termodinámicamente estables. Handoff a Dedalo: diseño de injector + binding table (Q1–Q5) → `spec.md` / `plan.md`.
