---
feature_name: inyeccion-dependencias-h7-nucleo-fs
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-h7-nucleo-fs
persist_ref: docs/features/inyeccion-dependencias-h7-nucleo-fs
document_id: PBI-043-H7-NUCLEO-FS
execution_id: b8e2a471-5c9d-4f3a-8e1b-6d0c9f2a4b7e
phases: 7
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
scope: "Hito 1 (H7) — Núcleo FS residual DI fs:persist (R1–R3 / AC-H7)"
---

# Plan / Blueprint — H7 Núcleo FS (PBI-043 Hito 1)

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md`, `spec.md`.

`target_executor_rbac.allowed_policies`: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `quality-assurance`, `event-routing`.

## Viabilidad RBAC

| Cápsula / acción | ¿Permitida? |
|------------------|-------------|
| `skill:filesystem-manager` | sí (docs persist_ref / evolution; no forja genoma huérfana) |
| `skill:git-manager` | sí (evidencia git vía ecosistema) |
| `skill:shell-executor` | sí (`cargo test`, `sddia-qa`) |
| `action:execute-process` | sí (`entity-manager`, smokes) |

Ninguna fase exige cápsula fuera de `allowed_policies`. Mutación genoma **solo** vía `entity-manager` → `emit-domain-mutation` (**L-R2-MUTATION**). Git solo vía `skill:git-manager`. **Sin** gate Racso de taxonomía (Q6-A / **L-NO-INVENT**).

---

## Fases declarativas

```yaml
phases:
  - name: "Baseline AC-INV post-042"
    intent: "Verificar taxonomía 3 términos, bindings v1.1.0, 8 ED §3.1 sin requires_capability; abort si drift vs clarify D1."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "R1 Ola N_ola=7 (no-EM)"
    intent: "entity-manager update sobre las 7 ED §4.3 excl. entity-manager; sello Domain_Entity_Updated; bump patch; path ciego/mixto según spec."
    delegates_to: ["action:execute-process"]
  - name: "R1 entity-manager (última)"
    intent: "entity-manager update self: fase Delete físico → fs:persist ciego (L-Q3-EM); sello Domain_Entity_Updated; bump patch."
    delegates_to: ["action:execute-process"]
  - name: "Evolution H7"
    intent: "Registrar evolution feature Hito 1 H7 + una entrada por ED tocada (8)."
    delegates_to: ["skill:filesystem-manager", "action:execute-process"]
  - name: "Evidencia AC-H7 (Q3)"
    intent: "audit-eda-coverage --scan orphan_count==0; muestra coverage/bus Domain_Entity_Updated por cada ED H7."
    delegates_to: ["skill:shell-executor", "action:execute-process"]
  - name: "Regresión R3 / Q4"
    intent: "Suites capability_di / cerbero_di MVP→H6; smoke opcional process homologado H6 (sin re-mutar)."
    delegates_to: ["skill:shell-executor"]
  - name: "Documentación ejecución"
    intent: "implementation.md + execution.md; handoff Argos."
    delegates_to: ["skill:filesystem-manager"]
```

---

## Detalle por fase

### 0 — Baseline AC-INV

| # | Entregable | Detalle |
|---|------------|---------|
| B.1 | Taxonomía | `catalog` = `doc:closure`, `proc:git-sync`, `fs:persist` |
| B.2 | Bindings | 3 filas v1.1.0 intactas (`fs:persist` → `skill:filesystem-manager`) |
| B.3 | §3.1 | Confirmación ausencia `requires_capability` en las 8 |
| B.4 | Abort | Si taxonomía/bindings rotos o lista ≠ §3.1 → `blocked` (no inventar) |

### 1 — R1 Ola no-EM (`N_ola` parcial = 7)

| # | ED | Mutación |
|---|-----|----------|
| R1.1 | `route-domain-event` | 3 fases FS → `fs:persist` ciego |
| R1.2 | `daemon-kill-switch` | Enumeración mixto; Verificación ciego |
| R1.3 | `governance-daemon-manager` | Resolución + Actuación OS mixtos |
| R1.4 | `daemon-heartbeat-audit` | Ingesta mixta (argos + `fs:persist`) |
| R1.5 | `fix-tool-process` | Preparación sandbox ciego |
| R1.6 | `telemetry-batch-stub` | Consumo ciego |
| R1.7 | `workspace-smoke` | Verificación ciego |

**Restricción:** sello R2 por ED; sin Write genoma huérfano; sin inventar términos; sin tocar H8+.

### 2 — R1 `entity-manager` (última)

| # | Entregable | Detalle |
|---|------------|---------|
| R1.8 | `entity-manager` | Delete físico → ciego `fs:persist`; Delegación/Sello intactos |

### 3 — Evolution

| # | Entrada | Detalle |
|---|---------|---------|
| E.0 | Feature H7 | `inyeccion-dependencias-h7-nucleo-fs` / R1–R3 |
| E.1–E.8 | Por ED | uuid + versión post-bump + nota DI `fs:persist` |

### 4 — Evidencia AC-H7

| # | Entregable | Detalle |
|---|------------|---------|
| V.1 | Scan | `./sddia-run.sh` / `sddia-qa audit-eda-coverage --scan --json` → `orphan_count == 0` |
| V.2 | Sellos | 8× `Domain_Entity_Updated` trazables (coverage / bus) |

### 5 — Regresión R3 / Q4

| Bloque | Evidencia |
|--------|-----------|
| AC-REG-DI | Suites `capability_di` / `cerbero_di` (MVP→H6) |
| Smoke opc. | Lectura/ignición 1 creator H6 (p. ej. `norm-creator`) — sin mutar |

### 6 — Documentación

`implementation.md` + `execution.md` con lista sellos, conteo `N_ola=8`, comandos test. Handoff Argos.

---

## Handoff Argos

Criterios en `spec.md` §5. `validacion.md` `global: APTO` solo si **AC-H7** + **AC-REG-DI** + **AC-ORPHAN** + **AC-SEAL** verdes. PBI-043 permanece en `pending/` (**L-PBI-LOC**) — `pbi_archived: false` en este ciclo.
