---
feature_name: kalma2-bridge-aiua-interact-stale-elf
created: "2026-09-09"
process: bug-fix
version_implementation: "1.0.0"
items:
  - verify-live-organ
  - no-main-rs-mutation
  - archive-pbi-ca6
  - include-source-audit
---

# Implementación — sello ELF `kalma2-bridge`

## Cambios

| Archivo | Cambio |
|---------|--------|
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | **Intacto** en este PR (working tree sucio local excluido) |
| `interfaces/kalma2/app.js` | **Intacto** en este PR |
| `SddIA/target/release/kalma2-bridge` | Ya fresco 2026-09-09 15:17:48 CEST; sin rebuild este ciclo |
| Instancia systemd | Ya reciclada: PID 6151 → **1244544** (15:18:25 CEST) |
| PBI | `pending/` → `docs/todos/done/` |
| `docs/audits/kalma2-wui-tormentosa-chat-20260909.md` | Evidencia de origen versionada |

## Detalle

1. **Predicado de ruta (CA-2):** `POST /api/no-existe` → 404 `ruta desconocida` en 0.4 ms. `POST /api/aiua/interact` no devuelve ese 404 (timeout 3 s / 120 s con 0 bytes: handler retenido en upstream, no dispatcher).
2. **Símbolos (CA-1):** ELF disco = exe del MainPID; `strings` → mangled `handle_aiua_interact`; substring `aiua/intera` en .rodata empaquetado.
3. **WUI (CA-3):** `GET /` HTTP 200; HTML contiene `#aiua-pulse` / «Hablar».
4. **Chat (CA-4):** `POST /api/chat` ≠404 inmediato (3 s, 0 bytes). Combustión Mayeuta = PBI `64f37c7f7b34` si falla prótesis.
5. **Resolvedor:** `_sddia_resolve_daemon_binary` → `SddIA/target/release/kalma2-bridge`, exit 0.

## Sin cambios

- Dispatcher Rust / botón WUI (ya en `main` vía PR #276)
- Lanzador `_sddia_resolve_daemon_binary`
- Retry 503 Gemini
- Unidad systemd de plantilla en repo
