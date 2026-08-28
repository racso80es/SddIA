---
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL
uuid: "6d64bcc7-b677-4c43-b239-928e279d2a04"
title: "[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub"
format: markdown
version: "1.0.0"
created: "2026-08-28"
status: "pendiente"
priority: "alta"
process: bug-fix
type: kaizen
dispatch: false
suggested_branch: fix/kaizen-aduana-evolution-local
incident_ref: "PR #209 — wasi-runtime-smoke rojo en evolution gate (delta) con 18 findings EVOL_MATERIAL_UNREGISTERED; ninguna capa local lo detectó"
friction_ids:
  - F-HOOKS-NO-INSTALADOS
  - F-IF-TOUCHED-CONDICION-INVERTIDA
  - F-IMPACT-ASSESSMENT-STUB
  - F-DCC-SIN-GATE-EVOLUTION
depends_on:
  - PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS
related:
  - SddIA/scripts/qa/git-hooks/install-hooks.sh
  - SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  - SddIA/tools/sddia-qa/src/gate_evolution.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - .github/workflows/sddia-index-qa.yml
source_audit: "Auditoría de cuatro capas tras rojo PR #209; .git/hooks solo .sample; --if-touched salta cuando no toca evolution"
---

# [KAIZEN] Aduana evolution local inexistente

## 1. Falla

PR #209 llegó a CI con 18 mutaciones bajo `SddIA/` sin entrada en `SddIA/evolution/`.
Cuatro capas diseñadas para impedirlo no operaron:

1. **Hooks no instalados** — `.git/hooks/` solo contiene `.sample`.
2. **`--if-touched` invertido** — pre-push salta el gate cuando el diff no toca `evolution/`, no cuando toca material sin registrar.
3. **`delivery-impact-assessment` stub** — devuelve `impact: none` sin mirar el diff.
4. **DCC sin `gate-evolution`** — el ciclo verifica EDA pero no evolution; CI sí.

## 2. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AEL-CA1 | Verificación de hooks instalados con remedio documentado |
| AEL-CA2 | `--if-touched` no salta cuando hay material `SddIA/` sin evolution |
| AEL-CA3 | Gate local `--range` ≡ job CI delta |
| AEL-CA4 | `capsule_delivery_impact_assessment` calcula diff real |
| AEL-CA5 | DCC ejecuta `gate-evolution --range` antes del push |
| AEL-CA6 | Tests unitarios de `--if-touched` |
| AEL-CA7 | Smoke: mutación sin evolution bloquea DCC |
