#!/usr/bin/env bash
# MAN-43 O13 — adopción de identidad x-sddia en los capability-contracts por cauce oficial.
# Una invocación `entity-manager` por contrato (sin atajos de IDE). Versión = catálogo de
# `capability-taxonomy.md`; contratos auxiliares fuera del catálogo (di.binding, llm-registry) = 1.0.0.
#
# Uso (desde la raíz del repo):  RUNNER=./sddia-run.sh docs/features/.../_lab/o13-adopt-all.sh [contrato ...]
# Sin argumentos adopta los 16 contratos de C2 (eda.event_manager.v1 se adopta aparte, antes de su parche).
set -euo pipefail
RUNNER="${RUNNER:-./sddia-run.sh}"
TAXONOMY="SddIA/library/norms/capability-taxonomy.md"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

DEFAULT=(agenda.persist audit.compliance bus.route channel.ingest di.binding doc.closure fs.persist
  gov.rbac llm.infer llm.interact llm-registry memory.pref_query memory.pref_write proc.git_sync
  qa.probe tracker.lifecycle)
TARGETS=("$@")
[[ ${#TARGETS[@]} -gt 0 ]] || TARGETS=("${DEFAULT[@]}")

for name in "${TARGETS[@]}"; do
  python3 - "$name" "$TAXONOMY" "$TMP/$name.json" <<'PY'
import json, re, sys
name, taxonomy, out = sys.argv[1:4]
version = "1.0.0"  # auxiliares fuera de catálogo
for line in open(taxonomy, encoding="utf-8"):
    m = re.match(r"\|\s*`[^`]+`\s*\|\s*`([^`]+)`\s*\|\s*([0-9]+\.[0-9]+\.[0-9]+)\s*\|", line)
    if m and m.group(1) == name:
        version = m.group(2)
json.dump({
    "entity_class": "capability-contract",
    "entity_name": name,
    "lifecycle_operation": "update",
    "semantic_seed": {"adopt": True, "schema_version": version},
}, open(out, "w"))
print(f"[o13-adopt] {name} -> x-sddia.version {version}")
PY
  resp="$($RUNNER --process entity-manager --inputs-file "$TMP/$name.json" 2>/dev/null | tail -n 1)"
  python3 - "$name" "$resp" <<'PY'
import json, sys
name, resp = sys.argv[1:3]
j = json.loads(resp)
h = (j.get("data") or {}).get("handoff") or {}
ok = j.get("success") is True and h.get("handoff_entity_uuid")
print(f"[o13-adopt] {name}: success={j.get('success')} event_type={h.get('event_type')} uuid={h.get('handoff_entity_uuid')}")
sys.exit(0 if ok else 1)
PY
done
