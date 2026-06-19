---
feature_name: kalma2-bridge-rust
created: "2026-06-19"
process: feature
branch_name: feat/kalma2-bridge-rust
persist_ref: docs/features/kalma2-bridge-rust
pbi_ref: docs/todos/pending/[FEATURE] kalma2-bridge — puente HTTP nativo Rust.md
uuid: 2afb1f2f-667c-4c39-ae5f-7bd7f626c7e2
status: executed
---

# Objetivos — kalma2-bridge (puente HTTP nativo Rust)

## Misión

Reemplazar `.SddIA/client/sddia-client-bridge.py` por binario nativo `kalma2-bridge` (Rust). Aduana inerte de E/S: sirve bundle estático y delega `POST /api/interact` al orquestador nativo sin lógica de negocio.

## Punto objetivo (añadido)

> **O-PUENTE-RUST:** Materializar `kalma2-bridge` como binario Rust ultraligero que sustituya al puente Python, completando la erradicación de runtime Python en la superficie de interacción Kalma2.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Crate `kalma2-bridge` | Miembro del workspace Rust; compila `cargo build -p kalma2-bridge` | ✅ |
| **O2** | Servidor estático | `GET /` y rutas relativas sirven `interfaces/kalma2/{index.html,app.js,style.css}` en `127.0.0.1:8765` | ✅ |
| **O3** | Enrutador de voluntad | `POST /api/interact` con `{"prompt": string}` → subproceso síncrono orquestador → stdout JSON transparente | ✅ |
| **O4** | Ceguera espacial | Sin análisis semántico, sin lógica de negocio, sin eventos asíncronos | ✅ |
| **O5** | Paridad operativa | `start-sddia.sh` y `start-sddia.md` invocan el binario, no `python3` | ✅ |
| **O6** | Poda Python | `sddia-client-bridge.py` retirado tras paridad verde | ✅ |

## Restricción de ceguera espacial (invariante)

El crate NO interpreta el `prompt`, NO ramifica por contenido, NO persiste estado, NO implementa colas/EDA. Solo: bytes entrada → subproceso → bytes salida.

## No objetivos

- Reescribir `kalma2-interact` (proceso genoma intacto).
- Autenticación, Cerbero, Karma2Token, TLS, despliegue remoto.
- Migrar el bundle frontend (`interfaces/kalma2/` sin cambios).
- WebSockets/SSE/async.

## Ley aplicada

- `.cursorrules` §4 (cápsulas Rust, JSON stdin/stdout), §5 (agnosticismo Core)
- `SddIA/norms/external-ai-constraints.md` (RAW Kernel DA-4)
- `SddIA/core/cumulo.paths.json` (SSOT rutas)
- `features-documentation-pattern` v1.2.0 / proceso `feature` v1.3.0
