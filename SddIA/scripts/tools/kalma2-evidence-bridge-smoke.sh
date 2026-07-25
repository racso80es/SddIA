#!/usr/bin/env bash
# Smoke T3 — Evidence Bridge R1/R2/R3 (PPR #136 residual agent-runtime).
# No inventa APTO: MOCK → NO_APTO; path nativo / subprocess → APTO o NO_APTO explícito.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT"
PY="$ROOT/SddIA/scripts/tools/kalma2-agent-runtime-cursor.py"
PERSIST="docs/features/kalma2-agent-runtime-cursor-f3-km-residual"
SMOKE_DIR="$ROOT/$PERSIST/.tmp/evidence-bridge-smoke"
mkdir -p "$SMOKE_DIR"

python3 - <<'PY' "$ROOT" "$PY" "$PERSIST" "$SMOKE_DIR"
import json, os, sys, tempfile, shutil
from pathlib import Path

root, py_path, persist, smoke = map(Path, sys.argv[1:5])
sys.path.insert(0, str(py_path.parent))

# Import functions from prosthesis without executing main
import importlib.util
spec = importlib.util.spec_from_file_location("kalma2_rt", py_path)
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)

lab = smoke / "lab"
if lab.exists():
    shutil.rmtree(lab)
lab.mkdir(parents=True)

# --- AC-TRUTH / MOCK: no inventa APTO ---
os.environ["SDDIA_AGENT_RUNTIME_MOCK"] = "1"
doc_mock = {
    "operation": "AGENT_PHASE",
    "process_name": "feature",
    "phase_name": "Verificación",
    "agents": ["argos"],
    "persist_ref": str(lab.relative_to(root)) if str(lab).startswith(str(root)) else str(lab),
    "repo_root": str(root),
}
# Use absolute persist under lab for isolation
persist_rel = str(lab.relative_to(root))
doc_mock["persist_ref"] = persist_rel
ev = mod.materialize_runtime_evidence(root, persist_rel, doc_mock)
assert ev.get("evidence_materialized") is False, ev
assert ev.get("notes") == "mock", ev
assert ev.get("TECH_FORMAL_EXECUTE_PROCESS") == "NO_APTO", ev
assert ev.get("GIT_EVIDENCE_VIA_GIT_MANAGER") == "NO_APTO", ev
print("OK mock-no-apto")

# --- Native flags → APTO sin subprocess ---
os.environ.pop("SDDIA_AGENT_RUNTIME_MOCK", None)
doc_native = {
    "phase_name": "Verificación",
    "agents": ["argos"],
    "git_manager_invoked": True,
    "formal_execute_process": True,
    "runtime_evidence": {
        "git_manager_invoked": True,
        "formal_execute_process": True,
        "git_evidence_digest": "deadbeef",
        "formal_evidence_detail": "ok",
    },
}
ev2 = mod.materialize_runtime_evidence(root, persist_rel, doc_native)
assert ev2.get("git_manager_invoked") is True, ev2
assert ev2.get("formal_execute_process") is True, ev2
assert ev2.get("TECH_FORMAL_EXECUTE_PROCESS") == "APTO", ev2
assert ev2.get("GIT_EVIDENCE_VIA_GIT_MANAGER") == "APTO", ev2
assert ev2.get("source") == "native_state", ev2
print("OK native-flags-apto")

# --- Prompt KM scoped ---
prompt = mod.build_prompt(
    {
        "process_name": "feature",
        "phase_name": "Verificación",
        "agents": ["argos"],
        "persist_ref": persist_rel,
    },
    ev2,
)
assert "RBAC_AUTHORING_KM_POLICY" in prompt
assert "docs/todos/**" in prompt
assert "Forja Core" in prompt
assert "TECH_FORMAL_EXECUTE_PROCESS" in prompt
print("OK argos-prompt-km-scoped")

# --- AGENT_PHASE mock gate: runtime_evidence en stdout ---
payload = {
    "operation": "AGENT_PHASE",
    "process_name": "feature",
    "phase_name": "Verificación",
    "agents": ["argos"],
    "persist_ref": persist_rel,
    "repo_root": str(root),
}
import subprocess
out = subprocess.check_output(
    [sys.executable, str(py_path)],
    input=json.dumps(payload).encode(),
    cwd=str(root),
    env={**os.environ, "SDDIA_AGENT_RUNTIME_MOCK": "1"},
)
body = json.loads(out.decode())
assert body["success"] and body["data"]["status"] == "executed"
re = body["data"].get("runtime_evidence") or {}
assert re.get("notes") == "mock"
assert re.get("GIT_EVIDENCE_VIA_GIT_MANAGER") == "NO_APTO"
handoff = root / persist_rel / "_agent_handoff.md"
text = handoff.read_text(encoding="utf-8")
assert "### Runtime evidence (machine)" in text
assert "schema: kalma2-agent-runtime-evidence/v1" in text
print("OK agent-phase-mock-evidence-block")

# --- Subprocess path (host): git-manager + formal si binarios existen ---
os.environ.pop("SDDIA_AGENT_RUNTIME_MOCK", None)
doc_sub = {
    "phase_name": "Verificación",
    "agents": ["argos"],
}
ev3 = mod.materialize_runtime_evidence(root, persist_rel, doc_sub)
print("SUBPROCESS_RESULT", json.dumps({
    "source": ev3.get("source"),
    "git": ev3.get("GIT_EVIDENCE_VIA_GIT_MANAGER"),
    "formal": ev3.get("TECH_FORMAL_EXECUTE_PROCESS"),
    "notes": ev3.get("notes"),
    "evidence_materialized": ev3.get("evidence_materialized"),
}, ensure_ascii=False))
# L-TRUTH: no assert APTO si host sin cápsula; solo que no fabricó sin source
if ev3.get("GIT_EVIDENCE_VIA_GIT_MANAGER") == "APTO":
    assert ev3.get("git_manager_invoked") is True
    assert ev3.get("source") in ("prosthesis_subprocess", "native_state")
    print("OK subprocess-git-apto")
else:
    assert ev3.get("GIT_EVIDENCE_VIA_GIT_MANAGER") == "NO_APTO"
    print("OK subprocess-git-explicit-no-apto")

if ev3.get("TECH_FORMAL_EXECUTE_PROCESS") == "APTO":
    assert ev3.get("formal_execute_process") is True
    print("OK subprocess-formal-apto")
else:
    print("OK subprocess-formal-explicit-no-apto")

print("EVIDENCE_BRIDGE_SMOKE_OK")
PY

echo "=== agent_runtime unit (forward state) ==="
cd "$ROOT/SddIA"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target}" cargo test -p execute-process --lib \
  runtime_evidence_forwards_native_state_flags -- --test-threads=1
echo "SMOKE T3 OK"
