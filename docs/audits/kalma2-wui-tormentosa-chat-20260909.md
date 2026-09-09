---
document_id: AUDIT-KALMA2-WUI-TORMENTOSA-CHAT-20260909
uuid: "100976ed-465b-4b48-98f0-3acd10aaa6a2"
title: "Auditoría empírica — Kalma2 WUI: Chat vs Tormentosa"
created: "2026-09-09"
process: audit
verdict: TORMENTOSA_404_CERRADO_503_RESIDUAL
updated: "2026-09-09"
pbi_ref: "docs/todos/pending/[FIX] kalma2-bridge ELF release fósil — POST api-aiua-interact 404.md"
document_id_pbi: PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF
pbi_spawned_503: PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE
related:
  - docs/todos/pending/[OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry).md
  - docs/todos/done/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
  - docs/todos/done/[KAIZEN] Aiúa — hallazgos auditoría live y thinking HIGH.md
  - docs/todos/kitchen/PBI-MULTI-LLM-ROUTER.md
  - docs/features/kalma2-aiua-perceptive-bridge/validacion.md
  - docs/todos/pending/[FIX] kalma2-bridge — fractura sistémica (64f37c7f7b34).md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/scripts/daemons/kalma2-bridge.sh
  - interfaces/kalma2/app.js
---

# Auditoría empírica — Kalma2 WUI ↔ Chat / Tormentosa

Fecha: 2026-09-09 (CEST). Host: `http://127.0.0.1:8765/`. HEAD: `479536b` (ancestro `492c673` PR #276).

## 0. Veredicto

| Canal WUI | Resultado empírico | Causa |
|-----------|--------------------|--------|
| **Chat** (`#chat` / `Ctrl+Enter` → `POST /api/chat`) | **APTO** (Vértice: respuesta Mayeuta/Tekton + meta sqlite). | Ruta presente ya en ELF 06-09. `mayeuta-llm` compilado 09-09. |
| **Tormentosa** (pre-recycle) | **FALLO 404** `ruta desconocida` | ELF release fósil pid 6151 (06-09 / arranque 07-09). |
| **Tormentosa** (post-recycle 15:18, pid 1244544) | **1º APTO** / **2º 503 UNAVAILABLE** | Puente OK. 2º = demanda Gemini; JSON crudo en WUI. |

No es un defecto del `textarea`/Enter. Los cuatro botones están en viewport. Split-brain **frontend fresco / binario viejo**.

## 1. Hipótesis y evidencia

| Id | Hipótesis | Veredicto | Evidencia |
|----|-----------|-----------|-----------|
| H3 | Botones fuera de viewport | **RECHAZADA** | Log WUI `vw=1648`, `#aiua-pulse` `inView:true` |
| H4 | Botones ocultos | **RECHAZADA** | Mismos rects; captura Firefox 720px |
| H2 | `Ctrl+Enter` → Mayeuta | **CONFIRMADA** | `willSendChat:true` → `enviarChat` |
| H5 | Click Tormentosa llega al fetch | **CONFIRMADA** | `enviarAiuaStimulus` + HTTP `status:404` `msg:ruta desconocida` |
| T1 | ELF runtime sin `/api/aiua/interact` | **CONFIRMADA** | `strings` release: **cero** literal `aiua/interact`. `curl` POST 404. `ps`: `…/target/release/kalma2-bridge` mtime 06-09 |
| T2 | Path/método malformado en WUI | **RECHAZADA** | Mismo 404 con `curl -X POST http://127.0.0.1:8765/api/aiua/interact` body `{"prompt":"audit-ping tormentosa"}` |
| M1 | `mayeuta-llm` ausente (fractura `64f37c7f7b34`) | **CERRADA en lab** | ELF nativo compilado 2026-09-09 15:02. Chat WUI responde. Distinto del 404 Aiúa. |

## 2. Probes HTTP (2026-09-09 ~15:08 CEST)

```text
POST /api/aiua/interact  → 404  {"success":false,"message":"ruta desconocida","exit_code":1}
POST /api/interact       → 200  data.response lab/LLM (Mayeuta sync; no es aiua-stimulus-processing)
POST /api/chat           → SSE (probe 25s sin bytes; WUI del Vértice sí completó stream)
```

`dispatch` en fuente HEAD:

```text
(Method::Post, "/api/chat")
(Method::Post, "/api/interact")          ← Mayeuta síncrono (kalma2-interact)
(Method::Post, "/api/aiua/interact")     ← Aiúa (ausente en ELF en ejecución)
_ => 404 "ruta desconocida"
```

WUI **no** llama `/api/interact`. El 200 de ese path **no** es el latido Tormentosa del PBI puente.

## 3. Proceso vivo

| Campo | Valor |
|-------|--------|
| pid | 6151 |
| exe | `SddIA/target/release/kalma2-bridge` |
| ELF mtime | 2026-09-06 06:48:40 CEST |
| arranque | 2026-09-07 14:34:55 CEST (uptime ~2 d) |
| bind | `127.0.0.1:8765` |
| UI estática | `interfaces/kalma2/` leída **en cada GET** (botón `#aiua-pulse` presente) |
| Lanzador | `SddIA/scripts/daemons/kalma2-bridge.sh` — `exec` ELF; **no** recompila |

PR #276 (`feat(kalma2): puente perceptivo WUI→Aiúa`) está en main. El órgano HTTP **no se reciclo** tras el merge.

## 4. Relación con PBI existentes

- `PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2` — **done**, CA APTO sobre **fuente + tests**. No cubre recycle del daemon release.
- `PBI-FIX-FRACTURE-64f37c7f7b34` — colapso `sse_chat_stream` / ELF `mayeuta-llm`. **Otro sello.** Chat ya opera. No explica el 404 Aiúa.

## 5. Remediación (no ejecutada en esta auditoría)

1. `cd SddIA && CARGO_TARGET_DIR=target cargo build --release -p kalma2-bridge`
2. Reciclar el órgano (systemd `sddia-kalma2-bridge@…` o kill+relaunch del script). **Prohibido** dos listeners en `:8765`.
3. Gate empírico: `POST /api/aiua/interact` ≠ 404; `strings` del nuevo ELF contiene `/api/aiua/interact`; Chat `#chat` no regresa.

PBI de ejecución 404: `PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF`.

## 6. Prueba biológica post-recycle (~15:18–15:25 CEST)

| Latido WUI `#aiua-pulse` | Resultado | Lectura |
|-------------------------|-----------|---------|
| 1º | Respuesta de Tormentosa OK | Ruta viva. 404 **cerrado**. |
| 2º | `[error] http-status-503: {… "status":"UNAVAILABLE" … high demand}` | Combustión Gemini saturada. El blob JSON llega al Vértice: `sanitize_bridge_message` no clasifica 503. |

Probe Tekton previo (15:19): `POST /api/aiua/interact` HTTP **500** en 81 s, mismo 503. Confirma handler, no dispatcher.

**No** es regresión del recycle. **No** autoriza retry (DA-5). Residuo de epidermis → `PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE`. Failover de proveedor → kitchen `PBI-MULTI-LLM-ROUTER`, no este sello.
