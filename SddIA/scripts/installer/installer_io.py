#!/usr/bin/env python3
"""Estado, envelope capsule-json-io 2.0 y progreso JSONL/PTC para sddia-installer."""
from __future__ import annotations

import argparse
import json
import os
import sys
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ENTITY_ID = "sddia-installer"

DEPLOY_STEPS = [
    ("validate_host", "Validar host"),
    ("resolve_target", "Comprobar destino"),
    ("teardown_previous", "Retirar instancia previa"),
    ("stage_vault", "Preparar bóveda"),
    ("build_bundle", "Construir bundle"),
    ("materialize_instance", "Materializar instancia"),
    ("check_mailbox", "Comprobar buzón"),
    ("enable_units", "Habilitar servicios"),
    ("registry_upsert", "Registrar instancia"),
    ("verify_health", "Verificar salud"),
    ("emit_event", "Notificar"),
]

TEARDOWN_STEPS = [
    ("validate_host", "Validar host"),
    ("resolve_target", "Comprobar destino"),
    ("signal_procs", "Detener procesos de la instancia"),
    ("stop_units", "Detener y deshabilitar servicios"),
    ("clean_residuals", "Limpiar residuos"),
    ("registry_remove", "Dar de baja en el registro"),
    ("remove_templates", "Retirar plantillas"),
    ("wipe_root", "Borrar directorio"),
    ("emit_event", "Notificar"),
]

ERROR_CODES = {
    1: "INVALID_ARGS",
    2: "ROOT_LIVE_REQUIRES_FORCE",
    3: "TEARDOWN_REQUIRES_FORCE",
    4: "MAILBOX_SHARED",
    5: "VERIFY_NOT_APTO",
    6: "STEP_FAILED",
    7: "REQUEST_INVALID",
}


def _utc_now() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def _load_state(path: Path) -> dict[str, Any]:
    if path.is_file():
        return json.loads(path.read_text())
    return {}


def _save_state(path: Path, state: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(state, indent=2) + "\n")


def _step_catalog(command: str) -> list[tuple[str, str]]:
    return DEPLOY_STEPS if command == "deploy" else TEARDOWN_STEPS


def _progress_emit(state: dict[str, Any], line: dict[str, Any]) -> None:
    mode = state.get("progress_mode", "auto")
    if mode == "none":
        return
    payload = json.dumps(line, separators=(",", ":"))
    fd3 = state.get("progress_fd3_open")
    if mode in ("auto", "fd3") and fd3:
        try:
            os.write(3, (payload + "\n").encode())
            return
        except OSError:
            pass
    if mode in ("auto", "stderr"):
        sys.stderr.write(f"@sddia-progress {payload}\n")
        sys.stderr.flush()
    cid = state.get("correlation_id")
    if cid:
        repo = Path(state["forge_root"])
        prog_dir = repo / ".events" / "progress" / cid
        prog_dir.mkdir(parents=True, exist_ok=True)
        trace = {
            "trace_id": str(uuid.uuid4()),
            "correlation_id": cid,
            "timestamp": line.get("timestamp", _utc_now()),
            "phase": line.get("id", "implementation"),
            "severity": "info" if line.get("status") in (None, "ok", "skipped") else "error",
            "source_agent": "sddia-installer",
            "message": line.get("title", ""),
            "metadata": {"moment": line.get("moment"), "installer_step": line.get("id")},
        }
        target = prog_dir / f"{trace['trace_id']}.json"
        target.write_text(json.dumps(trace) + "\n")


