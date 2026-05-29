---
feature_name: inmunidad-caos-fase2
created: "2026-05-29"
process: feature
base: main
scope: SddIA/process, execute_process_capsules, agents/tekton, agents/argos, scripts/qa
master_pbi_id: PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO
---

# Especificación técnica — Fase 2 · Nodos de Diagnóstico

## 1. Contexto

Estado actual (post Fase 1):

- **3 tools ofensivas** catalogadas con `context: chaos-engineering` (`io-choke`, `schema-corruptor`, `sandbox-breacher`).
- **0 procesos audit** caos en genoma (H16).
- `run_thermodynamic_toll` fail-soft **operativo** (D3.13).
- Fan-out `telemetry-compliance-audit` desde `Raw_Execution_Finished` **operativo**.
- `workspace-smoke` como **referencia** de handler lab + `workspace_template`.
- Tekton **sin** `chaos-engineering` en `allowed_policies` (pendiente D2.1 / D1.6).

Objetivo: materializar **tres procesos audit atómicos** sin Suite ni orquestador (Fase 3).

## 2. Convenciones transversales

| Atributo | Valor |
|----------|-------|
| `contract` | `process-contract v1.4.0` |
| `workspace_template` | `.SddIA/workspaces/{process_name}/{execution_id}/` |
| `context` | `chaos-engineering`, `quality-assurance` |
| Fases típicas | (1) Estímulo tool · (2) Certificación Argos |
| Atomicidad | **Una** tool ofensiva por proceso — prohibido `delegates_to` con más de una tool caos |

### 2.1 Ampliación RBAC Tekton (D2.1)

Añadir `chaos-engineering` a `allowed_policies` en `SddIA/agents/tekton.md` y fila correspondiente en `agents/index.md`.

### 2.2 Ampliación RBAC Argos (D2.8)

Añadir `event-routing` a `allowed_policies` en `SddIA/agents/argos.md` para lectura de eventos domain en certificación breach.

## 3. Proceso `audit-thermodynamic-toll-failsoft` (2.A)

### 3.1 Propósito

Validar que el **Peaje Termodinámico fail-soft** preserva `exit_code: 0` del negocio cuando `io-choke` estresa la escritura E/S asociada a telemetría.

### 3.2 Definición YAML (resumen)

```yaml
name: audit-thermodynamic-toll-failsoft
version: "1.0.0"
context: [chaos-engineering, quality-assurance]
inputs: []
outputs:
  - toll_failsoft_verified: Peaje completó sin abortar proceso pese a estrés E/S
phases:
  - name: Estímulo asfixia E/S
    intent: Invocar io-choke sobre target dentro del workspace inyectado.
    delegates_to: [agent:tekton, tool:io-choke]
  - name: Certificación Argos
    intent: Verificar exit 0 y bandera fail-soft en thermodynamic_toll.
    delegates_to: [agent:argos]
phase_invocations:
  - phase_name: Estímulo asfixia E/S
    invocations:
      - capsule: tool:io-choke
        stdin_spec:
          workspace_path:
            from_process_state: workspace_path
          target_file: ".telemetry-stress-target"
        on_error: abort
```

### 3.3 Handler lab (`execute_chaos_audit_thermodynamic_phase`)

1. Resolver `workspace_path` del estado CLI.
2. Ejecutar cápsula `io_choke.py` vía subprocess (patrón `test_chaos_tools._run_capsule`).
3. Al finalizar fases, `run_process()` invoca `run_thermodynamic_toll` (automático).
4. Fase Argos: assert `status_code == 0` y presencia de `thermodynamic_toll` con indicador fail-soft (`telemetry_io_failed` o canal equivalente documentado en Fase 3 telemetría).

## 4. Proceso `audit-telemetry-compliance-breach` (2.B)

### 4.1 Propósito

Validar cadena **tool corrupta → Raw_Execution_Finished → fan-out compliance → Telemetry_Compliance_Breached**.

### 4.2 Definición YAML (resumen)

```yaml
name: audit-telemetry-compliance-breach
version: "1.0.0"
context: [chaos-engineering, quality-assurance, event-routing]
inputs: []
outputs:
  - breach_event_path: Ruta relativa al JSON domain emitido
phases:
  - name: Estímulo alucinación recibo
    intent: Ejecutar schema-corruptor sin recibo válido.
    delegates_to: [agent:tekton, tool:schema-corruptor]
  - name: Certificación Argos
    intent: Verificar Telemetry_Compliance_Breached en ./.events/domain/.
    delegates_to: [agent:argos]
phase_invocations:
  - phase_name: Estímulo alucinación recibo
    invocations:
      - capsule: tool:schema-corruptor
        stdin_json:
          corruption_mode: empty
        on_error: abort
```

### 4.3 Handler lab

