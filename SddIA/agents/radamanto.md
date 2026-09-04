---
uuid: "4d5e6f7a-8b9c-4d0e-1f2a-3b4c5d6e7f8a"
name: "radamanto"
version: "1.1.0"
contract: "agents-contract v1.0.0"
allowed_policies:
  - "quality-assurance"
  - "ecosystem-evolution"
inputs:
  - "telemetry_batch": "Acumulado estadístico derivado exclusivamente de Raw_Execution_Finished"
  - "radamanto_thresholds": "Umbrales deterministas resueltos vía cumulo.paths.json"
outputs:
  - "domain_status_events": "Domain_Entity_Degraded, Domain_Entity_Restored, Domain_Entity_Deprecated en ./.events/domain/"
  - "domain_telemetry_snapshots": "Domain_Entity_Telemetry_Captured por cada consumo OK (trazabilidad vectorial)"
  - "dlt_seals": "Sellado iota-immutable-publisher exclusivo sobre gobernanza entidades / inmunidad"
llm_profile:
  tier: "none"
  description: "Actuario de umbrales; prohibido LLM"
---

# Agente Radamanto: Certificador / Actuario

## 1. Propósito y Naturaleza

Radamanto es el **actuario de confianza** del ecosistema SddIA: procesa el acumulado estadístico de telemetría CLI (batching), aplica umbrales deterministas, registra cambios de estatus de entidades en IOTA Rebased y emite **snapshots de ejecución** para ingesta vectorial.

No evalúa código ni interpreta intenciones. Opera bajo **genoma determinista** — solo métricas agregadas.

## 2. Prohibiciones contractuales (AC4.2)

| Prohibido | Motivo |
|-----------|--------|
| Invocar cronómetros, `skill:shell-executor`, medición directa | Telemetría solo vía Peaje CLI |
| Sellar `PullRequest_*` / CRUD `Domain_Entity_{Created\|Updated\|Deleted}` | Reservado Cúmulo / `entity-manager` (D0.1) |
| Emitir `Domain_Entity_Restored` desde Argos o `fix-tool-process` | Redención exclusiva Radamanto (D4.13) |
| Modificar `SddIA/tools/`, `SddIA/skills/` en producción | Jurisdicción sandbox + Tekton |

## 3. Exclusividad DLT (AC4.1)

Único agente autorizado a invocar `tool:iota-immutable-publisher` para:

- `Domain_Entity_Degraded`
- `Domain_Entity_Restored`
- `Domain_Entity_Deprecated`
- `System_Immunity_Certified` (certificación inmunidad Caos — Fase 4)

## 4. Emisión de telemetría activa (no-DLT)

Tras cada consumo válido de `Raw_Execution_Finished` (no duplicado), emite **`Domain_Entity_Telemetry_Captured`** en `./.events/domain/` (fail-soft). Suscriptor: proceso `memory-evolution-ingest` → store `.SddIA/vector_store/evolution/`.

Ortogonal a Self-Healing: el snapshot se emite también cuando la entidad permanece `healthy`.

**Prohibido** reutilizar `Domain_Entity_Updated` (CRUD genómico) para este estímulo.

## 5. Jurisdicción vs Argos

| Agente | Rol |
|--------|-----|
| **Argos** | Valida **estructura/contrato** del artefacto reparado en sandbox; auditoría compliance termodinámica |
| **Radamanto** | Consolida telemetría CLI; sella **redención** (`Domain_Entity_Restored`) cuando la deuda métrica está en cero (R4.3); emite snapshots `Telemetry_Captured` |

## 6. Instrucciones operativas

Ver `radamanto.instructions.json` y `radamanto.thresholds.json` (SSOT configurable).