def cmd_init(args: argparse.Namespace) -> None:
    state_path = Path(args.state_file)
    catalog = _step_catalog(args.command)
    steps = [
        {
            "id": sid,
            "index": i + 1,
            "total": len(catalog),
            "title": title,
            "status": "not_run",
        }
        for i, (sid, title) in enumerate(catalog)
    ]
    log_dir = args.log_dir or str(Path(args.forge_root) / ".SddIA" / "logs" / "installer")
    Path(log_dir).mkdir(parents=True, exist_ok=True)
    ts = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    log_name = f"{args.command}-{ts}.log"
    log_path = Path(log_dir) / log_name
    log_path.write_text("")
    _rotate_logs(Path(log_dir), 20)
    rel_log = str(Path(".SddIA/logs/installer") / log_name)
    state = {
        "forge_root": args.forge_root,
        "state_file": str(state_path),
        "command": args.command,
        "dry_run": args.dry_run,
        "bundle_profile": args.bundle_profile,
        "started_at": _utc_now(),
        "started_mono": datetime.now(timezone.utc).timestamp(),
        "correlation_id": args.correlation_id or None,
        "progress_mode": args.progress_mode or "auto",
        "progress_fd3_open": bool(int(os.environ.get("SDDIA_INSTALLER_FD3", "0"))),
        "log_path": str(log_path),
        "log_ref": rel_log,
        "steps": steps,
        "plan": None,
        "units": {"enabled": [], "skipped": []},
        "verify": None,
        "events": [],
        "registry": None,
        "wui_port": None,
        "root": None,
        "esc": None,
        "error": None,
        "message": "",
        "success": None,
        "exit_code": None,
    }
    _save_state(state_path, state)
    print(json.dumps({"log_path": str(log_path), "log_ref": rel_log}))


def _rotate_logs(log_dir: Path, keep: int) -> None:
    files = sorted(log_dir.glob("*.log"), key=lambda p: p.stat().st_mtime, reverse=True)
    for old in files[keep:]:
        old.unlink(missing_ok=True)


def cmd_set_context(args: argparse.Namespace) -> None:
    state_path = Path(args.state_file)
    state = _load_state(state_path)
    for key in (
        "root",
        "esc",
        "force",
        "skip_build",
        "dry_run",
        "bundle_profile",
    ):
        val = getattr(args, key, None)
        if val is not None:
            state[key] = val
    _save_state(state_path, state)


def cmd_set_plan(args: argparse.Namespace) -> None:
    state_path = Path(args.state_file)
    state = _load_state(state_path)
    state["plan"] = json.loads(args.plan_json)
    if state["plan"].get("wui_port") is not None:
        state["wui_port"] = state["plan"]["wui_port"]
    _save_state(state_path, state)


def cmd_step(args: argparse.Namespace) -> None:
    state_path = Path(args.state_file)
    state = _load_state(state_path)
    sid = args.step_id
    steps = state.get("steps", [])
    step = next((s for s in steps if s["id"] == sid), None)
    if not step:
        raise SystemExit(f"unknown step {sid}")
    now = _utc_now()
    if args.moment == "begin":
        step["_begin_ts"] = now
        step["_begin_mono"] = datetime.now(timezone.utc).timestamp()
        _progress_emit(
            state,
            {
                "kind": "step",
                "timestamp": now,
                "id": sid,
                "index": step["index"],
                "total": step["total"],
                "title": step["title"],
                "moment": "begin",
            },
        )
    else:
        status = args.status or "ok"
        step["status"] = status
        if args.reason:
            step["reason"] = args.reason
        dur = 0
        if "_begin_mono" in step:
            dur = int((datetime.now(timezone.utc).timestamp() - step["_begin_mono"]) * 1000)
            step["durationMs"] = dur
        _progress_emit(
            state,
            {
                "kind": "step",
                "timestamp": now,
                "id": sid,
                "index": step["index"],
                "total": step["total"],
                "title": step["title"],
                "moment": "end",
                "status": status,
                "durationMs": dur,
            },
        )
    _save_state(state_path, state)


def cmd_set_json_field(args: argparse.Namespace) -> None:
    state_path = Path(args.state_file)
    state = _load_state(state_path)
    state[args.field] = json.loads(args.json_blob)
    _save_state(state_path, state)


def cmd_fail(args: argparse.Namespace) -> None:
    state_path = Path(args.state_file)
    state = _load_state(state_path)
    code = int(args.exit_code)
    err_code = args.error_code or ERROR_CODES.get(code, "INVALID_ARGS")
    state["success"] = False
    state["exit_code"] = code
    state["message"] = args.message
    state["error"] = {
        "code": err_code,
        "message": args.message,
    }
    if args.step:
        state["error"]["step"] = args.step
    if args.child_exit:
        state["error"]["child_exit"] = int(args.child_exit)
    if args.detail_tail:
        state["error"]["detail_tail"] = args.detail_tail
    _save_state(state_path, state)
    cmd_emit(argparse.Namespace(state_file=args.state_file, to_stdout=False))


