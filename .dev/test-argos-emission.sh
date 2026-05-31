#!/bin/bash
python SddIA/scripts/qa/execute-action.py --action emit-domain-mutation --inputs "$(python -c '
import json
with open("mock-argos-output.json") as f:
    mock_data = json.load(f)
print(json.dumps({
    "entity_class": "event",
    "lifecycle_operation": "create",
    "entity_uuid": mock_data["audit_event_reference"],
    "entity_name": "pull-request-audited",
    "version": "1.0.0",
    "hash_signature_new": "sha256:dummy",
    "hash_signature_old": None,
    "changes_summary": json.dumps(mock_data),
    "emitter_agent": "argos",
    "origin_topology": "core"
}))
')"
