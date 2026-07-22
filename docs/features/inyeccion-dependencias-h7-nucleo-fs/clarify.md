---
feature_name: inyeccion-dependencias-h7-nucleo-fs
created: "2026-07-22"
purpose: Estabilización PBI-043 Hito 1 (H7) — núcleo FS residual con DI fs:persist (R1–R3 / AC-H7)
branch_name: feat/inyeccion-dependencias-h7-nucleo-fs
persist_ref: docs/features/inyeccion-dependencias-h7-nucleo-fs
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md
document_id: PBI-043-H7-NUCLEO-FS
execution_id: b8e2a471-5c9d-4f3a-8e1b-6d0c9f2a4b7e
phase: mayeuta-stabilization
agents: mayeuta
inventory_recount_date: "2026-07-22"
inventory_without_capability: 24
inventory_with_capability: 18
n_ola_floor: 8
---

# Clarificación — PBI-043 Hito 1 (H7 · Núcleo FS)

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] PBI-043 — DI residual H7+ (catálogo ED sin capacidades).md` (`document_id: PBI-043-DI-CATALOGO-RESIDUAL-H7`; `status: abierto`).
- **Ciclo:** feature `inyeccion-dependencias-h7-nucleo-fs` · rama `feat/inyeccion-dependencias-h7-nucleo-fs`.
- **Alcance declarado:** Hito 1 — **H7** · vectores **R1–R3**. Criterio **producto:** **AC-H7**. Regresión: suites DI MVP→H6 (`capability_di` / `cerbero_di`).
- **Precedente cerrado Done PBI-042:** `docs/features/inyeccion-dependencias-cierre-pbi` — PR [#142](https://github.com/racso80es/SddIA/pull/142) merge `90424f4`. Residual explícito: Ola H7+ ED residuales → este PBI.
- **Precedente cerrado Hito 6:** `docs/features/inyeccion-dependencias-barrido-creators` — R14 creators; path ciego / mixto crypto+FS.
- **Normas / SSOT (Cúmulo):** `capability-taxonomy.md` (`doc:closure`, `proc:git-sync`, `fs:persist`), `capability-bindings.md` v1.1.0, `eda-coverage.json`, `event_bus`, `capability_contracts`, `evolution`.
- **Runtime intacto a preservar:** `capability_di_gate` · `capability_di_resolver` · `cerbero_di_rbac` · `cerbero_di_envelope` · `capability_di_output_validator` · orden `resolve → gate → rbac → envelope → inject → output_validator`.
- **Fuera de alcance:** H8 routes; H9 auditorías; H10 gobernanza/interactores; R10 EDA-only; GesFer/F1; altas libres al Códice; deuda PPR #136 `delivery-close` revoked; archivo PBI-043 (queda en `pending/`).

## D1 — Matriz de validación (AC-INV · recuento start)

> Recuento 2026-07-22 sobre `SddIA/process/*.md` (excl. `process-contract`, `index`). Drift vs PBI §3: **ninguno**.

| Afirmación / residual | Estado actual | Evidencia |
|------------------------|---------------|-----------|
| Runtime DI MVP→H6 | **Hecho (main)** | PR #142 merge `90424f4`; no reescribir |
| Taxonomía 3 términos | **Hecho** | `doc:closure`, `proc:git-sync`, `fs:persist` |
| Bindings v1.1.0 | **Hecho** | fila `fs:persist` → `skill:filesystem-manager` |
| Process con `requires_capability` | **18** | feature/bug-fix/creators/delivery/accept-pr/… |
| Process **sin** `requires_capability` | **24** | inventario PBI §3 intacto |
| §3.1 consumidores FS (8) | **Ausente DI** | tienen `delegates_to` FS; sin `requires_capability` |
| Aduana EDA genómica | **Preservar** | `orphan_count == 0` post-mutación |
| H8–H10 / R10 | **Fuera** | no absorber en este `persist_ref` |

### Inventario H7 §3.1 — piso de ola (N_ola ≥ 8)

| ED | Capacidad | Notas estabilizadas |
|----|-----------|---------------------|
| `entity-manager` | `fs:persist` | Núcleo forja; fase Delete = FS (ciego preferente); create/update delegan creator + emit (no-FS) |
| `route-domain-event` | `fs:persist` | Bus domain + FS |
| `daemon-kill-switch` | `fs:persist` | + bus-operator / execute-process |
| `governance-daemon-manager` | `fs:persist` | + shell-executor + Cerbero (mixto OS/FS) |
| `daemon-heartbeat-audit` | `fs:persist` | + Argos |
| `fix-tool-process` | `fs:persist` | |
| `telemetry-batch-stub` | `fs:persist` | |
| `workspace-smoke` | `fs:persist` | |

**Piso Mayeuta:** exactamente estas **8**. Umbral `N_ola ≥ 8` = piso = lista; no bajar sin laudo Racso. Elevación fuera de §3.1 = **fuera** de H7 (H8+).

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-HIT7-SCOPE** | Este ciclo = **R1–R3** + **AC-H7** + **AC-INV** + regresión MVP→H6. H8–H10, R10, GesFer, F1, PPR #136 = fuera. |
| **L-BASELINE-042** | Baseline innegociable = taxonomía 3 términos + bindings v1.1.0 + runtime DI entregado en main. H7 **añade** homologación §3.1; no reescribe runtime salvo bug. |
| **L-R1-FLOOR** | Piso de ola = **8** ED §3.1 listadas. `N_ola = 8` (default estabilizado). Dedalo no baja; elevar solo con laudo Racso y sin absorber H8+. |
| **L-R1-CAP** | Toda anotación `requires_capability` de esta ola apunta a **`fs:persist`** (catálogo vigente). Binding table ≠ taxonomía (L-CODEX-ROLE reafirmado). |
| **L-NO-INVENT** | Prohibido inventar `capability_id` (`bus:route`, `llm:interact`, …). Altas al Códice = **fuera** de H7 (**AC-NO-INVENT** / Q1–Q2 PBI diferidas a H8+). |
| **L-BLIND-PREF** | Preferir path ciego (`requires_capability: fs:persist` sin `delegates_to` skill FS) en fases solo-FS. Fases mixtas (OS/crypto/bus/execute-process + FS): conservar `delegates_to` no-FS + anotar `fs:persist` en fases FS (patrón creators H6 / Q3). |
| **L-Q3-EM** | `entity-manager`: default **ciego** en fase Delete FS; mixto solo si forge/ Dedalo demuestra mismatch de proveedor. No forzar ceguera en fases `action:execute-process` / `emit-domain-mutation`. |
| **L-R2-MUTATION** | Mutación genoma vía `entity-manager` + emisión `Domain_Entity_Updated` + registro `SddIA/evolution/` por ED (o lote documentado). Prohibido forjar `{name}.md` a mano sin sello. |
| **L-R2-EDA** | Post-mutación: `orphan_count == 0` en aduana EDA genómica; cobertura SSOT coherente (**AC-SEAL** / **AC-ORPHAN**). |
| **L-R3-REG** | Regresión obligatoria suites `capability_di` / `cerbero_di` (MVP→H6). No reabrir diseño de cadena DI. |
| **L-RUNTIME-PRESERVE** | Gate, resolver, Cerbero RBAC, envelope y output validator **permanecen**. |
| **L-PBI-LOC** | PBI-043 permanece en `docs/todos/pending/`; `pbi_archived: false` en este ciclo. Done global = H7–H10 (R10 opcional). |
| **L-GESFER** | Ortogonal; no absorber Paciente 0 / Fractura Core / deuda delivery-close en este `persist_ref`. |

## D3 — Ambigüedades acotadas (handoff Dedalo — no diseño Mayeuta)

| # | Pregunta | Opciones admisibles | Criterio de cierre |
|---|----------|---------------------|--------------------|
| **Q1** | Densidad path ciego por ED | (A) ciego en todas las fases solo-FS de las 8; (B) mixto documentado donde coexisten shell/bus/crypto/execute-process | **L-BLIND-PREF** + **L-Q3-EM**; sin inventar capacidades |
| **Q2** | Estrategia de lotes | (A) un PR/lote con las 8; (B) sub-olas en el mismo `persist_ref` con evolution por lote | Blast-radius acotado; Argos audita 8/8 acumulado |
| **Q3** | Evidencia AC-H7 / sello / orphan | (A) fixture coverage + assert orphan; (B) auditoría Argos evolution + `Domain_Entity_Updated` por ED | Reproducible; sin Shell IDE crudo como SSOT |
| **Q4** | Alcance smoke regresión | Suites `capability_di` / `cerbero_di` mínimas vs pack completo MVP→H6 | **L-R3-REG**; no romper baseline H6 |
| **Q5** | `governance-daemon-manager` / `daemon-kill-switch` | ¿Fases FS ciegas vs mixto con shell/bus? | Preferencia ciega FS; conservar `delegates_to` no-FS |
| **Q6** | ¿Tocar `provides` además de `requires_capability`? | (A) solo `requires_capability` (default); (B) `provides` si Dedalo identifica proveedor | Coherencia taxonomía+bindings; sin altas nuevas |

## D4 — Criterios producto estabilizados (este ciclo)

| ID | Criterio | Verificación esperada (Argos) |
|----|----------|-------------------------------|
| **AC-H7** | 8/8 ED §3.1 con `requires_capability` → `fs:persist` coherente taxonomía+bindings; preferencia path ciego; mutación vía `entity-manager` + `Domain_Entity_Updated` + evolution; `orphan_count == 0`; runtime preservado | Diff genoma = las 8; sellos trazables; aduana EDA verde |
| **AC-INV** | Inventario recontado al start; drift documentado | Tabla D1; `without=24` pre-ola |
| **AC-NO-INVENT** | Ningún `capability_id` fuera del catálogo vigente | Diff sin altas taxonomía/bindings |

Regresión obligatoria:

| ID | Criterio | Origen |
|----|----------|--------|
| **AC-REG-DI** | Suites `capability_di` / `cerbero_di` verdes (MVP→H6) | R3 / PBI-042 |
| **AC-SEAL** | Sello `Domain_Entity_Updated` trazable vía entity-manager | AC global PBI-043 |
| **AC-ORPHAN** | `orphan_count == 0` post-ola | AC global PBI-043 |

## D5 — Veredicto

**ok** — Requisitos H7 termodinámicamente estables. Handoff a Dedalo: diseño ola H7 (`N_ola=8`, path ciego `fs:persist`, sellos EDA, Q1–Q6) → `spec.md` / `plan.md`.
