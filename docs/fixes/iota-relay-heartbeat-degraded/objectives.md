---
feature_name: iota-relay-heartbeat-degraded
created: "2026-08-30"
process: bug-fix
branch_name: fix/iota-relay-heartbeat-degraded
persist_ref: docs/fixes/iota-relay-heartbeat-degraded
pbi_ref: docs/todos/pending/[FIX] iota-publish-relay — Ola 1 latido degradado (701c77ebeab8).md
execution_id: "39567569-6670-42d6-8174-116954dda036"
---

# Objetivos — iota-relay-heartbeat-degraded

## Misión

Ola 1 fractura 701c77ebeab8 (PBI-FIX-FRACTURE-701c77ebeab8-OLA1). I1/I2 vigentes post Ola 0 (PR #233): emit_heartbeat privada fuerza status=alive; record_heartbeat_at ignora payload.status y classification=healthy; color_daemon green solo por last_heartbeat_at+missed<3. Omit-tick de CA4 viola daemons-contract §6.1 (emisión periódica; status alive|degraded|shutting_down). Laudo: tick/emit_heartbeat con status inyectable; audit persiste degraded (classification≠healthy, missed_cycles=0); espejo no green (reason heartbeat_degraded); relay post-gracia /health false → tick(degraded)+kill. Gates RELAY-CA9–CA12. Mandato: forja spec+plan y detente tras commit. Sin parche de código, sin delivery-close-cycle.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
