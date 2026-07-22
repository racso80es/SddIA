---
feature_name: inyeccion-dependencias-barrido-creators
created: "2026-07-22"
purpose: Estabilización Hito 6 PBI-042 — barrido creators residuales con DI por capacidades (R14)
branch_name: feat/inyeccion-dependencias-barrido-creators
persist_ref: docs/features/inyeccion-dependencias-barrido-creators
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md
document_id: PBI-042-BARRIDO-CREATORS
execution_id: c9d1e4f2-7a8b-4c5d-9e0f-1a2b3c4d5e6f
phase: mayeuta-stabilization
agents: mayeuta
---

# Clarificación — PBI-042 Hito 6 (barrido creators residuales)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-042 — DI por capacidades y contratos semánticos.md` (v1.2.0; Hito 5 entregado PR #138 merge `66a0f71`).
- **Ciclo:** feature `inyeccion-dependencias-barrido-creators` · rama `feat/inyeccion-dependencias-barrido-creators`.
- **Alcance declarado:** Hito 6 — **R14** (ola H6+). Criterio **producto:** **AC-R14**. Regresión: AC-R11/R12 (H5), AC-R9/R10 (H4), AC-R5–R8 (H3), AC-R1/R2 (H2), AC-P1–P3 (MVP).
- **Precedente cerrado Hito 5:** `docs/features/inyeccion-dependencias-migracion-catalogo` — R11 sello `Domain_Entity_Updated`; R12 `N_ola=8` + alta `fs:persist` (total ≥16); R13 omitido (Q6-A). Residual finalize: barrido creators restantes → este ciclo.
- **Precedente cerrado Hito 4:** `docs/features/inyeccion-dependencias-envelope-homologacion` — `cerbero_di_envelope`, baseline 8 ED.
- **Precedente cerrado Hito 3:** `docs/features/inyeccion-dependencias-gobernanza-asincronia` — Cerbero RBAC, piloto EDA, `proc:git-sync`, output validator.
- **Precedente cerrado Hito 2 / MVP:** resolución ciega + gate DI + taxonomía + DLQ.
- **Remisión explícita finalize Hito 5:** «Barrido restante creators (`norm-creator`, `codex-creator`, …)» → **R14** / Ola H6+.
- **Normas / SSOT (Cúmulo):** `capability-taxonomy.md` (`doc:closure`, `proc:git-sync`, `fs:persist`), `capability-bindings.md` v1.1.0, `eda-coverage.json`, `event_bus`, `capability_contracts`, `evolution`.
- **Runtime intacto a preservar:** `capability_di_gate` · `capability_di_resolver` · `cerbero_di_rbac` · `cerbero_di_envelope` · `capability_di_output_validator` · orden `resolve → gate → rbac → envelope → inject → output_validator`.
- **Fuera de alcance:** GesFer / Paciente 0; Fractura Core F1; EDA-only total sync→async; archivo PBI-042 padre (L-PBI-LOC); altas libres al Códice salvo laudo Racso.

## D1 — Matriz de validación (residual × estado post-Hito 5)

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| Sello `Domain_Entity_Updated` (R11 / AC-R11) | **Hecho (H5)** | PR #138 merge `66a0f71`; entity-manager + coverage |
| Ola migración ≥8 nuevas / total ≥16 (R12 / AC-R12) | **Hecho (H5)** | `N_ola=8` + `fs:persist`; creators forja parcial homologados |
| Ampliar piloto EDA DI (R13) | **Omitido (H5 Q6-A)** | No reabrir en H6 salvo laudo Racso |
| Taxonomía 3 términos | **Hecho (H5)** | `doc:closure`, `proc:git-sync`, `fs:persist` |
| Bindings 3 filas | **Hecho (H5)** | `capability-bindings.md` v1.1.0 |
| Creators homologados H5 | **Hecho (parcial)** | `process-creator`, `skill-creator`, `action-creator`, `event-creator`, `agent-creator`, `tool-creator` (+ otros no-creator de la ola) |
| Creators sin `requires_capability` (solo `delegates_to`) | **Ausente DI** | `norm-creator`, `codex-creator`, `daemon-creator`, `suite-creator` |
| Envelope / RBAC / gate / resolver / output validator | **Hecho** | Cadena H2–H4 intacta en main |
| Aduana EDA genómica (`orphan_count == 0`) | **Hecho (H5 gate)** | Debe preservarse tras mutaciones R14 |

