---
feature_name: poc-interface-comunicacion
created: "2026-06-18"
process: feature
branch_name: feat/poc-interface-comunicacion
persist_ref: docs/features/poc-interface-comunicacion
pbi_ref: docs/todos/pending/PBI_PoC_Interface_Comuniccion.md
document_id: PBI-POC-INTERFACE-COMUNICACION
status: implementation
decisions_ref: clarify.md#d5
items:
  - id: T1
    artifact: .SddIA/client/sddia-client-bridge.py
    nature: runtime-instance
    operation: create
    genome_mutation: false
  - id: T2
    artifact: interfaces/kalma2/index.html
    nature: ui-static
    operation: create
    genome_mutation: false
  - id: T3
    artifact: interfaces/kalma2/app.js
    nature: ui-static
    operation: create
    genome_mutation: false
  - id: T4
    artifact: interfaces/kalma2/style.css
    nature: ui-static
    operation: create
    genome_mutation: false
  - id: T5
    artifact: .SddIA/.dev/.env
    nature: vault
    operation: read
    genome_mutation: false
  - id: T6
    artifact: docs/features/poc-interface-comunicacion/_smoke-kalma2-interact.json
    nature: fixture
    operation: create
    genome_mutation: false
---

# Implementation — PoC Interface Comunicación (Kalma2)

Touchpoints y propuestas de forja tras ratificación **Q1–Q4** (`clarify.md` §D5). **Ola 1 (PoC) = stub eco**; sin mutación de genoma indexado (`external-ai-constraints` DA-2 respetada: solo rutas instancia + `interfaces/`).

## 1. Resolución de decisiones (cierre)

| Decisión | Valor final |
|----------|-------------|
| Bundle UI | `interfaces/kalma2/` (versionado, servido por el puente) |
| Puente | `.SddIA/client/sddia-client-bridge.py` (instancia) |
| Stack HTTP | `http.server` stdlib — un único script (estáticos + `/api/interact`) |
| Target Ola 1 | **stub eco** dentro del puente (validar UI + transporte) |
| Target Ola 2 (post-PoC) | proceso genoma `kalma2-interact` vía `execute-process.py` |
| Puerto | `8765`; override `SDDIA_CLIENT_PORT` (bóveda) |
| Bind | `127.0.0.1` exclusivo |

## 2. Touchpoints físicos

### T1 — Puente HTTP `.SddIA/client/sddia-client-bridge.py`

Responsabilidad: servir estáticos de `interfaces/kalma2/` + endpoint `POST /api/interact`. Carga jerarquía de bóvedas al arranque. Ola 1: respuesta eco; gancho `_invoke_engine()` aislado para sustituir por subprocess en Ola 2.

Esqueleto propuesto (sujeto a ajuste en forja Tekton):

