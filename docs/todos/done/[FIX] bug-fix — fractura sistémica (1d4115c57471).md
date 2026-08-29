---
document_id: PBI-FIX-FRACTURE-1d4115c57471
uuid: "7fa1bc76-f562-4040-b7e3-1e6a843745ff"
title: "[FIX] bug-fix — fractura sistémica (dirty-worktree en workspace-init)"
format: markdown
version: "1.2.0"
created: "2026-08-29"
updated: "2026-08-29"
status: cerrado
closed: "2026-08-29"
priority: media
process: bug-fix
fracture_hash: 1d4115c57471
fracture_process: bug-fix
friction_id: F-DIRTY-WORKTREE
incident_ref: "System_Fracture_Detected — 1d4115c57471"
resolution_ref: docs/fixes/bug-fix-fracture-1d4115c57471/
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - docs/features/kaizen-feature-lab-init-frictions/spec.md
  - docs/fixes/bug-fix-fracture-1d4115c57471/validacion.md
---

# [FIX] bug-fix — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `bug-fix` |
| Emisor | `execute-process` |
| Acción intentada | `workspace-init` |
| Guard físico | `workspace_init::dirty_paths_outside_scope` (L-DIRTY-INIT) |
| Clasificación | `F-DIRTY-WORKTREE` (abort pre-fetch, no colapso de runtime) |

## Traza de error

```
dirty-worktree: cambios fuera de persist_ref/pbi_ref: SddIA/agents/radamanto.thresholds.json, SddIA/core/cumulo.paths.json, SddIA/core/eda-coverage.json, SddIA/engine/execute-process/src/engine/delivery_close.rs, SddIA/engine/execute-process/src/engine/executor.rs, SddIA/engine/execute-process/src/engine/fractal_bus.rs, SddIA/engine/execute-process/src/engine/mod.rs, SddIA/engine/execute-process/src/engine/phase_capsules.rs, SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs, SddIA/engine/execute-process/src/engine/residual_runner.rs, SddIA/engine/execute-process/src/engine/telemetry_receipt.rs, SddIA/engine/execute-process/src/engine/thermodynamic.rs, SddIA/events/telemetry/raw-execution-finished.md, SddIA/interfaces/kalma2-bridge/src/main.rs, SddIA/skills/mayeuta-llm/src/main.rs, interfaces/kalma2/app.js, interfaces/kalma2/index.html, interfaces/kalma2/style.css
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async, refinada por laudo humano)*

### Diagnóstico de causa raíz

- **No es defecto de motor.** El abort es el guard `L-DIRTY-INIT` (laudo de `kaizen-feature-lab-init-frictions`) actuando **según diseño** en `workspace_init.rs` (`dirty_paths_outside_scope` → `return Err(msg)`, líneas 267-280).
- **Detonante:** se lanzó un ciclo `bug-fix`/`workspace-init` con el worktree sucio. Los 18 paths listados son **genoma/core/interfaces sin commitear** (engine Rust, `cumulo.paths.json`, `eda-coverage.json`, `radamanto.thresholds.json`, `kalma2-bridge`, `mayeuta-llm`, `interfaces/kalma2/*`) **fuera** del `persist_ref`/`pbi_ref` del fix.
- **Naturaleza:** fractura de **higiene operativa**, no deuda de código. La respuesta inmune protegió la integridad de la rama: impide arrancar un fix arrastrando trabajo ajeno no consolidado.
- **Estado verificado:** el worktree está **limpio** en `main`; los cambios que detonaron el abort ya fueron consolidados en sus ciclos (PRs #228/#229 y correlatos). El detonante ya no reproduce.

### Veredicto evolutivo

**Kaizen de discriminación** entregado: `F-DIRTY-WORKTREE` aborta sin `System_Fracture_Detected`. El guard `L-DIRTY-INIT` permanece.

### Propuestas

- **Operativa (inmediata):** consolidar/stash cambios ajenos en su propio ciclo antes de lanzar el fix (ya cumplido: `main` limpio). Ante reincidencia deliberada y acotada, escape documentado `SDDIA_LAB_ALLOW_DIRTY=1`.
- **Evolutiva (diferida):** taxonomizar `System_Fracture_Detected` por severidad (`guard-abort` vs `runtime-collapse`) para bajar ruido de PBIs. Requiere ciclo `feature`/`bug-fix` propio con laudo.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

## Criterio de cierre

- [x] Causa raíz clasificada: guard `F-DIRTY-WORKTREE` (L-DIRTY-INIT) actuando por diseño, no defecto de motor
- [x] Detonante resuelto: worktree limpio en `main`, cambios ajenos consolidados en sus ciclos
- [x] Kaizen: abort dirty sin `System_Fracture_Detected` (`workspace_init.rs`)
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
