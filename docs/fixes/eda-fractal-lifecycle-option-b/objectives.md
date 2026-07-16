---
feature_name: eda-fractal-lifecycle-option-b
created: "2026-07-16"
process: bug-fix
branch_name: fix/eda-fractal-lifecycle-option-b
persist_ref: docs/fixes/eda-fractal-lifecycle-option-b
---

# Objetivos — eda-fractal-lifecycle-option-b

## Misión

PBI-EDA-FRACTAL-LIFECYCLE-B: Opción B — domain fractal purge_after=true; stamp delivery_state por suscriptor; sweeper fractal unlink terminal; telegram offset ACK-first + seen(update_id); event-watcher no reintenta eco si side-effect success.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
