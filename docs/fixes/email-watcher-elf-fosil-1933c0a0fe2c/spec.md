---
feature_name: email-watcher-elf-fosil-1933c0a0fe2c
created: "2026-09-01"
process: bug-fix
base: main
scope: email-watcher-runtime-elf
version_spec: "1.0.0"
branch_name: fix/email-watcher-elf-fosil-1933c0a0fe2c
persist_ref: docs/fixes/email-watcher-elf-fosil-1933c0a0fe2c
pbi_ref: docs/todos/pending/[FIX] email-watcher — fractura sistémica (1933c0a0fe2c).md
document_id: PBI-FIX-FRACTURE-1933c0a0fe2c
execution_id: "a8e4d437-4c8c-42a4-888b-3fd1de477883"
fracture_hash: 1933c0a0fe2c
incident_ref: "System_Fracture_Detected — 1933c0a0fe2c"
laudo: C
---

# Especificación — Reciclo ELF fósil `email-watcher` (`1933c0a0fe2c`)

## Diagnóstico (causa raíz)

| Síntoma | Evidencia |
|---------|-----------|
| `System_Fracture_Detected` `1933c0a0fe2c` | Traza: omitió 3 ciclos; `last_heartbeat=2026-08-31T07:10:19Z`; umbral=3 ≈ 90 s |
| PID vivo + host despierto | Auditor exige `pid_alive`; journal sin suspend; hermanos latiendo |
| Keepalive **ya** en fuente | `spawn_heartbeat_worker` merge `d3ef9036` (PBI `6c0db1296181`) |
| ELF en ejecución fósil | PID 7064; `SddIA/target/release/email-watcher` mtime 2026-08-26; sin cadena keepalive |
| Trigger | DNS `imap connect: Name or service not known` ~09:09 CEST; github-bridge y telegram-watcher (con keepalive) no sellaron 3 ciclos |

**Laudo C (este ciclo):** reciclar instancia para que el ELF vigente pase `_sddia_daemon_elf_fresh_vs_source`. El sello es residual de entrega, no un segundo hueco de genoma. No existe `SddIA/process/email-watcher.md`.

## Corrección

### H1 — Recompilar crate (único CA de binario)

```text
cd SddIA && cargo build --release -p email-watcher
cd SddIA && cargo test -p email-watcher
```

El ELF elegido por `_sddia_resolve_daemon_binary` debe tener `mtime ≥` fuente del crate. Preferente release (es el que ejecuta PID 7064). Debug fósil no bloquea si el resolutor elige release fresco.

### H2 — Reciclar instancia

Unidad user: `sddia-email-watcher@home-racso-Proyectos-SddIA.service` (`KillMode=process`, `Restart=always`). Tras ELF fresco: stop/start (o kill del PID de lock y dejar que systemd relance `email-watcher.sh`). Lock nuevo: `pid ≠ 7064` o `started_at` posterior. Proceso nuevo debe contener la cadena de keepalive.

### H3 — Cierre documental

PBI `1933c0a0fe2c` → `docs/todos/done/` en esta rama. `validacion.md` `global: APTO`, `pbi_archived: true`. Sin re-parche de `main.rs`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `cargo build --release -p email-watcher` OK; ELF release `mtime ≥` fuente |
| CA2 | `cargo test -p email-watcher` verdes (regresión keepalive existente; sin mutar `main.rs`) |
| CA3 | Instancia reciclada: PID ≠ 7064 o `started_at` nuevo; keepalive presente en el proceso |
| CA4 | `heartbeat-audit.json` / sweep: `email-watcher` `missed_cycles=0` post-reciclo |
| CA5 | No mutar umbrales Argos ni re-forjar `spawn_heartbeat_worker` |
| CA6 | `validacion.md` global `APTO`, `pbi_archived: true`; PBI en `docs/todos/done/` en el mismo PR |

## Alcance prohibido

| Prohibido | Motivo |
|-----------|--------|
| Re-implementar `spawn_heartbeat_worker` / cambiar `HEARTBEAT_TICK_SECONDS` | A-NO-REFORJAR-KEEPALIVE; ya cerrado `6c0db1296181` |
| Mutar umbrales para silenciar el sello | A-NO-MUTAR-UMBRALES-PARA-SILENCIAR |
| Forjar `SddIA/process/email-watcher.md` | Ontología: es daemon, no proceso |
| Timeout IMAP / `uid_search("ALL")` | PBI aparte |
| Rama `fix/email-watcher-heartbeat-keepalive` | PBI: no reutilizar |
| `SDDIA_PHAGOCYTE_APPLY=1` | Predicado `trace_before_lock` no aplica |
| Alterar `fracture_hash` / traza / `fracture_process` | A-FRACTURE-HASH-INMUTABLE |

## Corte de esta fase (Dedalo)

Diseño: `spec.md` + `plan.md`. Sin reciclo ni cargo en esta fase. Ejecución = Tekton bajo el mismo `persist_ref`.
