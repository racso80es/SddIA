---
feature_name: inyeccion-dependencias-envelope-homologacion
created: "2026-07-22"
purpose: Estabilización Hito 4 PBI-042 — revalidación envelope Cerbero + homologación catálogo ED (R9–R10)
branch_name: feat/inyeccion-dependencias-envelope-homologacion
persist_ref: docs/features/inyeccion-dependencias-envelope-homologacion
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-ENVELOPE-HOMOLOGACION
execution_id: c3d8f1a2-9e4b-4c7d-8f6a-1b2e3d4c5f6a
phase: mayeuta-stabilization
agents: mayeuta
---

# Clarificación — PBI-042 Hito 4 (envelope Cerbero + homologación catálogo)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md` (v1.2.0; Hito 3 entregado PR #128 merge `51fd434`).
- **Ciclo:** feature `inyeccion-dependencias-envelope-homologacion` · rama `feat/inyeccion-dependencias-envelope-homologacion`.
- **Alcance declarado:** Hito 4 — **R9**, **R10**. Criterios **producto:** **AC-R9**, **AC-R10**. Regresión: AC-R1, AC-R2, AC-R5, AC-R6, AC-R7, AC-R8, AC-P1, AC-P2, AC-P3.
- **Precedente cerrado Hito 3:** `docs/features/inyeccion-dependencias-gobernanza-asincronia` — `cerbero_di_rbac` post-gate (RBAC-only), piloto EDA `CapabilityDi_*`, `proc:git-sync`, `capability_di_output_validator`.
- **Precedente cerrado Hito 2:** `docs/features/inyeccion-dependencias-resolucion-ciega` — `capability_di_resolver`, `di_binding` v2, `capability-bindings.md`, orden `resolve → gate → inject`.
- **Precedente MVP:** `docs/features/inyeccion-dependencias-capacidades` — `capability_di_gate`, taxonomía `doc:closure`, DLQ `./.events/dead-letter`.
- **Remisión explícita Hito 3:** Q2 (spec §3) — revalidación schema del envelope `di_binding` en Cerbero **diferida**; este ciclo la materializa como **R9**.
- **Normas / SSOT:** `capability-taxonomy.md`, `capability-bindings.md`, `cumulo.paths.json` (`capability_di.bindings`, `capability_contracts`, `event_bus`).
- **Fuera de alcance:** GesFer / Paciente 0; Fractura Core F1; sustitución total sync→EDA-only; archivo PBI padre salvo laudo Racso al Done global.

## D1 — Matriz de validación (residual × estado post-Hito 3)

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| Resolución ciega + `di_binding` v2 en stdin | **Hecho (H2)** | `capability_di_resolver.rs`, `di_binding_object`, PR #127 |
| Aduana DI pre-ignición (`capability_di_gate`) | **Hecho (MVP+H2)** | `capability_di_gate.rs`; valida `requires_capability` × proveedor resuelto |
| Cerbero RBAC post-gate pre-inject | **Hecho (H3)** | `cerbero_di_rbac.rs`; AC-R5 |
| Cerbero revalida schema del **envelope** `di_binding` empaquetado | **Ausente** | Q2 Hito 3 laudó omitir; `cerbero_di_rbac` no contrasta payload empaquetado |
| Schema machine-readable del objeto `di_binding` | **Ausente** | Solo contratos de capacidad (`doc.closure`, `proc.git_sync`); sin `di.binding` |
| Piloto EDA DI async | **Hecho (H3)** | `CapabilityDi_*`, `capability_di_reactor.rs`, AC-R6 |
| Validación schema salida runtime (R8) | **Hecho (H3)** | `capability_di_output_validator.rs` |
| Taxonomía multi-término (≥2) | **Hecho (H3)** | `doc:closure`, `proc:git-sync` en `capability-taxonomy.md` |
| Homologación ED piloto | **Parcial (4)** | `feature`, `bug-fix` (`requires_capability`); `filesystem-manager`, `git-manager` (`provides`) |
| Homologación ampliada catálogo ED | **Ausente** | R10; procesos como `refactorization`, `delivery-close-cycle` sin `requires_capability` |
| Orden canónico DI | **Hecho (H3)** | `resolve → gate → cerbero_rbac → inject → output_validator` |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT4-SCOPE** | Este ciclo = **R9** + **R10** + AC-R9/AC-R10 producto + regresión AC-R1/R2 (H2), AC-R5/R6/R7/R8 (H3), AC-P1/P2/P3 (MVP). GesFer, F1 y EDA-only total quedan fuera. |
| **L-GATE-PRESERVE** | `capability_di_gate` **permanece** Aduana Temprana sobre la fase/`requires_capability`. R9 **no** sustituye el gate ni re-ejecuta su lógica sobre la declaración de fase. |
| **L-CERBERO-ORDER** | Orden mínimo extendido: `resolve` → `capability_di_gate` → `cerbero_di_rbac` → **revalidación envelope** → inject. AC-R9 exige abort **después** de gate APTO y RBAC allow si el `di_binding` empaquetado es inválido. |
| **L-ENVELOPE-DELTA** | R9 contrasta el objeto `di_binding` **ya empaquetado** (`di_binding_object`) — campos `capability_id`, `contract`, `contract_schema_ref`, `provider`, `provider_ref`, `resolved_version`, `binding_ssot` — contra schema del envelope y coherencia con el `ResolvedBinding` / fila `capability-bindings.md`. **No** revalidar outputs declarados del proveedor (eso es gate pre-ignición). |
| **L-ENVELOPE-TAMPER** | AC-R9 debe demostrarse con escenario donde gate + RBAC pasan pero el envelope inyectado está **malformado o incoherente** (p. ej. `contract` alterado, `provider` ≠ binding table, campos obligatorios ausentes). |
| **L-R9-CODE** | Fallo envelope → código trazable distinto de RBAC (p. ej. `CERBERO_ENVELOPE_SCHEMA_MISMATCH` o `CERBERO_DI_BINDING_INVALID`) + DLQ `./.events/dead-letter`. Dedalo fija nombre exacto en spec. |
| **L-R10-NMIN** | **N_mínimo = 8** entidades ED con `provides` y/o `requires_capability` coherentes con `capability-taxonomy` y filas en `capability-bindings.md` cuando apliquen. Baseline en main = 4 (`feature`, `bug-fix`, `filesystem-manager`, `git-manager`). Este ciclo debe añadir **≥4 ED nuevas** homologadas. |
| **L-R10-NO-INVENT** | Prohibido declarar capacidades fuera del `catalog` de `capability-taxonomy.md`. Altas de términos nuevos = **fuera** de R10 salvo laudo Racso explícito; este ciclo opera sobre los 2 términos existentes. |
| **L-R10-MIGRATION** | Mutación genoma vía `entity-manager` + registro `SddIA/evolution/` por ED tocada. Migración **controlada**; no barrido masivo del catálogo completo. |
| **L-R10-PILOT-LIST** | Lista concreta de las ≥4 ED nuevas = handoff **Dedalo** en `spec.md`/`plan.md` (Q4). Candidatos admisibles: procesos con fases `git-manager` / cierre documental (`refactorization`, `delivery-close-cycle`, `accept-pr`, …) y skills/actions que consuman o provean los 2 términos vigentes. |
| **L-CODEX-ROLE** | Reafirmado: binding table ≠ taxonomía. R10 anota ED; no expande Códice salvo decisión explícita fuera de alcance. |
| **L-EDA-PILOT-PRESERVE** | Piloto R6/AC-R6 intacto; sin sustituir path síncrono H2/H3 por EDA-only. |
| **L-PBI-LOC** | PBI-042 permanece en `docs/todos/pending/` hasta Done global o laudo Racso; este feature no archiva el PBI padre solo por cerrar Hito 4. |
| **L-GESFER** | Ortogonal; no absorber Paciente 0 / Fractura Core en este `persist_ref`. |

