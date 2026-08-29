---
feature_name: telemetria-cognitiva-llm-kalma2
created: "2026-08-29"
process: feature
branch_name: feat/telemetria-cognitiva-llm-kalma2
persist_ref: docs/features/telemetria-cognitiva-llm-kalma2
pbi_ref: docs/todos/pending/[ARQUITECTURA] Telemetría Cognitiva: Captura de métricas LLM y exposición en Kalma2.md
document_id: PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2
uuid: a1535038-8db5-4351-8a81-cfa5586b8c5b
execution_id: "0cdb618e-51e4-461b-9d14-469a5363257b"
status: blueprint_locked
mayeuta_verdict: ok
dedalo_verdict: ok
depends_on:
  - docs/features/telemetria-reactiva-eda-fase6
  - docs/features/kalma2-canal-telemetria-progreso
  - docs/features/kalma2-bridge-rust
adjacent_not_merged:
  - docs/todos/pending/[OPERATIVO] Latido Ontológico (System Heartbeat).md
  - docs/todos/pending/[KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema.md
---

# Objetivos — telemetria-cognitiva-llm-kalma2

## Misión

Dotar al ecosistema de observabilidad del **coste cognitivo LLM** (tokens, modelo, latencia de proveedor) con un único contrato público (`telemetry_receipt`), sin tumbar negocio por métricas ausentes, sin poll WUI, y sin gobernanza Cerbero en la cuota ordinaria.

## Punto objetivo

> **O-COGNITIVE-PULSE:** El Vértice Biológico ve en Kalma2 el pulso de tokens/modelo/latencia. El bus fractal de peaje transporta el mismo schema. STREAM de chat no finge ser Peaje.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Contrato REF + schema default + compliance | `cognitive_metrics` como tercer estándar |
| Mapeo Aduana `thermodynamic_cost` → receipt | Backend Python |
| `mayeuta-llm` (JSON + STREAM degradado) | Hard fail por `usage` omitido |
| Consolidación Radamanto + umbral rate N1/N2 | Revocación en N1 |
| SSE broadcast + snapshot + widget | Sustituir progress/status/chat SSE |
| Corregir códice fósil “Fase 3.C” | Latido ontológico / Espejo de Consciencia |

## Ley aplicada

- DD-1…DD-4 del PBI v1.2.0 (clarify D1).
- Emisores REF: CLI indexado. Bridge **no** escribe `eda_fractal.telemetry`.
- Genoma (`events/`, `agents/radamanto.thresholds.json`, `skills/mayeuta-llm`) vía `entity-manager`.
- Ceguera espacial: orquestador no conoce WUI.

## Criterios (resumen)

Ver `clarify.md` D5 / `spec.md` AC. Pipeline REF→Radamanto **ya vivo** (I1); falta receipt en el payload y el canal STREAM (I2/I3).