### Inventario creators (10) — piso residual

| Creator | DI post-H5 | Fases FS típicas (candidatas `fs:persist`) |
|---------|------------|--------------------------------------------|
| `process-creator` … `tool-creator` (6) | **Homologados H5** | Forja mixto crypto+FS; Indexación ciega FS |
| `norm-creator` | **Residual** | Materialización; Indexación |
| `codex-creator` | **Residual** | Materialización; Indexación |
| `daemon-creator` | **Residual** | Forja del Markdown (crypto+FS); Indexación |
| `suite-creator` | **Residual** | Materialización; Indexación |

**Piso Mayeuta:** los **4** residuales sin DI. Umbral `N_ola` exacto y posibles ED adyacentes = handoff **Dedalo** (no bajar del piso sin laudo Racso).

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT6-SCOPE** | Este ciclo = **R14** + **AC-R14** producto + regresión AC-R11/R12, AC-R9/R10, AC-R5–R8, AC-R1/R2, AC-P1–P3. GesFer, F1, EDA-only total y archivo PBI quedan fuera. |
| **L-BASELINE-H5** | Baseline innegociable post-H5 = taxonomía 3 términos + bindings v1.1.0 + ≥16 ED homologadas + runtime DI. R14 **añade** homologación de creators residuales; no recontar los 6 creators H5 como progreso de ola. |
| **L-R14-FLOOR** | Piso de ola = **4** creators: `norm-creator`, `codex-creator`, `daemon-creator`, `suite-creator`. Dedalo fija `N_ola` ≥4 y lista exacta (**Q1**); puede elevar con justificación; no bajar sin laudo Racso. |
| **L-R14-COHERENCE** | Toda anotación `provides` / `requires_capability` coherente con `capability-taxonomy.catalog` y, si aplica proveedor, con fila en `capability-bindings.md`. Binding table ≠ taxonomía (**L-CODEX-ROLE** reafirmado). |
| **L-R14-NO-INVENT** | Prohibido declarar capacidades fuera del `catalog` vigente (`doc:closure`, `proc:git-sync`, `fs:persist`). Altas nuevas al Códice = **fuera** salvo laudo Racso (**Q3**). Default: operar solo sobre términos existentes. |
| **L-R14-BLIND-PREF** | Preferir path ciego (`requires_capability` sin `delegates_to` de skill/action FS/git) en fases solo-FS o solo-git. Fases mixtas (crypto + FS): `requires_capability: fs:persist` + conservar `delegates_to` no-FS (patrón H5 creators). |
| **L-R14-MUTATION** | Mutación genoma vía `entity-manager` + emisión `Domain_Entity_Updated` (CRUD puro) + registro `SddIA/evolution/` por ED tocada (o lote documentado). Prohibido forjar `{name}.md` a mano sin sello. Hash-only no sustituye sello (**L-R11-NO-BYPASS** heredado). |
| **L-R14-EDA** | Post-mutación: `orphan_count == 0` en aduana EDA genómica; cobertura SSOT coherente. |
| **L-RUNTIME-PRESERVE** | Gate, resolver, Cerbero RBAC, envelope y output validator **permanecen**; R14 no reabre diseño de cadena DI salvo bug de regresión. |
| **L-PBI-LOC** | PBI-042 permanece en `docs/todos/pending/` hasta Done global o laudo Racso; este feature **no** archiva el PBI padre solo por cerrar Hito 6. |
| **L-GESFER** | Ortogonal; no absorber Paciente 0 / Fractura Core en este `persist_ref`. |

## D3 — Ambigüedades acotadas (handoff Dedalo — no diseño Mayeuta)

