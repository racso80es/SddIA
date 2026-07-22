---
feature_name: inyeccion-dependencias-barrido-creators
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-barrido-creators
persist_ref: docs/features/inyeccion-dependencias-barrido-creators
document_id: PBI-042-BARRIDO-CREATORS
execution_id: c9d1e4f2-7a8b-4c5d-9e0f-1a2b3c4d5e6f
phases: 6
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
scope: "Hito 6 — Barrido creators residuales DI (R14)"
---

# Plan / Blueprint — Barrido creators residuales DI (Hito 6)

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md`, `spec.md`.

`target_executor_rbac.allowed_policies`: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula / acción | ¿Permitida? |
|------------------|-------------|
| `skill:filesystem-manager` | sí (docs persist_ref; no forja genoma huérfana) |
| `skill:git-manager` | sí (si runtime inyecta evidencia git) |
| `skill:shell-executor` | sí (`cargo test`, `sddia-qa`) |
| `action:execute-process` | sí (`entity-manager`, smokes) |

Ninguna fase exige cápsula fuera de `allowed_policies`. Mutación genoma **solo** vía `entity-manager` → `emit-domain-mutation` (R11 heredado). Git solo vía `skill:git-manager`. **Sin** gate Racso de taxonomía (Q3-A).

---

## Fases declarativas

```yaml
phases:
  - name: "Baseline post-H5"
    intent: "Verificar taxonomía 3 términos, bindings v1.1.0, creators H5 intactos y residuales sin requires_capability; abort si drift."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "R14 Ola N_ola=4"
    intent: "entity-manager update sobre norm/codex/daemon/suite-creator según spec §4.3–4.4; sello Domain_Entity_Updated por ED; bump patch."
    delegates_to: ["action:execute-process"]
  - name: "Evolution H6"
    intent: "Registrar evolution feature Hito 6 + una entrada por ED tocada (4)."
    delegates_to: ["skill:filesystem-manager", "action:execute-process"]
  - name: "Evidencia AC-R14 (Q6)"
    intent: "audit-eda-coverage --scan orphan_count==0; muestra coverage/bus Domain_Entity_Updated por cada ED R14."
    delegates_to: ["skill:shell-executor", "action:execute-process"]
  - name: "Regresión H5→MVP + smoke Q7"
    intent: "Suites DI globales + smoke lectura/ignición process-creator (H5); AC-REG-*."
    delegates_to: ["skill:shell-executor"]
  - name: "Documentación ejecución"
    intent: "implementation.md + execution.md; handoff Argos."
    delegates_to: ["skill:filesystem-manager"]
```

---

## Detalle por fase

### 0 — Baseline post-H5

| # | Entregable | Detalle |
|---|------------|---------|
| B.1 | Taxonomía | `catalog` = `doc:closure`, `proc:git-sync`, `fs:persist` (sin drift) |
| B.2 | Bindings | 3 filas v1.1.0 intactas |
| B.3 | Residuales | Confirmación ausencia `requires_capability` en los 4 |
| B.4 | Abort | Si taxonomía/bindings rotos → `blocked` (no improvisar altas) |

### 1 — R14 Ola (`N_ola = 4`)

| # | ED | Mutación |
|---|-----|----------|
| R14.1 | `norm-creator` | Materialización + Indexación → `fs:persist` ciego |
| R14.2 | `codex-creator` | Materialización + Indexación → `fs:persist` ciego |
| R14.3 | `daemon-creator` | Forja → mixto crypto+`fs:persist`; Indexación → ciego |
| R14.4 | `suite-creator` | Materialización + Indexación → `fs:persist` ciego |

**Restricción:** toda mutación con sello R11; sin Write genoma huérfano; sin inventar términos; sin tocar creators H5.

### 2 — Evolution

| # | Entrada | Detalle |
|---|---------|---------|
| E.0 | Feature H6 | `inyeccion-dependencias-barrido-creators` / R14 |
| E.1–E.4 | Por ED | uuid + versión post-bump + nota DI |

### 3 — Evidencia AC-R14

| # | Entregable | Detalle |
|---|------------|---------|
| V.1 | Scan | `./sddia-run.sh` / `sddia-qa audit-eda-coverage --scan --json` → `orphan_count == 0` |
| V.2 | Sellos | 4× `Domain_Entity_Updated` trazables (coverage / bus) |

### 4 — Regresión + smoke Q7

| Bloque | Evidencia |
|--------|-----------|
| Q7 | Smoke `process-creator` (H5) — no romper AC-R12 |
| AC-REG-H5 | AC-R11, AC-R12 |
| AC-REG-H4 | AC-R9, AC-R10 |
| AC-REG-H3 | AC-R5–R8 |
| AC-REG-H2 | AC-R1, AC-R2 |
| AC-REG-MVP | AC-P1–P3 |

### 5 — Documentación

`implementation.md` + `execution.md` con lista sellos, conteo `N_ola=4`, comandos test. Handoff Argos.

---

## Handoff Argos

Criterios en `spec.md` §5. `validacion.md` `global: APTO` solo si **AC-R14** + regresión verdes. PBI-042 padre permanece en `pending/` (**L-PBI-LOC**) — `pbi_archived: false` en este ciclo salvo laudo Racso / Done global.
