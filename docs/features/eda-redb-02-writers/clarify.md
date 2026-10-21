---
feature_name: eda-redb-02-writers
created: "2026-10-21"
purpose: decisiones del corte SDDIA_EVENT_STORE
---

# Clarify — eda-redb-02-writers

- `filesystem` (ausente o desconocido) no abre `events.redb`.
- `dual` escribe el store y después el fichero. Si el store falla, no hay fichero.
- `redb` no crea JSON bajo `.events/`.
- Un `event_id` repetido en el store es idempotente (`Conflict` no aborta al escritor).
- Los testigos usan `upsert_delivery`: el par puede aparecer después del insert.
- `System_Fracture_Detected` conserva el dedupe por hash de contenido.
- El side-channel `.SddIA/daemons/state/heartbeats/` no entra en `events`. El ECST `Daemon_Heartbeat` sí, porque ya es un evento del bus.
- El proof `dlt-telemetry` sigue en disco. En `dual` o `redb` se copia el digest a `dlt_audit_seals` sin quitar IOTA.
- La lectura de despacho (`try_sweep_event`) en `redb` usa solo el store; en `dual`, la fila si existe y si no el fichero.
- El modo de los tests es un override por hilo, no `set_var` del proceso.
