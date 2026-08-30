---
feature_name: iota-publish-relay-elf-fosil-r1
created: "2026-08-30"
process: bug-fix
branch_name: fix/iota-publish-relay-elf-fosil-r1
persist_ref: docs/fixes/iota-publish-relay-elf-fosil-r1
pbi_ref: docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (701c77ebeab8)-R1.md
execution_id: "e0adfc87-d73f-4c08-8413-3d446823e5f6"
---

# Objetivos — iota-publish-relay-elf-fosil-r1

## Misión

R1 701c77ebeab8: ELF fósil iota-publish-relay (debug 28-ago) + cola re-anclaje huérfana. Laudo v1.2.0: aduana ELF↔fuente en _sddia_resolve_daemon_binary + convergencia launchers; drain por UUID en processed/dead-letter; ELF vivo porta Ola 0+1. No reabrir gracia ni Ola 1.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
