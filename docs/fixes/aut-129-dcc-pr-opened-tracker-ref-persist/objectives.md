---
feature_name: aut-129-dcc-pr-opened-tracker-ref-persist
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-129-dcc-pr-opened-tracker-ref-persist
persist_ref: docs/fixes/aut-129-dcc-pr-opened-tracker-ref-persist
pbi_ref: AUT-129
tracker_ref: AUT-129
execution_id: "f491f6fc-fa55-488a-8299-742b816a254c"
---

# Objetivos — aut-129-dcc-pr-opened-tracker-ref-persist

## Misión

AUT-129: DCC milestone PrOpened / build_cycle_payload debe resolver tracker_ref desde persist_ref + objectives.md sin pbi_ref en stdin (paridad resolve_tracker_ref_for_lifecycle / attach_tracker_ref_from_persist).

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
