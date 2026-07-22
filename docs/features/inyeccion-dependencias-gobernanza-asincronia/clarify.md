---
feature_name: inyeccion-dependencias-gobernanza-asincronia
created: "2026-07-22"
purpose: Estabilización Hito 3 PBI-042 — gobernanza Cerbero, EDA DI, códice y schema salida (R5–R8)
branch_name: feat/inyeccion-dependencias-gobernanza-asincronia
persist_ref: docs/features/inyeccion-dependencias-gobernanza-asincronia
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-GOBERNANZA-ASINCRONIA
execution_id: f8b2c4d1-6e3a-4f7b-9c2d-1a0e5f8b3c7d
phase: mayeuta-stabilization
agents: mayeuta
---

# Clarificación — PBI-042 Hito 3 (gobernanza y asincronía)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md` (v1.2.0; Hito 2 entregado PR #127 merge `60c4635`).
- **Ciclo:** feature `inyeccion-dependencias-gobernanza-asincronia` · rama `feat/inyeccion-dependencias-gobernanza-asincronia`.
- **Alcance declarado:** Hito 3 — R5, R6, R7, R8. Criterios **producto:** **AC-R5**, **AC-R6**. Regresión: AC-R1, AC-R2, AC-P1, AC-P2, AC-P3.
- **Precedente cerrado:** `docs/features/inyeccion-dependencias-resolucion-ciega` — `capability_di_resolver`, `di_binding` v2, `capability-bindings.md`, orden `resolve → gate → inject`.
- **Precedente MVP:** `docs/features/inyeccion-dependencias-capacidades` — `capability_di_gate`, taxonomía `doc:closure`, DLQ `./.events/dead-letter`.
- **Normas / SSOT:** `capability-taxonomy.md`, `capability-bindings.md`, `cumulo.paths.json` (`capability_di.bindings`, `event_bus` → `./.events/`).
- **Fuera de alcance:** GesFer / Paciente 0; Fractura Core F1; migración masiva catálogo ED.

## D1 — Matriz de validación (residual × estado post-Hito 2)

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| Aduana DI pre-ignición (`capability_di_gate`) | **Hecho (MVP+H2)** | `capability_di_gate.rs`; orden canónico en `capability_di_resolver.rs` §resolve→gate→inject |
| Resolución ciega + `di_binding` en stdin | **Hecho (H2)** | `capability_di_resolver.rs`, `phase_capsules.rs`, PR #127 |
| Cerbero cruza RBAC antes de invocar cápsula | **Parcial** | `cerbero.md` define RBAC; runtime DI hoy no garantiza rechazo **posterior** al gate DI (AC-R5 ausente como test explícito) |
| Cerbero revalida schema DI en payload empaquetado | **Ausente** | O3 MVP: Cerbero **no** es aduana DI; gate único en `execute-process` |
| Composición DI 100% EDA (§2.6) sin hilo síncrono de fases | **Ausente** | H2 fijó **L-SYNC-PATH**; composición DI sigue en `executor.rs` / `residual_runner.rs` de forma síncrona |
| ECST post-cápsula en `./.events/` | **Parcial** | Bus Core = `./.events/` (`eda_bus_topology`); DI no emite evento de dominio piloto ni reacción async desacoplada |
| Códice multi-término (`doc:closure` único) | **Parcial** | `capability-taxonomy.md` catalog = 1 fila |
| Validación schema: `required` vs outputs **declarados** | **Hecho (MVP)** | `capability_di_gate.rs` `output_keys` × `schema_required_keys` pre-ignición |
| Validación schema: payload **real** de salida | **Ausente** | R8 — no hay validador post-ejecución del JSON stdout de cápsula |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT3-SCOPE** | Este ciclo = R5–R8 + AC-R5/AC-R6 producto + regresión AC-R1/R2 + AC-P1/P2/P3. GesFer, F1 y migración masiva ED quedan fuera. |
| **L-GATE-PRESERVE** | `capability_di_gate` **permanece** Aduana Temprana contractual. R5 **no** sustituye el gate; Cerbero opera en capa de gobernanza RBAC (y opcionalmente revalidación DI del envelope). |
| **L-CERBERO-ORDER** | Orden mínimo en path DI: `resolve` → `capability_di_gate` → **Cerbero RBAC** → ignición/inject. AC-R5 exige que Cerbero pueda abortar **después** de gate APTO. |
| **L-CERBERO-RBAC-ONLY** | MVP Hito 3: Cerbero = RBAC (`allowed_policies` × `context` de cápsula destino resuelta). Revalidación schema sobre `di_binding` = **opcional** (R5); si Dedalo la omite, no bloquea Done si AC-R5 RBAC está demostrado. |
| **L-EDA-PILOT** | R6/AC-R6 **no** elimina el path síncrono H2 en este ciclo. Se añade **piloto EDA**: evento de dominio DI + reacción async que completa/resuelve binding **sin bloquear** el orquestador de fases. Regresión sync permanece verde. |
| **L-BUS** | Bus Core = `./.events/` (`cumulo.event_bus` / `eda_bus`). Prohibido `.SddIA/events/` como bus de composición DI. ECST post-cápsula escribe bajo topología `eda_bus_topology`. |
| **L-R7-MIN** | R7 = al menos **un** término nuevo en `capability-taxonomy` catalog (≠ `doc:closure`) vía `entity-manager` update + `SddIA/evolution/`. Sin invención libre; contrato JSON Schema bajo `capability_contracts`. |
| **L-R8-DELTA** | R8 extiende MVP: además del contraste pre-ignición (declarado), validar **payload real** de salida de cápsula contra schema del contrato. Fallo → abort/DLQ/análogo gate (Dedalo fija locus). |
| **L-CODEX-ROLE** | Reafirmado: binding table ≠ taxonomía. R7 solo expande `capability-taxonomy`; R3 mapa sigue en `capability-bindings.md`. |
| **L-PBI-LOC** | PBI-042 permanece en `pending/` hasta cerrar residual o laudo Racso; este feature no archiva el PBI padre solo por cerrar Hito 3. |
| **L-GESFER** | Ortogonal; no absorber Paciente 0 / Fractura Core en este `persist_ref`. |

