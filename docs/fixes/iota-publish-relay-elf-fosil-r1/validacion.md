---
feature_name: iota-publish-relay-elf-fosil-r1
created: "2026-08-30"
updated: "2026-08-30"
process: bug-fix
branch_name: fix/iota-publish-relay-elf-fosil-r1
persist_ref: docs/fixes/iota-publish-relay-elf-fosil-r1
pbi_ref: docs/todos/done/[REGRESIÓN] route-domain-event — fractura sistémica (701c77ebeab8)-R1.md
document_id: PBI-FIX-FRACTURE-701c77ebeab8-R1
global: APTO
pbi_archived: true
branch: fix/iota-publish-relay-elf-fosil-r1
checks:
  RELAY-R1-CA1: APTO
  RELAY-R1-CA2: APTO
  RELAY-R1-CA3: PENDIENTE_INSTANCIA
  RELAY-R1-CA4: APTO_CODIGO
  RELAY-R1-CA5: APTO
  RELAY-R1-CA6: APTO
  CASCADE_SPEC: APTO
  CASCADE_PLAN: APTO
  CASCADE_IMPLEMENTATION: APTO
  CASCADE_EXECUTION: APTO
  CASCADE_VALIDACION: APTO
git_changes:
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/daemons/event-watcher.sh
  - SddIA/daemons/telegram-watcher.sh
  - SddIA/daemons/github-bridge-watcher.sh
  - SddIA/daemons/event-sweeper.sh
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/scripts/qa/test-daemon-binary-resolver.sh
  - docs/fixes/iota-publish-relay-elf-fosil-r1/
  - docs/todos/done/[REGRESIÓN] route-domain-event — fractura sistémica (701c77ebeab8)-R1.md
---

# Validación — fractura `701c77ebeab8` R1 (Argos)

## Veredicto

**APTO** — aduana ELF↔fuente en resolutor compartido; launchers convergidos; drain por UUID. ELF vivo reconstruido; `/health` 200 post-restart. Ola 0/1 no reabiertas.

CA3 (nuevo sello DLT) y vaciado físico de la cola son del siguiente `route-domain-event` con relay sano; no bloquean el código de esta rama.

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| RELAY-R1-CA1 | APTO | ELF debug 2026-08-30 19:57; `curl /health` 200; `server.mjs` vivo |
| RELAY-R1-CA2 | APTO | journal post-restart: spawn sin kill en el mismo segundo |
| RELAY-R1-CA3 | PENDIENTE_INSTANCIA | requiere lote Merkle post-fix en bus |
| RELAY-R1-CA4 | APTO_CODIGO | 4 tests `resolve_reanchor_*`; cola instancia no drenada a mano |
| RELAY-R1-CA5 | APTO | `post_grace_refused_kills_and_ticks_degraded` 7/7; ELF vivo = HEAD |
| RELAY-R1-CA6 | APTO | `test-daemon-binary-resolver.sh` OK; 6 launchers con resolutor |
