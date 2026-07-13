---
feature_name: poda-python-rust-clientes
created: "2026-07-11"
updated: "2026-07-11"
process: feature
branch_name: feat/poda-python-rust-clientes
persist_ref: docs/features/poda-python-rust-clientes
pbi_ref: docs/todos/done/[REFACTOR] Poda ejecutables Python — adecuación de clientes a cápsulas Rust.md
document_id: PBI-REFACTOR-PODA-PYTHON-RUST
status: cerrado
priority: alta
related:
  - README.md
  - SddIA/CONSTITUTION_CORE.md
  - SddIA/core/cumulo.paths.json
  - docs/todos/done/[ARQUITECTURA] Migración execute-process a Rust nativo (orquestador soberano).md
  - docs/todos/done/[FIX] Porte procesos residuales capsules bridge a Rust.md
  - docs/todos/done/[FIX] Porte route-domain-event core a Rust (eliminar route bridge).md
  - docs/features/kaizen-rust-capsule-structure/spec.md
---

# Objetivos — poda-python-rust-clientes

## Misión

Eliminar **Python del repositorio SddIA** en su totalidad operativa: runtime, capa QA/aduana, cores duplicados y documentación genómica obsoleta. El binario `execute-process`, cápsulas Rust y clientes shell/bat son el único delivery. **Rotura de compatibilidad** con scripts, hooks y clientes no adaptados: asumida y no mitigada con shims Python.

## Contexto

| Hecho | Implicación |
|-------|-------------|
| Olas 1–3 cerradas | Ruta caliente + limbo sin Python (~19 `.py` eliminados) |
| ~75 `.py` persisten | Capa QA, wrappers legacy, PoC docs |
| 15 cores QA duplicados | Paridad Rust ya en `execute-process`; Python es fósil de referencia |
| `SddIA/skills/*.md` | Citas a `.py` inexistentes; drift documental |
| DEBT-K2 | `github-bridge-watcher` aún spawnnea `github_bridge_process_pr.py` |

## Objetivos medibles

| ID | Objetivo | Criterio de verificación |
|----|----------|--------------------------|
| **O1** | Ruta caliente sin Python | `rg -l 'python3?' SddIA/engine SddIA/daemons SddIA/skills SddIA/tools` → vacío en spawn (excl. comentarios) |
| **O2** | Cores EDA nativos | `python_core.rs` eliminado; fan-out en Rust |
| **O3** | Acciones nativas | Sin `execute-action.py` en hot path |
| **O4** | Auditoría EDA nativa | Sin `audit-entity-eda-coverage.py` en engine |
| **O5** | Telegram fallback Rust | Cápsula `send-telegram-notification` |
| **O6** | Clientes adecuados | `sddia-run`, `invoke`, `_exec_daemon` → binarios Rust |
| **O7** | Purga limbo | `scripts/limbo` ausente; Cúmulo sin `scripts_limbo` |
| **O8** | Paridad JSON | Golden / `cargo test -p execute-process` verdes |
| **O9** | Smoke EDA | Suite e2e post-ola en verde |
| **O10** | Cierre documental | PR + `validacion.md` APTO + PBI en `done/` (O18) |
| **O11** | **Cero `.py` en repo** | `find . -name '*.py'` excl. `.venv/`, `.tools/` → vacío |
| **O12** | **Cero referencias operativas a Python** | Sin rutas `.py` ni `python3` como delivery en genoma operativo (skills, contratos, normas, README, daemons) |
| **O13** | Purga cores QA duplicados | 15 cores Python eliminados; tests migrados a Rust |
| **O14** | Capa QA/aduana sin Python | `SddIA/scripts/qa/` sin `.py`; hooks/runners Rust o shell |
| **O15** | Docs skills alineadas | `SddIA/skills/{name}.md` → crates + `compiled_capsules` |
| **O16** | DEBT-K2 cerrado | `github-bridge-watcher` sin spawn Python |
| **O17** | Binarios Rust coherentes | `sddia-qa verify-compiled-capsules` 24/24 tras `cargo build --workspace` |
| **O18** | Cierre documental | PR + `validacion.md` APTO + PBI en `done/` |

## Política de compatibilidad

| Principio | Decisión |
|-----------|----------|
| Clientes legacy | No se mantienen wrappers Python; adaptación obligatoria |
| Hooks git Python | Reemplazar por gates Rust/shell o subcomandos orquestador |
| Tests Python | Migrar a `cargo test` / smokes Rust o retirar |
| PoC `.py` en docs | Eliminar o convertir; no excepción permanente |

## Ley aplicada

- Git vía `skill:git-manager`.
- Mutaciones genoma vía `entity-manager`.
- SSOT: `SddIA/core/cumulo.paths.json`.
- Cápsulas Rust, JSON stdin/stdout.

## Fases (runtime IDE)

| Fase | Agente | Estado |
|------|--------|--------|
| Inicialización de Espacio de Trabajo | git-manager | ✅ ejecutada |
| Estabilización de Requisitos | Mayeuta | ✅ `clarify.md` (ampliado v2) |
| Diseño de Blueprint | Dedalo | ✅ `spec.md`, `plan.md` (8 olas) |
| Ejecución Olas 1–6 | Tekton | ✅ |
| Ejecución Olas 7–8 | Tekton | ✅ |
| Verificación | Argos | ✅ `validacion.md` APTO |
| Cierre documental en rama | filesystem-manager | ✅ PBI → done |
| Cierre de entrega | delivery-close-cycle | ⏳ PR pendiente |