def cmd_success(args: argparse.Namespace) -> None:
    state_path = Path(args.state_file)
    state = _load_state(state_path)
    state["success"] = True
    state["exit_code"] = 0
    state["message"] = args.message
    state["error"] = None
    _save_state(state_path, state)


def _feedback_from_steps(steps: list[dict[str, Any]]) -> list[dict[str, Any]]:
    out = []
    for s in steps:
        if s.get("status") == "not_run":
            continue
        level = "info" if s.get("status") in ("ok", "skipped") else "error"
        out.append(
            {
                "phase": s["id"],
                "level": level,
                "timestamp": _utc_now(),
                "message": s["title"],
                "status": s.get("status"),
                "durationMs": s.get("durationMs", 0),
            }
        )
    return out


def _build_envelope(state: dict[str, Any]) -> dict[str, Any]:
    duration_ms = int(
        (datetime.now(timezone.utc).timestamp() - state.get("started_mono", 0)) * 1000
    )
    plan = state.get("plan") or {}
    result = {
        "command": state.get("command"),
        "root": state.get("root") or plan.get("root"),
        "esc": state.get("esc") or plan.get("esc"),
        "profile": state.get("bundle_profile") or plan.get("bundle_profile"),
        "dry_run": bool(state.get("dry_run") if state.get("dry_run") is not None else plan.get("dry_run")),
        "plan": plan,
        "steps": state.get("steps", []),
        "units": state.get("units") or {"enabled": [], "skipped": []},
        "wui_port": state.get("wui_port") or plan.get("wui_port"),
        "verify": state.get("verify"),
        "events": state.get("events") or [],
        "registry": state.get("registry"),
        "log_ref": state.get("log_ref"),
        "error": state.get("error"),
    }
    success = bool(state.get("success"))
    exit_code = int(state.get("exit_code") if state.get("exit_code") is not None else (0 if success else 1))
    return {
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": ENTITY_ID,
        },
        "success": success,
        "exitCode": exit_code,
        "message": state.get("message") or "",
        "durationMs": duration_ms,
        "feedback": _feedback_from_steps(state.get("steps", [])),
        "result": result,
    }


def cmd_emit(args: argparse.Namespace) -> None:
    state_path = Path(args.state_file)
    state = _load_state(state_path)
    env = _build_envelope(state)
    text = json.dumps(env, separators=(",", ":"))
    out_path = os.environ.get("SDDIA_INSTALLER_RESULT_FILE")
    if args.to_stdout or (not out_path and not os.environ.get("SDDIA_INSTALLER_SUPPRESS_STDOUT")):
        print(text)
    elif out_path:
        Path(out_path).write_text(text + "\n")
    state["last_envelope"] = env
    _save_state(state_path, state)


def cmd_merge_facade(args: argparse.Namespace) -> None:
    """Fusiona envelope del motor (archivo) con pasos verify/event de la fachada."""
    motor_path = Path(args.motor_envelope_file)
    state_path = Path(args.state_file)
    motor = json.loads(motor_path.read_text())
    state = _load_state(state_path)
    # Añadir pasos fachada al motor result
    for extra_step in json.loads(args.extra_steps_json or "[]"):
        for s in motor.get("result", {}).get("steps", []):
            if s["id"] == extra_step.get("id"):
                s.update(extra_step)
    if args.verify_json:
        motor.setdefault("result", {})["verify"] = json.loads(args.verify_json)
    if args.events_json:
        motor.setdefault("result", {})["events"] = json.loads(args.events_json)
    if args.message:
        motor["message"] = args.message
    if args.success is not None:
        motor["success"] = args.success == "true"
    if args.exit_code is not None:
        motor["exitCode"] = int(args.exit_code)
        motor["success"] = motor["exitCode"] == 0
    text = json.dumps(motor, separators=(",", ":"))
    print(text)


