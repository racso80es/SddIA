---
feature_name: eda-outbound-native-remediation
created: "2026-07-13"
process: bug-fix
branch_name: fix/eda-outbound-native-remediation
persist_ref: docs/fixes/eda-outbound-native-remediation
---

# Objetivos — eda-outbound-native-remediation

## Misión

PBI PBI-EDA-OUTBOUND-NATIVE-REMEDIATION: remediar los dead-letters deterministas de cumulo.iota-immutable-publisher y argos.send-telegram-notification. Las cápsulas WASI carecen de red; planificar una ejecución nativa con capacidades explícitas, configuración inyectada y pruebas de laboratorio, sin purgar dead-letters históricos.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
