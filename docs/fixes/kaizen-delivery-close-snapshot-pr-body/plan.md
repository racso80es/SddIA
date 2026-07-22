---
feature_name: kaizen-delivery-close-snapshot-pr-body
created: "2026-07-22"
process: bug-fix
version_plan: "1.0.0"
branch_name: fix/kaizen-delivery-close-snapshot-pr-body
persist_ref: docs/fixes/kaizen-delivery-close-snapshot-pr-body
---

# Plan de ejecución — kaizen-delivery-close-snapshot-pr-body

## Fase 0 — Diseño (Dedalo) ✅

- [x] `objectives.md` consumido
- [x] `spec.md` emitido (K1–K4, contratos, CA)
- [x] `plan.md` emitido (este archivo)

## Fase 1 — Helpers puros (Tekton)

| # | Tarea | Detalle |
|---|-------|---------|
| 1.1 | `parse_porcelain_paths` | Función privada en `phase_capsules.rs`; tests con fixtures del incidente (26 paths) |
| 1.2 | `resolve_pr_body_file_path` | `persist_ref/.tmp/pr-body.md` o workspace fallback |
| 1.3 | `classify_delivery_error` | Mapeo string → `SNAPSHOT_DIRTY_SKIPPED` / `PR_BODY_METACHAR` |
| 1.4 | `write_pr_body_file` | `create_dir_all` + `fs::write`; error si path no es safe token |

**Gate:** `cargo test -p execute-process` módulo `phase_capsules` verde antes de tocar handlers.

## Fase 2 — K1 Snapshot final (Tekton)

| # | Tarea | Detalle |
|---|-------|---------|
| 2.1 | Refactor `capsule_delivery_snapshot_final_with_repo` | Flujo spec § K1 |
| 2.2 | Gate post-commit | `status` limpio + `hash_after != hash_before` |
| 2.3 | State | Solo insertar `snapshot_commit_hash` si fase `executed` |
| 2.4 | Error tipado | `failed` + `error_code: SNAPSHOT_DIRTY_SKIPPED` |

**Riesgo:** porcelain con paths escapados (`"` en nombres raros). Mitigación: test con paths simples; documentar limitación si git quote aparece.

## Fase 3 — K2/K3 Apertura en forja (Tekton)

| # | Tarea | Detalle |
|---|-------|---------|
| 3.1 | Sustituir `--body` por `--body-file` | Escribir archivo bajo `persist_ref/.tmp/` |
| 3.2 | Preflight path | Fallar antes de `shell-executor` si path inválido |
| 3.3 | Mapeo error shell | `PR_BODY_METACHAR` en catch de `invoke_shell_executor` |
| 3.4 | Preservar idempotencia | `pr_url` preset, `SDDIA_LAB_SIMULATE_GH_PR`, `gh pr view` fallback sin cambio |

## Fase 4 — K4 Smoke y regresión (Tekton)

| # | Tarea | Archivo |
|---|-------|---------|
| 4.1 | Payload smoke | `_smoke-close-cycle.json` |
| 4.2 | Tests adicionales | `phase_capsules.rs` § K4 spec |
| 4.3 | `implementation.md` | Comandos, salida envelope, hashes |
| 4.4 | Regresión DI | Confirmar payloads existentes con `pr_body` una línea siguen funcionando |

Variables lab recomendadas:

```bash
SDDIA_LAB_SKIP_GIT_PUSH=1
SDDIA_LAB_SIMULATE_GH_PR=1
```

## Fase 5 — Verificación y cierre (Argos + documental)

| # | Tarea | Entregable |
|---|-------|------------|
| 5.1 | `validacion.md` | `global: APTO`, checks K1–K4 |
| 5.2 | PBI → `docs/todos/done/` | mismo `document_id` |
| 5.3 | `delivery-close-cycle` en rama | PR único pre-merge |

## Orden de dependencias

```
Fase 1 ──► Fase 2 ──► Fase 4 (tests snapshot)
Fase 1 ──► Fase 3 ──► Fase 4 (tests pr_body)
Fase 4 ──► Fase 5
```

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| `commit` con lista parcial de paths deja tree sucio | Postcondición `status` + `SNAPSHOT_DIRTY_SKIPPED` |
| `persist_ref` ausente en hook pre-push | Fallback workspace `execution_id` |
| Path `.tmp` con caracteres prohibidos | Solo segmentos alfanuméricos bajo `docs/` |
| Bypass git raw en lab | Mantener `invoke_git_manager` / `invoke_shell_executor` exclusivos |
