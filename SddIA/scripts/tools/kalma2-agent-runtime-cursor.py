#!/usr/bin/env python3
"""kalma2-agent-runtime-cursor — wrapper producción AGENT_PHASE (deuda B-prod).

Backends:
  cli  (default) — SDDIA_AGENT_RUNTIME_CLI || SDDIA_LLM_CLI_COMMAND || "cursor-agent --print"
  sdk            — Cursor SDK Python (cursor_sdk) con CURSOR_API_KEY / local cwd

Mock lab/CI:
  SDDIA_AGENT_RUNTIME_MOCK=1 → no invoca Cursor; status=executed y handoff mock.

Salida (stdout, última línea JSON):
  {"success":true,"data":{"status":"executed|awaiting_agents|failed","message":"..."},"error":null}
"""
from __future__ import annotations

import json
import os
import shlex
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


def emit(success: bool, data: dict[str, Any] | None, error: str | None, code: int = 0) -> None:
    print(
        json.dumps(
            {"success": success, "data": data, "error": error},
            ensure_ascii=False,
        )
    )
    raise SystemExit(code if not success else 0)


def env_truthy(key: str) -> bool:
    return os.environ.get(key, "").strip().lower() in ("1", "true", "yes", "on")


def split_command(raw: str) -> list[str]:
    parts = shlex.split(raw.strip())
    if not parts:
        raise ValueError("comando CLI vacío")
    return parts


def resolve_cli() -> list[str]:
    for key in ("SDDIA_AGENT_RUNTIME_CLI", "SDDIA_LLM_CLI_COMMAND"):
        raw = os.environ.get(key, "").strip()
        if raw:
            return split_command(raw)
    return ["cursor-agent", "--print"]


def role_brief(agent: str, phase: str, process: str) -> str:
    a = (agent or "").lower()
    if a == "mayeuta" or "estabil" in phase.lower():
        return (
            f"Actúa como Mayeuta en proceso `{process}`. "
            "Estabiliza requisitos: produce/actualiza clarify.md y objectives.md bajo persist_ref "
            "con frontmatter features-documentation-pattern."
        )
    if a == "dedalo" or "diseño" in phase.lower() or "blueprint" in phase.lower():
        return (
            f"Actúa como Dedalo en proceso `{process}`. "
            "Consume objectives.md; produce spec.md (y plan.md si hace falta blueprint) bajo persist_ref."
        )
    if a == "tekton" or phase.lower().startswith("ejecuc"):
        return (
            f"Actúa como Tekton (Vértice Productivo) en proceso `{process}`. "
            "Materializa el cambio de código según spec/plan; genera implementation.md y execution.md."
        )
    if a == "argos" or "verific" in phase.lower():
        return (
            f"Actúa como Argos en proceso `{process}`. "
            "Audita la entrega; escribe validacion.md (global/checks/git_changes/branch) bajo persist_ref."
        )
    return (
        f"Actúa como agente `{agent or '?'}` en fase `{phase}` del proceso `{process}`. "
        "Cumple el contrato documental y de código del Core SddIA."
    )


