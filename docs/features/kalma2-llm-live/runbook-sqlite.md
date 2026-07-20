---
feature_name: kalma2-llm-live
created: "2026-07-20"
process: feature
purpose: Runbook S4 — SQLite Cursor (L-IDE / L-WAL) AC8
---

# Runbook — SQLite Cursor (S4)

## Laudos

| ID | Norma |
|----|--------|
| **L-IDE** | Insertar en `state.vscdb` **no** dispara el agente del IDE. Persistencia/continuabilidad ≠ oráculo. |
| **L-WAL** | Escribir la DB live con Cursor abierto contiende por WAL. Preferir **copia** o Cursor cerrado. |

## Rutas

| OS | Global DB |
|----|-----------|
| Linux | `~/.config/Cursor/User/globalStorage/state.vscdb` |
| macOS | `~/Library/Application Support/Cursor/User/globalStorage/state.vscdb` |

Override: `SDDIA_CURSOR_VSCDB=/ruta/state.vscdb`

## Lab seguro (recomendado)

```bash
# 1) Copia (incluye -wal/-shm si existen, o backup sqlite)
SRC="$HOME/.config/Cursor/User/globalStorage/state.vscdb"
DST="/tmp/kalma2-state-copy.vscdb"
sqlite3 "$SRC" ".backup '$DST'"   # consistente aunque haya WAL

# 2) CHAT_STREAM contra la copia
export SDDIA_CURSOR_VSCDB="$DST"
export SDDIA_CURSOR_SQLITE_WRITE=1
unset SDDIA_LLM_CHAT_MOCK SDDIA_AGENT_RUNTIME_MOCK
export SDDIA_LLM_INFER_COMMAND=SddIA/scripts/tools/kalma2-llm-infer-lab.sh  # o cursor-agent

./SddIA/scripts/tools/kalma2-sqlite-smoke.sh
```

## Live (operador)

1. Cerrar Cursor **o** aceptar riesgo WAL.
2. `SDDIA_CURSOR_VSCDB` → DB real; `SDDIA_CURSOR_SQLITE_WRITE=1`.
3. Chat Kalma2 → verificar en UI Cursor que aparece composer `Kalma2: …` (puede requerir recarga).
4. No esperar respuesta autónoma del agente IDE (L-IDE).

## Criterios AC8

| Check | OK |
|-------|-----|
| Keys `composerData:` + 2× `bubbleId:` en DB de prueba | smoke |
| Entrada en `composer.composerHeaders` / tabla `composerHeaders` | smoke |
| Runbook L-IDE/L-WAL presente | este doc |