## D3 — Ambigüedades acotadas (handoff Dedalo — no diseño Mayeuta)

| # | Pregunta | Opciones admisibles | Criterio de cierre |
|---|----------|---------------------|--------------------|
| **Q1** | Punto de intercepción Cerbero en cadena DI | (A) tras gate, pre-`phase_capsules` inject; (B) dentro del wrapper de invocación cápsula con `target_capsule` = proveedor resuelto | Debe demostrar AC-R5: gate OK + Cerbero RBAC deny → sin ignición |
| **Q2** | Revalidación schema DI en Cerbero (R5 opcional) | (A) omitir en Hito 3; (B) Cerbero contrasta `di_binding.contract` + refs schema vs envelope | Si (B), no duplicar lógica gate salvo sobre payload empaquetado |
| **Q3** | Evento piloto EDA (R6) | Tipo de dominio, payload mínimo (`requires_capability`, `di_binding`, correlation), emisor | Flujo async observable en `./.events/pending/` sin bloquear hilo fases |
| **Q4** | Reactor async DI | Cápsula/handler existente vs módulo engine dedicado; ack ECST post-cápsula | AC-R6: orquestador no espera resolución DI del piloto |
| **Q5** | Coexistencia sync / async | Path H2 intacto para regresión; flag/env o fase piloto que usa solo EDA | AC-R1/R2 + AC-P1–P3 verdes en CI |
| **Q6** | Locus validación R8 (payload salida) | (A) hook post-cápsula en `execute-process`; (B) cápsula validator dedicada invocada tras stdout | Schema = contrato de `requires_capability`; fallo trazable |
| **Q7** | Término R7 + fila binding | Par capability nueva ↔ proveedor piloto + schema + fila en `capability-bindings.md` | Alta norma + evolution UUID; al menos un binding usable por piloto EDA o sync |

## D4 — Criterios producto estabilizados (este ciclo)

| ID | Criterio | Verificación esperada (Argos) |
|----|----------|-------------------------------|
| **AC-R5** | Cerbero rechaza inject por RBAC aunque `capability_di_gate` haya pasado | Test engine / smoke: proveedor resuelto con contexto no autorizado → abort; gate previo APTO |
| **AC-R6** | Flujo piloto DI vía evento de dominio + reacción async sin bloquear orquestador de fases | Traza `./.events/` + test: fase no espera sync resolve del piloto |
| **AC-R7** *(derivado R7)* | ≥1 término nuevo en `capability-taxonomy` + evolution | Diff norma + registro evolution |
| **AC-R8** *(derivado R8)* | Validación JSON Schema del payload **real** de salida vs contrato | Test con stdout inválido → fallo contractual trazable |

Regresión obligatoria:

| ID | Criterio |
|----|----------|
| **AC-R1** | Resolución ciega sin `delegates_to` hardcodeado (H2) |
| **AC-R2** | `di_binding` en stdin cápsula (H2) |
| **AC-P1** | Homologación OK → ignición (MVP) |
| **AC-P2** | Incumplimiento contrato pre-ignición → DLQ (MVP) |
| **AC-P3** | Capacidad no indexada → abort limpio (MVP) |

## D5 — Veredicto

**ok** — Requisitos Hito 3 termodinámicamente estables. Handoff a Dedalo: diseño Cerbero/EDA/códice/schema-salida (Q1–Q7) → `spec.md` / `plan.md`.
