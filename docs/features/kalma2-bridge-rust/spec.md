---
feature_name: kalma2-bridge-rust
created: "2026-06-19"
process: feature
base: main
scope: kalma2-bridge-rust
version_spec: "1.0.0"
uuid: 2afb1f2f-667c-4c39-ae5f-7bd7f626c7e2
status: pre-execution
---

# Especificación — kalma2-bridge (Rust)

## 1. Ubicación del crate (decisión)

Nuevo grupo de miembros del workspace `SddIA/interfaces/*`; crate en `SddIA/interfaces/kalma2-bridge/`.

```toml
# SddIA/Cargo.toml (members) — añadir vía vía autorizada (mutación genoma)
"interfaces/*"
```

Justificación: el binario es genoma (workspace Rust); el bundle servido permanece en instancia `interfaces/kalma2/` (raíz repo). No es Centinela (`daemons/*`) ni cápsula invocada por orquestador (`tools/*`): es superficie cliente long-running → grupo propio.

## 2. Dependencias (óptimo termodinámico)

```toml
# SddIA/interfaces/kalma2-bridge/Cargo.toml
[package]
name = "kalma2-bridge"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "kalma2-bridge"
path = "src/main.rs"

[dependencies]
tiny_http = "0.12"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Decisión `tiny_http`: servidor síncrono, sin runtime async (tokio/axum), homólogo termodinámico de `ThreadingHTTPServer` del puente Python. Mínima superficie binaria; modelo subproceso-síncrono 1:1 con la petición.

## 3. Superficie HTTP

| Método | Ruta | Comportamiento |
|--------|------|----------------|
| `GET` | `/` | Sirve `interfaces/kalma2/index.html` |
| `GET` | `/{rel}` | Sirve asset relativo bajo `interfaces/kalma2/` (anti path-traversal) |
| `POST` | `/api/interact` | Delega al orquestador; retorna stdout JSON sin manipular |
| `*` | otro | `404` JSON `{"success":false,"message":"...","exit_code":1}` |

### 3.1 Resolución de assets (ceguera + seguridad)

- Raíz canónica: `REPO_ROOT/interfaces/kalma2/` (`REPO_ROOT` = `parents[3]` desde el binario, o `SDDIA_REPO_ROOT`).
- Canonicalizar destino; rechazar si escapa de la raíz (`403`/`404`).
- MIME: `html→text/html`, `js→text/javascript`, `css→text/css`, resto `application/octet-stream`.

### 3.2 `POST /api/interact` (enrutador de voluntad)

```text
entrada: {"prompt": "<texto>"}   (string no vacío; si no → 400)
acción : subproceso síncrono del orquestador con --process kalma2-interact --inputs {"prompt":...}
salida : última línea JSON de stdout del orquestador, retornada TAL CUAL (passthrough)
```

## 4. Invocación del orquestador (SSOT — discrepancia resuelta)

> **Discrepancia detectada (sin conjetura):** la orden táctica exige `./sddia-run.sh --process kalma2-interact --inputs '...'`, pero `sddia-run.sh` está **borrado** en el working tree (la migración a Rust lo sustituyó por `orchestrator_resolve.py` → binario `execute-process`).

Resolución SSOT (alineada a `SddIA/scripts/qa/orchestrator_resolve.py`):

| Prioridad | Fuente del ejecutable |
|-----------|-----------------------|
| 1 | `SDDIA_EXECUTE_PROCESS_BIN` (override) |
| 2 | `SddIA/target/debug/execute-process` |
| 3 | `SddIA/target/release/execute-process` |

**Decisión:** `kalma2-bridge` resuelve el binario nativo directamente (mismo SSOT, sin salto Python/shell en la ruta caliente). Además se **restaura `sddia-run.sh`** como wrapper CLI fino (paridad de contrato shell exigido por la orden), delegando en `orchestrator_resolve.py`. La UI no cambia.

`argv` efectivo del subproceso:

```text
<execute-process-bin> --process kalma2-interact --inputs {"prompt":"<texto>"}
cwd = REPO_ROOT ; env = heredado ; timeout = SDDIA_CLIENT_TIMEOUT_SECONDS (def 120)
```

## 5. Passthrough de salida

- Tomar líneas no vacías de stdout; parsear la **última** como JSON.
- Sin stdout válido → `{"success":false,"message":<stderr|"sin salida del motor">,"exit_code":<rc|1>}`.
- Con JSON válido → reenviar bytes sin manipular (Content-Type `application/json; charset=utf-8`).

Compatibilidad UI (`app.js`): consume `data.success`, `data.response`, `data.message` — el orquestador ya emite ese esquema; passthrough lo preserva.

## 6. Variables de entorno

| Variable | Default | Efecto |
|----------|---------|--------|
| `SDDIA_CLIENT_PORT` | `8765` | Puerto de escucha |
| `SDDIA_CLIENT_TIMEOUT_SECONDS` | `120` | Timeout subproceso |
| `SDDIA_EXECUTE_PROCESS_BIN` | — | Override binario orquestador |
| `SDDIA_REPO_ROOT` | autodetectado | Raíz repo (assets + cwd) |

## 7. Criterios de aceptación

1. `cargo build -p kalma2-bridge` verde; binario en `SddIA/target/{debug,release}/kalma2-bridge`.
2. `GET /` retorna `index.html` (HTTP 200, `text/html`).
3. `POST /api/interact {"prompt":"hola"}` retorna JSON idéntico a la salida del orquestador para `kalma2-interact`.
4. Paridad de respuesta con `sddia-client-bridge.py` para el mismo prompt.
5. `start-sddia.sh` arranca el binario; health check HTTP 200; sin `python3` en la ruta del puente.
6. Path traversal (`GET /../Cargo.toml`) → bloqueado.
