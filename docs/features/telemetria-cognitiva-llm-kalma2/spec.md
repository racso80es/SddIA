---
feature_name: telemetria-cognitiva-llm-kalma2
created: "2026-08-29"
process: feature
base: main
scope: telemetria-cognitiva-llm-kalma2
version_spec: "1.0.0"
uuid: a1535038-8db5-4351-8a81-cfa5586b8c5b
status: dedalo_locked
agent: dedalo
branch_name: feat/telemetria-cognitiva-llm-kalma2
persist_ref: docs/features/telemetria-cognitiva-llm-kalma2
pbi_ref: docs/todos/pending/[ARQUITECTURA] Telemetría Cognitiva: Captura de métricas LLM y exposición en Kalma2.md
document_id: PBI-TELEMETRY-LLM-COGNITIVE-METRICS-KALMA2
execution_id: "0cdb618e-51e4-461b-9d14-469a5363257b"
depends_on:
  - docs/features/telemetria-reactiva-eda-fase6
  - docs/features/kalma2-canal-telemetria-progreso
adjacent_not_merged:
  - docs/todos/pending/[OPERATIVO] Latido Ontológico (System Heartbeat).md
---

# Especificación — telemetria-cognitiva-llm-kalma2

## 1. Topología de responsabilidades

```text
mayeuta-llm SYNTHESIZE/CLASSIFY     ← evoluciona (receipt en JSON; DD-2)
email_triage / handlers             ← INTANGIBLE contrato; CLI mapea thermodynamic_cost
execute-process thermodynamic.rs    ← evoluciona (adjuntar telemetry_receipt + capsule_id)
./.events/telemetry/                ← REF (fan-out YA vivo)
radamanto-batch                     ← evoluciona (bloque cognitive + rate N1/N2)
.SddIA/radamanto/                   ← stats + inbox STREAM (Cúmulo)
mayeuta-llm STREAM                  ← evoluciona (sidecar inbox; stderr no-ECST)
kalma2-bridge                       ← evoluciona (SSE broadcast + GET snapshot)
  GET /api/progress/stream          ← INTANGIBLE (correlation_id)
  POST /api/chat                    ← INTANGIBLE semántica; post-wait lee sidecar
interfaces/kalma2                   ← evoluciona (widget pulso + alerta N1)
capsule-json-io.md                  ← INTANGIBLE (solo xref opcional)
```

