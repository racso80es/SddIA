---
feature_name: inyeccion-dependencias-migracion-catalogo
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-migracion-catalogo
persist_ref: docs/features/inyeccion-dependencias-migracion-catalogo
document_id: PBI-042-MIGRACION-CATALOGO
execution_id: a8f4c2e1-6b9d-4e3a-9c7f-1d2e5a8b0c4f
phases: 7
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
scope: "Hito 5 — Sellado EDA + ola migración catálogo (R11–R12)"
---

# Plan / Blueprint — DI sellado EDA + migración catálogo (Hito 5)

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md`, `spec.md`.

`target_executor_rbac.allowed_policies`: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula / acción | ¿Permitida? |
|------------------|-------------|
| `skill:filesystem-manager` | sí |
| `skill:git-manager` | sí |
| `skill:shell-executor` | sí (`cargo test`, `sddia-qa`) |
| `action:execute-process` | sí (`entity-manager`, smokes) |

Ninguna fase exige cápsula fuera de `allowed_policies`. Mutación genoma **solo** vía `entity-manager` → `emit-domain-mutation` (R11). Git solo vía `skill:git-manager`.

**Gate duro pre-fase Q3:** countersign Racso de alta `fs:persist` registrado en `execution.md`. Sin él → abort `blocked` (no improvisar Q3-A ni bajar `N_ola`).

---

## Fases declarativas

```yaml
phases:
  - name: "Gate Racso Q3-B"
    intent: "Registrar countersign Racso para alta fs:persist; si ausente, abort blocked sin mutar taxonomía."
    delegates_to: ["skill:filesystem-manager"]
  - name: "R12-prep Taxonomía + contrato + binding + provides"
    intent: "Alta fs:persist en capability-taxonomy; crear fs.persist.schema.json; fila capability-bindings; provides en filesystem-manager; sellar cada mutación vía entity-manager."
    delegates_to: ["action:execute-process", "skill:filesystem-manager"]
  - name: "R11 Backfill sello H4 (Q1-B)"
    intent: "entity-manager update sobre las 8 ED baseline; Domain_Entity_Updated + coverage; cierra L-R10-SEAL."
    delegates_to: ["action:execute-process"]
  - name: "R12 Ola N_ola=8"
    intent: "Anotar §4.4 spec (task-queue-manager, sddia-difusion, 6 *-creator) vía entity-manager; ≥4 fases ciegas; evolution por ED."
    delegates_to: ["action:execute-process", "skill:filesystem-manager"]
  - name: "R11 Evidencia fixture + aduana EDA"
    intent: "Fixture Q7-A emit/assert coverage; audit-eda-coverage --scan orphan_count==0; demo ≥1 sello R12."
    delegates_to: ["skill:shell-executor", "action:execute-process"]
  - name: "Regresión H4+H3+H2+MVP"
    intent: "cargo test capability_di + cerbero_di + envelope + di_reactor + di_output; AC-R9/R10 + R5–R8 + R1/R2 + P1–P3."
    delegates_to: ["skill:shell-executor"]
  - name: "Documentación ejecución"
    intent: "implementation.md + execution.md (incl. countersign Q3); handoff Argos."
    delegates_to: ["skill:filesystem-manager"]
```

---

## Detalle por fase

### 0 — Gate Racso Q3-B

| # | Entregable | Detalle |
|---|------------|---------|
| G.1 | Countersign | Texto explícito en `execution.md`: Racso aprueba alta `fs:persist` (Q3-B K=1) |
| G.2 | Abort | Sin countersign → no tocar taxonomía/bindings/ola creators; veredicto feature `blocked` |

### 1 — R12-prep (taxonomía / contrato / binding)

| # | Entregable | Detalle |
|---|------------|---------|
| T.1 | `capability-taxonomy.md` | `catalog` += `fs:persist` / `fs.persist` / 1.0.0 vía entity-manager (`entity_class: norm`) |
| T.2 | `fs.persist.schema.json` | Bajo `capability_contracts`; I/O alineado filesystem-manager |
| T.3 | `capability-bindings.md` | Fila provider `skill:filesystem-manager` |
| T.4 | `filesystem-manager.md` | `provides` += `fs:persist` (entity-manager skill update) |
| T.5 | Evolution | Entradas por artefacto SSOT mutado |

**Salida:** Catálogo y mapa coherentes (**L-R12-COHERENCE**); listos para consumidores R12.

### 2 — R11 Backfill Q1-B

| # | ED baseline | Acción |
|---|-------------|--------|
| B.1–B.8 | Las 8 de R10 | `entity-manager` update → `Domain_Entity_Updated` |

**Salida:** L-R10-SEAL cerrado; coverage actualizado; muestra trazable AC-R11.

### 3 — R12 Ola (`N_ola = 8`)

| # | ED | Mutación |
|---|-----|----------|
| R12.1 | `task-queue-manager` | `proc:git-sync` (+ `fs:persist` si se parte fase) — preferir ciego |
| R12.2 | `sddia-difusion` | Snapshot → `proc:git-sync` ciego |
| R12.3 | `process-creator` | fases FS → `fs:persist` |
| R12.4 | `skill-creator` | fases FS → `fs:persist` |
| R12.5 | `action-creator` | fases FS → `fs:persist` |
| R12.6 | `event-creator` | fases FS → `fs:persist` |
| R12.7 | `agent-creator` | fases FS → `fs:persist` |
| R12.8 | `tool-creator` | fases FS → `fs:persist` |
| R12.9 | Evolution | Por ED + entrada feature Hito 5 |
| R12.10 | Bonus (opc.) | Inicialización `feature`/`bug-fix`/`refactorization` → `proc:git-sync` (no cuenta N_ola) |

**Restricción:** toda mutación con sello R11; sin Write genoma huérfano; sin inventar términos extra (**K=1**).

### 4 — Evidencia AC-R11

| # | Entregable | Detalle |
|---|------------|---------|
| E.1 | Fixture / test | Emit path → assert `last_emitted_event: Domain_Entity_Updated` |
| E.2 | Scan | `./sddia-run.sh` / `sddia-qa audit-eda-coverage --scan --json` → `orphan_count == 0` |

### 5 — Regresión

| Bloque | Evidencia |
|--------|-----------|
| AC-REG-H4 | envelope + baseline 8 |
| AC-REG-H3 | RBAC, reactor EDA, `proc:git-sync` legado, output validator |
| AC-REG-H2 | resolver ciego, `di_binding` |
| AC-REG-MVP | gate P1–P3 |

Flag EDA ausente salvo tests AC-R6.

### 6 — Documentación

`implementation.md` + `execution.md` con countersign, lista sellos, conteo N_ola, comandos test. Handoff Argos.

---

## Handoff Argos

Criterios en `spec.md` §5. `validacion.md` `global: APTO` solo si AC-R11, AC-R12 y regresión verdes **y** countersign Q3-B presente. PBI-042 padre permanece en `pending/` (**L-PBI-LOC**). R13 no es gate.
