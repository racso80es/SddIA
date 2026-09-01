---
feature_name: email-watcher-elf-fosil-1933c0a0fe2c
created: "2026-09-01"
process: bug-fix
version_implementation: "1.0.0"
items:
  - rebuild-release-elf
  - recycle-systemd-instance
  - no-main-rs-mutation
---

# Implementación — Reciclo ELF `email-watcher` (`1933c0a0fe2c`)

## Cambios

| Archivo | Cambio |
|---------|--------|
| `SddIA/daemons/email-watcher/src/main.rs` | **Intacto** (A-NO-REFORJAR-KEEPALIVE) |
| `SddIA/target/release/email-watcher` | Recompilado 2026-09-01 09:03 CEST; mtime ≥ fuente |
| Instancia systemd | Reciclada: PID 7064 → 638582 |

## Detalle

1. **ELF:** `cargo build --release -p email-watcher`. Resolutor `_sddia_resolve_daemon_binary` selecciona release fresco. Debug sigue fósil (2026-08-28); no es candidato mientras release esté fresco.
2. **Keepalive en binario:** cadena `bucle iniciado (keepalive heartbeat cada` presente en el ELF release.
3. **Reciclo:** `systemctl --user restart sddia-email-watcher@home-racso-Proyectos-SddIA.service`. Lock `started_at=2026-09-01T07:03:20Z`.
4. **Auditoría:** `daemon-heartbeat-audit` sweep `fractures_emitted: []`, `email-watcher` `missed_cycles=0`, `classification=healthy`.

## Sin cambios

- Umbrales Argos
- `spawn_heartbeat_worker` / `HEARTBEAT_TICK_SECONDS`
- Timeout IMAP / `uid_search("ALL")`
- Fagoctio apply