## 2. Laudos Dedalo

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | Schema público | `telemetry_receipt` en payload REF | DD-1; evento ya OPTIONAL |
| **L2** | Campos | Obligatorios para compliance si `telemetry_provided`: `prompt_tokens`, `completion_tokens`. Opcionales: `llm_model`, `tier`, `provider_latency_ms`, `cognitive-degraded` | Extiende `DEFAULT_TELEMETRY_SCHEMA`; no romper cápsulas que solo emiten tokens |
| **L3** | Mapeo interno | `tokens_in`→`prompt_tokens`, `tokens_out`→`completion_tokens`; `duration_ms` del coste **no** pisa `duration_ms` del Peaje (ese es wall-clock proceso). Latencia proveedor = `provider_latency_ms` | Evita colisión con payload REF REQUIRED |
| **L4** | Dónde extrae el CLI | Acumulador `state.telemetry_receipts[]` durante invocaciones de cápsula; al Peaje: **último** receipt no vacío, o **suma** de tokens si hay varios en el mismo proceso, + `capsule_id` del último LLM | Hoy `thermodynamic.rs` no lee stdout. Un proceso puede invocar N cápsulas |
| **L5** | Suma vs último | Tokens: **suma**. `llm_model`/`tier`: último no degradado, else último. `provider_latency_ms`: **max**. `cognitive-degraded`: OR | Un REF = un proceso |
| **L6** | STREAM | Sidecar instancia: clave Cúmulo nueva `radamanto.cognitive_inbox` = `.SddIA/radamanto/inbox/{uuid}.json` con el **mismo** objeto `telemetry_receipt` (+ `source: stream`, `capsule_id: mayeuta-llm`). Bridge **no** escribe fractal telemetry | I3; emisores REF = CLI |
| **L7** | Ingesta STREAM | `radamanto-batch` (o gancho al inicio del mismo core si se invoca) **también** drena `cognitive_inbox` hacia el bloque `cognitive` de `stats.json` | Un consolidado; sin process nuevo en este ciclo si el drain se engancha al batch existente + al snapshot/SSE del bridge (leer inbox+stats) |
| **L8** | SSE | `GET /api/telemetry/stream` `text/event-stream`. Replay: últimos N JSON de `eda_fractal.telemetry` cuyo payload tenga `telemetry_receipt` **o** frames proyectados desde inbox. Watch: mtime de ambas hojas. **Sin** query `correlation_id` | DD-3; ≠ `/api/progress/stream` |
| **L9** | Snapshot | `GET /api/telemetry/cognitive` lee `radamanto.stats` (+ quota flag N1). No sustituye SSE | DD-3 fallback |
| **L10** | Rate | Bloque `cognitive` en `radamanto.thresholds.json`: `max_tokens_per_minute`, `critical_tokens_per_minute` (N2). Ventana 60s sobre muestras con timestamp | Clase nueva; genoma vía entity-manager |
| **L11** | N1 / N2 | N1: `stats.cognitive.quota_alert=true` (WUI). N2: `Domain_Entity_Degraded` reason `cognitive_critical_quota` **solo** si `critical_*` cruzado o bucle (mismo `capsule_id` + tokens/min ≥ critical) | DD-4 |
| **L12** | Códice evento | Quitar “Reservado Fase 3.C”; listar suscriptores reales; documentar campos OPTIONAL del receipt | I1 fósil |
| **L13** | Genoma | Evento + thresholds + `mayeuta-llm.md`/binario skill vía `entity-manager` / skill-creator. `cumulo.paths.json` `radamanto.cognitive_inbox` en el mismo PR de feature (topología; no creator de evento) | DA-2 |
| **L14** | Compliance | Campos nuevos **no** obligan breach si ausentes en cápsulas no-LLM. `cognitive-degraded` con tokens 0 **satisface** schema de tokens | DD-2 vs audit existente |

## 3. Contrato `telemetry_receipt`

```json
{
  "prompt_tokens": 0,
  "completion_tokens": 0,
  "llm_model": "string|null",
  "tier": "string|null",
  "provider_latency_ms": 0,
  "cognitive-degraded": false
}
```

Extracción prótesis (best-effort, sin hard fail): `usage.prompt_tokens` / `usage.input_tokens` / `usage.prompt_token_count`; análogos completion; `model` / `llm_model`. Ausencia → L14 + DD-2.

STREAM: mismo objeto en inbox; no mezclar con PTC de progreso.

## 4. Consolidado Radamanto

`stats.json` gana hermano de `entities`:

```json
{
  "entities": {},
  "cognitive": {
    "tokens_prompt_total": 0,
    "tokens_completion_total": 0,
    "by_model": {},
    "last_model": null,
    "latency_ms_avg": 0,
    "window": [],
    "quota_alert": false,
    "quota_critical": false
  }
}
```

`window[]`: `{ts, tokens, model}` para rate. No alterar semántica de `entities` (calidad/self-heal).

## 5. WUI

Widget inerte: tokens acumulados, último modelo, latencia media, badge N1. EventSource `/api/telemetry/stream`; snapshot inicial GET. No tocar consola de progreso ni status.

## 6. AC (mapeo PBI)

| AC | Gate |
|----|------|
| AC1 | Evento + `DEFAULT_TELEMETRY_SCHEMA` + xref opcional; grep `cognitive_metrics` vacío en persist+genoma tocado |
| AC2 | Test: `thermodynamic_cost` → receipt en payload REF |
| AC3 | Test: mayeuta sin usage → degraded + exit 0 |
| AC4 | REF con receipt en `./.events/telemetry/` (unit/lab) |
| AC5 | Batch actualiza `cognitive` sin romper test entidades |
| AC6 | N1 no emite Degraded; N2 sí (unit) |
| AC7 | Rutas bridge en `dispatch`; SSE ≠ progress |
| AC8 | WUI consume stream + badge |

## 7. Intangibles

Familia progress; status; chat SSE semántico; Python; `capsule-json-io` como SSOT; Espejo/Latido.
