---
feature_name: inmunidad-caos-fase2
created: "2026-05-29"
purpose: Decisiones Fase 2 y herencia del gate Fase 1
---

# Clarificación — Fase 2 (Nodos de Diagnóstico)

## Precondición (gate Fase 1)

Fase 1 cerrada con `validacion.md` APTO (AC1.1–AC1.3): contexto `chaos-engineering`, tres tools ofensivas, `assert_workspace_bound`, `tools-contract` v1.3.0. No se reabre el Arsenal salvo hallazgo bloqueante durante Tekton.

## Decisiones heredadas

| ID | Resolución | Uso en Fase 2 |
|----|------------|---------------|
| D0.3 | Inocuidad acotada a `workspace_path` | Procesos inyectan `workspace_path`; tools validan destinos |
| D0.9 | PBI en `pending/` | `validacion.md` con `pbi_archived: false` |
| D1.6 | Tekton sin `chaos-engineering` en Fase 1 | **Fase 2** amplía `allowed_policies` de Tekton |
| H25 | Cerbero stub solo PR review | Argos certifica reacción vía handler lab + envelope |

## Decisiones cerradas — Fase 2

| ID | Pregunta | Resolución |
|----|----------|------------|
| **D2.1** | ¿Contexto RBAC de procesos audit? | **`chaos-engineering`** + **`quality-assurance`** — auditoría de resiliencia bajo contexto caos |
| **D2.2** | ¿Forja de procesos? | **`process-creator`** vía entity-manager o forja lab Tekton; tres archivos bajo `SddIA/process/` |
| **D2.3** | ¿Invocación tool en runtime? | Handler dedicado en `execute_process_capsules.py` (patrón `workspace-smoke`); subprocess a cápsula Python Fase 1 |
| **D2.4** | ¿`phase_invocations` en YAML? | **Sí, documentales** — una invocación `tool:{name}` por fase de estímulo; ejecución física en handler lab hasta fan-out tool nativo |
| **D2.5** | **`audit-thermodynamic-toll-failsoft`** — estímulo | `io-choke` sobre ruta **dentro** del workspace que simula bloqueo E/S; al cierre `run_thermodynamic_toll` debe registrar `telemetry_io_failed` sin elevar `exit_code` negocio |
| **D2.6** | **`audit-telemetry-compliance-breach`** — cadena | Handler ejecuta `schema-corruptor` (`corruption_mode: empty`); dispara Peaje → `Raw_Execution_Finished` → fan-out compliance → JSON `Telemetry_Compliance_Breached` en `./.events/domain/` |
| **D2.7** | **`audit-sandbox-isolation-rbac`** — éxito | Tool retorna `success: false`, `exitCode: 1`; Argos confirma ausencia de marker fuera del workspace y mensaje auditable |
| **D2.8** | ¿Ampliar Argos `allowed_policies`? | **Sí** — añadir `event-routing` para lectura de `./.events/domain/` en certificación breach |
| **D2.9** | ¿Tests? | Nuevo `test_chaos_audit_processes.py` — smoke `execute-process` por proceso + asserts AC2.x |
| **D2.10** | ¿Fixtures smoke? | Plantillas `_smoke-audit-{nombre}.json` en `persist_ref` (patrón features-documentation-pattern) |

## Contrato común procesos audit

| Campo / regla | Valor |
|---------------|-------|
| `workspace_template` | `.SddIA/workspaces/{process_name}/{execution_id}/` |
| `contract` | `process-contract v1.4.0` |
| Tools por proceso | **Exactamente 1** (AC2.3) |
| `workspace_path` | Inyectado por CLI; propagado a stdin tool |
| Salida Argos | Fase final `Certificación Argos` con `delegates_to: agent:argos` |

## Referencias

- Gate Fase 1: `docs/features/inmunidad-caos-fase1/validacion.md`
- Hallazgos: `docs/features/inmunidad-caos-fase0/impact-analysis.md` (H16, H22–H26)
- PBI: `docs/todos/pending/PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO.md` § Fase 2
