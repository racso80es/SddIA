#!/usr/bin/env python3
"""kalma2-agent-runtime-cursor — prótesis Python (Foso Biológico).

Modos:
  AGENT_PHASE (default) — JSON stdin → última línea JSON stdout (full-cycle B).
  CHAT_STREAM — inserta en SQLite local de Cursor (state.vscdb) + tokens por stdout (SSE).

Backends AGENT_PHASE:
  cli  (default) — SDDIA_AGENT_RUNTIME_CLI || SDDIA_LLM_CLI_COMMAND || "cursor-agent --print"
  sdk            — Cursor SDK Python (cursor_sdk) con CURSOR_API_KEY / local cwd

CHAT_STREAM / SQLite (entropía absorbida aquí; cero crates en Core):
  SDDIA_CURSOR_VSCDB          — ruta absoluta a globalStorage/state.vscdb
  SDDIA_CURSOR_SQLITE_WRITE   — 1 (default si DB existe) escribe; 0 solo dry-run + stream
  SDDIA_CURSOR_COMPOSER_ID    — reutilizar composer existente (opcional)
  SDDIA_CURSOR_WORKSPACE_ID   — hash workspaceStorage (opcional; se infiere del repo)
  SDDIA_LLM_INFER_COMMAND     — CLI de tokens (preferente; no reentrar prótesis)
  SDDIA_LLM_REQUIRE_INFER=1   — fallar si no hay CLI (demo live S1)
  SDDIA_AGENT_RUNTIME_REQUIRE_CLI=1 — AGENT_PHASE: CLI missing → failed (no awaiting soft)
  Autodetect cursor-agent/agent inyecta --trust (host no-interactivo).
  SDDIA_CURSOR_IDE_WATCH_ONLY=1 — rechazado (L-IDE); oráculo = CLI.
  SDDIA_CURSOR_WAKE_AGENT=1   — segundo disparo CLI post-persist SQLite.

Mock lab/CI:
  SDDIA_AGENT_RUNTIME_MOCK=1 → AGENT_PHASE executed; CHAT_STREAM eco de tokens.
  SDDIA_LLM_CHAT_MOCK=1 → CHAT_STREAM eco (sin tocar SQLite).
  SDDIA_AGENT_RUNTIME_LAB_AUTO=1 → lab wrapper status=executed
"""
from __future__ import annotations

import json
import os
import shlex
import sqlite3
import subprocess
import sys
import uuid
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


def _ensure_noninteractive_agent_flags(parts: list[str]) -> list[str]:
    """Añade --trust a cursor-agent/agent si falta (host no-interactivo / Workspace Trust)."""
    if not parts:
        return parts
    bin0 = Path(parts[0]).name
    if bin0 not in ("cursor-agent", "agent"):
        return parts
    flags = set(parts[1:])
    if flags & {"--trust", "-f", "--yolo"}:
        return parts
    # Tras --print suele ir el resto; insertar --trust al final de flags conocidos
    out = list(parts)
    out.append("--trust")
    return out


