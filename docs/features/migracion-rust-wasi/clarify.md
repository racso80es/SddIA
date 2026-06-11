---
feature_name: migracion-rust-wasi
created: "2026-06-11"
process: feature
purpose: Estabilización de requisitos — certificación migración Rust/WASI
---

# Clarificación — migracion-rust-wasi

Transcript de decisiones (2026-06-11). Resuelve ambigüedades del PBI y del estado parcial en `main`.

---

## D1 — Naturaleza de la feature

| Pregunta | Decisión |
|----------|----------|
| ¿Re-forjar cápsulas desde cero? | **No.** Las cápsulas Rust ya existen; el trabajo es **certificación + poda + normativa**. |
| ¿Proceso? | `feature` v1.3.0 |
| Rama | `feat/migracion-rust-wasi-certificacion` |
| `persist_ref` | `docs/features/migracion-rust-wasi` (reutilizar topología existente) |
| Manifiesto | `docs/todos/pending/OPERATIVO-PBI-Migracion-Rust-WASI.md` |

---

## D2 — Fallbacks Python en laboratorio

| Opción | Decisión |
|--------|----------|
| Mantener `scripts/skills/*.py` indefinidamente | **Rechazada** — viola criterio O1 del PBI |
| Eliminar fallbacks y exigir `wasmtime` + `.wasm` | **Adoptada** — coherente con CI `SDDIA_CI_REQUIRE_WASI=1` |
| Excepción para `execute-process.py` | **Sí** — orquestador lab permanece Python; no es cápsula skill/tool |

**Touchpoints:** `execute_process_capsules.py` (`_crypto`, `_git`, `_shell`), `scripts/skills/*.py`.

---

## D3 — `requirements.txt` (PyYAML)

| Pregunta | Decisión |
|----------|----------|
| ¿Eliminar `requirements.txt`? | **Condicional** — solo si ningún script QA/orquestador requiere PyYAML tras la poda |
| Acción | Auditar `verify-process-integrity.py`, `execute-action.py`; mantener dependencia documentada mientras el orquestador lab la use |

---

## D4 — Target de compilación

| Pregunta | Decisión |
|----------|----------|
| Target canónico | `wasm32-wasip1` |
| Binarios nativos `dev` | Permitidos en desarrollo local; **runtime de producción/CI = WASI** |
| Config `.cargo/config.toml` | Por cápsula o workspace según patrón `wasi-poc` |

---

## D5 — Contratos y README

| Documento | Cambio requerido |
|-----------|------------------|
| `skills-contract.md` | Declarar Rust/WASI como sustrato de ejecución |
| `tools-contract.md` | Idem en sección Delivery |
| `README.md` | Retirar «Python permitido» para cápsulas skills/tools |

**Nota:** mutación de genoma en `SddIA/norms/` vía procesos autorizados o operador con feature activa.

---

## D6 — Precedentes cerrados

| Feature | Relación |
|---------|----------|
| `wasi-poc-ignition` | PoC físico del puente wasmtime |
| `ci-wasi-runtime-validation` | Validación CI en runner (PR #84) |
| Rama previa `feat/migracion-rust-wasi-12481127328253895075` | Obsoleta; sustituida por rama de certificación |

---

## D8 — Excepción estructural: git-manager.py y bus-operator.py

| Cápsula | Motivo de excepción |
|---------|---------------------|
| `scripts/skills/git-manager.py` | `git-manager.wasm` invoca `git` como subprocess (`std::process::Command`). WASI no soporta subprocess spawning con el flag `--dir=.` estándar. La ruta Python es el camino funcional en entorno lab. |
| `scripts/skills/bus-operator.py` | `bus-operator.wasm` internamente invoca herramientas vía `Command::new("wasmtime")`. Misma limitación WASI. La ruta Python (`execute-action.py` → `_invoke_bus_operator`) es el camino funcional. |

**Decisión:** Ambos archivos permanecen hasta que las cápsulas Rust sean rediseñadas para operar sin subprocess interno (ej. delegación al host vía capability WASI, o refactoring del modelo de invocación). Se añade nota en `implementation.md` como deuda técnica conocida.

**No eliminadas:** `scripts/skills/git-manager.py`, `scripts/skills/bus-operator.py`.  
**Sí eliminadas:** `scripts/skills/cryptography-manager.py`, `scripts/skills/shell-executor.py`.

---

## D7 — Criterios de Done

```text
Done = PR mergeado en main
     + validacion.md global: APTO, pbi_archived: true
     + OPERATIVO-PBI-Migracion-Rust-WASI.md en docs/todos/done/
     + cargo build --workspace --target wasm32-wasip1 sin errores
     + wasi-runtime-smoke + eda-bus-e2e-smoke verdes
```