1. Ejecutar `schema_corruptor.py` con `corruption_mode: empty`.
2. Invocar `run_thermodynamic_toll` con recibo extraído (ausente/inválido).
3. Simular fan-out: `audit_telemetry_compliance(repo, rel_telemetry_path)` o flag `SDDIA_LAB_ROUTE_SYNC=1` según `test_telemetry_compliance.py`.
4. Fase Argos: buscar en `./.events/domain/` evento con `event_type: Telemetry_Compliance_Breached` y payload coherente (`breach_reason`, `capsule_id`).

## 5. Proceso `audit-sandbox-isolation-rbac` (2.C)

### 5.1 Propósito

Certificar que `sandbox-breacher` **no** escribe fuera del `workspace_path` y reporta bloqueo auditable.

### 5.2 Definición YAML (resumen)

```yaml
name: audit-sandbox-isolation-rbac
version: "1.0.0"
context: [chaos-engineering, quality-assurance]
inputs: []
outputs:
  - isolation_verified: Bloqueo confirmado sin marker de escape
phases:
  - name: Estímulo intento de fuga
    intent: Ejecutar sandbox-breacher con escape_target por defecto.
    delegates_to: [agent:tekton, tool:sandbox-breacher]
  - name: Certificación Argos
    intent: Confirmar envelope error y ausencia de archivo fuera del workspace.
    delegates_to: [agent:argos]
phase_invocations:
  - phase_name: Estímulo intento de fuga
    invocations:
      - capsule: tool:sandbox-breacher
        stdin_spec:
          workspace_path:
            from_process_state: workspace_path
          escape_target: "../breach-marker.txt"
        on_error: abort
```

### 5.3 Handler lab

1. Ejecutar `sandbox_breacher.py`.
2. Assert tool envelope: `success: false`, `exitCode: 1`.
3. Verificar que `(workspace_path.parent / "breach-marker.txt")` **no** existe.
4. Fase Argos: registrar `isolation_verified: true`.

## 6. Integración `execute_process_capsules.py`

| Función | Responsabilidad |
|---------|-----------------|
| `invoke_chaos_tool_capsule(repo, tool_name, payload)` | Subprocess unificado a `scripts/tools/{tool}/` |
| `execute_chaos_audit_phase(repo, process_name, phase_name, ...)` | Dispatcher por proceso + fase |
| Rama en `execute_phase()` | `if process_def.get("name") in CHAOS_AUDIT_PROCESSES` |

Procesos registrados en constante:

```python
CHAOS_AUDIT_PROCESSES = frozenset({
    "audit-thermodynamic-toll-failsoft",
    "audit-telemetry-compliance-breach",
    "audit-sandbox-isolation-rbac",
})
```

## 7. Tests QA

Nuevo archivo `SddIA/scripts/qa/test_chaos_audit_processes.py`:

| Test | AC |
|------|-----|
| `test_audit_thermodynamic_toll_failsoft_exit_zero` | AC2.1, AC2.2 |
| `test_audit_telemetry_compliance_breach_event` | AC2.1, AC2.2 |
| `test_audit_sandbox_isolation_blocks_escape` | AC2.1, AC2.2 |
| `test_chaos_audit_atomicity_one_tool_each` | AC2.3 — inspección YAML/delegates |

Fixtures plantilla en `persist_ref`:

- `_smoke-audit-thermodynamic-toll-failsoft.json`
- `_smoke-audit-telemetry-compliance-breach.json`
- `_smoke-audit-sandbox-isolation-rbac.json`

## 8. Touchpoints (resumen)

| Artefacto | Operación |
|-----------|-----------|
| `SddIA/process/audit-thermodynamic-toll-failsoft.md` | nuevo |
| `SddIA/process/audit-telemetry-compliance-breach.md` | nuevo |
| `SddIA/process/audit-sandbox-isolation-rbac.md` | nuevo |
| `SddIA/process/index.md` | +3 filas |
| `SddIA/agents/tekton.md` | `chaos-engineering` en policies |
| `SddIA/agents/argos.md` | `event-routing` en policies |
| `SddIA/agents/index.md` | sincronizar columnas |
| `SddIA/scripts/qa/execute_process_capsules.py` | handlers chaos audit |
| `SddIA/scripts/qa/test_chaos_audit_processes.py` | nuevo |
| `docs/features/inmunidad-caos-fase2/_smoke-audit-*.json` | plantillas smoke |

## 9. Criterios de aceptación (trazabilidad)

| AC PBI | Verificador spec |
|--------|------------------|
| AC2.1 | §3–§5 workspace_template + §8 índice |
| AC2.2 | §6 handlers + §7 tests smoke |
| AC2.3 | §2 convención atomicidad + test dedicado |

## 10. Riesgos técnicos

| Riesgo | Mitigación |
|--------|------------|
| Fan-out compliance asíncrono en CI | Handler invoca `audit_telemetry_compliance` síncrono en lab |
| `phase_invocations` tool no cableadas en runtime general | Handler lab es SSOT Fase 2; fan-out nativo en Fase 3+ |
| EDA coverage huérfana al crear 3 procesos | Upsert `eda-coverage.json` vía entity-manager o backfill documentado |
| Tekton sin policy → diseño inválido Dedalo | D2.1 obligatorio antes de forja procesos |
