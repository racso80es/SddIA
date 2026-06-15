---
document_id: ED-CENTINELA-SOBERANIA
title: "[ARQUITECTURA] ED Centinela: Soberanía de Ejecución y Autogestión SddIA"
format: markdown
version: "1.0.0"
created: "2026-06-15"
status: done
priority: critica
process: feature
closed: "2026-06-15"
branch_name: feat/centinela-soberania-ejecucion
feature_ref: docs/features/centinela-soberania-ejecucion
origin: docs/todos/kitchen/ED Centinela Soberanía de Ejecución y Autogestión SddIA.md
pr_url: https://github.com/racso80es/SddIA/pull/92
merged_pr: 92
merge_commit: ce272c6315ef5c8528b2e8fb9109fcb820d1e16a
validacion_ref: docs/features/centinela-soberania-ejecucion/validacion.md
global: APTO
pbi_archived: true
---

# [ARQUITECTURA] ED Centinela: Soberanía de Ejecución y Autogestión SddIA

**Origen:** `docs/todos/kitchen/` (ED arquitectónica)  
**Cierre:** 2026-06-15 — feature `centinela-soberania-ejecucion`  
**PR:** [#92](https://github.com/racso80es/SddIA/pull/92) · merge `ce272c6315ef5c8528b2e8fb9109fcb820d1e16a`  
**Validación:** `docs/features/centinela-soberania-ejecucion/validacion.md` (`global: APTO`)

---

## 1. Visión S+ Grade y Contexto

Transformar los scripts de monitorización físicos (*watchers*, *daemons*) de "satélites inertes y dispersos" a Entidades de Dominio (ED) gobernadas por contrato. SddIA adquiere capacidad táctica de arrancar, auditar y purgar procesos de frontera, garantizando latido del bus EDA sin intervención manual del Vértice Biológico.

---

## 2. Definición del Contrato del Centinela (materializado)

Norma vigente: `SddIA/daemons/daemons-contract.md` v1.0.0. Formato canónico `{name}.md` (no `spec.json`).

```yaml
execution:
  entrypoint: "SddIA/scripts/daemons/<artefacto>"
  runtime: "python3"
  heartbeat_interval_seconds: <entero ≥ 5>
jurisdiction: "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus"
```

**Obligaciones termodinámicas implementadas:**

| Obligación | Implementación |
|------------|----------------|
| `Daemon_Heartbeat` | Clase ECST `SddIA/events/telemetry/daemon-heartbeat.md`; emisión vía `daemon_centinel_runtime.py` |
| Lock PID | `.SddIA/daemons/status/{name}.lock` |
| Idempotencia | State por daemon en `.SddIA/daemons/state/` (delivery) |

Telemetría vital en bus fractal `./.events/telemetry/` (no contamina DLT V3+ pending).

---

## 3. Capa de Control Táctico (realizado)

| Componente ED (kitchen) | Entrega real | UUID proceso |
|-------------------------|--------------|--------------|
| governance-daemon-manager | `SddIA/process/governance-daemon-manager.md` + `governance_daemon_manager_core.py` | `5a89793a-ba98-4b4f-9287-43c087e312df` |
| Kill-Switch | `SddIA/process/daemon-kill-switch.md` + hooks `execute-process.py` | `b0de6585-11fc-4b3c-8b19-ad6b727d820e` |
| Forja definiciones | `SddIA/process/daemon-creator.md` | `c172f130-532f-4714-be4e-fcd80b84a5dc` |
| Triaje Argos | `SddIA/process/daemon-heartbeat-audit.md` + suscripción `Daemon_Heartbeat` | `f45bda9d-40d9-471e-82a1-b9404b5a0dfd` |

Invocación canónica CEN-02:

```bash
python3 SddIA/scripts/qa/execute-process.py --process governance-daemon-manager \
  --inputs '{"operation":"start","daemon_id":"telegram-watcher","repository_path":"<abs>"}'
```

---

## 4. Desglose Kaizen — histórico de realización

| ID | Tarea (kitchen) | DoD | Estado | Evidencia |
|:---|:---|:---|:---:|:---|
| CEN-01 | Contrato base + estructura | Normas aislamiento, telemetría, JSON | ✅ | `SddIA/daemons/`, `cumulo.paths.json`, `daemon-creator` |
| CEN-02 | Actuador OS start/status/kill | `execute-process governance-daemon-manager` | ✅ | Handler lab + smoke status |
| CEN-03 | Kill-Switch SIGTERM/SIGKILL | Sin huérfanos al apagar Core | ✅ | `daemon-kill-switch`, `register_kill_switch_hooks` |
| CEN-04 | Refactor legacy watchers | Heartbeats + definición `{name}.md` | ✅ | 3 Centinelas indexados + runtime en `.py` |
| CEN-05 | Triaje Argos 3 ciclos | `System_Fracture_Detected` | ✅ | `daemon-heartbeat-audit`, `argos.md` §5 |

**Post-entrega (mismo PR):** runtime compartido `SddIA/scripts/qa/daemon_centinel_runtime.py` integrado en `event-watcher`, `telegram-watcher`, `github_bridge_watcher`.

---

## 5. Control de Riesgos (Filtro A) — disposición

| Riesgo | Disposición |
|--------|-------------|
| Permisos SO / sudo | Actuador limitado a espacio de usuario (`governance_daemon_manager_core`) |
| Saturación bus EDA | Heartbeats en `./.events/telemetry/`; fan-out Argos sin DLT directo |

---

## 6. Sellos EDA (entity-manager)

| Entidad | event_id `Domain_Entity_Created` |
|---------|----------------------------------|
| daemon-creator | `7009cd90-d36f-4c53-b4eb-e063c906be3d` |
| governance-daemon-manager | `86f44d56-a892-4035-a718-3c8aab7e5866` |
| daemon-kill-switch | `c420a839-ae26-4be8-aa5c-8edfabe9f8ee` |
| daemon-heartbeat (event) | `d65b2fad-44f7-44c1-853b-4f47907d7587` |
| daemon-heartbeat-audit | `112c4f64-d4d6-405b-b5bf-34491369be31` |

---

## 7. Referencias

- Feature: `docs/features/centinela-soberania-ejecucion/`
- Implementación: `implementation.md` · Validación: `validacion.md`
- Commits: `e46613d` (feat), `acb6fb6` (fix hash_signature procesos)