## D3 — Ambigüedades acotadas (handoff Dedalo — no diseño Mayeuta)

| # | Pregunta | Opciones admisibles | Criterio de cierre |
|---|----------|---------------------|--------------------|
| **Q1** | Locus módulo R9 en engine | (A) extender `cerbero_di_rbac.rs`; (B) módulo dedicado `cerbero_di_envelope.rs` invocado tras RBAC | Debe insertarse en cadena post-RBAC pre-inject sin romper AC-R5 |
| **Q2** | Schema del envelope `di_binding` | (A) nuevo `{contract}.schema.json` bajo `capability_contracts` (p. ej. `di.binding`); (B) validación estructural Rust sin schema externo | Debe ser machine-readable y versionable; compatible con `capsule-json-io` v2 |
| **Q3** | Profundidad de contraste envelope | (A) solo forma/schema del objeto; (B) forma + cruce `capability_id`/`provider`/`contract` vs `ResolvedBinding` y fila binding | AC-R9 exige rechazo ante incoherencia semántica del empaquetado, no solo JSON malformado |
| **Q4** | Lista piloto R10 (≥4 ED nuevas) | Dedalo enumera ED + tipo de anotación (`provides` / `requires_capability`) + fase consumidora si aplica | Suma total ≥8 ED homologadas; sin capacidades inventadas |
| **Q5** | Paths ciegos nuevos | ¿Qué fases pasan a `requires_capability`-only (sin `delegates_to` identidad)? | Al menos 1 consumidor ciego nuevo además de `feature`/`bug-fix` si aporta valor medible |
| **Q6** | Integración piloto EDA | ¿Reactor `capability_di_reactor` propaga `di_bindings` ya validados por envelope Cerbero? | Regresión AC-R6 verde; envelope check coherente en path sync (obligatorio) y EDA (si aplica) |

## D4 — Criterios producto estabilizados (este ciclo)

| ID | Criterio | Verificación esperada (Argos) |
|----|----------|-------------------------------|
| **AC-R9** | Cerbero rechaza inject si `di_binding` empaquetado incumple contrato/schema del envelope aunque `capability_di_gate` y RBAC hayan pasado | Test engine: gate APTO + RBAC allow + envelope inválido → abort pre-ignición; DLQ trazable |
| **AC-R10** | ≥**8** entidades ED homologadas con `provides`/`requires_capability` + bindings coherentes (≥4 nuevas respecto al piloto H2/H3) | Diff genoma acotado + evolution; conteo verificable en CI o auditoría Argos |

Regresión obligatoria:

| ID | Criterio | Origen |
|----|----------|--------|
| **AC-R1** | Resolución ciega sin `delegates_to` hardcodeado | H2 |
| **AC-R2** | `di_binding` en stdin cápsula | H2 |
| **AC-R5** | Cerbero RBAC deny post-gate | H3 |
| **AC-R6** | Piloto EDA async sin bloquear orquestador | H3 |
| **AC-R7** | Término `proc:git-sync` en taxonomía | H3 |
| **AC-R8** | Validación schema payload salida real | H3 |
| **AC-P1** | Homologación OK → ignición | MVP |
| **AC-P2** | Incumplimiento contrato pre-ignición → DLQ | MVP |
| **AC-P3** | Capacidad no indexada → abort limpio | MVP |

## D5 — Veredicto

**ok** — Requisitos Hito 4 termodinámicamente estables. Handoff a Dedalo: diseño revalidación envelope Cerbero (Q1–Q3, Q6) + ola homologación catálogo (Q4–Q5) → `spec.md` / `plan.md`.
