---
id: start-sddia
uuid: d5aae800-06b0-4acc-b0fc-476d8e241eb1
type: process
version: 1.1.0
---

# Ignición del ecosistema SddIA (`start-sddia.sh`)

Script de arranque unificado del nodo local: levanta los **Centinelas** (Sistema Nervioso EDA) y el puente **Kalma2** (binario Rust `kalma2-bridge`). Vive en la raíz del repositorio como artefacto de **instancia**.

## Objetivo

| Componente | Rol | Ruta de arranque |
|------------|-----|------------------|
| **Centinelas obligatorios** | Bus EDA y barrido de pendientes | `SddIA/scripts/daemons/event-watcher.sh`, `event-sweeper.sh` |
| **Centinelas opcionales** | Telegram y GitHub bridge | `SddIA/scripts/daemons/telegram-watcher.sh`, `github-bridge-watcher.sh` |
| **Kalma2** | UI + `POST /api/interact` | `SddIA/target/{debug,release}/kalma2-bridge` |

## Separación genoma / instancia

- **Genoma** (`SddIA/`): centinelas, orquestador `execute-process`, crate `kalma2-bridge` en `SddIA/interfaces/kalma2-bridge/`.
- **Instancia** (`.SddIA/`): locks, estado y logs de demonios (`.SddIA/daemons/{status,state,logs}`).
- **Bundle UI** (`interfaces/kalma2/`): estáticos servidos por el bridge.

SSOT de rutas: `SddIA/core/cumulo.paths.json`.

## Secuencia de ignición

```mermaid
flowchart TD
    A[start-sddia.sh] --> B[cd REPO_ROOT]
    B --> C[Centinelas obligatorios]
    C --> D{event-watcher + event-sweeper OK?}
    D -->|no| X[cleanup + exit 1]
    D -->|sí| E[Centinelas opcionales]
    E --> F[kalma2-bridge]
    F --> G{HTTP GET / responde?}
    G -->|no| X
    G -->|sí| H[wait — ecosistema operativo]
    H --> I[SIGINT/SIGTERM]
    I --> J[cleanup: jobs + pkill centinelas + kalma2-bridge]
```

1. Ancla `REPO_ROOT`.
2. Lanza centinelas vía `SddIA/scripts/daemons/<name>.sh`.
3. Resuelve y arranca `kalma2-bridge` (`SDDIA_KALMA2_BRIDGE_BIN` o `SddIA/target/{debug,release}/`).
4. Health check HTTP en `http://127.0.0.1:8765/`.
5. `wait` hasta señal de terminación.

## Uso

```bash
cd SddIA && cargo build -p kalma2-bridge -p execute-process
./start-sddia.sh
```

Variables de entorno:

| Variable | Default | Efecto |
|----------|---------|--------|
| `SDDIA_CLIENT_PORT` | `8765` | Puerto HTTP Kalma2 |
| `SDDIA_CLIENT_TIMEOUT_SECONDS` | `120` | Timeout subproceso orquestador |
| `SDDIA_KALMA2_BRIDGE_BIN` | autodetect | Override binario bridge |
| `SDDIA_EXECUTE_PROCESS_BIN` | autodetect | Override orquestador (bridge) |
| `SDDIA_REPO_ROOT` | autodetect | Raíz repo (bridge) |

## Apagado limpio

- `kill` jobs en background.
- `pkill -x` centinelas + `kalma2-bridge`.

## Requisitos previos

```bash
cd SddIA && cargo build -p kalma2-bridge -p execute-process \
  -p event-watcher -p event-sweeper -p telegram-watcher -p github-bridge-watcher
```

- Bundle UI en `interfaces/kalma2/`.
- `curl` para health check.

## Diagnóstico

| Síntoma | Acción |
|---------|--------|
| `kalma2-bridge no encontrado` | `cargo build -p kalma2-bridge` |
| `orquestador no encontrado` (POST) | `cargo build -p execute-process` |
| Centinela no arranca | `.SddIA/daemons/logs/<name>.log` |

## Verificación rápida

```bash
pgrep -x event-watcher && pgrep -x event-sweeper
pgrep -x kalma2-bridge
curl -sf -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8765/
curl -sf -XPOST http://127.0.0.1:8765/api/interact -H 'Content-Type: application/json' -d '{"prompt":"hola"}'
```

## Referencias

- Feature bridge Rust: `docs/features/kalma2-bridge-rust/`
- Kalma2 UI: `interfaces/kalma2/README.MD`
- Proceso motor: `SddIA/process/kalma2-interact.md`
- CLI orquestador: `./sddia-run.sh --process kalma2-interact --inputs '{"prompt":"..."}'`
