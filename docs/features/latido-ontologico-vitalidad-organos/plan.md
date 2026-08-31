---
feature_name: latido-ontologico-vitalidad-organos
created: "2026-08-31"
process: feature
phases:
  - contrato-emisores
  - censo-kalma2-muerte
  - sonda-vitalidad
pbi_ref: docs/todos/pending/[OPERATIVO] Latido Ontológico (System Heartbeat).md
execution_id: "cb141830-b5e3-4b9e-904d-014922254734"
---

# Plan — latido-ontologico-vitalidad-organos

## Fase 1 — Contrato de emisores

1. Habilitar `markdown_body_replacements` en `run_event_forge` (update conserva uuid).
2. `entity-manager` update `daemon-heartbeat`: lista de 6 stems actuales (+ `kalma2-bridge` en el mismo ciclo tras create, o segundo update).
3. Recalcular `hash_signature` vía forja.

## Fase 2 — Censo interfaz + muerte audible

1. Brazo `daemon` en `creator_inputs_from_entity` (hoy `other` aborta).
2. `entity-manager` create `kalma2-bridge`.
3. `kalma2-bridge`: dep `sddia-daemon-runtime` + `ctrlc`; bootstrap + hilo `tick`; shutdown en SIGINT/SIGTERM.
4. `audit_running_daemon`: rama PID muerto → fractura. Test con lock y PID inexistente.
5. `entity-manager` update `daemon-heartbeat` para incluir `kalma2-bridge` si Fase 1 no lo cubrió.

## Fase 3 — Invariantes + HTTP runtime

1. `entity-manager` create event `system-vitality-probed`.
2. `entity-manager` create process `system-vitality-probe` (core, `quality-assurance`).
3. Handler nativo: 4 sondas; ECST telemetry; fractura en rojo; estado idempotente.
4. `event-sweeper`: parseo cadencia + invoke. Update genoma `vitality-probe-sweep` vía replacements.
5. Units: parseo piso 30; sonda cerbero con fichero ausente; auditor PID muerto.
6. `cargo test -p execute-process` (filtros heartbeat + vitality) y `cargo test -p kalma2-bridge`.

## Fase 4

No abrir. Sin medición de peaje.

## Orden de aduana

Rebuild `execute-process` tras parches de forja **antes** de invocar `entity-manager`. Evolution `{uuid}.md` al cerrar material. DCC al final.