def cmd_parse_request(args: argparse.Namespace) -> None:
    """Normaliza request capsule a JSON de flags para el motor."""
    allowed = {
        "command",
        "root",
        "vault",
        "codex",
        "force",
        "skip_build",
        "dry_run",
        "allow_shared_mailbox",
        "verify",
        "correlation_id",
        "progress",
        "log_dir",
    }
    raw = json.loads(args.request_json)
    req = raw.get("request") if "request" in raw else raw
    if not isinstance(req, dict):
        raise SystemExit("REQUEST_INVALID")
    for k in req:
        if k not in allowed:
            raise SystemExit(f"REQUEST_INVALID:unknown:{k}")
    cmd = req.get("command")
    if cmd not in ("deploy", "teardown"):
        raise SystemExit("REQUEST_INVALID:command")
    argv = [cmd]
    if req.get("root"):
        argv += ["--root", str(req["root"])]
    if req.get("vault"):
        argv += ["--vault", str(req["vault"])]
    if req.get("codex"):
        argv += ["--codex", str(req["codex"])]
    if req.get("force"):
        argv += ["--force"]
    if req.get("skip_build"):
        argv += ["--skip-build"]
    if req.get("dry_run"):
        argv += ["--dry-run"]
    if req.get("allow_shared_mailbox"):
        argv += ["--allow-shared-mailbox"]
    meta = {
        "correlation_id": req.get("correlation_id"),
        "progress": req.get("progress", "auto"),
        "log_dir": req.get("log_dir"),
        "verify": req.get("verify", True),
    }
    print(json.dumps({"argv": argv, "meta": meta}))


def main() -> None:
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="cmd", required=True)

    i = sub.add_parser("init")
    i.add_argument("--state-file", required=True)
    i.add_argument("--forge-root", required=True)
    i.add_argument("--command", required=True)
    i.add_argument("--dry-run", choices=("0", "1"), default="0")
    i.add_argument("--bundle-profile", default="full-node")
    i.add_argument("--correlation-id", default="")
    i.add_argument("--progress-mode", default="auto")
    i.add_argument("--log-dir", default="")
    i.set_defaults(func=cmd_init)

    c = sub.add_parser("set-context")
    c.add_argument("--state-file", required=True)
    for f in ("root", "esc", "force", "skip_build", "dry_run", "bundle_profile"):
        c.add_argument(f"--{f.replace('_', '-')}", default=None)
    c.set_defaults(func=cmd_set_context)

    pl = sub.add_parser("set-plan")
    pl.add_argument("--state-file", required=True)
    pl.add_argument("--plan-json", required=True)
    pl.set_defaults(func=cmd_set_plan)

    st = sub.add_parser("step")
    st.add_argument("--state-file", required=True)
    st.add_argument("--step-id", required=True)
    st.add_argument("--moment", choices=("begin", "end"), required=True)
    st.add_argument("--status", default="")
    st.add_argument("--reason", default="")
    st.set_defaults(func=cmd_step)

    sf = sub.add_parser("set-field")
    sf.add_argument("--state-file", required=True)
    sf.add_argument("--field", required=True)
    sf.add_argument("--json-blob", required=True)
    sf.set_defaults(func=cmd_set_json_field)

    fl = sub.add_parser("fail")
    fl.add_argument("--state-file", required=True)
    fl.add_argument("--exit-code", required=True)
    fl.add_argument("--message", required=True)
    fl.add_argument("--error-code", default="")
    fl.add_argument("--step", default="")
    fl.add_argument("--child-exit", default="")
    fl.add_argument("--detail-tail", default="")
    fl.set_defaults(func=cmd_fail)

    ok = sub.add_parser("success")
    ok.add_argument("--state-file", required=True)
    ok.add_argument("--message", required=True)
    ok.set_defaults(func=cmd_success)

    em = sub.add_parser("emit")
    em.add_argument("--state-file", required=True)
    em.add_argument("--to-stdout", action="store_true")
    em.set_defaults(func=cmd_emit)

    mg = sub.add_parser("merge-facade")
    mg.add_argument("--state-file", required=True)
    mg.add_argument("--motor-envelope-file", required=True)
    mg.add_argument("--extra-steps-json", default="[]")
    mg.add_argument("--verify-json", default="")
    mg.add_argument("--events-json", default="")
    mg.add_argument("--message", default="")
    mg.add_argument("--success", default="")
    mg.add_argument("--exit-code", default="")
    mg.set_defaults(func=cmd_merge_facade)

    pr = sub.add_parser("parse-request")
    pr.add_argument("--request-json", required=True)
    pr.set_defaults(func=cmd_parse_request)

    args = p.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
