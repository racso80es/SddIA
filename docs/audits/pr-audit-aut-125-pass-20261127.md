---
audit_event_reference: "bb83a08c-9a2a-41e8-becf-50d6fcd5786b"
event_id: "d2f96dc4-32e8-498f-8bed-4d10d24d050b"
event_uri: "eda://domain/d2f96dc4-32e8-498f-8bed-4d10d24d050b"
supersedes_audit_event_reference: "7a3e390d-86f8-4167-893c-2875c67bdb23"
previous_audit_report: "docs/audits/pr-audit-aut-125-ratchet-final-20261127.md"
target_entity_id: "refactor/gate-anti-anemia-entidades-dominio"
pr_number: 480
pr_url: "https://github.com/racso80es/SddIA/pull/480"
tracker_ref: "AUT-125"
persist_ref: "docs/refactors/gate-anti-anemia-entidades-dominio"
contract_ref: "SddIA/events/domain/pull-request-audited.md"
event_type: "PullRequest_Audited"
emitter_action: "emit-pr-audited-event"
date: "2026-11-27"
ppr_execution_id: "982404c8-5fc6-49c1-94ac-4a66ee48dc56"
verdict: "APTO"
resolution: "PASS"
---

# PullRequest_Audited PASS — AUT-125

La pasada `pull-request-review` en perfil `full` `982404c8-5fc6-49c1-94ac-4a66ee48dc56` (Argos en triaje documental y veredicto, Cerbero determinista) dejó `validacion.md` en `global: APTO`. El YAML de esa escritura no parseaba (`verify-process-integrity: OK` sin comillar); se citó el escalar para que el sello sea legible, sin cambiar el dictamen.

El `FLAG` `7a3e390d-86f8-4167-893c-2875c67bdb23` queda superado en este registro: la atestación `docs-only` `aa13ad63` se eliminó, `dlq-kaizen-alerts.json` volvió al árbol limpio, y `validacion.md` lleva `diff_seal` con `validated_head_sha` `a062b1ede966c64157648db71a399b4b85c62f8f`.

```json
{
  "audit_event_reference": "bb83a08c-9a2a-41e8-becf-50d6fcd5786b",
  "target_entity_id": "refactor/gate-anti-anemia-entidades-dominio",
  "resolution": "PASS",
  "tracker_ref": "AUT-125",
  "violated_rules": []
}
```
