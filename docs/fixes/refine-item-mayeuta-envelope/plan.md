---
feature_name: refine-item-mayeuta-envelope
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/refine-item-mayeuta-envelope
tracker_ref: AUT-37
execution_id: "1f6f7316-ed62-4ded-922a-6b95fbb9d44a"
phases:
  - id: T1
    title: Copiar data.message a entry.error
  - id: T2
    title: Transcript en el fallo del CLI y tests
  - id: T3
    title: Nota evolution AUT-37
---

# Plan — AUT-37

Rama `Fix_Events_Multiple` @ `28a16d61`. Un commit por tarea. El motor, con git activo, habría creado `fix/refine-item-mayeuta-envelope`. No se hace ese checkout.

AUT-34 permanece en `docs/fixes/refine-item-mayeuta-blocked/`. Este plan no lo reabre.

## T1 — Sobre

En `SddIA/engine/execute-process/src/engine/agent_runtime.rs`, después de copiar `data.message` y `body.error`:

- Si el status normalizado es `blocked` o `failed`, `entry.error` está vacío y `data.message` tiene texto, asignar ese texto a `entry.error`.

Test junto a `configured_cli_can_block`:

- stdout `{"success":true,"data":{"status":"blocked","message":"motivo-x"}}` → `entry.error` y `entry.message` son `motivo-x`.
- El mock actual sin mensaje sigue con `entry.error` ausente.

## T2 — Transcript

En `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py`, en el camino `not ok` de `run_agent_phase`, llamar a `write_workspace_transcript` con el stdout o, si viene vacío, con el mensaje de error.

En `test_kalma2_runtime_timeout.py`:

- `write_workspace_transcript` con `persist_ref` vacío y `workspace_path` escribe `transcript.md`.
- Con `persist_ref` no vacío no escribe.
- Un doc de fase `refine-item` cuyo transcript contiene `Veredicto: blocked`, persist vacío y workspace temporal, deja `transcript.md` tras la misma función que usa el camino ok (`write_workspace_transcript`). No se arranca el CLI.

## T3 — Evolution

Nota nueva en `SddIA/evolution/` con UUID, vinculada a AUT-37 y a los dos ficheros de código. No menciona `PARENT_REF_REQUIRED`.

## Hecho que este plan no repite

Prompt de handoff, workspace MCP, parser `**blocked**`, fallback del handler y `System_Fracture_Detected` (AUT-34). Relevo `SDDIA_AGENT_RELAY_IDE=1`. La spec ya registra que Recepción de `forge-pbi` no invoca Mayeuta.
