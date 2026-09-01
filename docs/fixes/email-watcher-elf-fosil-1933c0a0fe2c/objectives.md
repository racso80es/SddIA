---
feature_name: email-watcher-elf-fosil-1933c0a0fe2c
created: "2026-09-01"
process: bug-fix
branch_name: fix/email-watcher-elf-fosil-1933c0a0fe2c
persist_ref: docs/fixes/email-watcher-elf-fosil-1933c0a0fe2c
pbi_ref: docs/todos/pending/[FIX] email-watcher — fractura sistémica (1933c0a0fe2c).md
execution_id: "a8e4d437-4c8c-42a4-888b-3fd1de477883"
---

# Objetivos — email-watcher-elf-fosil-1933c0a0fe2c

## Misión

Laudo C PBI-FIX-FRACTURE-1933c0a0fe2c: reciclar ELF fósil de email-watcher (PID 7064, release 2026-08-26) para que mtime ≥ fuente con keepalive ya mergeado en 6c0db1296181. Prohibido re-forjar spawn_heartbeat_worker. Compilar crate, reciclar instancia, verificar keepalive en proceso nuevo y missed_cycles=0, archivar PBI en el mismo PR. Rama distinta de fix/email-watcher-heartbeat-keepalive.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