def build_prompt(doc: dict[str, Any]) -> str:
    process = doc.get("process_name") or "?"
    phase = doc.get("phase_name") or "?"
    agents = doc.get("agents") or []
    agent = agents[0] if agents else "?"
    persist = (doc.get("persist_ref") or "").strip()
    pbi_ref = doc.get("pbi_ref") or ""
    corr = doc.get("correlation_id") or ""
    inputs = doc.get("inputs") or {}
    pbi_body = ""
    if isinstance(inputs, dict):
        pbi_body = (inputs.get("pbi_body") or "")[:12000]
        seed = (
            inputs.get("bug_summary")
            or inputs.get("refined_requirements")
            or inputs.get("refactor_goal")
            or ""
        )
    else:
        seed = ""

    brief = role_brief(str(agent), str(phase), str(process))
    parts = [
        "[HARD OVERRIDE — SddIA kalma2-agent-runtime-cursor]",
        brief,
        "",
        f"- process: {process}",
        f"- phase: {phase}",
        f"- agents: {', '.join(str(a) for a in agents)}",
        f"- persist_ref: {persist}",
        f"- branch_name: {doc.get('branch_name') or inputs.get('branch_name') if isinstance(inputs, dict) else ''}",
        f"- pbi_ref: {pbi_ref}",
        f"- correlation_id: {corr}",
        "",
        "Reglas:",
        "- Git solo vía skill:git-manager del ecosistema (no bypass raw destructivo).",
        "- No inventes éxito: si no puedes materializar, dilo explícitamente.",
        "- Trabaja en el repositorio local (cwd = repo_root).",
        "",
    ]
    if seed:
        parts.extend(["## Semilla / inputs", str(seed)[:8000], ""])
    if pbi_body:
        parts.extend(["## Cuerpo PBI (pbi_body)", pbi_body, ""])
    parts.append(
        "Al terminar, resume en ≤8 líneas qué archivos tocaste y el veredicto (ok|blocked)."
    )
    return "\n".join(parts)


def append_handoff(
    repo: Path,
    persist: str,
    *,
    phase: str,
    process: str,
    agents: list[Any],
    corr: Any,
    pbi_ref: Any,
    backend: str,
    status: str,
    message: str,
    transcript: str | None = None,
) -> str | None:
    if not persist:
        return None
    d = repo / persist
    d.mkdir(parents=True, exist_ok=True)
    handoff = d / "_agent_handoff.md"
    ts = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    if not handoff.exists():
        handoff.write_text(
            "---\n"
            "generated_by: kalma2-agent-runtime-cursor\n"
            f"persist_ref: {persist}\n"
            "---\n\n# Agent handoff log\n",
            encoding="utf-8",
        )
    block = (
        f"\n## {ts} — {phase}\n"
        f"- process: `{process}`\n"
        f"- agents: {', '.join(f'`{a}`' for a in agents) or '(ninguno)'}\n"
        f"- correlation_id: `{corr or ''}`\n"
        f"- pbi_ref: `{pbi_ref or ''}`\n"
        f"- runtime: kalma2-agent-runtime-cursor\n"
        f"- backend: `{backend}`\n"
        f"- status: `{status}`\n"
        f"- message: {message}\n"
    )
    with handoff.open("a", encoding="utf-8") as f:
        f.write(block)
        if transcript:
            f.write("\n### Transcript (tail)\n\n```\n")
            f.write(transcript[-4000:])
            f.write("\n```\n")
    return str(Path(persist) / "_agent_handoff.md")


def run_cli(repo: Path, prompt: str) -> tuple[bool, str, str]:
    cmd = resolve_cli()
    timeout = int(os.environ.get("SDDIA_AGENT_RUNTIME_TIMEOUT_SECS", "600") or "600")
    try:
        proc = subprocess.run(
            cmd,
            input=prompt.encode("utf-8"),
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=str(repo),
            timeout=timeout,
            check=False,
        )
    except FileNotFoundError as e:
        return False, "", f"CLI no encontrado: {e}"
    except subprocess.TimeoutExpired:
        return False, "", f"timeout {timeout}s"
    out = proc.stdout.decode("utf-8", errors="replace").strip()
    err = proc.stderr.decode("utf-8", errors="replace").strip()
    if proc.returncode != 0:
        return False, out, err or f"exit {proc.returncode}"
    if not out:
        return False, "", err or "CLI sin stdout"
    return True, out, err


