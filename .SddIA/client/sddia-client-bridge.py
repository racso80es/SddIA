#!/usr/bin/env python3
"""PoC Kalma2 — puente HTTP local SddIA (instancia, no genoma)."""
from __future__ import annotations

import json
import os
import sys
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
UI_DIR = REPO_ROOT / "interfaces" / "kalma2"
HOST = "127.0.0.1"
PORT = int(os.environ.get("SDDIA_CLIENT_PORT", "8765"))

sys.path.insert(0, str(REPO_ROOT / "SddIA" / "scripts" / "qa"))
try:
    from env_loader import load_hierarchical_env  # type: ignore

    load_hierarchical_env(REPO_ROOT)
except Exception as exc:
    sys.stderr.write(f"[kalma2] bóveda no cargada: {exc}\n")


def invoke_engine(prompt: str) -> dict:
    """Ola 1: eco determinista. Ola 2: subprocess execute-process kalma2-interact."""
    return {"success": True, "response": f"[eco PoC] {prompt}"}


class Handler(BaseHTTPRequestHandler):
    def _json(self, code: int, body: dict) -> None:
        raw = json.dumps(body, ensure_ascii=False).encode("utf-8")
        self.send_response(code)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)

    def do_POST(self) -> None:  # noqa: N802
        if self.path != "/api/interact":
            self._json(404, {"success": False, "message": "ruta desconocida", "exit_code": 1})
            return
        try:
            length = int(self.headers.get("Content-Length", "0"))
            payload = json.loads(self.rfile.read(length) or b"{}")
            prompt = payload.get("prompt")
            if not isinstance(prompt, str) or not prompt.strip():
                self._json(400, {"success": False, "message": "prompt requerido", "exit_code": 1})
                return
            t0 = time.time()
            out = invoke_engine(prompt.strip())
            out["duration_ms"] = int((time.time() - t0) * 1000)
            self._json(200, out)
        except Exception as exc:
            self._json(500, {"success": False, "message": str(exc), "exit_code": 1})

    def do_GET(self) -> None:  # noqa: N802
        rel = self.path.split("?", 1)[0].lstrip("/") or "index.html"
        target = (UI_DIR / rel).resolve()
        ui_root = UI_DIR.resolve()
        if not str(target).startswith(str(ui_root)) or not target.is_file():
            self._json(404, {"success": False, "message": "asset no encontrado", "exit_code": 1})
            return
        data = target.read_bytes()
        ctype = {
            "html": "text/html",
            "js": "text/javascript",
            "css": "text/css",
        }.get(target.suffix.lstrip("."), "application/octet-stream")
        self.send_response(200)
        self.send_header("Content-Type", f"{ctype}; charset=utf-8")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def log_message(self, *_args) -> None:
        return


def main() -> int:
    if not UI_DIR.is_dir():
        sys.stderr.write(f"[kalma2] falta bundle UI: {UI_DIR}\n")
        return 1
    srv = ThreadingHTTPServer((HOST, PORT), Handler)
    sys.stderr.write(f"[kalma2] puente activo en http://{HOST}:{PORT}\n")
    try:
        srv.serve_forever()
    except KeyboardInterrupt:
        sys.stderr.write("[kalma2] apagado\n")
        srv.shutdown()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