def resolve_cli() -> list[str]:
    """CLI para AGENT_PHASE — resuelve ~/.local/bin; no reentra la prótesis como infer."""
    for key in ("SDDIA_AGENT_RUNTIME_CLI", "SDDIA_LLM_INFER_COMMAND", "SDDIA_LLM_CLI_COMMAND"):
        raw = os.environ.get(key, "").strip()
        if not raw:
            continue
        if key == "SDDIA_LLM_CLI_COMMAND" and "kalma2-agent-runtime-cursor.py" in raw:
            continue
        return _ensure_noninteractive_agent_flags(_normalize_infer_argv(split_command(raw)))
    for name in ("cursor-agent", "agent"):
        hit = _which_on_path(name)
        if hit:
            return _ensure_noninteractive_agent_flags([hit, "--print"])
    return _ensure_noninteractive_agent_flags(["cursor-agent", "--print"])


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

    branch = ""
    if doc.get("branch_name"):
        branch = str(doc.get("branch_name"))
    elif isinstance(inputs, dict):
        branch = str(inputs.get("branch_name") or inputs.get("pr_branch") or "")

    brief = role_brief(str(agent), str(phase), str(process))
    parts = [
        "[HARD OVERRIDE — SddIA kalma2-agent-runtime-cursor]",
        brief,
        "",
        f"- process: {process}",
        f"- phase: {phase}",
        f"- agents: {', '.join(str(a) for a in agents)}",
        f"- persist_ref: {persist}",
        f"- branch_name: {branch}",
        f"- pbi_ref: {pbi_ref}",
        f"- correlation_id: {corr}",
        "",
        "Reglas:",
        "- Git solo vía skill:git-manager del ecosistema (no bypass raw destructivo).",
        "- Evidencia git: preferir `./sddia-run.sh --tool git-manager` (JSON stdin) o evidencia ya materializada por handler nativo PPR; no depender del Shell IDE.",
        "- KM / docs/todos/: materializar semillas Kaizen solo como agent:cumulo (Cosecha Kaizen) o vía event Kaizen_Alert_Required; Tekton/Argos NO escriben TODOs bajo docs/todos/.",
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

    op = (doc.get("operation") or "").strip()
    if op == "CHAT_STREAM":
        run_chat_stream(doc)
        return

    # Default / AGENT_PHASE (contrato full-cycle B)
    run_agent_phase(doc)


def run_chat_stream(doc: dict[str, Any]) -> None:
    """CHAT_STREAM: prótesis SQLite Cursor + tokens por stdout para SSE."""
    prompt = (doc.get("prompt") or "").strip()
    if not prompt:
        print("prompt vacío", file=sys.stderr)
        raise SystemExit(1)

    if env_truthy("SDDIA_LLM_CHAT_MOCK") or env_truthy("SDDIA_AGENT_RUNTIME_MOCK"):
        words = prompt.split() or ["(vacío)"]
        for w in words:
            print(w, flush=True)
        print("\n[kalma2-chat-stream mock ok]", flush=True)
        raise SystemExit(0)

    repo = Path(doc.get("repo_root") or os.getcwd()).resolve()
    db = resolve_cursor_vscdb()
    write = env_truthy("SDDIA_CURSOR_SQLITE_WRITE") or (
        os.environ.get("SDDIA_CURSOR_SQLITE_WRITE", "").strip() == ""
        and db is not None
        and db.is_file()
    )

    composer_id = (os.environ.get("SDDIA_CURSOR_COMPOSER_ID") or "").strip() or str(uuid.uuid4())
    user_bubble = str(uuid.uuid4())
    asst_bubble = str(uuid.uuid4())
    now_iso = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"
    now_ms = int(datetime.now(timezone.utc).timestamp() * 1000)
    name = ("Kalma2: " + prompt[:48]).strip()
    workspace_id = resolve_workspace_id(repo)

    # Oráculo = CLI (L-IDE: el IDE no dispara solo por insert SQLite).
    # SDDIA_CURSOR_IDE_WATCH_ONLY=1 está prohibido en live (fallar explícito).
    if env_truthy("SDDIA_CURSOR_IDE_WATCH_ONLY"):
        emit_meta(
            "error",
            error="ide_watch_only_forbidden",
            laudo="L-IDE",
            hint="Usar infer CLI (SDDIA_LLM_INFER_COMMAND); insert SQLite ≠ oráculo",
        )
        print(
            "[kalma2] SDDIA_CURSOR_IDE_WATCH_ONLY=1 rechazado (L-IDE). "
            "El disparo autónomo es el CLI post-prompt, no el watch del IDE.",
            flush=True,
        )
        raise SystemExit(4)

    reply, infer_backend = stream_infer_tokens(prompt)
    emit_meta("oracle", mode="cli", ide_auto_fire=False, infer_backend=infer_backend)

    if write and db is not None and db.is_file():
        try:
            persist_chat_to_sqlite(
                db,
                composer_id=composer_id,
                user_bubble_id=user_bubble,
                asst_bubble_id=asst_bubble,
                prompt=prompt,
                reply=reply,
                name=name,
                now_iso=now_iso,
                now_ms=now_ms,
                workspace_id=workspace_id,
                repo=repo,
            )
            print(
                f"\n[kalma2-sqlite ok composer={composer_id[:8]}… backend={infer_backend}]",
                flush=True,
            )
        except Exception as e:  # noqa: BLE001 — prótesis: no tumbar Core
            print(f"\n[kalma2-sqlite error] {e}", flush=True)
            raise SystemExit(1)
    else:
        reason = "DB ausente" if db is None or not db.is_file() else "SDDIA_CURSOR_SQLITE_WRITE=0"
        print(f"\n[kalma2-sqlite skip] {reason} backend={infer_backend}", flush=True)

    # DEBT-L-IDE: wake opcional vía CLI (no vía watch IDE) tras persistir.
    if write and env_truthy("SDDIA_CURSOR_WAKE_AGENT") and infer_backend.startswith("cli"):
        wake_prompt = (
            f"Contexto Kalma2 composer `{composer_id}` ya persistido en state.vscdb. "
            f"No reescribas la DB. Confirma con una sola palabra: awake"
        )
        ok, out, err = run_cli(repo, wake_prompt)
        if ok:
            print(f"\n[kalma2-wake ok] {(out.splitlines()[-1] if out else 'ok')[:120]}", flush=True)
        else:
            print(f"\n[kalma2-wake skip] {err or out or 'wake failed'}", flush=True)

    raise SystemExit(0)


def resolve_cursor_vscdb() -> Path | None:
    override = (os.environ.get("SDDIA_CURSOR_VSCDB") or "").strip()
    if override:
        return Path(override).expanduser()
    home = Path.home()
    candidates = [
        home / ".config/Cursor/User/globalStorage/state.vscdb",
        home / "Library/Application Support/Cursor/User/globalStorage/state.vscdb",
        home / "AppData/Roaming/Cursor/User/globalStorage/state.vscdb",
    ]
    for c in candidates:
        if c.is_file():
            return c
    return candidates[0]


def resolve_workspace_id(repo: Path) -> str | None:
    override = (os.environ.get("SDDIA_CURSOR_WORKSPACE_ID") or "").strip()
    if override:
        return override
    ws_root = Path.home() / ".config/Cursor/User/workspaceStorage"
    if not ws_root.is_dir():
        mac = Path.home() / "Library/Application Support/Cursor/User/workspaceStorage"
        ws_root = mac if mac.is_dir() else ws_root
    if not ws_root.is_dir():
        return None
    repo_s = str(repo.resolve())
    best: tuple[int, str] | None = None
    for d in ws_root.iterdir():
        wj = d / "workspace.json"
        if not wj.is_file():
            continue
        try:
            folder = str(json.loads(wj.read_text(encoding="utf-8")).get("folder") or "")
        except Exception:
            continue
        path = folder.replace("file://", "")
        if path.rstrip("/") == repo_s.rstrip("/"):
            return d.name
        if repo_s in path or path in repo_s:
            try:
                score = len(os.path.commonpath([path or "/", repo_s]))
            except ValueError:
                score = 0
            if best is None or score > best[0]:
                best = (score, d.name)
    return best[1] if best else None


def _rich_text(text: str) -> str:
    return json.dumps(
        {
            "root": {
                "children": [
                    {
                        "children": [
                            {
                                "detail": 0,
                                "format": 0,
                                "mode": "normal",
                                "style": "",
                                "text": text,
                                "type": "text",
                                "version": 1,
                            }
                        ],
                        "direction": "ltr",
                        "format": "",
                        "indent": 0,
                        "type": "paragraph",
                        "version": 1,
                    }
                ],
                "direction": "ltr",
                "format": "",
                "indent": 0,
                "type": "root",
                "version": 1,
            }
        },
        ensure_ascii=False,
    )


def _minimal_bubble(bubble_id: str, typ: int, text: str, created_at: str) -> dict[str, Any]:
    return {
        "_v": 3,
        "type": typ,
        "bubbleId": bubble_id,
        "text": text,
        "richText": _rich_text(text),
        "createdAt": created_at,
        "approximateLintErrors": [],
        "lints": [],
        "codebaseContextChunks": [],
        "commits": [],
        "pullRequests": [],
        "attachedCodeChunks": [],
        "assistantSuggestedDiffs": [],
        "gitDiffs": [],
        "interpreterResults": [],
        "images": [],
        "attachedFolders": [],
        "attachedFoldersNew": [],
        "userResponsesToSuggestedCodeBlocks": [],
        "suggestedCodeBlocks": [],
        "diffsForCompressingFiles": [],
        "relevantFiles": [],
        "toolResults": [],
        "notepads": [],
        "capabilities": [],
        "multiFileLinterErrors": [],
    }


def _minimal_composer(
    composer_id: str,
    name: str,
    headers: list[dict[str, Any]],
    now_ms: int,
) -> dict[str, Any]:
    return {
        "_v": 16,
        "composerId": composer_id,
        "name": name,
        "richText": _rich_text(""),
        "hasLoaded": True,
        "text": "",
        "fullConversationHeadersOnly": headers,
        "conversationMap": {},
        "status": "completed",
        "unifiedMode": "chat",
        "context": {},
        "generatingBubbleIds": [],
        "isReadingLongFile": False,
        "codeBlockData": {},
        "originalFileStates": {},
        "newlyCreatedFiles": [],
        "newlyCreatedFolders": [],
        "createdAt": now_ms,
        "lastUpdatedAt": now_ms,
        "conversationCheckpointLastUpdatedAt": now_ms,
        "hasChangedContext": False,
        "activeTabsShouldBeReactive": True,
        "capabilities": [],
        "isFileListExpanded": False,
    }


def _which_on_path(name: str) -> str | None:
    """Busca ejecutable incluyendo ~/.local/bin (Cursor Agent CLI post-install)."""
    extras = [
        str(Path.home() / ".local/bin"),
        "/usr/local/bin",
    ]
    path = os.environ.get("PATH", "")
    search = os.pathsep.join([*extras, path])
    for d in search.split(os.pathsep):
        if not d:
            continue
        cand = Path(d) / name
        if cand.is_file() and os.access(cand, os.X_OK):
            return str(cand)
    return None


def resolve_infer_cli() -> list[str]:
    """CLI de inferencia — nunca reentrar en esta prótesis (evita recursión STREAM)."""
    for key in ("SDDIA_LLM_INFER_COMMAND", "SDDIA_AGENT_RUNTIME_CLI"):
        raw = os.environ.get(key, "").strip()
        if raw:
            return _ensure_noninteractive_agent_flags(_normalize_infer_argv(split_command(raw)))
    raw = os.environ.get("SDDIA_LLM_CLI_COMMAND", "").strip()
    if raw and "kalma2-agent-runtime-cursor.py" not in raw:
        return _ensure_noninteractive_agent_flags(_normalize_infer_argv(split_command(raw)))
    # Autodetección post-install Cursor CLI
    for name in ("cursor-agent", "agent"):
        hit = _which_on_path(name)
        if hit:
            return _ensure_noninteractive_agent_flags([hit, "--print", "--mode", "ask"])
    return []


def _normalize_infer_argv(parts: list[str]) -> list[str]:
    """Si el binario no está en PATH, resuelve vía ~/.local/bin."""
    if not parts:
        return parts
    bin0 = parts[0]
    if Path(bin0).is_file() or "/" in bin0:
        return parts
    hit = _which_on_path(bin0)
    if hit:
        return [hit, *parts[1:]]
    return parts


def emit_meta(backend: str, **extra: Any) -> None:
    """Primera línea de telemetría para SSE (UI/ops). No es token de modelo."""
    payload = {"backend": backend, **extra}
    print(f"[kalma2-meta] {json.dumps(payload, ensure_ascii=False)}", flush=True)


def stream_infer_tokens(prompt: str) -> tuple[str, str]:
    """CLI stream si existe; si no, ack determinista (o fail si REQUIRE_INFER)."""
    require = env_truthy("SDDIA_LLM_REQUIRE_INFER")
    try:
        cmd = resolve_infer_cli()
    except Exception as e:
        cmd = []
        if require:
            emit_meta("error", error=str(e))
            print(f"[infer resolve error] {e}", flush=True)
            raise SystemExit(2)

    if cmd:
        emit_meta("cli", command=cmd[0])
        try:
            timeout = int(os.environ.get("SDDIA_LLM_CLI_TIMEOUT_SECS", "120") or "120")
            proc = subprocess.Popen(
                cmd,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            assert proc.stdin is not None and proc.stdout is not None
            try:
                proc.stdin.write(prompt)
                proc.stdin.close()
            except BrokenPipeError:
                pass
            chunks: list[str] = []
            try:
                for line in proc.stdout:
                    for w in line.split():
                        print(w, flush=True)
                        chunks.append(w)
                proc.wait(timeout=timeout)
            except subprocess.TimeoutExpired:
                proc.kill()
                print("\n[infer timeout]", flush=True)
            err = (proc.stderr.read() if proc.stderr else "") or ""
            if chunks and proc.returncode == 0:
                return " ".join(chunks), "cli"
            if chunks:
                return " ".join(chunks), f"cli-rc{proc.returncode}"
            if err.strip():
                print(f"[infer stderr] {err.strip()[:200]}", flush=True)
            if require:
                emit_meta("error", error="cli_empty_or_failed", rc=proc.returncode)
                raise SystemExit(3)
        except FileNotFoundError as e:
            if require:
                emit_meta("error", error=f"cli_not_found:{e}")
                print(f"[infer] CLI no encontrado: {e}", flush=True)
                raise SystemExit(3)
        except SystemExit:
            raise
        except Exception as e:  # noqa: BLE001
            print(f"[infer error] {e}", flush=True)
            if require:
                emit_meta("error", error=str(e))
                raise SystemExit(3)

    if require:
        emit_meta("error", error="no_infer_cli")
        print(
            "[infer] SDDIA_LLM_REQUIRE_INFER=1 pero no hay CLI "
            "(instala Cursor Agent CLI o fija SDDIA_LLM_INFER_COMMAND).",
            flush=True,
        )
        raise SystemExit(3)

    emit_meta("sqlite-ack")
    ack = (
        f"[kalma2→cursor-sqlite] Prompt encolado en state.vscdb. "
        f"Sin CLI de inferencia; abre Cursor para continuar. "
        f"Semilla: {prompt[:240]}"
    )
    for w in ack.split():
        print(w, flush=True)
    return ack, "sqlite-ack"


def persist_chat_to_sqlite(
    db: Path,
    *,
    composer_id: str,
    user_bubble_id: str,
    asst_bubble_id: str,
    prompt: str,
    reply: str,
    name: str,
    now_iso: str,
    now_ms: int,
    workspace_id: str | None,
    repo: Path,
) -> None:
    headers = [
        {
            "bubbleId": user_bubble_id,
            "type": 1,
            "grouping": {"isRenderable": True, "hasText": True, "isShortPlainText": True},
        },
        {
            "bubbleId": asst_bubble_id,
            "type": 2,
            "grouping": {"isRenderable": True, "hasText": True, "isShortPlainText": True},
        },
    ]
    user_b = _minimal_bubble(user_bubble_id, 1, prompt, now_iso)
    asst_b = _minimal_bubble(asst_bubble_id, 2, reply, now_iso)
    composer = _minimal_composer(composer_id, name, headers, now_ms)

    uri = {
        "$mid": 1,
        "fsPath": str(repo),
        "external": f"file://{repo}",
        "path": str(repo),
        "scheme": "file",
    }
    header_value = {
        "type": "head",
        "composerId": composer_id,
        "name": name,
        "lastUpdatedAt": now_ms,
        "createdAt": now_ms,
        "unifiedMode": "chat",
        "forceMode": "edit",
        "hasUnreadMessages": False,
        "isArchived": False,
        "isDraft": False,
        "workspaceIdentifier": {
            "id": workspace_id or "kalma2-unknown",
            "uri": uri,
        },
    }

    con = sqlite3.connect(str(db), timeout=8.0)
    try:
        con.execute("PRAGMA busy_timeout=8000")
        cur = con.cursor()
        rows = [
            (f"composerData:{composer_id}", json.dumps(composer, ensure_ascii=False)),
            (f"bubbleId:{composer_id}:{user_bubble_id}", json.dumps(user_b, ensure_ascii=False)),
            (f"bubbleId:{composer_id}:{asst_bubble_id}", json.dumps(asst_b, ensure_ascii=False)),
        ]
        cur.executemany(
            "INSERT OR REPLACE INTO cursorDiskKV (key, value) VALUES (?, ?)",
            rows,
        )

        row = cur.execute(
            "SELECT value FROM ItemTable WHERE key=?",
            ("composer.composerHeaders",),
        ).fetchone()
        if row and row[0]:
            try:
                idx = json.loads(row[0])
            except json.JSONDecodeError:
                idx = {"allComposers": []}
        else:
            idx = {"allComposers": []}
        composers = idx.get("allComposers")
        if not isinstance(composers, list):
            composers = []
        composers = [
            c
            for c in composers
            if not (isinstance(c, dict) and c.get("composerId") == composer_id)
        ]
        composers.insert(0, header_value)
        idx["allComposers"] = composers
        cur.execute(
            "INSERT OR REPLACE INTO ItemTable (key, value) VALUES (?, ?)",
            ("composer.composerHeaders", json.dumps(idx, ensure_ascii=False)),
        )

        try:
            cur.execute(
                "INSERT OR REPLACE INTO composerHeaders "
                "(composerId, workspaceId, createdAt, lastUpdatedAt, isArchived, isSubagent, "
                "recency, checkpointAt, value) VALUES (?,?,?,?,0,0,?,?,?)",
                (
                    composer_id,
                    workspace_id or "",
                    now_ms,
                    now_ms,
                    now_ms,
                    now_ms,
                    json.dumps(header_value, ensure_ascii=False),
                ),
            )
        except sqlite3.Error:
            pass

        con.commit()
    finally:
        con.close()


def run_agent_phase(doc: dict[str, Any]) -> None:
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
    # S3 live: no enmascarar ausencia de CLI como awaiting_agents
    if soft and env_truthy("SDDIA_AGENT_RUNTIME_REQUIRE_CLI"):
        status = "failed"
        message = f"REQUIRE_CLI: {err or 'CLI ausente'}"
        soft = False
    else:
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
