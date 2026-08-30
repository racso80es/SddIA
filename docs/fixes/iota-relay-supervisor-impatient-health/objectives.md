---
feature_name: iota-relay-supervisor-impatient-health
created: "2026-08-30"
process: bug-fix
branch_name: fix/iota-relay-supervisor-impatient-health
persist_ref: docs/fixes/iota-relay-supervisor-impatient-health
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (701c77ebeab8).md
execution_id: "dd623714-7946-4eef-bc25-6dd67f3c2ce3"
---

# Objetivos — iota-relay-supervisor-impatient-health

## Misión

Fractura 701c77ebeab8: merkle-batch-preseal → iota-relay-unreachable connection refused 8787 (os error 111). Causa raíz: supervisor iota-publish-relay mata al hijo Node en el mismo tick del spawn (probe_health refused instantáneo; timeout 1500 ms no aplica). Espejo green porque tick() emite alive siempre. Ola 0 (laudo v1.2.0): SOLO SddIA/daemons/iota-publish-relay/src/main.rs. RELAY-CA1 gracia GRACE_SECS=10 post-spawn exitoso (child_spawned_at, no last_restart). RELAY-CA4 omitir centinela.tick() si /health false fuera de gracia. Conservar kill+respawn post-gracia. Tests unitarios fn pura. Fuera de Ola 0: runtime degraded, CA5–CA7, genoma .md. Mandato: forja spec+plan y detente tras commit. Sin ejecución de código, sin delivery-close-cycle.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