```python
#!/usr/bin/env python3
"""PoC Kalma2 — puente HTTP local SddIA (instancia, no genoma)."""
from __future__ import annotations
import json, os, sys, time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
UI_DIR = REPO_ROOT / "interfaces" / "kalma2"
HOST = "127.0.0.1"
PORT = int(os.environ.get("SDDIA_CLIENT_PORT", "8765"))

# Bóvedas: reutilizar env_loader del Core (solo lectura, sin mutar genoma)
sys.path.insert(0, str(REPO_ROOT / "SddIA" / "scripts" / "qa"))
try:
    from env_loader import load_hierarchical_env  # type: ignore
    load_hierarchical_env(REPO_ROOT)
except Exception as exc:  # fail-soft en PoC; log a stderr
    sys.stderr.write(f"[kalma2] bóveda no cargada: {exc}\n")


def invoke_engine(prompt: str) -> dict:
    """Ola 1: eco determinista. Ola 2: subprocess execute-process kalma2-interact."""
    return {"success": True, "response": f"[eco PoC] {prompt}", "duration_ms": 0}


class Handler(BaseHTTPRequestHandler):
    def _json(self, code: int, body: dict) -> None:
        raw = json.dumps(body).encode("utf-8")
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)

    def do_POST(self):  # noqa: N802
        if self.path != "/api/interact":
            return self._json(404, {"success": False, "message": "ruta desconocida", "exit_code": 1})
        try:
            length = int(self.headers.get("Content-Length", "0"))
            payload = json.loads(self.rfile.read(length) or b"{}")
            prompt = payload.get("prompt")
            if not isinstance(prompt, str) or not prompt.strip():
                return self._json(400, {"success": False, "message": "prompt requerido", "exit_code": 1})
            t0 = time.time()
            out = invoke_engine(prompt)
            out["duration_ms"] = int((time.time() - t0) * 1000)
            return self._json(200, out)
        except Exception as exc:
            return self._json(500, {"success": False, "message": str(exc), "exit_code": 1})

    def do_GET(self):  # noqa: N802 — sirve estáticos del bundle
        rel = self.path.lstrip("/") or "index.html"
        target = (UI_DIR / rel).resolve()
        if not str(target).startswith(str(UI_DIR.resolve())) or not target.is_file():
            return self._json(404, {"success": False, "message": "asset no encontrado", "exit_code": 1})
        data = target.read_bytes()
        ctype = {"html": "text/html", "js": "text/javascript", "css": "text/css"}.get(
            target.suffix.lstrip("."), "application/octet-stream")
        self.send_response(200)
        self.send_header("Content-Type", f"{ctype}; charset=utf-8")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def log_message(self, *_):  # silencio salvo errores
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
        srv.shutdown()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

Notas:
- **R1/R5:** bind fijo `127.0.0.1`; protección path-traversal en `do_GET`.
- **R4:** `load_hierarchical_env(REPO_ROOT)` antes de servir.
- **O7:** `invoke_engine()` es el único punto que muta entre Ola 1 (eco) y Ola 2 (subprocess).

### T2 — `interfaces/kalma2/index.html`

Estructura mínima: `<textarea>` prompt, `<button>` envío, `<pre>`/`<textarea readonly>` salida. Enlaza `style.css` y `app.js`.

### T3 — `interfaces/kalma2/app.js`

`fetch('/api/interact', {method:'POST', body: JSON.stringify({prompt})})`. Deshabilita botón mientras la promesa está pendiente (**O6/AC4**); re-habilita en `finally`. Renderiza `response` o `message`. **Sin** estado persistente (**R2/O5**).

```javascript
const $ = (id) => document.getElementById(id);
async function enviar() {
  const btn = $("send"), out = $("output"), prompt = $("prompt").value.trim();
  if (!prompt) return;
  btn.disabled = true; out.value = "…procesando";
  try {
    const r = await fetch("/api/interact", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ prompt }),
    });
    const data = await r.json();
    out.value = data.success ? data.response : `[error] ${data.message}`;
  } catch (e) {
    out.value = `[fallo red] ${e}`;
  } finally {
    btn.disabled = false;
  }
}
document.addEventListener("DOMContentLoaded", () => $("send").addEventListener("click", enviar));
```

### T4 — `interfaces/kalma2/style.css`

Layout vertical mínimo (textarea ancho, botón, salida monoespaciada). Sin dependencias externas.

### T5 — Bóvedas `.SddIA/.dev/.env` (solo lectura)

Reutiliza `load_hierarchical_env`. Variable opcional `SDDIA_CLIENT_PORT`. Sin secretos nuevos en Ola 1 (el eco no llama IA externa).

### T6 — Fixture smoke `_smoke-kalma2-interact.json`

Plantilla reproducible para validar contrato HTTP:

```json
{ "prompt": "ping de prueba kalma2" }
```

Smoke manual: `curl -s -X POST localhost:8765/api/interact -d @_smoke-kalma2-interact.json -H 'Content-Type: application/json'` → `success:true`, `response` contiene el prompt.

## 3. Olas de forja

| Ola | Alcance | Artefactos | Gate salida |
|-----|---------|------------|-------------|
| **W1** | Puente + UI + eco | T1–T4, T6 | AC1, AC2, AC4 (UI+HTTP vivos; eco visible) |
| **W2** | Bóvedas + smoke | T5, T6 | AC3, AC5 (env cargado; prompt → respuesta) |
| **W3 (post-PoC)** | Motor real `kalma2-interact` | proceso genoma vía `entity-manager` | O7 con engine real; **requiere execute-process**, no forja manual |

## 4. Frontera de genoma (cumplimiento DA-2)

| Artefacto | ¿Genoma indexado? | Vía de forja |
|-----------|-------------------|--------------|
| `.SddIA/client/*` | No (instancia) | Escritura directa permitida |
| `interfaces/kalma2/*` | No (fuera de `directories.*` Cúmulo) | Escritura directa permitida |
| Documentación feature | No (excepción `docs/features/`) | Escritura directa permitida |
| `kalma2-interact` (W3) | **Sí** (`directories.process`) | **`execute-process.py --process entity-manager`** — prohibido bisturí IDE |

## 5. Riesgos y deuda

| Riesgo | Mitigación PoC | Deuda |
|--------|----------------|-------|
| Sin auth (R3) | Bind 127.0.0.1 | Cerbero/RBAC en feature posterior |
| Eco no es motor real | Aislar en `invoke_engine()` | W3 proceso genoma |
| Puente fuera de versionado (instancia) | Documentado en `implementation.md` | Promoción a `SddIA/scripts/clients/` si se estabiliza |
| CORS si se abre `index.html` vía `file://` | Servir SIEMPRE desde el puente (`http://127.0.0.1:PORT`) | — |

## 6. Trazabilidad PBI → touchpoints

| AC PBI | Touchpoint |
|--------|------------|
| AC1 | T2, T3, T4 |
| AC2 | T1 (`/api/interact`) |
| AC3 | T1 (`load_hierarchical_env`) |
| AC4 | T3 (botón disabled) |
| AC5 | T1 + T6 (smoke) |
| AC6 | cierre documental (PBI→done, validacion.md) |
