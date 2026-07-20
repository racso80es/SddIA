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

1. Preferir **backup** (`kalma2-sqlite-live-smoke.sh`) o Cursor cerrado (L-WAL).
2. `SDDIA_CURSOR_VSCDB` → DB/copia; `SDDIA_CURSOR_SQLITE_WRITE=1`.
3. Chat Kalma2 → composer `Kalma2: …` en DB; reply viene del **CLI** (oráculo), no del watch IDE.
4. `SDDIA_CURSOR_IDE_WATCH_ONLY=1` → rechazado (exit 4).
5. Opcional: `SDDIA_CURSOR_WAKE_AGENT=1` → segundo disparo CLI post-persist (`kalma2-wake`).

## Criterios AC8 / HOST-D

| Check | OK |
|-------|-----|
| Keys `composerData:` + 2× `bubbleId:` en DB de prueba | smoke lab |
| Entrada en `composer.composerHeaders` / tabla `composerHeaders` | smoke |
| Backup host L-WAL + write + Kalma2 | `kalma2-sqlite-live-smoke.sh` |
| Runbook L-IDE/L-WAL presente | este doc |
| Oráculo CLI (`ide_auto_fire=false`) | meta oracle |
