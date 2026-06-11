# -*- coding: utf-8 -*-
"""REGISTRY de cápsulas físicas y orquestación de fases (laboratorio SddIA)."""

from __future__ import annotations

import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from env_loader import load_hierarchical_env
from execute_process_core import (
    delegates_are_only_agents,
    load_process_def,
    parse_frontmatter,
    phase_invocations_index,
    validate_process_inputs,
)

from execute_process_forges import FORGE_BY_ENTITY_CLASS
from eda_bus_utils import (
    build_process_execution_completed_event,
    build_raw_execution_finished_event,
    find_existing_domain_event,
    infer_persist_ref_from_branch,
    ensure_event_bus_topology,
    load_eda_bus,
    write_fractal_event,
)
from ecst_validation import validate_domain_mutation_event
from workspace_utils import (
    bootstrap_process_workspace,
    materialize_child_workspace,
    resolve_documentation_features_path,
    resolve_documentation_fixes_path,
    sync_workspace_context,
)

try:
    import yaml
except ImportError:
    yaml = None  # type: ignore

SCRIPT = Path(__file__).resolve()
EXECUTE_PROCESS_CLI = SCRIPT.parent / "execute-process.py"
EXECUTE_ACTION_CLI = SCRIPT.parent / "execute-action.py"
AUDIT_EDA_CLI = SCRIPT.parent / "audit-entity-eda-coverage.py"
AUDIT_DOC_PARITY_CLI = SCRIPT.parent / "audit-doc-parity.py"
_GH_PR_URL_RE = re.compile(r"https://github\.com/[^\s/]+/[^\s/]+/pull/\d+", re.I)

THERMODYNAMIC_EXEMPT = frozenset(
    {
        "route-domain-event",
        "route-telemetry",
        "route-orchestration",
        "route-domain",
        "telemetry-batch-stub",
        "radamanto-batch",
        "cerbero-governance-react",
        "fix-tool-process",
    }
)

CHAOS_AUDIT_PROCESSES = frozenset(
    {
        "audit-thermodynamic-toll-failsoft",
        "audit-telemetry-compliance-breach",
        "audit-sandbox-isolation-rbac",
    }
)

CHAOS_OFFENSIVE_TOOLS = frozenset({"io-choke", "schema-corruptor", "sandbox-breacher"})

CHAOS_TOOL_SCRIPTS: dict[str, Path] = {
    "io-choke": SCRIPT.parent.parent.parent / "tools" / "io-choke" / "io-choke.wasm",
    "schema-corruptor": SCRIPT.parent.parent.parent / "tools" / "schema-corruptor" / "schema-corruptor.wasm",
    "sandbox-breacher": SCRIPT.parent.parent.parent / "tools" / "sandbox-breacher" / "sandbox-breacher.wasm",
}

_THERMODYNAMIC_EMERGENCY_PREFIX = "[THERMODYNAMIC-TOLL-EMERGENCY]"

_ACTIVE_CAPSULE_CAPTURE_STATE: dict[str, Any] | None = None

ROUTE_FRACTAL_HANDLERS: dict[str, str] = {
    "route-telemetry": "route_telemetry_event",
    "route-orchestration": "route_orchestration_event",
    "route-domain": "route_domain_fractal_event",
}

CREATOR_BY_CLASS: dict[str, str] = {
    "skill": "skill-creator",
    "process": "process-creator",
    "agent": "agent-creator",
    "tool": "tool-creator",
    "action": "action-creator",
    "norm": "norm-creator",
    "codex": "codex-creator",
    "event": "event-creator",
    "suite": "suite-creator",
}

DIR_BY_CLASS: dict[str, str] = {
    "skill": "SddIA/skills",
    "process": "SddIA/process",
    "agent": "SddIA/agents",
    "tool": "SddIA/tools",
    "action": "SddIA/actions",
    "norm": "SddIA/library/norms",
    "codex": "SddIA/library/codexes",
    "event": "SddIA/events",
    "suite": "SddIA/suites",
}

TRINITY_EVENT_FAMILIES = frozenset({"telemetry", "orchestration", "domain"})

PILOT_ENTITY_CLASSES = frozenset({
    "skill", "event", "process", "agent", "tool", "action", "norm", "codex", "suite",
})

# Cápsulas action:* con handler físico en execute-action.py
CAPSULE_ACTION_REGISTRY: dict[str, str] = {
    "action:emit-domain-mutation": "emit-domain-mutation",
    "action:emit-pr-merged-event": "emit-pr-merged-event",
    "action:emit-pr-audited-event": "emit-pr-audited-event",
    "action:emit-pr-presented-event": "emit-pr-presented-event",
}


def _load_cumulo(repo: Path) -> dict[str, Any]:
    return json.loads((repo / "SddIA" / "core" / "cumulo.paths.json").read_text(encoding="utf-8"))


def _parse_crypto_envelope(out: dict[str, Any]) -> Any:
    if isinstance(out.get("data"), dict) and "result" in out["data"]:
        return out["data"]["result"]
    return out.get("result")