def run_sdk(repo: Path, prompt: str) -> tuple[bool, str, str]:
    try:
        from cursor_sdk import Agent  # type: ignore
    except ImportError:
        return False, "", "cursor_sdk no instalado (pip install cursor-sdk)"

    api_key = os.environ.get("CURSOR_API_KEY", "").strip() or None
    model_id = os.environ.get("SDDIA_AGENT_RUNTIME_MODEL", "composer-2.5").strip() or "composer-2.5"
    try:
        # API pública beta: Agent.prompt one-shot local
        kwargs: dict[str, Any] = {
            "model": model_id,
            "local": {"cwd": str(repo)},
        }
        if api_key:
            kwargs["api_key"] = api_key
        result = Agent.prompt(prompt, **kwargs)
        status = getattr(result, "status", None) or (result.get("status") if isinstance(result, dict) else None)
        text = getattr(result, "result", None) or (result.get("result") if isinstance(result, dict) else None) or str(result)
        if status and str(status).lower() in ("error", "failed"):
            return False, str(text), f"sdk status={status}"
        return True, str(text), ""
    except Exception as e:  # noqa: BLE001 — aduana: cualquier fallo SDK → awaiting/failed
        return False, "", f"sdk error: {e}"


def main() -> None:
    raw = sys.stdin.read()
    if not raw.strip():
        emit(False, None, "stdin vacío", 1)
    try:
        doc = json.loads(raw)
    except json.JSONDecodeError as e:
        emit(False, None, f"JSON inválido: {e}", 1)

    repo = Path(doc.get("repo_root") or os.getcwd()).resolve()
    persist = (doc.get("persist_ref") or "").strip()
    phase = doc.get("phase_name") or "?"
    process = doc.get("process_name") or "?"
    agents = doc.get("agents") or []
    backend = (os.environ.get("SDDIA_AGENT_RUNTIME_BACKEND") or "cli").strip().lower()
    if backend not in ("cli", "sdk"):
        backend = "cli"

    if env_truthy("SDDIA_AGENT_RUNTIME_MOCK"):
        msg = "mock: AGENT_PHASE sin invocar Cursor"
        handoff = append_handoff(
            repo,
            persist,
            phase=str(phase),
            process=str(process),
            agents=list(agents),
            corr=doc.get("correlation_id"),
            pbi_ref=doc.get("pbi_ref"),
            backend="mock",
            status="executed",
            message=msg,
        )
        emit(
            True,
            {
                "status": "executed",
                "message": msg,
                "handoff_path": handoff,
                "backend": "mock",
            },
            None,
        )

    prompt = build_prompt(doc)
    if backend == "sdk":
        ok, out, err = run_sdk(repo, prompt)
    else:
        ok, out, err = run_cli(repo, prompt)

    if ok:
        status = "executed"
        message = (out.splitlines()[-1] if out else "ok")[:500]
        handoff = append_handoff(
            repo,
            persist,
            phase=str(phase),
            process=str(process),
            agents=list(agents),
            corr=doc.get("correlation_id"),
            pbi_ref=doc.get("pbi_ref"),
            backend=backend,
            status=status,
            message=message,
            transcript=out,
        )
        emit(
            True,
            {
                "status": status,
                "message": message,
                "handoff_path": handoff,
                "backend": backend,
            },
            None,
        )

    # CLI/SDK ausente o fallo blando → awaiting_agents (no tumbar ciclo si es config)
    soft = any(
        x in (err or "").lower()
        for x in ("no encontrado", "not found", "no instalado", "timeout", "api_key", "401", "auth")
    )
    status = "awaiting_agents" if soft else "failed"
    message = err or "runtime falló"
    handoff = append_handoff(
        repo,
        persist,
        phase=str(phase),
        process=str(process),
        agents=list(agents),
        corr=doc.get("correlation_id"),
        pbi_ref=doc.get("pbi_ref"),
        backend=backend,
        status=status,
        message=message,
        transcript=out or None,
    )
    emit(
        True if status == "awaiting_agents" else False,
        {
            "status": status,
            "message": message,
            "handoff_path": handoff,
            "backend": backend,
        },
        None if status == "awaiting_agents" else message,
        0 if status == "awaiting_agents" else 1,
    )


if __name__ == "__main__":
    main()
