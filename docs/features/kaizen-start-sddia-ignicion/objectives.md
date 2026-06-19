---
feature_name: kaizen-start-sddia-ignicion
created: "2026-06-19"
process: feature
branch_name: feat/kaizen-start-sddia-ignicion
persist_ref: docs/features/kaizen-start-sddia-ignicion
pbi_ref: docs/todos/pending/[Kaizen] start-sddia — ignición ecosistema Centinelas y Kalma2.md
uuid: aad51b93-198b-4cf7-b6a8-195d7f988fb5
---

# Objetivos — Kaizen start-sddia ignición ecosistema

## Misión

Restablecer el **arranque unificado del nodo SddIA** mediante `start-sddia.sh`: Centinelas (Sistema Nervioso EDA) + puente Kalma2 operativos con una sola orden, rutas alineadas al SSOT genoma/instancia y apagado limpio verificable.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| `start-sddia.sh` apuntaba a `.SddIA/scripts/daemons/` | Directorio inexistente; centinelas nunca arrancaban |
| Lanzadores canónicos en `SddIA/scripts/daemons/` | SSOT `cumulo.paths.json`: genoma sin punto |
| Puente Kalma2 en `.SddIA/client/sddia-client-bridge.py` | Ruta de instancia correcta |
| `interfaces/kalma2/README.MD` cita `./start-sddia.sh` | Script roto bloqueaba el flujo documentado del PoC |
| Binarios Rust ya compilados en `SddIA/target/release/` | Infraestructura lista; solo fallaba el cableado |

## Objetivos medibles

| ID | Objetivo | Criterio | Estado |
|----|----------|----------|--------|
| **O1** | Rutas genoma/instancia correctas | Centinelas vía `SddIA/scripts/daemons/`; bridge vía `.SddIA/client/` | ✅ |
| **O2** | Verificación post-arranque | `_start_daemon` confirma proceso vivo; fallo si faltan obligatorios | ✅ |
| **O3** | Health check Kalma2 | `curl` a `GET /` antes de declarar operativo | ✅ |
| **O4** | Apagado limpio | `cleanup`: jobs + `pkill -x` centinelas + bridge | ✅ |
| **O5** | Documentación operativa | `start-sddia.md` con secuencia, requisitos y diagnóstico | ✅ |
| **O6** | Validación en caliente | 4/4 centinelas + HTTP 200 Kalma2 + SIGTERM cleanup | ✅ |

## Entregables

| Artefacto | Descripción |
|-----------|-------------|
| `start-sddia.sh` | Script de ignición corregido y endurecido |
| `start-sddia.md` | Proceso documentado (frontmatter atómico) |
| `docs/features/kaizen-start-sddia-ignicion/` | Trazabilidad Kaizen del cambio |

## Deuda conocida (fuera de alcance)

- **EDA routing post-migración Rust:** `event-watcher` emite errores al enrutar eventos pendientes que aún invocan `execute-process.py` (Python retirado). No impide arranque; requiere Kaizen/fix independiente en centinelas.
- **Ruido stdout:** centinelas en foreground inundan la terminal; redirección a `.SddIA/daemons/logs/` no incluida en este Kaizen.

## No objetivos

- Crear entidad `process` en genoma vía `entity-manager` (artefacto de instancia en raíz).
- Sustituir `_run_daemon.sh` / entrypoints Rust nativos.
- Integrar Cerbero, Karma2Token o despliegue remoto en el script.

## Ley aplicada

- `SddIA/core/cumulo.paths.json` (SSOT rutas)
- `features-documentation-pattern` v1.2.0
- Proceso `feature` v1.3.0