def _invoke_crypto_native(repo: Path, payload: dict[str, Any]) -> Any:
    """Fallback laboratorio: cryptography-manager.py cuando WASI/wasmtime no disponible."""
    py_script = repo / "scripts" / "skills" / "cryptography-manager.py"
    if not py_script.is_file():
        raise FileNotFoundError(str(py_script))
    proc = subprocess.run(
        [sys.executable, str(py_script)],
        input=json.dumps(payload, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    line = (proc.stdout or "").strip()
    if not line:
        raise RuntimeError(proc.stderr or "cryptography-manager.py sin salida")
    out = json.loads(line)
    if not out.get("success"):
        raise RuntimeError(out.get("error") or "cryptography-manager.py failed")
    return _parse_crypto_envelope(out)


def _crypto_wasm_ready(repo: Path) -> bool:
    wasm = repo / "SddIA" / "target" / "wasm32-wasip1" / "debug" / "cryptography-manager.wasm"
    return wasm.is_file() and shutil.which("wasmtime") is not None


def crypto(repo: Path, payload: dict[str, Any]) -> Any:
    if not _crypto_wasm_ready(repo):
        return _invoke_crypto_native(repo, payload)
    crypto_script = repo / "SddIA" / "target" / "wasm32-wasip1" / "debug" / "cryptography-manager.wasm"
    try:
        proc = subprocess.run(
            ["wasmtime", "run", "--dir=/", str(crypto_script)],
            input=json.dumps(payload),
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            cwd=str(repo),
            check=False,
        )
    except FileNotFoundError:
        return _invoke_crypto_native(repo, payload)
    out = json.loads(proc.stdout or "{}")
    if not out.get("success"):
        raise RuntimeError(out.get("error") or proc.stderr or "cryptography-manager failed")
    return _parse_crypto_envelope(out)


def _invoke_git_manager_native(
    repo: Path, operation_type: str, payload: dict[str, Any]
) -> dict[str, Any]:
    """Fallback laboratorio: git-manager.py cuando WASI no puede ejecutar git."""
    py_script = repo / "scripts" / "skills" / "git-manager.py"
    if not py_script.is_file():
        raise FileNotFoundError(str(py_script))
    req = {
        "operation_type": operation_type,
        "repository_path": str(repo.resolve()),
        "operation_payload_json": payload,
    }
    proc = subprocess.run(
        [sys.executable, str(py_script)],
        input=json.dumps(req, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    line = (proc.stdout or "").strip()
    if not line:
        raise RuntimeError(proc.stderr or "git-manager.py sin salida")
    body = json.loads(line)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "git-manager.py failed")
    return body.get("data") or {}


def invoke_git_manager(
    repo: Path,
    operation_type: str,
    payload: dict[str, Any],
    extra_env: dict[str, str] | None = None,
) -> dict[str, Any]:
    git_script = repo / "SddIA" / "target" / "wasm32-wasip1" / "debug" / "git-manager.wasm"
    if not git_script.is_file():
        raise FileNotFoundError(str(git_script))
    req = {
        "operation_type": operation_type,
        "repository_path": str(repo.resolve()),
        "operation_payload_json": payload,
    }
    env = os.environ.copy()
    if extra_env:
        env.update(extra_env)
    proc = subprocess.run(
        ["wasmtime", "run", "--dir=.", str(git_script)],
        input=json.dumps(req, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
        env=env,
    )
    stdout = (proc.stdout or "").strip()

    if "failed to execute git" in (proc.stderr or "") or "operation not supported" in (proc.stderr or "") or "failed to execute git" in stdout or "operation not supported" in stdout:
        return _invoke_git_manager_native(repo, operation_type, payload)

    if not stdout:
        if "operation not supported" in (proc.stderr or ""):
            return _invoke_git_manager_native(repo, operation_type, payload)
        raise RuntimeError(proc.stderr or "git-manager sin salida")
    body = json.loads(stdout)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "git-manager failed")
    return body.get("data") or {}


def _try_delete_branch_op(repo: Path, branch: str, *, remote: bool) -> dict[str, Any]:
    op_name = "delete_branch_remote" if remote else "delete_branch_local"
    command = "git push origin --delete" if remote else "git branch -d"
    try:
        invoke_git_manager(
            repo,
            "delete_branch",
            {"branch_name": branch, "remote": remote, "force": False},
        )
        return {"op": op_name, "command": command, "success": True}
    except RuntimeError as exc:
        return {"op": op_name, "command": command, "success": False, "error": str(exc)}


def _delete_branch_hygiene(repo: Path, branch: str) -> tuple[str | None, dict[str, Any] | None]:
    branch = branch.strip()
    if not branch:
        return None, None
    local = _try_delete_branch_op(repo, branch, remote=False)
    remote = _try_delete_branch_op(repo, branch, remote=True)
    local_ok = local.get("success") is True
    remote_ok = remote.get("success") is True
    if local_ok and remote_ok:
        return branch, None
    return None, {
        "survived_branch": branch,
        "branch_deleted_local": local_ok,
        "branch_deleted_remote": remote_ok,
        "operations": [local, remote],
    }


def _apply_branch_hygiene_state(
    state: dict[str, Any], closed: str | None, hygiene_failure: dict[str, Any] | None
) -> None:
    state["closed_branch"] = closed
    if hygiene_failure is not None:
        state["hygiene_failure"] = hygiene_failure
    else:
        state.pop("hygiene_failure", None)


def invoke_shell_executor(repo: Path, executable: str, arguments: list[str]) -> dict[str, Any]:
    shell_script = repo / "SddIA" / "target" / "wasm32-wasip1" / "debug" / "shell-executor.wasm"
    if not shell_script.is_file():
        raise FileNotFoundError(str(shell_script))
    req = {
        "executable": executable,
        "arguments": arguments,
        "working_directory": str(repo.resolve()),
        "environment_vars": {},
    }
    proc = subprocess.run(
        ["wasmtime", "run", "--dir=.", str(shell_script)],
        input=json.dumps(req, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    stdout = (proc.stdout or "").strip()
    if not stdout:
        raise RuntimeError(proc.stderr or "shell-executor sin salida")
    body = json.loads(stdout)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "shell-executor failed")
    return body.get("data") or {}


def _parse_gh_pr_url(stdout: str) -> str | None:
    for line in reversed((stdout or "").splitlines()):
        m = _GH_PR_URL_RE.search(line)
        if m:
            return m.group(0)
    m = _GH_PR_URL_RE.search(stdout or "")
    return m.group(0) if m else None


def _delivery_pr_title(inputs: dict[str, Any]) -> str:
    title = inputs.get("pr_title")
    if isinstance(title, str) and title.strip():
        return title.strip()
    branch = inputs.get("branch_name")
    if isinstance(branch, str) and branch.strip():
        return f"feat: {branch.strip()}"
    return "feat: delivery-close-cycle"


def capsule_delivery_remote_push(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    branch = inputs.get("branch_name")
    if not isinstance(branch, str) or not branch.strip():
        raise ValueError("branch_name es obligatorio para Publicación remota")
    if os.environ.get("SDDIA_LAB_SKIP_GIT_PUSH", "").strip().lower() in ("1", "true", "yes"):
        return {"skipped": True, "reason": "SDDIA_LAB_SKIP_GIT_PUSH"}
    # Push interno desde hook pre-push: skip hooks solo en subproceso git (no global).
    push_env: dict[str, str] | None = None
    if inputs.get("source_process") == "git-hook-pre-push":
        push_env = {"SDDIA_SKIP_HOOKS": "1"}
    data = invoke_git_manager(
        repo,
        "push",
        {"remote": "origin", "branch": branch.strip(), "force": False},
        extra_env=push_env,
    )
    state["delivery_push"] = data
    return data


def capsule_delivery_gh_pr(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    branch = inputs.get("branch_name")
    if not isinstance(branch, str) or not branch.strip():
        raise ValueError("branch_name es obligatorio para Apertura en forja")
    branch = branch.strip()
    target = inputs.get("target_branch", "main")
    if not isinstance(target, str) or not target.strip():
        target = "main"
    else:
        target = target.strip()

    preset = inputs.get("pr_url")
    if isinstance(preset, str) and preset.strip():
        pr_url = preset.strip()
        state["pr_url"] = pr_url
        return {"pr_url": pr_url, "simulated": True, "source": "inputs.pr_url"}

    if os.environ.get("SDDIA_LAB_SIMULATE_GH_PR", "").strip().lower() in ("1", "true", "yes"):
        pr_url = f"https://github.com/lab-simulated/SddIA/pull/0-{branch.replace('/', '-')}"
        state["pr_url"] = pr_url
        return {"pr_url": pr_url, "simulated": True}

    title = _delivery_pr_title(inputs)
    body_text = inputs.get("pr_body")
    args = [
        "pr",
        "create",
        "--title",
        title,
        "--head",
        branch,
        "--base",
        target,
    ]
    if isinstance(body_text, str) and body_text.strip():
        args.extend(["--body", body_text.strip()])
    else:
        args.append("--fill")

    data = invoke_shell_executor(repo, "gh", args)
    stdout = str(data.get("stdout") or "")
    pr_url = _parse_gh_pr_url(stdout)
    if not pr_url:
        view = invoke_shell_executor(
            repo,
            "gh",
            ["pr", "view", branch, "--json", "url", "-q", ".url"],
        )
        pr_url = (view.get("stdout") or "").strip()
    if not pr_url:
        raise RuntimeError("no se pudo resolver pr_url desde gh")
    state["pr_url"] = pr_url
    return {"pr_url": pr_url, "gh_stdout": stdout[:500]}


def capsule_delivery_snapshot_final(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    branch = inputs.get("branch_name")
    if not isinstance(branch, str) or not branch.strip():
        raise ValueError("branch_name es obligatorio para Snapshot final")
    if os.environ.get("SDDIA_LAB_SKIP_SNAPSHOT", "").strip().lower() in ("1", "true", "yes"):
        return {"skipped": True, "reason": "SDDIA_LAB_SKIP_SNAPSHOT"}
    data = invoke_git_manager(
        repo,
        "get_last_commit",
        {"ref": branch.strip()},
    )
    commit_hash = data.get("commitHash") or data.get("commit_hash")
    state["snapshot_commit_hash"] = commit_hash
    return {"commit_hash": commit_hash, "branch": branch.strip()}


def capsule_delivery_local_hygiene(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    if os.environ.get("SDDIA_LAB_SKIP_HIGIENE", "").strip().lower() in ("1", "true", "yes"):
        return {"skipped": True, "reason": "SDDIA_LAB_SKIP_HIGIENE"}
    branch = inputs.get("branch_name")
    closed: str | None = None
    if isinstance(branch, str) and branch.strip() and os.environ.get(
        "SDDIA_LAB_DELETE_FEATURE_BRANCH", ""
    ).strip().lower() in ("1", "true", "yes"):
        invoke_git_manager(repo, "checkout", {"branch_name": "main", "create_if_not_exists": False})
        closed, hygiene_failure = _delete_branch_hygiene(repo, branch.strip())
    else:
        closed, hygiene_failure = None, None
    _apply_branch_hygiene_state(state, closed, hygiene_failure)
    result: dict[str, Any] = {
        "closed_branch": closed,
        "note": "higiene parcial en laboratorio; delete requiere SDDIA_LAB_DELETE_FEATURE_BRANCH",
    }
    if hygiene_failure is not None:
        result["hygiene_failure"] = hygiene_failure
    return result


def capsule_delivery_emit_presented(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    branch = inputs.get("branch_name")
    if not isinstance(branch, str) or not branch.strip():
        raise ValueError("branch_name es obligatorio para Sello Presentación ECST")
    action_inputs: dict[str, Any] = {
        "branch": branch.strip(),
        "status": inputs.get("status", "presented"),
        "emitter_agent": "delivery-close-cycle",
    }
    pr_url = state.get("pr_url") or inputs.get("pr_url")
    if isinstance(pr_url, str) and pr_url.strip():
        action_inputs["pr_url"] = pr_url.strip()
    corr = inputs.get("correlation_id")
    if isinstance(corr, str) and corr.strip():
        action_inputs["correlation_id"] = corr.strip()
    seal = invoke_capsule_action(repo, "emit-pr-presented-event", action_inputs)
    state["handoff"].update(seal)
    state["event_id"] = seal.get("event_id")
    state["target_path"] = seal.get("target_path")
    return seal


def _eda_bus_scan_dirs(repo: Path) -> list[Path]:
    bus = load_eda_bus(repo)
    dirs: list[Path] = []
    for key in ("pending",):
        p = repo / bus[key]
        if p.is_dir():
            dirs.append(p)
    for legacy in ("docs/events/pending", "docs/events/processing", "docs/events/processed"):
        p = repo / legacy
        if p.is_dir() and p not in dirs:
            dirs.append(p)
    return dirs


def _scan_presented_for_branch(repo: Path, branch_name: str) -> bool:
    target = branch_name.strip()
    for bus_dir in _eda_bus_scan_dirs(repo):
        for path in bus_dir.glob("*.json"):
            try:
                event = json.loads(path.read_text(encoding="utf-8"))
            except (OSError, json.JSONDecodeError):
                continue
            if event.get("event_type") != "PullRequest_Presented":
                continue
            payload = event.get("payload") or {}
            if payload.get("branch") == target:
                return True
    return False


def capsule_accept_genomic_audit(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    source = inputs.get("source_branch")
    if not isinstance(source, str) or not source.strip():
        raise ValueError("source_branch es obligatorio para accept-pr")
    source = source.strip()
    presented = _scan_presented_for_branch(repo, source)
    orphan = not presented
    state["orphan_merge"] = orphan
    state["source_branch"] = source
    if orphan:
        state.setdefault("handoff", {})["traceability_warning"] = (
            "Merge Huérfano: sin PullRequest_Presented previo en bus local"
        )
    return {"orphan_merge": orphan, "presented_found": presented}


def capsule_accept_merge_sovereign(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    source = state.get("source_branch") or inputs.get("source_branch")
    if not isinstance(source, str) or not source.strip():
        raise ValueError("source_branch es obligatorio para Fusión Soberana")
    source = source.strip()

    if inputs.get("merge_already_done") in (True, "true", "1", 1):
        data = invoke_git_manager(repo, "get_last_commit", {"ref": "HEAD"})
        merge_hash = data.get("commitHash") or data.get("commit_hash")
        state["merge_commit_hash"] = merge_hash
        return {"skipped": True, "reason": "merge_already_done", "merge_commit_hash": merge_hash}

    invoke_git_manager(repo, "checkout", {"branch_name": "main", "create_if_not_exists": False})
    merge_data = invoke_git_manager(
        repo,
        "merge",
        {"branch_name": source, "no_ff": True},
    )
    merge_hash = (
        merge_data.get("commitHash")
        or merge_data.get("commit_hash")
        or merge_data.get("mergeCommitHash")
    )
    if not merge_hash:
        head = invoke_git_manager(repo, "get_last_commit", {"ref": "HEAD"})
        merge_hash = head.get("commitHash") or head.get("commit_hash")
    state["merge_commit_hash"] = merge_hash
    return {"merge_commit_hash": merge_hash, "source_branch": source}


def capsule_accept_emit_merged(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    source = state.get("source_branch") or inputs.get("source_branch")
    if not isinstance(source, str) or not source.strip():
        raise ValueError("source_branch es obligatorio para sello Merged")
    merge_hash = state.get("merge_commit_hash")
    if not isinstance(merge_hash, str) or not merge_hash.strip():
        raise ValueError("merge_commit_hash ausente antes del sello Merged")

    correlation_id = inputs.get("correlation_id")
    if not isinstance(correlation_id, str) or not correlation_id.strip():
        correlation_id = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})

    action_inputs: dict[str, Any] = {
        "source_branch": source.strip(),
        "author": inputs.get("author", "integration-operator"),
        "correlation_id": correlation_id,
        "merge_commit_hash": merge_hash.strip(),
        "emitter_agent": "accept-pr",
    }
    audit_ref = state.get("audit_event_reference") or inputs.get("audit_event_reference")
    if isinstance(audit_ref, str) and audit_ref.strip():
        action_inputs["audit_event_reference"] = audit_ref.strip()
    if state.get("orphan_merge"):
        action_inputs["traceability_anomaly"] = "merge_huérfano"
        action_inputs["traceability_note"] = (
            "Fusión física sin PullRequest_Presented previo en bus local"
        )

    seal = invoke_capsule_action(repo, "emit-pr-merged-event", action_inputs)
    state["handoff"].update(seal)
    state["event_id"] = seal.get("event_id")
    state["target_path"] = seal.get("target_path")
    return seal


def capsule_accept_sync_cleanup(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    if os.environ.get("SDDIA_LAB_SKIP_GIT_PUSH", "").strip().lower() in ("1", "true", "yes"):
        return {"skipped": True, "reason": "SDDIA_LAB_SKIP_GIT_PUSH"}
    source = state.get("source_branch") or inputs.get("source_branch")
    prev_skip = os.environ.get("SDDIA_SKIP_HOOKS")
    os.environ["SDDIA_SKIP_HOOKS"] = "1"
    try:
        push_data = invoke_git_manager(
            repo,
            "push",
            {"remote": "origin", "branch": "main", "force": False},
        )
    finally:
        if prev_skip is None:
            os.environ.pop("SDDIA_SKIP_HOOKS", None)
        else:
            os.environ["SDDIA_SKIP_HOOKS"] = prev_skip
    closed: str | None = None
    hygiene_failure: dict[str, Any] | None = None
    if isinstance(source, str) and source.strip():
        closed, hygiene_failure = _delete_branch_hygiene(repo, source.strip())
    _apply_branch_hygiene_state(state, closed, hygiene_failure)
    result: dict[str, Any] = {"push": push_data, "closed_branch": closed}
    if hygiene_failure is not None:
        result["hygiene_failure"] = hygiene_failure
    return result


def execute_accept_pr_phase(
    repo: Path,
    phase_name: str | None,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if phase_name == "Auditoría Genómica":
        result = capsule_accept_genomic_audit(repo, inputs, state)
        return {"status": "executed", "handler": "accept-genomic-audit", **result}
    if phase_name == "Fusión Soberana":
        result = capsule_accept_merge_sovereign(repo, inputs, state)
        return {"status": "executed", "handler": "accept-merge-sovereign", **result}
    if phase_name == "Sello Criptográfico de Fusión":
        result = capsule_accept_emit_merged(repo, inputs, state)
        return {
            "status": "executed",
            "handler": "accept-emit-merged",
            **{k: result[k] for k in ("event_id", "target_path", "event_type") if k in result},
        }
    if phase_name == "Sincronización y Limpieza":
        result = capsule_accept_sync_cleanup(repo, inputs, state)
        return {"status": "executed", "handler": "accept-sync-cleanup", **result}
    return None


_PR_REVIEW_REQUIRED_DOCS = ("objectives.md", "spec.md", "plan.md", "implementation.md")
_PR_REVIEW_REQUIRED_DOCS_FIX = ("objectives.md", "spec.md", "implementation.md")


def _pr_review_required_docs(persist_ref: str) -> tuple[str, ...]:
    ref = persist_ref.strip().replace("\\", "/")
    if ref.startswith("docs/fixes/"):
        return _PR_REVIEW_REQUIRED_DOCS_FIX
    return _PR_REVIEW_REQUIRED_DOCS


def _validate_feature_documentation(repo: Path, persist_ref: str) -> list[str]:
    base = repo / persist_ref.strip().replace("\\", "/")
    errors: list[str] = []
    if not base.is_dir():
        return [f"persist_ref inexistente: {persist_ref}"]
    for name in _pr_review_required_docs(persist_ref):
        path = base / name
        if not path.is_file():
            errors.append(f"ausente {name}")
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except OSError as e:
            errors.append(f"{name}: lectura fallida ({e})")
            continue
        if not text.startswith("---"):
            errors.append(f"{name}: frontmatter YAML obligatorio")
    return errors


def _normalize_pr_review_inputs(repo: Path, process_inputs: dict[str, Any]) -> None:
    branch = process_inputs.get("pr_branch")
    if not process_inputs.get("pr_id_or_path"):
        process_inputs["pr_id_or_path"] = process_inputs.get("pr_url") or branch
    if not process_inputs.get("correlation_id"):
        process_inputs["correlation_id"] = crypto(
            repo, {"operation": "GENERATE_UUID", "target_payload": None}
        )
    if not process_inputs.get("persist_ref") and isinstance(branch, str):
        inferred = infer_persist_ref_from_branch(repo, branch)
        if inferred:
            process_inputs["persist_ref"] = inferred


def _sync_pr_review_worktree(repo: Path, branch: str) -> dict[str, Any]:
    """Alinea worktree con origin/<branch> tras fetch (evita verify con fases obsoletas)."""
    invoke_git_manager(repo, "fetch", {"remote": "origin", "prune": True})
    remote_ref = f"origin/{branch}"
    try:
        invoke_git_manager(repo, "get_last_commit", {"ref": remote_ref})
        invoke_git_manager(repo, "checkout", {"branch_name": branch, "create_if_not_exists": True})
        return {"branch": branch, "synced_to": remote_ref, "mode": "origin-tracking"}
    except RuntimeError:
        invoke_git_manager(repo, "checkout", {"branch_name": branch, "create_if_not_exists": False})
        return {"branch": branch, "mode": "local-checkout"}


def capsule_pr_review_branch_prep(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    branch = inputs.get("pr_branch")
    if not isinstance(branch, str) or not branch.strip():
        raise ValueError("pr_branch es obligatorio para pull-request-review")
    branch = branch.strip()
    state["pr_branch"] = branch
    if inputs.get("merge_already_done") in (True, "true", "1", 1):
        return {"skipped": True, "reason": "merge_already_done", "branch": branch}
    if os.environ.get("SDDIA_LAB_SKIP_GIT_CHECKOUT", "").strip().lower() in ("1", "true", "yes"):
        return {"skipped": True, "reason": "SDDIA_LAB_SKIP_GIT_CHECKOUT", "branch": branch}
    sync = _sync_pr_review_worktree(repo, branch)
    state["pr_worktree_sync"] = sync
    return sync


def capsule_pr_review_documental(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    persist_ref = inputs.get("persist_ref") or state.get("persist_ref")
    if not isinstance(persist_ref, str) or not persist_ref.strip():
        err = "persist_ref no resuelto para triaje documental"
        state.setdefault("review_failures", []).append(err)
        return {"passed": False, "errors": [err]}
    persist_ref = persist_ref.strip()
    state["persist_ref"] = persist_ref
    if os.environ.get("SDDIA_LAB_PR_REVIEW_DOC_FAIL", "").strip().lower() in ("1", "true", "yes"):
        err = "fallo simulado triaje documental (SDDIA_LAB_PR_REVIEW_DOC_FAIL)"
        state.setdefault("review_failures", []).append(err)
        return {"passed": False, "errors": [err]}
    errors = _validate_feature_documentation(repo, persist_ref)
    if errors:
        state.setdefault("review_failures", []).extend(errors)
    return {"passed": not errors, "errors": errors, "persist_ref": persist_ref}


def _kaizen_alert_hash(review_id: str, implicated_files: list[Any]) -> str:
    key = review_id + "".join(sorted(str(h) for h in implicated_files))
    return hashlib.sha256(key.encode("utf-8")).hexdigest()[:8]


def _emit_kaizen_alert_required(
    repo: Path,
    *,
    review_id: str,
    alert_justification: str,
    implicated_files: list[str],
    persist_ref: str | None = None,
    pr_branch: str | None = None,
    impacts_doc: Any = None,
    alert_kind: str = "doc_parity",
) -> dict[str, Any]:
    event_id = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    payload: dict[str, Any] = {
        "review_id": review_id,
        "alert_justification": alert_justification,
        "implicated_files": implicated_files,
        "alert_kind": alert_kind,
    }
    if isinstance(persist_ref, str) and persist_ref.strip():
        payload["persist_ref"] = persist_ref.strip()
    if isinstance(pr_branch, str) and pr_branch.strip():
        payload["pr_branch"] = pr_branch.strip()
    if impacts_doc is not None:
        payload["impacts_doc"] = impacts_doc
    event = {
        "event_id": event_id,
        "event_type": "Kaizen_Alert_Required",
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": "pull-request-review",
        "correlation_id": review_id,
        "payload": payload,
        "delivery_state": {},
    }
    ok, errors = validate_domain_mutation_event(repo, event)
    if not ok:
        return {"dia_event_error": "; ".join(errors)}
    seal = write_pending_event(repo, event)
    return {
        "kaizen_alert_emitted": True,
        "event_id": seal["event_id"],
        "target_path": seal["target_path"],
        "hash8": _kaizen_alert_hash(review_id, implicated_files),
    }


def _invoke_dia_audit(
    repo: Path, inputs: dict[str, Any], state: dict[str, Any]
) -> dict[str, Any] | None:
    """Sensor DIA — sin llamadas a agentes; alerta no bloqueante."""
    if not AUDIT_DOC_PARITY_CLI.is_file():
        return None
    persist_ref = state.get("persist_ref") or inputs.get("persist_ref")
    if not isinstance(persist_ref, str) or not persist_ref.strip():
        return None
    persist_ref = persist_ref.strip()
    branch = state.get("pr_branch") or inputs.get("pr_branch") or "HEAD"
    correlation = inputs.get("correlation_id") or state.get("correlation_id")
    alert_file = ""
    if isinstance(correlation, str) and correlation.strip():
        alert_file = f".tmp/audit-doc-parity-{correlation.strip()[:36]}.json"
    cmd = [
        sys.executable,
        str(AUDIT_DOC_PARITY_CLI),
        "--persist-ref",
        persist_ref,
        "--base-ref",
        str(inputs.get("base_ref") or "main"),
        "--head-ref",
        str(branch),
        "--json",
    ]
    if alert_file:
        cmd.extend(["--alert-file", alert_file])
    if isinstance(correlation, str) and correlation.strip():
        cmd.extend(["--correlation-hint", correlation.strip()])
    env = os.environ.copy()
    env["SDDIA_REPO_ROOT"] = str(repo.resolve())
    proc = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        env=env,
        check=False,
    )
    if proc.returncode == 2:
        return {
            "dia_error": (proc.stdout or proc.stderr or "")[-500:],
            "dia_exit_code": 2,
        }
    try:
        payload = json.loads(proc.stdout.strip() or "{}")
    except json.JSONDecodeError:
        return {"dia_error": "JSON inválido de audit-doc-parity", "dia_exit_code": proc.returncode}
    if not isinstance(payload, dict):
        return {"dia_error": "payload DIA no es objeto"}
    if payload.get("alert_required"):
        hits = payload.get("monitored_hits") or []
        if not isinstance(hits, list):
            hits = []
        implicated = [str(h) for h in hits if str(h).strip()]
        raw_corr = inputs.get("correlation_id") or state.get("correlation_id")
        if isinstance(raw_corr, str) and raw_corr.strip():
            review_id = raw_corr.strip()
        else:
            review_id = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
        reason = str(payload.get("reason") or "doc_parity_alert")
        persist_ref = payload.get("persist_ref") or state.get("persist_ref") or inputs.get("persist_ref")
        branch = state.get("pr_branch") or inputs.get("pr_branch") or "HEAD"
        emit_result = _emit_kaizen_alert_required(
            repo,
            review_id=review_id,
            alert_justification=reason,
            implicated_files=implicated,
            persist_ref=str(persist_ref) if isinstance(persist_ref, str) else None,
            pr_branch=str(branch) if isinstance(branch, str) else None,
            impacts_doc=payload.get("impacts_doc"),
        )
        return {"dia_audit": payload, "kaizen_alert": emit_result}
    return {"dia_audit": payload}


def capsule_pr_review_technical(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    if os.environ.get("SDDIA_LAB_PR_REVIEW_TECH_FAIL", "").strip().lower() in ("1", "true", "yes"):
        err = "fallo simulado triaje técnico (SDDIA_LAB_PR_REVIEW_TECH_FAIL)"
        state.setdefault("review_failures", []).append(err)
        return {"passed": False, "errors": [err]}
    integrity = repo / "SddIA" / "scripts" / "qa" / "verify-process-integrity.py"
    if integrity.is_file():
        vpi_env = os.environ.copy()
        vpi_env["SDDIA_REPO_ROOT"] = str(repo.resolve())
        proc = subprocess.run(
            [sys.executable, str(integrity)],
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            cwd=str(repo),
            env=vpi_env,
            check=False,
        )
        if proc.returncode != 0:
            err = "verify-process-integrity falló en triaje técnico"
            state.setdefault("review_failures", []).append(err)
            return {"passed": False, "errors": [err], "stderr": (proc.stderr or "")[-500:]}
    dia_result = _invoke_dia_audit(repo, inputs, state)
    out: dict[str, Any] = {"passed": True}
    if dia_result:
        out["dia"] = dia_result
    return out


def capsule_pr_review_rbac(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    if os.environ.get("SDDIA_LAB_PR_REVIEW_RBAC_FAIL", "").strip().lower() in ("1", "true", "yes"):
        err = "Cerbero RBAC: permiso denegado (simulado)"
        state.setdefault("review_failures", []).append(err)
        return {"passed": False, "agent": "cerbero", "errors": [err]}
    return {"passed": True, "agent": "cerbero", "note": "RBAC lab stub success"}


def _emit_pull_request_audited(
    repo: Path,
    inputs: dict[str, Any],
    state: dict[str, Any],
    *,
    verdict: str,
    failures: list[Any],
) -> dict[str, Any]:
    branch = state.get("pr_branch") or inputs.get("pr_branch") or "unknown"
    correlation_id = inputs.get("correlation_id") or state.get("correlation_id")
    if not isinstance(correlation_id, str) or not correlation_id.strip():
        correlation_id = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    resolution = "PASS" if verdict == "aprobado" else "REJECT"
    action_inputs: dict[str, Any] = {
        "target_entity_id": str(branch),
        "resolution": resolution,
        "violated_rules": [str(item) for item in failures if str(item).strip()],
        "audit_event_reference": correlation_id.strip(),
        "correlation_id": correlation_id.strip(),
        "emitter_agent": "argos",
    }
    seal = invoke_capsule_action(repo, "emit-pr-audited-event", action_inputs)
    state["audit_event_reference"] = seal.get("audit_event_reference") or correlation_id.strip()
    state["pull_request_audited_event_id"] = seal.get("event_id")
    state["pull_request_audited_target_path"] = seal.get("target_path")
    return seal


def capsule_pr_review_verdict(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    failures = state.get("review_failures") or []
    if failures:
        state["verdict"] = "rechazado"
        state["delivery_state"] = "failed"
        state["argos_feedback"] = [
            {"kind": "norm_collision", "detail": item} for item in failures
        ]
        audited = _emit_pull_request_audited(
            repo, inputs, state, verdict="rechazado", failures=failures
        )
        return {
            "verdict": "rechazado",
            "delivery_state": "failed",
            "failures": failures,
            "pull_request_audited": audited,
        }
    state["verdict"] = "aprobado"
    state["delivery_state"] = "success"
    audited = _emit_pull_request_audited(
        repo, inputs, state, verdict="aprobado", failures=[]
    )
    return {
        "verdict": "aprobado",
        "delivery_state": "success",
        "pull_request_audited": audited,
    }


def capsule_pr_review_kaizen(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    items: list[str] = list(state.get("kaizen_items") or [])
    if os.environ.get("SDDIA_LAB_PR_REVIEW_KAIZEN", "").strip().lower() in ("1", "true", "yes"):
        items.append("Deuda simulada laboratorio — optimizar handlers aduana PR review")
    seeds: list[str] = []
    branch = state.get("pr_branch") or inputs.get("pr_branch") or "unknown"

    generic_idx = 0
    for item in items:
        generic_idx += 1
        slug = re.sub(r"[^\w\-]+", "-", str(branch)).strip("-")[:48]
        todo_name = f"[OPERATIVO] Kaizen PR review — {slug}.md"
        if generic_idx > 1:
            todo_name = f"[OPERATIVO] Kaizen PR review — {slug} ({generic_idx}).md"
        todo_path = repo / "docs" / "todos" / todo_name
        todo_path.parent.mkdir(parents=True, exist_ok=True)
        body = (
            f"# {todo_name}\n\n"
            f"> Origen: pull-request-review / rama `{branch}`\n\n"
            f"- [ ] {item}\n"
        )
        todo_path.write_text(body, encoding="utf-8")
        seeds.append(todo_path.relative_to(repo).as_posix())
    state["kaizen_seeds"] = seeds
    return {"kaizen_seeds": seeds, "count": len(seeds)}


def capsule_pr_review_handoff_accept(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    if state.get("verdict") != "aprobado":
        return {"skipped": True, "reason": state.get("verdict", "sin veredicto")}
    if os.environ.get("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF", "").strip().lower() in ("1", "true", "yes"):
        state["accept_pr_handoff"] = True
        return {"skipped": True, "reason": "SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF", "simulated_handoff": True}
    source = state.get("pr_branch") or inputs.get("pr_branch")
    if not isinstance(source, str) or not source.strip():
        raise ValueError("pr_branch ausente para handoff accept-pr")
    child_inputs: dict[str, Any] = {
        "source_branch": source.strip(),
        "author": inputs.get("author", "pull-request-review-aduana"),
        "correlation_id": inputs.get("correlation_id"),
    }
    audit_ref = state.get("audit_event_reference")
    if isinstance(audit_ref, str) and audit_ref.strip():
        child_inputs["audit_event_reference"] = audit_ref.strip()
    if inputs.get("merge_already_done") in (True, "true", "1", 1):
        child_inputs["merge_already_done"] = True
    data = invoke_subprocess_process(repo, "accept-pr", child_inputs)
    state["accept_pr_handoff"] = True
    state["handoff"].update(data.get("handoff") or {})
    if data.get("event_id"):
        state["event_id"] = data["event_id"]
    if data.get("target_path"):
        state["target_path"] = data["target_path"]
    return {"child_process": "accept-pr", "handoff": state.get("handoff"), **data}


def execute_pull_request_review_phase(
    repo: Path,
    phase_name: str | None,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if phase_name == "Preparación de rama":
        result = capsule_pr_review_branch_prep(repo, inputs, state)
        return {"status": "executed", "handler": "pr-review-branch-prep", **result}
    if phase_name == "Triaje documental":
        result = capsule_pr_review_documental(repo, inputs, state)
        st = "blocked" if not result.get("passed") else "executed"
        return {"status": st, "handler": "pr-review-documental", **result}
    if phase_name == "Triaje técnico":
        result = capsule_pr_review_technical(repo, inputs, state)
        st = "blocked" if not result.get("passed") else "executed"
        return {"status": st, "handler": "pr-review-technical", **result}
    if phase_name == "Certificación RBAC":
        result = capsule_pr_review_rbac(repo, inputs, state)
        st = "blocked" if not result.get("passed") else "executed"
        return {"status": st, "handler": "pr-review-rbac", **result}
    if phase_name == "Veredicto y bloqueo":
        result = capsule_pr_review_verdict(repo, inputs, state)
        st = "blocked" if result.get("verdict") == "rechazado" else "executed"
        return {"status": st, "handler": "pr-review-verdict", **result}
    if phase_name == "Cosecha Kaizen":
        result = capsule_pr_review_kaizen(repo, inputs, state)
        return {"status": "executed", "handler": "pr-review-kaizen", **result}
    if phase_name == "Handoff materialización":
        result = capsule_pr_review_handoff_accept(repo, inputs, state)
        return {"status": "executed", "handler": "pr-review-handoff-accept", **result}
    return None


def _git_diff_name_only(repo: Path, base_ref: str, head_ref: str) -> list[str]:
    candidates = [
        f"origin/{base_ref}...{head_ref}",
        f"{base_ref}...{head_ref}",
    ]
    for ref_spec in candidates:
        try:
            data = invoke_shell_executor(repo, "git", ["diff", "--name-only", ref_spec])
            stdout = str(data.get("stdout") or "")
            return [ln.strip() for ln in stdout.splitlines() if ln.strip()]
        except RuntimeError:
            continue
    return []


def capsule_delivery_impact_assessment(
    repo: Path, inputs: dict[str, Any], state: dict[str, Any]
) -> dict[str, Any]:
    if os.environ.get("SDDIA_LAB_SKIP_IMPACT_ASSESSMENT", "").strip().lower() in (
        "1",
        "true",
        "yes",
    ):
        return {"skipped": True, "reason": "SDDIA_LAB_SKIP_IMPACT_ASSESSMENT"}
    source = inputs.get("source_process")
    if source != "feature":
        return {"skipped": True, "reason": "source_process != feature"}
    branch = inputs.get("branch_name")
    if not isinstance(branch, str) or not branch.strip():
        raise ValueError("branch_name es obligatorio para Impacto SddIA condicional")
    branch = branch.strip()
    target = inputs.get("target_branch", "main")
    if not isinstance(target, str) or not target.strip():
        target = "main"
    else:
        target = target.strip()
    changed = _git_diff_name_only(repo, target, branch)
    sddia_paths = [p for p in changed if p.replace("\\", "/").startswith("SddIA/")]
    impact = "core_mutation" if sddia_paths else "none"
    result = {"impact": impact, "sddia_paths": sddia_paths, "branch": branch, "base_ref": target}
    state["sddia_impact"] = result
    return result


def _resolve_related_todo_path(repo: Path, inputs: dict[str, Any]) -> Path | None:
    todo = inputs.get("related_todo")
    if isinstance(todo, str) and todo.strip():
        path = repo / todo.strip().replace("\\", "/")
        if path.is_file():
            return path
    persist_ref = inputs.get("persist_ref")
    if isinstance(persist_ref, str) and persist_ref.strip():
        objectives = repo / persist_ref.strip().replace("\\", "/") / "objectives.md"
        if objectives.is_file():
            fm = parse_frontmatter(objectives)
            related = fm.get("related_todo")
            if isinstance(related, str) and related.strip():
                path = repo / related.strip().replace("\\", "/")
                if path.is_file():
                    return path
    return None


def _validacion_allows_pbi_archive(repo: Path, persist_ref: str) -> tuple[bool, str]:
    val_path = repo / persist_ref.replace("\\", "/") / "validacion.md"
    if not val_path.is_file():
        return False, "validacion.md ausente"
    fm = parse_frontmatter(val_path)
    global_v = str(fm.get("global", "")).strip().upper()
    if global_v != "APTO":
        return False, f"global={fm.get('global')}"
    archived = fm.get("pbi_archived")
    if archived not in (True, "true", "True", 1, "1"):
        return False, "pbi_archived != true"
    return True, "ok"


def capsule_feature_pbi_archive(
    repo: Path, inputs: dict[str, Any], state: dict[str, Any]
) -> dict[str, Any]:
    if os.environ.get("SDDIA_LAB_SKIP_PBI_ARCHIVE", "").strip().lower() in ("1", "true", "yes"):
        return {"skipped": True, "reason": "SDDIA_LAB_SKIP_PBI_ARCHIVE"}
    persist_ref = inputs.get("persist_ref")
    if not isinstance(persist_ref, str) or not persist_ref.strip():
        return {"skipped": True, "reason": "persist_ref ausente"}
    persist_ref = persist_ref.strip().replace("\\", "/")
    ok, reason = _validacion_allows_pbi_archive(repo, persist_ref)
    if not ok:
        return {"skipped": True, "reason": reason}
    pbi_path = _resolve_related_todo_path(repo, inputs)
    if pbi_path is None:
        return {"skipped": True, "reason": "related_todo no resuelto"}
    rel = pbi_path.relative_to(repo).as_posix()
    if rel.startswith("docs/todos/done/"):
        dest_rel = rel
        state["pbi_archived_path"] = dest_rel
        return {"already_archived": True, "pbi_path": dest_rel}
    if "docs/todos/pending/" not in rel.replace("\\", "/"):
        return {"skipped": True, "reason": f"PBI fuera de pending/: {rel}"}
    done_dir = repo / "docs" / "todos" / "done"
    done_dir.mkdir(parents=True, exist_ok=True)
    dest = done_dir / pbi_path.name
    if dest.is_file():
        dest_rel = dest.relative_to(repo).as_posix()
        state["pbi_archived_path"] = dest_rel
        return {"already_archived": True, "pbi_path": dest_rel}
    shutil.move(str(pbi_path), str(dest))
    dest_rel = dest.relative_to(repo).as_posix()
    state["pbi_archived_path"] = dest_rel
    return {"archived": True, "pbi_path": dest_rel}


def capsule_feature_invoke_delivery_close(
    repo: Path, inputs: dict[str, Any], state: dict[str, Any]
) -> dict[str, Any]:
    if os.environ.get("SDDIA_LAB_SKIP_DELIVERY_CLOSE", "").strip().lower() in ("1", "true", "yes"):
        return {"skipped": True, "reason": "SDDIA_LAB_SKIP_DELIVERY_CLOSE"}
    branch = inputs.get("branch_name") or (state.get("workspace") or {}).get("branch_name")
    persist_ref = inputs.get("persist_ref") or (state.get("workspace") or {}).get("persist_ref")
    if not isinstance(branch, str) or not branch.strip():
        raise ValueError("branch_name es obligatorio para Cierre de entrega")
    if not isinstance(persist_ref, str) or not persist_ref.strip():
        raise ValueError("persist_ref es obligatorio para Cierre de entrega")
    child_inputs: dict[str, Any] = {
        "source_process": "feature",
        "persist_ref": persist_ref.strip(),
        "branch_name": branch.strip(),
        "pr_title": _delivery_pr_title(inputs),
        "target_branch": inputs.get("base_branch") or inputs.get("target_branch") or "main",
    }
    pr_body = inputs.get("pr_body")
    if isinstance(pr_body, str) and pr_body.strip():
        child_inputs["pr_body"] = pr_body.strip()
    pr_url = inputs.get("pr_url")
    if isinstance(pr_url, str) and pr_url.strip():
        child_inputs["pr_url"] = pr_url.strip()
    data = invoke_subprocess_process(repo, "delivery-close-cycle", child_inputs)
    for key in ("pr_url", "event_id", "target_path", "closed_branch", "snapshot_commit_hash"):
        if data.get(key) is not None:
            state[key] = data[key]
    state["delivery_close"] = data
    return {
        "child_process": "delivery-close-cycle",
        **{k: data[k] for k in ("pr_url", "event_id", "target_path", "closed_branch") if k in data},
    }


def execute_feature_phase(
    repo: Path,
    phase_name: str | None,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if phase_name == "Cierre documental en rama":
        result = capsule_feature_pbi_archive(repo, inputs, state)
        status = "skipped" if result.get("skipped") else "executed"
        return {"status": status, "handler": "feature-pbi-archive", **result}
    if phase_name == "Cierre de entrega":
        result = capsule_feature_invoke_delivery_close(repo, inputs, state)
        status = "skipped" if result.get("skipped") else "executed"
        return {"status": status, "handler": "feature-delivery-close", **result}
    return None


def execute_delivery_close_phase(
    repo: Path,
    phase_name: str | None,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if phase_name == "Impacto SddIA condicional":
        result = capsule_delivery_impact_assessment(repo, inputs, state)
        status = "skipped" if result.get("skipped") else "executed"
        return {"status": status, "handler": "delivery-impact-assessment", **result}
    if phase_name == "Snapshot final":
        result = capsule_delivery_snapshot_final(repo, inputs, state)
        return {"status": "executed", "handler": "delivery-snapshot-final", **result}
    if phase_name == "Publicación remota":
        result = capsule_delivery_remote_push(repo, inputs, state)
        return {"status": "executed", "handler": "delivery-remote-push", **result}
    if phase_name == "Apertura en forja":
        result = capsule_delivery_gh_pr(repo, inputs, state)
        return {"status": "executed", "handler": "delivery-gh-pr", **result}
    if phase_name == "Sello Presentación ECST":
        result = capsule_delivery_emit_presented(repo, inputs, state)
        return {
            "status": "executed",
            "handler": "delivery-emit-pr-presented",
            **{k: result[k] for k in ("event_id", "target_path", "event_type") if k in result},
        }
    if phase_name == "Higiene local":
        result = capsule_delivery_local_hygiene(repo, inputs, state)
        return {"status": "executed", "handler": "delivery-local-hygiene", **result}
    return None


def invoke_subprocess_process(repo: Path, process_name: str, process_inputs: dict[str, Any]) -> dict[str, Any]:
    body = invoke_subprocess_process_full(repo, process_name, process_inputs)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or f"subproceso {process_name} falló")
    return body.get("data") or {}


def invoke_subprocess_process_full(
    repo: Path, process_name: str, process_inputs: dict[str, Any]
) -> dict[str, Any]:
    proc = subprocess.run(
        [
            sys.executable,
            str(EXECUTE_PROCESS_CLI),
            "--process",
            process_name,
            "--inputs",
            json.dumps(process_inputs, ensure_ascii=False),
        ],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or f"subproceso {process_name} sin salida")
    return json.loads(line)


def _workspace_task_name(inputs: dict[str, Any]) -> str | None:
    for key in ("feature_name", "fix_name", "refactor_name"):
        value = inputs.get(key)
        if isinstance(value, str) and value.strip():
            return value.strip()
    branch = inputs.get("branch_name")
    if isinstance(branch, str) and "/" in branch.strip():
        prefix, slug = branch.strip().split("/", 1)
        if prefix in ("feat", "fix") and slug.strip():
            return slug.strip()
    return None


def _workspace_process_label(
    inputs: dict[str, Any],
    branch_name: str,
    process_name: str | None = None,
) -> str:
    label = inputs.get("process_label")
    if isinstance(label, str) and label.strip():
        return label.strip()
    if process_name == "refactorization" or inputs.get("source_process") == "refactorization":
        return "refactorization"
    if process_name == "bug-fix" or inputs.get("source_process") == "bug-fix":
        return "bug-fix"
    if branch_name.startswith("fix/"):
        return "bug-fix"
    return "feature"


def is_workspace_init_phase(
    phase: dict[str, Any],
    inputs: dict[str, Any],
    process_def: dict[str, Any] | None = None,
) -> bool:
    delegates = phase.get("delegates_to") or []
    if not isinstance(delegates, list):
        return False
    has_git = any(isinstance(d, str) and d == "skill:git-manager" for d in delegates)
    if not has_git:
        return False
    process_name = (process_def or {}).get("name") if isinstance(process_def, dict) else None
    if process_name not in ("feature", "bug-fix", "refactorization"):
        return False
    if phase.get("name") != "Inicialización de Espacio de Trabajo":
        return False
    if _workspace_task_name(inputs):
        return True
    branch = inputs.get("branch_name")
    persist = inputs.get("persist_ref")
    if process_name == "bug-fix" and isinstance(branch, str) and branch.strip():
        if isinstance(persist, str) and persist.strip():
            return True
    return False


def run_workspace_init(
    repo: Path,
    inputs: dict[str, Any],
    process_name: str | None = None,
) -> dict[str, Any]:
    """Handler genérico: fase git-manager → rama + objectives.md (feature, bug-fix o refactorization)."""
    task_name = _workspace_task_name(inputs)
    branch_name = inputs.get("branch_name")
    if not isinstance(branch_name, str) or not branch_name.strip():
        if task_name:
            branch_name = f"feat/{task_name}"
        else:
            raise ValueError("branch_name inválido")
    else:
        branch_name = branch_name.strip()
    process_label = _workspace_process_label(inputs, branch_name, process_name)
    if not task_name:
        if "/" in branch_name:
            task_name = branch_name.split("/", 1)[1]
        else:
            task_name = branch_name
    default_prefix = "fix" if process_label == "bug-fix" else "feat"
    if not branch_name.startswith(f"{default_prefix}/"):
        branch_name = f"{default_prefix}/{task_name}"
    base_branch = inputs.get("base_branch") or "main"
    if process_label == "bug-fix":
        default_docs = resolve_documentation_fixes_path(repo)
    else:
        default_docs = resolve_documentation_features_path(repo)
    persist_ref = inputs.get("persist_ref") or f"{default_docs}/{task_name}"
    refined = (
        inputs.get("refined_requirements")
        or inputs.get("refactor_goal")
        or inputs.get("bug_summary")
        or inputs.get("description")
        or ""
    )

    if not isinstance(branch_name, str) or not branch_name.strip():
        raise ValueError("branch_name inválido")
    if not isinstance(base_branch, str) or not base_branch.strip():
        raise ValueError("base_branch inválido")
    if not isinstance(persist_ref, str) or not persist_ref.strip():
        raise ValueError("persist_ref inválido")

    git_steps: list[dict[str, Any]] = [
        {"op": "fetch", "result": invoke_git_manager(repo, "fetch", {"remote": "origin", "prune": True})},
        {
            "op": "checkout_base",
            "result": invoke_git_manager(
                repo,
                "checkout",
                {"branch_name": base_branch.strip(), "create_if_not_exists": False},
            ),
        },
        {
            "op": "pull_base",
            "result": invoke_git_manager(
                repo,
                "pull",
                {"remote": "origin", "branch": base_branch.strip()},
            ),
        },
    ]
    try:
        git_steps.append(
            {
                "op": "checkout_feature",
                "result": invoke_git_manager(
                    repo,
                    "checkout",
                    {"branch_name": branch_name.strip(), "create_if_not_exists": True},
                ),
            }
        )
    except RuntimeError:
        git_steps.append(
            {
                "op": "checkout_feature_existing",
                "result": invoke_git_manager(
                    repo,
                    "checkout",
                    {"branch_name": branch_name.strip(), "create_if_not_exists": False},
                ),
            }
        )

    persist_dir = repo / persist_ref
    persist_dir.mkdir(parents=True, exist_ok=True)
    objectives_path = persist_dir / "objectives.md"
    created = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    if not objectives_path.is_file():
        summary = refined.strip() if isinstance(refined, str) else f"{process_label} {task_name}"
        objectives_path.write_text(
            f"""---
feature_name: {task_name}
created: "{created}"
process: {process_label}
branch_name: {branch_name.strip()}
persist_ref: {persist_ref.strip()}
---

# Objetivos — {task_name}

## Misión

{summary}

## Alcance (manifiesto)

Inicialización de contexto vía intérprete dinámico `execute-process.py` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
""",
            encoding="utf-8",
        )

    return {
        "feature_name": task_name,
        "task_name": task_name,
        "process_label": process_label,
        "branch_name": branch_name.strip(),
        "persist_ref": persist_ref.strip(),
        "objectives_path": str(objectives_path.relative_to(repo)).replace("\\", "/"),
        "git_steps": git_steps,
    }


def run_skill_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("skill_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("skill_name requerido")
    skill_path = repo / "SddIA" / "skills" / f"{name}.md"
    if skill_path.is_file() and inputs.get("lifecycle_operation", "create") == "create":
        raise FileExistsError(f"Ya existe {skill_path}")

    context = inputs.get("skill_context", "ecosystem-evolution")
    version = inputs.get("skill_version", "1.0.0")
    contract_ver = inputs.get("skills_contract_version", "1.1.0")
    desc = inputs.get("skill_description", f"Skill {name}")
    in_schema = inputs.get("skill_inputs_schema", [])
    out_schema = inputs.get("skill_outputs_schema", [])

    skill_uuid = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    canon = {
        "skill_context": context,
        "skill_inputs_schema": in_schema,
        "skill_name": name,
        "skill_outputs_schema": out_schema,
        "skill_version": version,
    }
    hex_sig = crypto(
        repo,
        {
            "operation": "GENERATE_SHA256",
            "target_type": "STRING",
            "target_payload": json.dumps(
                canon, sort_keys=True, separators=(",", ":"), ensure_ascii=False
            ),
        },
    )
    hash_sig = f"sha256:{hex_sig}"
    cap = name.replace("-", "_")[:32] or "skill-cap"

    body = f"""---
uuid: "{skill_uuid}"
name: "{name}"
version: "{version}"
contract: "skills-contract v{contract_ver}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
inputs:
  - "inputs_placeholder": "definir segun skill_inputs_schema en forja completa"
outputs:
  - "success": "boolean"
---

# Skill: {name}

{desc}
"""
    skill_path.parent.mkdir(parents=True, exist_ok=True)
    skill_path.write_text(body, encoding="utf-8")

    index_path = repo / "SddIA" / "skills" / "index.md"
    row = (
        f"| `{name}.md` | `{skill_uuid}` | {name} | {version} | "
        f"skills-contract v{contract_ver} | {context} | `{cap}` |"
    )
    idx = index_path.read_text(encoding="utf-8")
    if name not in idx:
        idx = idx.replace("| `shell-executor.md` |", row + "\n| `shell-executor.md` |", 1)
        if name not in idx:
            idx = idx.rstrip() + "\n" + row + "\n"
        index_path.write_text(idx, encoding="utf-8")

    return {
        "artifact_skill_md": str(skill_path.relative_to(repo)).replace("\\", "/"),
        "artifact_skills_index": "SddIA/skills/index.md",
        "handoff_entity_uuid": skill_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


def resolve_effective_event_family(inputs: dict[str, Any]) -> str:
    """Familia Trinidad obligatoria en invocación (sin fallback domain)."""
    raw = inputs.get("event_family")
    if not isinstance(raw, str) or not raw.strip():
        raise ValueError("event_family requerido (telemetry | orchestration | domain)")
    effective = raw.strip().lower()
    if effective not in TRINITY_EVENT_FAMILIES:
        raise ValueError(
            f"event_family inválido: {raw!r}; debe ser telemetry, orchestration o domain"
        )
    return effective


def _event_family_subscriptions_note(family: str, event_type: str) -> str:
    registry = {
        "domain": "event-domain-subscriptions.json",
        "telemetry": "event-telemetry-subscriptions.json",
        "orchestration": "event-orchestration-subscriptions.json",
    }
    rel = registry.get(family, "event-subscriptions.json")
    return f"Ver `SddIA/core/{rel}` → clave `{event_type}`."


def run_event_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("event_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("event_name requerido")
    effective_family = resolve_effective_event_family(inputs)
    events_root = repo / "SddIA" / "events" / effective_family
    event_path = events_root / f"{name}.md"
    if event_path.is_file() and inputs.get("lifecycle_operation", "create") == "create":
        raise FileExistsError(f"Ya existe {event_path}")

    event_type = inputs.get("event_type")
    if not isinstance(event_type, str) or not event_type.strip():
        raise ValueError("event_type requerido")
    context = inputs.get("event_context", "ecosystem-evolution")
    version = inputs.get("event_version", "1.0.0")
    contract_ver = inputs.get("events_contract_version", "1.1.0")
    desc = inputs.get("event_description", f"Clase de Evento {event_type}")
    payload_required = inputs.get("payload_required", [])
    payload_optional = inputs.get("payload_optional", [])
    payload_forbidden = inputs.get("payload_forbidden", [])
    emitters = inputs.get("emitter_agents", [])

    event_uuid = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    canon = {
        "event_name": name,
        "event_type": event_type,
        "event_family": effective_family,
        "event_version": version,
        "event_context": context,
        "payload_required": payload_required,
        "payload_optional": payload_optional,
        "payload_forbidden": payload_forbidden,
    }
    hex_sig = crypto(
        repo,
        {
            "operation": "GENERATE_SHA256",
            "target_type": "STRING",
            "target_payload": json.dumps(
                canon, sort_keys=True, separators=(",", ":"), ensure_ascii=False
            ),
        },
    )
    hash_sig = f"sha256:{hex_sig}"
    cap = name.replace("-", "_")[:32] or "event-cap"

    req_lines = "\n".join(f"- `{f}`" for f in payload_required) or "- *(ninguno)*"
    opt_lines = "\n".join(f"- `{f}`" for f in payload_optional) or "- *(ninguno)*"
    forb_lines = "\n".join(f"- `{f}`" for f in payload_forbidden) or "- *(ninguno)*"
    emitter_lines = "\n".join(f"- `{e}`" for e in emitters) or "- *(definir en forja completa)*"

    subs_note = _event_family_subscriptions_note(effective_family, event_type)
    body = f"""---
uuid: "{event_uuid}"
name: "{name}"
version: "{version}"
contract: "events-contract v{contract_ver}"
event_family: "{effective_family}"
event_type: "{event_type}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
---

# Event: {event_type}

{desc}

## Payload ECST

### REQUIRED
{req_lines}

### OPTIONAL
{opt_lines}

### FORBIDDEN
{forb_lines}

## Emisores autorizados

{emitter_lines}

## Suscripciones

{subs_note}
"""
    event_path.parent.mkdir(parents=True, exist_ok=True)
    event_path.write_text(body, encoding="utf-8")

    index_path = events_root / "index.md"
    row = (
        f"| `{name}.md` | `{event_uuid}` | {name} | {event_type} | {version} | "
        f"events-contract v{contract_ver} | {context} | `{cap}` |"
    )
    if index_path.is_file():
        idx = index_path.read_text(encoding="utf-8")
        marker = "| Archivo fuente | uuid | name | event_type |"
        if name not in idx:
            if marker in idx:
                idx = idx.replace(
                    "| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |\n"
                    "|----------------|------|------|------------|---------|----------|---------|--------------|\n",
                    "| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |\n"
                    "|----------------|------|------|------------|---------|----------|---------|--------------|\n"
                    + row + "\n",
                    1,
                )
            else:
                idx = idx.rstrip() + "\n" + row + "\n"
            index_path.write_text(idx, encoding="utf-8")

    rel_index = str(index_path.relative_to(repo)).replace("\\", "/")
    return {
        "artifact_event_md": str(event_path.relative_to(repo)).replace("\\", "/"),
        "artifact_events_index": rel_index,
        "handoff_entity_uuid": event_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


def materialize_forge_by_inputs(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    """Forja física según entity_class o forma del contrato de inputs."""
    entity_class = inputs.get("entity_class")
    if isinstance(entity_class, str) and entity_class in FORGE_BY_ENTITY_CLASS:
        return FORGE_BY_ENTITY_CLASS[entity_class](repo, inputs)
    if inputs.get("skill_name") is not None or (
        inputs.get("skill_inputs_schema") is not None and inputs.get("skill_context") is not None
    ):
        return run_skill_forge(repo, inputs)
    if inputs.get("event_type") is not None or inputs.get("event_name") is not None:
        return run_event_forge(repo, {**inputs, "lifecycle_operation": inputs.get("lifecycle_operation", "create")})
    if inputs.get("tool_name") is not None:
        return FORGE_BY_ENTITY_CLASS["tool"](repo, inputs)
    if inputs.get("action_name") is not None:
        return FORGE_BY_ENTITY_CLASS["action"](repo, inputs)
    if inputs.get("process_name") is not None:
        return FORGE_BY_ENTITY_CLASS["process"](repo, inputs)
    if inputs.get("agent_name") is not None:
        return FORGE_BY_ENTITY_CLASS["agent"](repo, inputs)
    if inputs.get("tactical_norm_name") is not None:
        return FORGE_BY_ENTITY_CLASS["norm"](repo, inputs)
    if inputs.get("domain_codex_slug") is not None:
        return FORGE_BY_ENTITY_CLASS["codex"](repo, inputs)
    if inputs.get("suite_name") is not None or inputs.get("atomic_nodes") is not None:
        return FORGE_BY_ENTITY_CLASS["suite"](repo, inputs)
    raise NotImplementedError(
        "Forja física no disponible para esta forma de inputs"
    )


def _base_creator_inputs(
    entity_class: str, entity_name: str, lifecycle: str, seed: dict[str, Any]
) -> dict[str, Any]:
    scope = seed.get("scope", "core")
    origin = "local" if scope == "local" else "core"
    return {
        "entity_class": entity_class,
        "lifecycle_operation": lifecycle,
        "origin_topology": seed.get("origin_topology", origin),
    }


def creator_inputs_from_entity(
    entity_class: str, entity_name: str, lifecycle: str, seed: dict[str, Any]
) -> dict[str, Any]:
    base = _base_creator_inputs(entity_class, entity_name, lifecycle, seed)
    if entity_class == "skill":
        return {
            **base,
            "skill_name": seed.get("skill_name", entity_name),
            "skill_context": seed.get("skill_context", "ecosystem-evolution"),
            "skill_description": seed.get("skill_description", ""),
            "skill_inputs_schema": seed.get("skill_inputs_schema", []),
            "skill_outputs_schema": seed.get("skill_outputs_schema", []),
            "skill_version": seed.get("skill_version", "1.0.0"),
            "skills_contract_version": seed.get("skills_contract_version", "1.1.0"),
        }
    if entity_class == "event":
        out: dict[str, Any] = {
            **base,
            "event_name": seed.get("event_name", entity_name),
            "event_type": seed.get("event_type", ""),
            "event_context": seed.get("event_context", "ecosystem-evolution"),
            "event_description": seed.get("event_description", ""),
            "payload_required": seed.get("payload_required", []),
            "payload_optional": seed.get("payload_optional", []),
            "payload_forbidden": seed.get("payload_forbidden", []),
            "emitter_agents": seed.get("emitter_agents", []),
            "event_version": seed.get("event_version", "1.0.0"),
            "events_contract_version": seed.get("events_contract_version", "1.1.0"),
        }
        if "event_family" in seed:
            out["event_family"] = seed["event_family"]
        return out
    if entity_class == "tool":
        tname = seed.get("tool_name", entity_name)
        return {
            **base,
            "tool_name": tname,
            "tool_id": seed.get("tool_id", tname),
            "scope": seed.get("scope", "core"),
            "domain_origin": seed.get("domain_origin", "SddIA"),
            "tool_context": seed.get("tool_context", "ecosystem-evolution"),
            "required_secrets": seed.get("required_secrets", []),
            "dependencies": seed.get("dependencies", []),
            "tool_outputs": seed.get("tool_outputs", []),
            "execution_logic": seed.get("execution_logic", f"Tool {entity_name}"),
            "tools_contract_version": seed.get("tools_contract_version", "1.2.0"),
        }
    if entity_class == "action":
        return {
            **base,
            "action_name": seed.get("action_name", entity_name),
            "action_context": seed.get("action_context", "ecosystem-evolution"),
            "action_inputs": seed.get("action_inputs", []),
            "action_outputs": seed.get("action_outputs", []),
            "orchestration_logic": seed.get("orchestration_logic", f"Acción {entity_name}"),
            "actions_contract_version": seed.get("actions_contract_version", "1.2.0"),
        }
    if entity_class == "process":
        return {
            **base,
            "process_name": seed.get("process_name", entity_name),
            "process_description": seed.get("process_description", f"Proceso {entity_name}"),
            "process_context": seed.get("process_context", "ecosystem-evolution"),
            "process_phases": seed.get("process_phases", [{"name": "Fase inicial", "intent": "stub"}]),
            "process_contract_version": seed.get("process_contract_version", "1.3.0"),
            "process_aliases": seed.get("process_aliases", []),
        }
    if entity_class == "agent":
        return {
            **base,
            "agent_name": seed.get("agent_name", entity_name),
            "allowed_policies": seed.get("allowed_policies", ["ecosystem-evolution"]),
            "agent_inputs": seed.get("agent_inputs", []),
            "agent_outputs": seed.get("agent_outputs", []),
            "agent_purpose": seed.get("agent_purpose", f"Agente {entity_name}"),
            "agents_contract_version": seed.get("agents_contract_version", "1.0.0"),
        }
    if entity_class == "norm":
        return {
            **base,
            "tactical_norm_name": seed.get("tactical_norm_name", entity_name),
            "tactical_norm_version": seed.get("tactical_norm_version", "1.0.0"),
            "tactical_norm_friction": seed.get("tactical_norm_friction", f"Norma {entity_name}"),
            "tactical_norm_author": seed.get("tactical_norm_author", "laboratorio"),
            "tactical_norm_dependencies": seed.get("tactical_norm_dependencies", []),
            "norms_contract_version": seed.get("norms_contract_version", "1.0.0"),
            "norm_scope": seed.get("norm_scope", "agnostic"),
            "norm_category": seed.get("norm_category", "workflow"),
        }
    if entity_class == "codex":
        return {
            **base,
            "domain_codex_slug": seed.get("domain_codex_slug", entity_name),
            "domain_codex_name": seed.get("domain_codex_name", entity_name),
            "domain_codex_version": seed.get("domain_codex_version", "1.0.0"),
            "domain_codex_author": seed.get("domain_codex_author", "laboratorio"),
            "target_environment": seed.get("target_environment", ["dev"]),
            "tactical_norm_inventory": seed.get("tactical_norm_inventory", []),
            "codex_contract_version": seed.get("codex_contract_version", "1.0.0"),
            "domain_codex_certification_grade": seed.get("domain_codex_certification_grade", "Pendiente"),
        }
    if entity_class == "suite":
        return {
            **base,
            "suite_name": seed.get("suite_name", entity_name),
            "suite_context": seed.get("suite_context", "chaos-engineering"),
            "execution_strategy": seed.get("execution_strategy", "run_all"),
            "atomic_nodes": seed.get("atomic_nodes", []),
            "suite_version": seed.get("suite_version", "1.0.0"),
            "suites_contract_version": seed.get("suites_contract_version", "1.0.0"),
        }
    raise NotImplementedError(f"mapeo semantic_seed no definido para entity_class={entity_class}")


def write_pending_event(repo: Path, event: dict[str, Any]) -> dict[str, str]:
    bus = ensure_event_bus_topology(repo)
    pending = repo / bus["pending"]
    event_id = event["event_id"]
    target = pending / f"{event_id}.json"
    target.write_text(json.dumps(event, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return {
        "event_id": event_id,
        "target_path": str(target.relative_to(repo)).replace("\\", "/"),
    }


def emit_domain_mutation(repo: Path, payload: dict[str, Any]) -> dict[str, Any]:
    op = payload["lifecycle_operation"]
    event_type = {
        "create": "Domain_Entity_Created",
        "update": "Domain_Entity_Updated",
        "delete": "Domain_Entity_Deleted",
    }[op]

    entity_uuid = payload.get("entity_uuid")
    if entity_uuid and op != "delete":
        existing = find_existing_domain_event(repo, entity_uuid, op, event_type)
        if existing and existing.get("event_id"):
            return {"idempotent": True, **existing}

    origin_topology = payload.get("origin_topology", "core")
    if origin_topology not in ("core", "local"):
        origin_topology = "core"

    event_id = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    event = {
        "event_id": event_id,
        "event_type": event_type,
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": payload.get("emitter_agent", "entity-manager"),
        "payload": {
            "entity_class": payload["entity_class"],
            "entity_type": payload["entity_class"],
            "entity_id": entity_uuid,
            "lifecycle_operation": op,
            "entity_uuid": entity_uuid,
            "entity_name": payload["entity_name"],
            "version": payload.get("version"),
            "hash_signature_new": payload.get("hash_signature_new"),
            "hash_signature_old": payload.get("hash_signature_old"),
            "origin_topology": origin_topology,
            "changes_summary": payload.get(
                "changes_summary",
                f"{op} {payload['entity_class']} {payload['entity_name']}",
            ),
        },
        "delivery_state": {},
    }
    ok, errors = validate_domain_mutation_event(repo, event)
    if not ok:
        raise ValueError("; ".join(errors))
    return write_pending_event(repo, event)


def run_phase_invocations(
    repo: Path,
    inv_block: dict[str, Any],
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> list[dict[str, Any]]:
    invocations = inv_block.get("invocations") or []
    log: list[dict[str, Any]] = []
    for inv in invocations:
        if not isinstance(inv, dict):
            continue
        capsule = inv.get("capsule", "")
        if capsule != "action:crypto-broker":
            log.append({"capsule": capsule, "status": "skipped", "note": "cápsula no ejecutada en lab v1"})
            continue
        stdin = inv.get("stdin_json")
        if not isinstance(stdin, dict):
            log.append({"capsule": capsule, "status": "skipped", "note": "stdin_json ausente"})
            continue
        result = crypto(repo, stdin)
        binds = inv.get("bind") or {}
        if isinstance(binds, dict):
            for path, var in binds.items():
                if path == "data.result":
                    state[var] = result
        log.append({"capsule": capsule, "status": "executed", "bind": binds})
    return log


def capsule_action_execute_process(
    repo: Path,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any]:
    entity_class = inputs.get("entity_class")
    entity_name = inputs.get("entity_name")
    lifecycle = inputs.get("lifecycle_operation")
    if lifecycle == "delete":
        return {"skipped": True, "reason": "delete omite delegación al creator"}
    if not isinstance(entity_class, str) or entity_class not in CREATOR_BY_CLASS:
        raise ValueError(f"entity_class no resuelta: {entity_class}")
    if entity_class not in PILOT_ENTITY_CLASSES:
        raise NotImplementedError(
            f"entity_class '{entity_class}' fuera del piloto v1 ({', '.join(sorted(PILOT_ENTITY_CLASSES))})"
        )
    creator = CREATOR_BY_CLASS[entity_class]
    seed = dict(inputs.get("semantic_seed") or {})
    child_inputs = creator_inputs_from_entity(
        entity_class, str(entity_name), str(lifecycle), seed
    )
    forge: dict[str, Any] = {}
    try:
        forge = materialize_forge_by_inputs(repo, child_inputs)
        state["handoff"].update(forge)
    except NotImplementedError:
        pass
    if forge.get("handoff_entity_uuid"):
        return {"child_process": creator, "handoff": state["handoff"], "forge_only": True}
    data = invoke_subprocess_process(repo, creator, child_inputs)
    if data.get("handoff"):
        state["handoff"].update(data["handoff"])
    else:
        state["handoff"].update({k: v for k, v in data.items() if k.startswith("handoff_")})
    return {"child_process": creator, "handoff": state["handoff"]}


def capsule_filesystem_delete(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    entity_class = inputs.get("entity_class")
    entity_name = inputs.get("entity_name")
    if inputs.get("lifecycle_operation") != "delete":
        return {"skipped": True}
    rel_dir = DIR_BY_CLASS.get(str(entity_class))
    if not rel_dir:
        raise ValueError(f"entity_class desconocida: {entity_class}")
    artifact = repo / rel_dir / f"{entity_name}.md"
    if not artifact.is_file():
        raise FileNotFoundError(str(artifact))
    fm = parse_frontmatter(artifact)
    handoff = {
        "handoff_entity_uuid": fm.get("uuid"),
        "handoff_hash_signature_new": None,
        "handoff_hash_signature_old": fm.get("hash_signature"),
        "handoff_version": fm.get("version"),
    }
    artifact.unlink()
    state["handoff"].update(handoff)
    return {"deleted": str(artifact), **handoff}


def invoke_capsule_action(
    repo: Path, action_name: str, action_inputs: dict[str, Any]
) -> dict[str, Any]:
    env = os.environ.copy()
    env["PYTHONIOENCODING"] = "utf-8"
    proc = subprocess.run(
        [
            sys.executable,
            str(EXECUTE_ACTION_CLI),
            "--action",
            action_name,
            "--inputs",
            json.dumps(action_inputs, ensure_ascii=False),
        ],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
        env=env,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or f"acción {action_name} sin salida")
    body = json.loads(line)
    if _ACTIVE_CAPSULE_CAPTURE_STATE is not None:
        _ACTIVE_CAPSULE_CAPTURE_STATE["last_capsule_id"] = action_name
        _ACTIVE_CAPSULE_CAPTURE_STATE["last_capsule_envelope"] = body
    if not body.get("success"):
        raise RuntimeError(body.get("error") or f"acción {action_name} falló")
    return body.get("data") or {}


def run_eda_audit_scan(repo: Path) -> dict[str, Any]:
    proc = subprocess.run(
        [sys.executable, str(AUDIT_EDA_CLI), "--scan", "--json"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    line = (proc.stdout or "").strip()
    if not line:
        raise RuntimeError(proc.stderr or "audit-entity-eda-coverage sin salida")
    return json.loads(line)


def _backfill_manifest_active(repo: Path, persist_ref: str | None) -> bool:
    if not isinstance(persist_ref, str) or not persist_ref.strip():
        return False
    manifest = repo / persist_ref.strip() / "backfill-manifest.json"
    if not manifest.is_file():
        return False
    try:
        data = json.loads(manifest.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return False
    return bool(data.get("correlation_id")) and not data.get("merkle_anchored")


def capsule_eda_genomic_audit_gate(
    repo: Path, inputs: dict[str, Any], state: dict[str, Any]
) -> dict[str, Any]:
    report = run_eda_audit_scan(repo)
    orphan_count = int(report.get("orphan_count") or 0)
    state["eda_audit"] = report
    persist_ref = inputs.get("persist_ref")
    if orphan_count > 0 and _backfill_manifest_active(repo, str(persist_ref) if persist_ref else None):
        verdict = "warn"
        noise = "backfill Fase C en curso"
    elif orphan_count > 0:
        verdict = "block"
        noise = "Ruido de Sistema"
        state["argos_verdict"] = "block"
    else:
        verdict = "pass"
        noise = None
    return {
        "verdict": verdict,
        "orphan_count": orphan_count,
        "argos_noise": noise,
        "scanned_at": report.get("scanned_at"),
    }


def capsule_emit_domain_mutation(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    handoff = state.get("handoff") or {}
    seed = dict(inputs.get("semantic_seed") or {})
    scope = seed.get("scope", "core")
    origin_topology = handoff.get("origin_topology") or seed.get("origin_topology")
    if not origin_topology:
        origin_topology = "local" if scope == "local" else "core"
    action_inputs = {
        "entity_class": inputs.get("entity_class"),
        "entity_name": inputs.get("entity_name"),
        "lifecycle_operation": inputs.get("lifecycle_operation"),
        "entity_uuid": handoff.get("handoff_entity_uuid"),
        "version": handoff.get("handoff_version"),
        "hash_signature_new": handoff.get("handoff_hash_signature_new"),
        "hash_signature_old": handoff.get("handoff_hash_signature_old"),
        "origin_topology": origin_topology,
        "emitter_agent": inputs.get("emitter_agent", "entity-manager"),
        "changes_summary": inputs.get(
            "changes_summary",
            f"{inputs.get('lifecycle_operation')} {inputs.get('entity_class')} {inputs.get('entity_name')}",
        ),
    }
    seal = invoke_capsule_action(repo, "emit-domain-mutation", action_inputs)
    state["handoff"].update(seal)
    return seal


def try_execute_registered_action_capsules(
    repo: Path,
    delegates: list[Any],
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if not isinstance(delegates, list):
        return None
    for capsule in delegates:
        if not isinstance(capsule, str):
            continue
        action_name = CAPSULE_ACTION_REGISTRY.get(capsule)
        if not action_name:
            continue
        if action_name == "emit-domain-mutation" and inputs.get("entity_class"):
            return capsule_emit_domain_mutation(repo, inputs, state)
        if action_name == "emit-pr-presented-event":
            branch = inputs.get("branch") or inputs.get("branch_name")
            if isinstance(branch, str) and branch.strip():
                action_inputs: dict[str, Any] = {
                    "branch": branch.strip(),
                    "status": inputs.get("status", "presented"),
                    "emitter_agent": inputs.get("emitter_agent", "delivery-close-cycle"),
                }
                pr_url = inputs.get("pr_url")
                if isinstance(pr_url, str) and pr_url.strip():
                    action_inputs["pr_url"] = pr_url.strip()
                corr = inputs.get("correlation_id")
                if isinstance(corr, str) and corr.strip():
                    action_inputs["correlation_id"] = corr.strip()
                return invoke_capsule_action(repo, action_name, action_inputs)
        if action_name == "emit-pr-merged-event":
            if inputs.get("merge_commit_hash") or inputs.get("hash_signature"):
                return invoke_capsule_action(repo, action_name, dict(inputs))
    return None


def invoke_chaos_tool_capsule(
    repo: Path,
    tool_name: str,
    payload: dict[str, Any],
) -> tuple[int, dict[str, Any]]:
    script = CHAOS_TOOL_SCRIPTS.get(tool_name)
    if script is None or not script.is_file():
        raise FileNotFoundError(f"cápsula caos no encontrada: {tool_name}")
    proc = subprocess.run(
        ["wasmtime", "run", "--dir=.", str(script)],
        input=json.dumps(payload, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        cwd=str(repo),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    body: dict[str, Any] = {}
    if line:
        try:
            parsed = json.loads(line)
            if isinstance(parsed, dict):
                body = parsed
        except json.JSONDecodeError:
            body = {"parse_error": line[:200]}
    return proc.returncode, body


def _chaos_workspace_path(inputs: dict[str, Any], state: dict[str, Any]) -> str:
    sync_workspace_context(inputs, state)
    ws = inputs.get("workspace_path") or state.get("workspace_path")
    if not isinstance(ws, str) or not ws.strip():
        raise ValueError("workspace_path ausente")
    return ws.strip()


def _chaos_stimulus_thermodynamic(
    repo: Path,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any]:
    ws = _chaos_workspace_path(inputs, state)
    code, body = invoke_chaos_tool_capsule(
        repo,
        "io-choke",
        {"workspace_path": ws, "target_file": ".telemetry-stress-target"},
    )
    state["last_capsule_id"] = "io-choke"
    state["last_capsule_envelope"] = body
    state["chaos_stimulus"] = {"tool": "io-choke", "exit_code": code, "envelope": body}
    state["chaos_simulate_telemetry_io_fail"] = True
    ok = code == 0 and bool(body.get("success")) and bool((body.get("result") or {}).get("io_choked"))
    if not ok:
        return {
            "status": "failed",
            "handler": "chaos-audit-stimulus",
            "tool": "io-choke",
            "exit_code": code,
            "envelope": body,
        }
    return {
        "status": "executed",
        "handler": "chaos-audit-stimulus",
        "tool": "io-choke",
        "io_choked": True,
    }


def _chaos_stimulus_compliance(
    repo: Path,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any]:
    _chaos_workspace_path(inputs, state)
    code, body = invoke_chaos_tool_capsule(
        repo,
        "schema-corruptor",
        {"corruption_mode": "empty"},
    )
    state["last_capsule_id"] = "schema-corruptor"
    state["last_capsule_envelope"] = body
    state["chaos_stimulus"] = {"tool": "schema-corruptor", "exit_code": code, "envelope": body}
    ok = code == 0 and bool(body.get("success")) and "telemetry_receipt" not in body
    if not ok:
        return {
            "status": "failed",
            "handler": "chaos-audit-stimulus",
            "tool": "schema-corruptor",
            "exit_code": code,
            "envelope": body,
        }
    return {
        "status": "executed",
        "handler": "chaos-audit-stimulus",
        "tool": "schema-corruptor",
        "corruption_mode": "empty",
    }


def _chaos_stimulus_sandbox(
    repo: Path,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any]:
    ws = _chaos_workspace_path(inputs, state)
    escape = "../breach-marker.txt"
    code, body = invoke_chaos_tool_capsule(
        repo,
        "sandbox-breacher",
        {"workspace_path": ws, "escape_target": escape},
    )
    state["last_capsule_id"] = "sandbox-breacher"
    state["last_capsule_envelope"] = body
    state["chaos_stimulus"] = {"tool": "sandbox-breacher", "exit_code": code, "envelope": body}
    escape_path = (Path(ws).resolve().parent / "breach-marker.txt").resolve()
    blocked = (
        code == 1
        and not body.get("success")
        and body.get("exitCode") == 1
        and not escape_path.is_file()
    )
    if not blocked:
        return {
            "status": "failed",
            "handler": "chaos-audit-stimulus",
            "tool": "sandbox-breacher",
            "exit_code": code,
            "envelope": body,
            "escape_exists": escape_path.is_file(),
        }
    return {
        "status": "executed",
        "handler": "chaos-audit-stimulus",
        "tool": "sandbox-breacher",
        "breach_blocked": True,
    }


def _find_telemetry_compliance_breach(repo: Path) -> str | None:
    from eda_bus_utils import load_eda_fractal

    fractal = load_eda_fractal(repo)
    domain_dir = repo / fractal["domain"]
    if not domain_dir.is_dir():
        return None
    for path in sorted(domain_dir.glob("*.json"), key=lambda p: p.stat().st_mtime, reverse=True):
        try:
            body = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
        if body.get("event_type") == "Telemetry_Compliance_Breached":
            try:
                return str(path.relative_to(repo)).replace("\\", "/")
            except ValueError:
                return str(path)
    return None


def _chaos_argos_certify(
    repo: Path,
    process_name: str,
    state: dict[str, Any],
    toll: dict[str, Any] | None,
) -> dict[str, Any]:
    if process_name == "audit-thermodynamic-toll-failsoft":
        ok = bool(toll and toll.get("telemetry_io_failed"))
        if ok:
            state["toll_failsoft_verified"] = True
        return {
            "phase_name": "Certificación Argos",
            "status": "executed" if ok else "failed",
            "handler": "chaos-audit-argos",
            "toll_failsoft_verified": ok,
            "telemetry_io_failed": bool(toll and toll.get("telemetry_io_failed")),
        }
    if process_name == "audit-telemetry-compliance-breach":
        breach_path = _find_telemetry_compliance_breach(repo)
        ok = breach_path is not None
        if ok:
            state["breach_event_path"] = breach_path
        audit = state.get("compliance_audit") or {}
        return {
            "phase_name": "Certificación Argos",
            "status": "executed" if ok else "failed",
            "handler": "chaos-audit-argos",
            "breach_event_path": breach_path,
            "compliance_status": audit.get("status"),
        }
    if process_name == "audit-sandbox-isolation-rbac":
        stimulus = state.get("chaos_stimulus") or {}
        ok = bool(stimulus.get("envelope", {}).get("result", {}).get("breach_blocked"))
        if ok:
            state["isolation_verified"] = True
        return {
            "phase_name": "Certificación Argos",
            "status": "executed" if ok else "failed",
            "handler": "chaos-audit-argos",
            "isolation_verified": ok,
        }
    return {
        "phase_name": "Certificación Argos",
        "status": "failed",
        "handler": "chaos-audit-argos",
        "error": f"proceso caos desconocido: {process_name}",
    }


def load_suite_spec(repo: Path, suite_id: str) -> dict[str, Any]:
    suite_path = repo / "SddIA" / "suites" / f"{suite_id}.md"
    if not suite_path.is_file():
        raise FileNotFoundError(f"Suite no encontrada: {suite_id}")
    spec = parse_frontmatter(suite_path)
    nodes = spec.get("atomic_nodes")
    if not isinstance(nodes, list) or not nodes:
        raise ValueError(f"atomic_nodes inválido en Suite {suite_id}")
    strategy = spec.get("execution_strategy", "run_all")
    if strategy not in ("fail_fast", "run_all"):
        raise ValueError(f"execution_strategy inválida: {strategy}")
    return spec


def emit_system_immunity_certified(
    repo: Path,
    *,
    suite_id: str,
    survival_manifest_path: str,
    orchestrator_execution_id: str,
    node_reports: list[dict[str, Any]],
    asset_id: str | None = None,
) -> dict[str, Any]:
    manifest_path = repo / survival_manifest_path
    hash_sig: str | None = None
    if manifest_path.is_file():
        digest = hashlib.sha256(manifest_path.read_bytes()).hexdigest()
        hash_sig = f"sha256:{digest}"
    nodes_passed = sum(1 for n in node_reports if n.get("verdict") == "pass")
    nodes_total = len(node_reports)
    payload: dict[str, Any] = {
        "suite_id": suite_id,
        "survival_manifest_path": survival_manifest_path,
        "orchestrator_execution_id": orchestrator_execution_id,
        "nodes_passed": nodes_passed,
        "nodes_total": nodes_total,
    }
    if isinstance(asset_id, str) and asset_id.strip():
        payload["asset_id"] = asset_id.strip()
    if hash_sig:
        payload["hash_signature_manifest"] = hash_sig
    event_id = str(uuid.uuid4())
    event = {
        "event_id": event_id,
        "event_type": "System_Immunity_Certified",
        "event_family": "domain",
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": "execute-suite",
        "payload": payload,
        "delivery_state": {},
    }
    seal = write_fractal_event(repo, event, "domain")
    route_out: dict[str, Any] | None = None
    if os.environ.get("SDDIA_LAB_ROUTE_SYNC", "").strip().lower() in ("1", "true", "yes"):
        from route_fractal_event_core import route_domain_fractal_event

        route_out = route_domain_fractal_event(repo, seal["target_path"])
    return {"event_id": event_id, "seal": seal, "route": route_out}


def compile_survival_manifest(
    repo: Path,
    orchestrator_ws: Path,
    suite_id: str,
    orchestrator_execution_id: str,
    execution_strategy: str,
    node_reports: list[dict[str, Any]],
) -> Path:
    manifest_path = orchestrator_ws / "survival-manifest.md"
    compiled_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    lines = [
        f"# Survival Manifest — {suite_id}",
        "",
        "| Campo | Valor |",
        "|-------|-------|",
        f"| suite_id | {suite_id} |",
        f"| orchestrator_execution_id | {orchestrator_execution_id} |",
        f"| execution_strategy | {execution_strategy} |",
        f"| compiled_at | {compiled_at} |",
        "",
        "## Nodos",
        "",
        "| # | process_name | execution_id | workspace_path | expected | actual | verdict |",
        "|---|--------------|--------------|----------------|----------|--------|---------|",
    ]
    for report in node_reports:
        ws_rel = report.get("workspace_path", "")
        if isinstance(ws_rel, str) and ws_rel:
            try:
                ws_rel = str(Path(ws_rel).resolve().relative_to(repo.resolve())).replace("\\", "/")
            except ValueError:
                pass
        lines.append(
            f"| {report.get('index', '')} | {report.get('process_name', '')} | "
            f"{report.get('execution_id', '')} | {ws_rel} | "
            f"{report.get('expected_exit_code', '')} | {report.get('actual_exit_code', '')} | "
            f"{report.get('verdict', '')} |"
        )
    manifest_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return manifest_path


def run_execute_suite(
    repo: Path,
    canonical: str,
    process_def: dict[str, Any],
    phases: list[dict[str, Any]],
    process_inputs: dict[str, Any],
) -> dict[str, Any]:
    suite_id = process_inputs.get("suite_id")
    if not isinstance(suite_id, str) or not suite_id.strip():
        return {
            "success": False,
            "status_code": 1,
            "data": None,
            "error": "suite_id requerido",
            "execution_report": {"process_name": canonical, "phases": [], "nodes": []},
        }

    state: dict[str, Any] = {"handoff": {}, "inputs": process_inputs}
    try:
        ws_boot = bootstrap_process_workspace(repo, canonical, process_def, process_inputs, state)
        state["workspace_boot"] = ws_boot
    except ValueError as exc:
        return {
            "success": False,
            "status_code": 1,
            "data": None,
            "error": str(exc),
            "execution_report": {"process_name": canonical, "phases": [], "nodes": []},
        }

    phase_reports: list[dict[str, Any]] = []
    try:
        suite_spec = load_suite_spec(repo, suite_id.strip())
    except (FileNotFoundError, ValueError) as exc:
        return {
            "success": False,
            "status_code": 1,
            "data": None,
            "error": str(exc),
            "execution_report": {"process_name": canonical, "phases": [], "nodes": []},
        }

    strategy_override = process_inputs.get("execution_strategy")
    execution_strategy = (
        strategy_override
        if isinstance(strategy_override, str) and strategy_override in ("fail_fast", "run_all")
        else str(suite_spec.get("execution_strategy", "run_all"))
    )
    atomic_nodes = suite_spec.get("atomic_nodes") or []
    orchestrator_ws = Path(str(state["workspace_path"])).resolve()

    phase_reports.append(
        {
            "phase_name": "Resolución Suite",
            "status": "executed",
            "handler": "load-suite-spec",
            "suite_id": suite_id.strip(),
            "node_count": len(atomic_nodes),
        }
    )

    node_reports: list[dict[str, Any]] = []
    abort = False
    for index, node in enumerate(atomic_nodes):
        if abort:
            break
        if not isinstance(node, dict):
            continue
        process_name = node.get("process_name")
        if not isinstance(process_name, str) or not process_name.strip():
            continue
        expected_exit = int(node.get("expected_exit_code", 0))
        child_execution_id = str(uuid.uuid4())
        child_ws = materialize_child_workspace(
            orchestrator_ws, index, process_name.strip(), child_execution_id
        )
        child_inputs: dict[str, Any] = {
            "workspace_path": str(child_ws),
            "execution_id": child_execution_id,
            "parent_execution_id": state.get("execution_id"),
            "parent_suite_id": suite_id.strip(),
        }
        started = time.monotonic()
        child_error: str | None = None
        try:
            child_body = invoke_subprocess_process_full(repo, process_name.strip(), child_inputs)
            actual_exit = int(child_body.get("status_code", 1 if not child_body.get("success") else 0))
            if not child_body.get("success"):
                child_error = str(child_body.get("error") or "subproceso falló")
        except RuntimeError as exc:
            actual_exit = 1
            child_error = str(exc)
        duration_ms = max(0, int((time.monotonic() - started) * 1000))
        verdict = "pass" if actual_exit == expected_exit else "fail"
        report = {
            "index": index,
            "process_name": process_name.strip(),
            "execution_id": child_execution_id,
            "workspace_path": str(child_ws),
            "expected_exit_code": expected_exit,
            "actual_exit_code": actual_exit,
            "duration_ms": duration_ms,
            "verdict": verdict,
        }
        if child_error:
            report["error"] = child_error
        node_reports.append(report)
        if execution_strategy == "fail_fast" and verdict != "pass":
            abort = True

    phase_reports.append(
        {
            "phase_name": "Orquestación nodos",
            "status": "executed",
            "handler": "execute-suite-orchestrator",
            "nodes_executed": len(node_reports),
        }
    )

    manifest_path = compile_survival_manifest(
        repo,
        orchestrator_ws,
        suite_id.strip(),
        str(state.get("execution_id", "")),
        execution_strategy,
        node_reports,
    )
    manifest_rel = str(manifest_path.relative_to(repo)).replace("\\", "/")
    phase_reports.append(
        {
            "phase_name": "Compilación manifiesto",
            "status": "executed",
            "handler": "compile-survival-manifest",
            "survival_manifest_path": manifest_rel,
        }
    )

    all_pass = bool(node_reports) and all(n.get("verdict") == "pass" for n in node_reports)
    data: dict[str, Any] = {
        "process_name": canonical,
        "suite_id": suite_id.strip(),
        "execution_strategy": execution_strategy,
        "survival_manifest_path": manifest_rel,
        "nodes_executed": len(node_reports),
        "workspace_path": str(orchestrator_ws),
        "execution_id": state.get("execution_id"),
    }

    if all_pass:
        asset_id = suite_spec.get("uuid")
        if not isinstance(asset_id, str):
            asset_id = process_inputs.get("asset_id")
        immunity = emit_system_immunity_certified(
            repo,
            suite_id=suite_id.strip(),
            survival_manifest_path=manifest_rel,
            orchestrator_execution_id=str(state.get("execution_id", "")),
            node_reports=node_reports,
            asset_id=asset_id if isinstance(asset_id, str) else None,
        )
        phase_reports.append(
            {
                "phase_name": "Certificación inmunidad",
                "status": "executed",
                "handler": "emit-system-immunity-certified",
                "immunity_event_id": immunity.get("event_id"),
                "immunity_event_path": (immunity.get("seal") or {}).get("target_path"),
            }
        )
        data["immunity_event_id"] = immunity.get("event_id")
        data["immunity_event_path"] = (immunity.get("seal") or {}).get("target_path")

    return {
        "success": all_pass,
        "status_code": 0 if all_pass else 1,
        "data": data,
        "error": None if all_pass else "execute-suite: uno o más nodos fallaron",
        "execution_report": {
            "process_name": canonical,
            "suite_id": suite_id.strip(),
            "execution_strategy": execution_strategy,
            "nodes": node_reports,
            "phases": phase_reports,
        },
    }


def run_chaos_audit_process(
    repo: Path,
    canonical: str,
    process_def: dict[str, Any],
    phases: list[dict[str, Any]],
    process_inputs: dict[str, Any],
) -> dict[str, Any]:
    from telemetry_compliance_audit_core import audit_telemetry_compliance

    state: dict[str, Any] = {"handoff": {}, "inputs": process_inputs}
    state["asset_id"] = str(uuid.uuid4())
    toll_start = time.monotonic()
    try:
        ws_boot = bootstrap_process_workspace(repo, canonical, process_def, process_inputs, state)
        state["workspace_boot"] = ws_boot
    except ValueError as exc:
        return {
            "success": False,
            "status_code": 1,
            "data": None,
            "error": str(exc),
            "execution_report": {"process_name": canonical, "phases": []},
        }

    phase_reports: list[dict[str, Any]] = []
    stimulus_handlers = {
        "Estímulo asfixia E/S": _chaos_stimulus_thermodynamic,
        "Estímulo alucinación recibo": _chaos_stimulus_compliance,
        "Estímulo intento de fuga": _chaos_stimulus_sandbox,
    }

    for phase in phases:
        if not isinstance(phase, dict):
            continue
        phase_name = str(phase.get("name") or "")
        if phase_name == "Certificación Argos":
            continue
        handler = stimulus_handlers.get(phase_name)
        if handler is None:
            phase_reports.append(
                {
                    "phase_name": phase_name,
                    "status": "simulated",
                    "delegates_to": phase.get("delegates_to"),
                }
            )
            continue
        result = handler(repo, process_inputs, state)
        phase_reports.append({"phase_name": phase_name, **result})
        if result.get("status") != "executed":
            return {
                "success": False,
                "status_code": 1,
                "data": {"process_name": canonical, "workspace_path": state.get("workspace_path")},
                "error": f"estímulo caos falló: {phase_name}",
                "execution_report": {"process_name": canonical, "phases": phase_reports},
            }

    duration_ms = max(0, int((time.monotonic() - toll_start) * 1000))
    toll: dict[str, Any] | None = None
    if canonical != "audit-sandbox-isolation-rbac":
        toll = run_thermodynamic_toll(
            repo,
            canonical,
            state,
            process_inputs,
            exit_code=0,
            duration_ms=duration_ms,
            success=True,
        )
        if canonical == "audit-telemetry-compliance-breach":
            telemetry = toll.get("telemetry") or {}
            rel = telemetry.get("target_path")
            if isinstance(rel, str) and rel.strip():
                state["compliance_audit"] = audit_telemetry_compliance(repo, rel.strip())

    argos_report = _chaos_argos_certify(repo, canonical, state, toll)
    phase_reports.append(argos_report)

    ok = argos_report.get("status") == "executed"
    data: dict[str, Any] = {
        "process_name": canonical,
        "handoff": state.get("handoff"),
    }
    if state.get("workspace_path"):
        data["workspace_path"] = state["workspace_path"]
    if state.get("execution_id"):
        data["execution_id"] = state["execution_id"]
    if toll:
        data["thermodynamic_toll"] = toll
    if state.get("toll_failsoft_verified"):
        data["toll_failsoft_verified"] = True
    if state.get("breach_event_path"):
        data["breach_event_path"] = state["breach_event_path"]
    if state.get("isolation_verified"):
        data["isolation_verified"] = True

    return {
        "success": ok,
        "status_code": 0 if ok else 1,
        "data": data,
        "error": None if ok else "Argos: certificación caos fallida",
        "execution_report": {"process_name": canonical, "phases": phase_reports},
    }


def execute_workspace_smoke_phase(
    repo: Path,
    phase_name: str | None,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if phase_name != "Verificación de workspace":
        return None
    sync_workspace_context(inputs, state)
    ws = inputs.get("workspace_path") or state.get("workspace_path")
    if not isinstance(ws, str) or not ws.strip():
        return {
            "status": "failed",
            "handler": "workspace-smoke",
            "error": "workspace_path ausente",
        }
    marker = Path(ws.strip()) / ".workspace_ok"
    marker.parent.mkdir(parents=True, exist_ok=True)
    marker.write_text("ok", encoding="utf-8")
    state["workspace_verified"] = True
    return {
        "status": "executed",
        "handler": "workspace-smoke",
        "marker": str(marker),
        "workspace_path": ws.strip(),
    }


def execute_phase(
    repo: Path,
    phase: dict[str, Any],
    process_def: dict[str, Any],
    inputs: dict[str, Any],
    state: dict[str, Any],
    pi_index: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    phase_name = phase.get("name")
    delegates = phase.get("delegates_to") or []
    entry: dict[str, Any] = {
        "phase_name": phase_name,
        "delegates_to": delegates,
    }
    sync_workspace_context(inputs, state)

    if is_workspace_init_phase(phase, inputs, process_def):
        proc_name = process_def.get("name") if isinstance(process_def, dict) else None
        result = run_workspace_init(repo, inputs, str(proc_name) if proc_name else None)
        entry["status"] = "executed"
        entry["handler"] = "workspace-init"
        entry.update({k: result[k] for k in ("git_steps", "objectives_path", "branch_name") if k in result})
        state["workspace"] = result
        return entry

    if str(phase_name) == "Aduana EDA genómica":
        gate = capsule_eda_genomic_audit_gate(repo, inputs, state)
        entry["status"] = "executed" if gate["verdict"] != "block" else "blocked"
        entry["handler"] = "eda-genomic-audit"
        entry["argos_verdict"] = gate["verdict"]
        entry["orphan_count"] = gate["orphan_count"]
        if gate.get("argos_noise"):
            entry["argos_noise"] = gate["argos_noise"]
        return entry

    if process_def.get("name") == "delivery-close-cycle":
        dc = execute_delivery_close_phase(repo, str(phase_name) if phase_name else None, inputs, state)
        if dc is not None:
            entry.update(dc)
            return entry

    if process_def.get("name") == "accept-pr":
        ap = execute_accept_pr_phase(repo, str(phase_name) if phase_name else None, inputs, state)
        if ap is not None:
            entry.update(ap)
            return entry

    if process_def.get("name") == "feature":
        feat = execute_feature_phase(repo, str(phase_name) if phase_name else None, inputs, state)
        if feat is not None:
            entry.update(feat)
            return entry

    if process_def.get("name") == "pull-request-review":
        prr = execute_pull_request_review_phase(
            repo, str(phase_name) if phase_name else None, inputs, state
        )
        if prr is not None:
            entry.update(prr)
            return entry

    if process_def.get("name") == "workspace-smoke":
        smoke = execute_workspace_smoke_phase(
            repo, str(phase_name) if phase_name else None, inputs, state
        )
        if smoke is not None:
            entry.update(smoke)
            return entry

    pi = pi_index.get(str(phase_name))
    if pi and pi.get("invocations"):
        inv_log = run_phase_invocations(repo, pi, inputs, state)
        entry["invocations"] = inv_log
        if any(i.get("status") == "executed" for i in inv_log):
            try:
                forge = materialize_forge_by_inputs(repo, inputs)
                state["handoff"].update(forge)
                entry["status"] = "executed"
                entry["forge"] = True
            except NotImplementedError:
                entry["status"] = "simulated"
                entry["note"] = "invocations parciales; forja no aplicable"
        else:
            entry["status"] = "simulated"
        return entry

    if isinstance(delegates, list):
        if "action:execute-process" in delegates and inputs.get("entity_class"):
            if inputs.get("lifecycle_operation") == "delete":
                entry["status"] = "skipped"
                entry["note"] = "fase omitida en delete"
                return entry
            child = capsule_action_execute_process(repo, inputs, state)
            entry["status"] = "executed"
            entry["child"] = child.get("child_process")
            return entry

        if "skill:filesystem-manager" in delegates and inputs.get("lifecycle_operation") == "delete":
            capsule_filesystem_delete(repo, inputs, state)
            entry["status"] = "executed"
            entry["handler"] = "filesystem-delete"
            return entry

        action_result = try_execute_registered_action_capsules(repo, delegates, inputs, state)
        if action_result is not None:
            entry["status"] = "executed"
            entry["action_capsule"] = action_result
            if isinstance(action_result, dict):
                entry.update({k: action_result[k] for k in ("event_id", "target_path", "event_type") if k in action_result})
            return entry

        if delegates_are_only_agents(delegates):
            entry["status"] = "simulated"
            entry["note"] = "agentes IDE; sin handler físico en laboratorio"
            return entry

        if any(isinstance(d, str) and d.startswith(("skill:", "tool:", "action:")) for d in delegates):
            entry["status"] = "simulated"
            entry["note"] = "cápsulas sin handler físico registrado"
            return entry

    entry["status"] = "simulated"
    return entry


def _log_thermodynamic_emergency(
    process_name: str,
    channel: str,
    exc: BaseException,
) -> None:
    """Log de emergencia stderr — fail-soft; el hilo de negocio no debe depender de este canal."""
    print(
        f"{_THERMODYNAMIC_EMERGENCY_PREFIX} process={process_name} channel={channel}: {exc}",
        file=sys.stderr,
        flush=True,
    )


def _route_handler_result(canonical: str, out: dict[str, Any], handler: str) -> dict[str, Any]:
    ok = bool(out.get("success")) and out.get("exitCode", 1) == 0
    return {
        "success": ok,
        "status_code": out.get("exitCode", 0 if ok else 1),
        "data": out.get("data"),
        "error": out.get("error"),
        "execution_report": {
            "process_name": canonical,
            "phases": [
                {
                    "phase_name": f"Orquestación {canonical}",
                    "status": "executed" if ok else "failed",
                    "handler": handler,
                }
            ],
        },
    }


def extract_telemetry_receipt(envelope: dict[str, Any] | None) -> dict[str, Any] | None:
    if not isinstance(envelope, dict):
        return None
    try:
        direct = envelope.get("telemetry_receipt")
        if isinstance(direct, dict) and direct:
            return direct
        data = envelope.get("data")
        if isinstance(data, dict):
            nested = data.get("telemetry_receipt")
            if isinstance(nested, dict) and nested:
                return nested
        result = envelope.get("result")
        if isinstance(result, dict):
            nested = result.get("telemetry_receipt")
            if isinstance(nested, dict) and nested:
                return nested
    except (TypeError, AttributeError):
        return None
    return None


def run_thermodynamic_toll(
    repo: Path,
    process_name: str,
    state: dict[str, Any],
    process_inputs: dict[str, Any],
    *,
    exit_code: int,
    duration_ms: int,
    success: bool,
) -> dict[str, Any]:
    """Peaje Termodinámico: observador pasivo — telemetría/orquestación fail-soft (D3.13).

    Aislamiento de Excepciones de E/S: ningún fallo al escribir chispazos en ./.events/*
    altera el veredicto del proceso de negocio principal.
    """
    asset_id = state.get("asset_id")
    if not isinstance(asset_id, str) or not asset_id.strip():
        asset_id = str(uuid.uuid4())
    execution_id = state.get("execution_id")
    workspace_path = state.get("workspace_path")
    if not isinstance(workspace_path, str):
        ws = state.get("workspace") or {}
        if isinstance(ws, dict):
            workspace_path = ws.get("workspace_path")
    persist_ref = process_inputs.get("persist_ref")
    result: dict[str, Any] = {"asset_id": asset_id, "duration_ms": duration_ms, "exit_code": exit_code}
    capsule_id = state.get("last_capsule_id")
    receipt: dict[str, Any] | None = None
    try:
        receipt = extract_telemetry_receipt(state.get("last_capsule_envelope"))
    except Exception as exc:
        result["receipt_parse_error"] = str(exc)
        _log_thermodynamic_emergency(process_name, "receipt-parse", exc)
    if state.get("chaos_simulate_telemetry_io_fail"):
        result["telemetry_io_failed"] = True
        result["telemetry_error"] = "chaos lab: simulated telemetry I/O failure"
        _log_thermodynamic_emergency(
            process_name,
            "telemetry",
            OSError(28, "chaos lab simulated telemetry I/O failure"),
        )
    else:
        try:
            telemetry_id = str(uuid.uuid4())
            telemetry_event = build_raw_execution_finished_event(
                event_id=telemetry_id,
                asset_id=asset_id,
                exit_code=exit_code,
                duration_ms=duration_ms,
                process_name=process_name,
                execution_id=execution_id if isinstance(execution_id, str) else None,
                workspace_path=workspace_path if isinstance(workspace_path, str) else None,
                capsule_id=capsule_id if isinstance(capsule_id, str) else None,
                telemetry_receipt=receipt,
            )
            telemetry_seal = write_fractal_event(repo, telemetry_event, "telemetry")
            result["telemetry"] = telemetry_seal
        except Exception as exc:
            result["telemetry_error"] = str(exc)
            result["telemetry_io_failed"] = True
            _log_thermodynamic_emergency(process_name, "telemetry", exc)
    if success and isinstance(workspace_path, str) and workspace_path.strip():
        try:
            orch_id = str(uuid.uuid4())
            phase_count = len(state.get("phase_reports") or [])
            orchestration_event = build_process_execution_completed_event(
                event_id=orch_id,
                asset_id=asset_id,
                process_name=process_name,
                status="success",
                workspace_path=workspace_path if isinstance(workspace_path, str) else None,
                execution_id=execution_id if isinstance(execution_id, str) else None,
                phase_count=phase_count if phase_count else None,
                persist_ref=persist_ref if isinstance(persist_ref, str) else None,
            )
            orch_seal = write_fractal_event(repo, orchestration_event, "orchestration")
            result["orchestration"] = orch_seal
        except Exception as exc:
            result["orchestration_error"] = str(exc)
            result["orchestration_io_failed"] = True
            _log_thermodynamic_emergency(process_name, "orchestration", exc)
    return result


def execute_radamanto_batch_phase(
    repo: Path,
    phase_name: str | None,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if phase_name != "Consumo batch Radamanto":
        return None
    rel = inputs.get("event_file_path")
    if not isinstance(rel, str) or not rel.strip():
        return {"status": "failed", "handler": "radamanto-batch", "error": "event_file_path ausente"}
    from radamanto_batch_core import process_telemetry_file

    result = process_telemetry_file(repo, rel.strip())
    if not result.get("ok"):
        return {
            "status": "failed",
            "handler": "radamanto-batch",
            "error": result.get("error"),
        }
    state["radamanto_batch"] = result
    return {
        "status": "executed",
        "handler": "radamanto-batch",
        "entity_id": result.get("entity_id"),
        "actions": result.get("actions"),
        "purged": result.get("purged"),
    }


def execute_telemetry_compliance_audit_phase(
    repo: Path,
    phase_name: str | None,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if phase_name != "Auditoría cumplimiento termodinámico":
        return None
    rel = inputs.get("event_file_path")
    if not isinstance(rel, str) or not rel.strip():
        return {
            "status": "failed",
            "handler": "telemetry-compliance-audit",
            "error": "event_file_path ausente",
        }
    from telemetry_compliance_audit_core import audit_telemetry_compliance

    result = audit_telemetry_compliance(repo, rel.strip())
    if not result.get("ok"):
        return {
            "status": "failed",
            "handler": "telemetry-compliance-audit",
            "error": result.get("error"),
        }
    state["telemetry_compliance_audit"] = result
    return {
        "status": "executed",
        "handler": "telemetry-compliance-audit",
        "audit_status": result.get("status"),
        "breach": result.get("breach"),
    }


def execute_telemetry_batch_stub_phase(
    repo: Path,
    phase_name: str | None,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if phase_name != "Consumo batch stub":
        return None
    rel = inputs.get("event_file_path")
    if not isinstance(rel, str) or not rel.strip():
        return {"status": "failed", "handler": "telemetry-batch-stub", "error": "event_file_path ausente"}
    event_path = (repo / rel.strip()).resolve()
    if not event_path.is_file():
        return {"status": "failed", "handler": "telemetry-batch-stub", "error": f"no existe: {rel}"}
    try:
        body = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        return {"status": "failed", "handler": "telemetry-batch-stub", "error": str(exc)}
    event_path.unlink(missing_ok=True)
    state["telemetry_consumed"] = True
    state["telemetry_event_id"] = body.get("event_id")
    return {
        "status": "executed",
        "handler": "telemetry-batch-stub",
        "event_id": body.get("event_id"),
        "event_type": body.get("event_type"),
        "purged": True,
    }


def run_process(repo: Path, process_name: str, process_inputs: dict[str, Any]) -> dict[str, Any]:
    load_hierarchical_env(repo)
    canonical, process_def, phases = load_process_def(repo, process_name)
    if canonical == "radamanto-batch":
        state = {"handoff": {}, "inputs": process_inputs}
        phase_reports: list[dict[str, Any]] = []
        for phase in phases:
            if not isinstance(phase, dict):
                continue
            batch = execute_radamanto_batch_phase(
                repo, phase.get("name"), process_inputs, state
            )
            if batch is not None:
                phase_reports.append({"phase_name": phase.get("name"), **batch})
            else:
                phase_reports.append(
                    {"phase_name": phase.get("name"), "status": "simulated", "delegates_to": phase.get("delegates_to")}
                )
        ok = all(r.get("status") == "executed" for r in phase_reports if r.get("handler") == "radamanto-batch")
        return {
            "success": ok,
            "status_code": 0 if ok else 1,
            "data": {"process_name": canonical, "radamanto_batch": state.get("radamanto_batch")},
            "execution_report": {"process_name": canonical, "phases": phase_reports},
        }
    if canonical == "telemetry-compliance-audit":
        state = {"handoff": {}, "inputs": process_inputs}
        phase_reports: list[dict[str, Any]] = []
        for phase in phases:
            if not isinstance(phase, dict):
                continue
            audit = execute_telemetry_compliance_audit_phase(
                repo, phase.get("name"), process_inputs, state
            )
            if audit is not None:
                phase_reports.append({"phase_name": phase.get("name"), **audit})
            else:
                phase_reports.append(
                    {"phase_name": phase.get("name"), "status": "simulated", "delegates_to": phase.get("delegates_to")}
                )
        ok = all(
            r.get("status") == "executed"
            for r in phase_reports
            if r.get("handler") == "telemetry-compliance-audit"
        )
        return {
            "success": ok,
            "status_code": 0 if ok else 1,
            "data": {
                "process_name": canonical,
                "telemetry_compliance_audit": state.get("telemetry_compliance_audit"),
            },
            "execution_report": {"process_name": canonical, "phases": phase_reports},
        }
    if canonical == "telemetry-batch-stub":
        state: dict[str, Any] = {"handoff": {}, "inputs": process_inputs}
        phase_reports: list[dict[str, Any]] = []
        for phase in phases:
            if not isinstance(phase, dict):
                continue
            stub = execute_telemetry_batch_stub_phase(
                repo, phase.get("name"), process_inputs, state
            )
            if stub is not None:
                phase_reports.append({"phase_name": phase.get("name"), **stub})
            else:
                phase_reports.append(
                    {"phase_name": phase.get("name"), "status": "simulated", "delegates_to": phase.get("delegates_to")}
                )
        ok = all(r.get("status") == "executed" for r in phase_reports if r.get("handler") == "telemetry-batch-stub")
        return {
            "success": ok,
            "status_code": 0 if ok else 1,
            "data": {"process_name": canonical, "telemetry_consumed": state.get("telemetry_consumed")},
            "execution_report": {"process_name": canonical, "phases": phase_reports},
        }
    if canonical in ROUTE_FRACTAL_HANDLERS:
        from route_fractal_event_core import (
            route_domain_fractal_event,
            route_orchestration_event,
            route_telemetry_event,
        )

        rel = process_inputs.get("event_file_path")
        if not isinstance(rel, str) or not rel.strip():
            return {
                "success": False,
                "status_code": 1,
                "data": None,
                "error": "event_file_path requerido",
                "execution_report": {"process_name": canonical, "phases": []},
            }
        dispatch = {
            "route-telemetry": route_telemetry_event,
            "route-orchestration": route_orchestration_event,
            "route-domain": route_domain_fractal_event,
        }[canonical]
        out = dispatch(repo, rel.strip())
        return _route_handler_result(canonical, out, f"{canonical}-core")
    if canonical == "telegram-gateway":
        from telegram_gateway_core import run_telegram_gateway

        text = process_inputs.get("text")
        if not isinstance(text, str):
            return {
                "success": False,
                "status_code": 1,
                "error": "text requerido",
                "execution_report": {"process_name": canonical, "phases": []},
            }
        out = run_telegram_gateway(repo, text)
        ok = bool(out.get("ok"))
        return {
            "success": ok,
            "status_code": 0 if ok else 1,
            "data": out,
            "execution_report": {
                "process_name": canonical,
                "phases": [
                    {
                        "phase_name": "Transmutación e inyección",
                        "status": "executed",
                        "handler": "telegram-gateway-core",
                        "emitted": out.get("emitted"),
                        "event_type": out.get("event_type"),
                    }
                ],
            },
        }
    if canonical == "route-domain-event":
        from route_domain_event_core import route_domain_event

        rel = process_inputs.get("event_file_path")
        if not isinstance(rel, str) or not rel.strip():
            return {
                "success": False,
                "status_code": 1,
                "data": None,
                "error": "event_file_path requerido",
                "execution_report": {"process_name": canonical, "phases": []},
            }
        out = route_domain_event(repo, rel.strip())
        ok = bool(out.get("success")) and out.get("exitCode", 1) == 0
        return {
            "success": ok,
            "status_code": out.get("exitCode", 0 if ok else 1),
            "data": out.get("data"),
            "error": out.get("error"),
            "execution_report": {
                "process_name": canonical,
                "phases": [
                    {
                        "phase_name": "Orquestación route-domain-event",
                        "status": "executed",
                        "handler": "route-domain-event-core",
                        "dispatch_mode": (out.get("data") or {}).get("dispatch_mode"),
                    }
                ],
            },
        }
    if canonical in CHAOS_AUDIT_PROCESSES:
        return run_chaos_audit_process(repo, canonical, process_def, phases, process_inputs)
    if canonical == "execute-suite":
        return run_execute_suite(repo, canonical, process_def, phases, process_inputs)
    if canonical == "pull-request-review":
        _normalize_pr_review_inputs(repo, process_inputs)
    validate_process_inputs(process_def, process_inputs, canonical)

    state: dict[str, Any] = {"handoff": {}, "inputs": process_inputs}
    global _ACTIVE_CAPSULE_CAPTURE_STATE
    _ACTIVE_CAPSULE_CAPTURE_STATE = state
    toll_start: float | None = None
    try:
        if canonical not in THERMODYNAMIC_EXEMPT:
            state["asset_id"] = str(uuid.uuid4())
            toll_start = time.monotonic()
        try:
            ws_boot = bootstrap_process_workspace(repo, canonical, process_def, process_inputs, state)
            state["workspace_boot"] = ws_boot
        except ValueError as exc:
            return {
                "success": False,
                "status_code": 1,
                "data": None,
                "error": str(exc),
                "execution_report": {"process_name": canonical, "phases": []},
            }
        pi_index = phase_invocations_index(process_def)
        phase_reports: list[dict[str, Any]] = []

        for phase in phases:
            if not isinstance(phase, dict):
                continue
            phase_reports.append(
                execute_phase(repo, phase, process_def, process_inputs, state, pi_index)
            )
        state["phase_reports"] = phase_reports

        data: dict[str, Any] = {"process_name": canonical, "handoff": state.get("handoff")}
        if state.get("workspace_path"):
            data["workspace_path"] = state["workspace_path"]
        if state.get("execution_id"):
            data["execution_id"] = state["execution_id"]
        if state.get("workspace"):
            data.update(state["workspace"])
        if state.get("eda_audit"):
            data["eda_audit"] = state["eda_audit"]
        if state.get("argos_verdict"):
            data["argos_verdict"] = state["argos_verdict"]
        if state.get("pr_url"):
            data["pr_url"] = state["pr_url"]
        if state.get("event_id"):
            data["event_id"] = state["event_id"]
        if state.get("target_path"):
            data["target_path"] = state["target_path"]
        if "closed_branch" in state:
            data["closed_branch"] = state["closed_branch"]
        if state.get("hygiene_failure"):
            data["hygiene_failure"] = state["hygiene_failure"]
        if state.get("snapshot_commit_hash"):
            data["snapshot_commit_hash"] = state["snapshot_commit_hash"]
        if state.get("verdict"):
            data["verdict"] = state["verdict"]
        if state.get("delivery_state"):
            data["delivery_state"] = state["delivery_state"]
        if state.get("kaizen_seeds"):
            data["kaizen_seeds"] = state["kaizen_seeds"]
        if state.get("accept_pr_handoff"):
            data["accept_pr_handoff"] = state["accept_pr_handoff"]
        if state.get("sddia_impact"):
            data["sddia_impact"] = state["sddia_impact"]
        if state.get("pbi_archived_path"):
            data["pbi_archived_path"] = state["pbi_archived_path"]
        if state.get("delivery_close"):
            data["delivery_close"] = state["delivery_close"]

        blocked = state.get("argos_verdict") == "block"
        if state.get("verdict") in ("rechazado", "requiere_cambios"):
            blocked = True
        err_msg = None
        if state.get("argos_verdict") == "block":
            err_msg = "Argos: Ruido de Sistema (huérfanas EDA)"
        elif state.get("verdict") == "rechazado":
            err_msg = "pull-request-review: aduana bloqueó materialización"
        elif state.get("verdict") == "requiere_cambios":
            err_msg = "pull-request-review: requiere cambios antes de merge"
        blocked_success = not blocked
        status_code = 1 if blocked else 0
        duration_ms = 0
        if toll_start is not None:
            duration_ms = max(0, int((time.monotonic() - toll_start) * 1000))
        if canonical not in THERMODYNAMIC_EXEMPT and toll_start is not None:
            try:
                data["thermodynamic_toll"] = run_thermodynamic_toll(
                    repo,
                    canonical,
                    state,
                    process_inputs,
                    exit_code=status_code,
                    duration_ms=duration_ms,
                    success=blocked_success,
                )
            except Exception as exc:
                data["thermodynamic_toll_error"] = str(exc)
                _log_thermodynamic_emergency(canonical, "toll-envelope", exc)
        return {
            "success": blocked_success,
            "status_code": status_code,
            "data": data,
            "error": err_msg,
            "execution_report": {"process_name": canonical, "phases": phase_reports},
        }
    finally:
        _ACTIVE_CAPSULE_CAPTURE_STATE = None
