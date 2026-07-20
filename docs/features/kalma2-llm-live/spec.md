---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
base: main
scope: kalma2-llm-live
version_spec: "1.0.0"
uuid: f0f1b1ec-4b79-47c6-85e2-a0ac2ca3164b
status: dedalo_locked
laudo: PBI-v2.2-L-EP-through-L-STOP
---

# Especificación — kalma2-llm-live

## 1. Topología

```text
Kalma2 UI
  ├─ Botón Chat ──────────► POST /api/chat (SSE)
  │                            └─ mayeuta-llm → subproceso Python (bóveda)
  │                                 └─ stdout tokens → bridge → UI
  │                                 └─ colapso/timeout → System_Fracture_Detected
  │
  └─ Botón Forjar Proceso ► POST /api/execute
                               └─ aduana bridge → Kalma2_Process_Requested (EDA)
                                    └─ TQM → hijo (baseline APTO; no reabrir)
```

`/api/interact` (legado): alias compat o deprecación en el mismo PR (L-EP). Sin clasificación LLM.

## 2. Contratos

### 2.1 `/api/chat` (SSE)

| Aspecto | Contrato |
|---------|----------|
| Request | JSON `{ "prompt": string }` (mínimo) |
| Response | `text/event-stream`; frames con tokens |
| Timeout | Estricto (env/bóveda); aborta espera infinita |
| Fractura | Emitir `System_Fracture_Detected` `{ source: "kalma2-bridge", fracture_kind: "sse_watchdog" \| "prosthetic_collapse" }` |
| Side-effects | **Prohibido** emitir `Kalma2_Process_Requested` / mutar estado de proyecto |

### 2.2 `/api/execute`

| Aspecto | Contrato |
|---------|----------|
| Request | JSON con prompt/process allowlist (alineado a emisión existente) |
| Response | Acuse JSON inmediato (no SSE de negocio) |
| Side-effects | Emisión EDA / invocación inerte orquestador (camino ya APTO) |

### 2.3 `mayeuta-llm`

| Aspecto | Contrato |
|---------|----------|
| Rol | Transductor: JSON stdin → spawn comando bóveda → stdout stream |
| Ceguera | Absoluta sobre Cursor/SQLite/Python deps |
| CLASSIFY_INTENT | No gobierna despacho (L-CI); limpieza vía evolución skill |

### 2.4 Prótesis Python (L-FILE)

| Modo | I/O |
|------|-----|
| chat-stream | Payload chat → tokens por stdout (línea a línea o chunks) |
| AGENT_PHASE | JSON stdin → última línea JSON envelope (contrato full-cycle B) |

## 3. Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/interfaces/kalma2-bridge` | `/api/chat` SSE + `/api/execute` + fractura |
| `SddIA/skills/mayeuta-llm` | Streaming + subproceso; derogar aduana CLASSIFY |
| `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` | Dual-mode chat-stream |
| `SddIA/engine/.../handlers/kalma2.rs` | Dejar de usar CLASSIFY como aduana (ruta execute en bridge) |
| `interfaces/kalma2/` | Bifurcación UI |
| `.dev/.env.example` | Documentar timeout SSE / comando prótesis chat |

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC1 | SSE tokens desde stdout Python |
| AC2 | kill prótesis → cierre limpio + `System_Fracture_Detected` |
| AC3 | Proceso UI → `/api/execute` → orquestación async (no texto libre) |
| AC4 | Sin `.py` → `cargo build --release` Core OK |
| AC5 | AGENT_PHASE JSON válido post-refino |