| # | Pregunta | Opciones admisibles | Criterio de cierre |
|---|----------|---------------------|--------------------|
| **Q1** | Umbral y lista ola R14 | Dedalo fija `N_ola` (≥4 = piso) + enumeración exacta; default piso = los 4 residuales; (B) elevar con ED adyacentes justificadas | Sin bajar piso; total creators residuales homologados ≥4 |
| **Q2** | Paths ciegos por fase | ¿Qué fases de cada creator pasan a `requires_capability`-only (Indexación / Materialización / Forja)? | Preferencia **L-R14-BLIND-PREF**; patrón H5: Indexación ciega FS; Forja mixto crypto+`fs:persist` |
| **Q3** | Expansión taxonomía | (A) ninguna alta (default **L-R14-NO-INVENT**); (B) ≤K términos nuevos con laudo Racso + contrato + binding | Sin (B) solo `doc:closure` / `proc:git-sync` / `fs:persist` |
| **Q4** | Estrategia de lotes | (A) un PR/lote único con los ≥4; (B) sub-olas en el mismo `persist_ref` con evolution por lote | Blast-radius acotado; Argos audita conteo acumulado |
| **Q5** | Consumo `proc:git-sync` | (A) no aplicar en creators (default si no hay fase git); (B) anotar fases git si Dedalo identifica consumo real | Coherencia semántica; no forzar `proc:git-sync` donde no hay git |
| **Q6** | Evidencia AC-R14 / sello | (A) fixture coverage + assert orphan; (B) auditoría Argos evolution + `Domain_Entity_Updated` por ED | Reproducible; sin depender de Shell IDE crudo |
| **Q7** | Regresión creators H5 | ¿Smoke mínimo sobre 1 creator H5 ya homologado además de tests DI globales? | No romper AC-R12 baseline; Dedalo decide alcance smoke |

## D4 — Criterios producto estabilizados (este ciclo)

| ID | Criterio | Verificación esperada (Argos) |
|----|----------|-------------------------------|
| **AC-R14** | Creators residuales homologados (mínimo `norm-creator`, `codex-creator`, `daemon-creator`, `suite-creator`) con `provides`/`requires_capability` coherentes a taxonomía+bindings; preferencia path ciego FS/git; mutación vía `entity-manager` + `Domain_Entity_Updated` + evolution; `orphan_count == 0` | Diff genoma acotado; conteo ≥4 residuales; sellos trazables; aduana EDA verde |

Regresión obligatoria:

| ID | Criterio | Origen |
|----|----------|--------|
| **AC-R11** | Sello `Domain_Entity_Updated` trazable en mutaciones vía entity-manager | H5 |
| **AC-R12** | Baseline ola H5 (≥16 homologadas; `fs:persist`) intacto | H5 |
| **AC-R9** | Cerbero envelope rechaza `di_binding` inválido | H4 |
| **AC-R10** | ≥8 ED homologadas baseline H4 intacto | H4 |
| **AC-R5** | Cerbero RBAC deny post-gate | H3 |
| **AC-R6** | Piloto EDA async sin bloquear orquestador | H3 |
| **AC-R7** | Término `proc:git-sync` en taxonomía | H3 |
| **AC-R8** | Validación schema payload salida real | H3 |
| **AC-R1** | Resolución ciega sin `delegates_to` hardcodeado | H2 |
| **AC-R2** | `di_binding` en stdin cápsula | H2 |
| **AC-P1** | Homologación OK → ignición | MVP |
| **AC-P2** | Incumplimiento contrato pre-ignición → DLQ | MVP |
| **AC-P3** | Capacidad no indexada → abort limpio | MVP |

## D5 — Veredicto

**ok** — Requisitos Hito 6 termodinámicamente estables. Handoff a Dedalo: diseño ola creators residuales (Q1–Q2, Q4–Q5) + decisión taxonomía (Q3) + evidencia sello/orphan (Q6) + smoke regresión H5 (Q7) → `spec.md` / `plan.md`.
