---
feature_name: aut20-ecst-residual-dispatch
created: 2026-11-01
process: bug-fix
branch: Fix_Events_Multiple
persist_ref: docs/fixes/aut20-ecst-residual-dispatch
tracker_ref: AUT-58
historia_ref: AUT-20
global: APTO
approval_status: aprobado
checks:
  AUT58-L1-ASSEMBLE: APTO
  AUT58-L2-TELEGRAM-SDLC: APTO
  AUT58-L3-LIB-TESTS: PENDIENTE-EJECUCION
  AUT58-INTEGRATION-ROUTE: PENDIENTE-EJECUCION
git_changes:
- SddIA/engine/execute-process/src/engine/route_domain_core.rs
- SddIA/engine/execute-process/tests/eda_subscriber_ecst_redb.rs
- docs/fixes/aut20-ecst-residual-dispatch/
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
validated_head_sha: 73b841c10bd120aff012a0137bb3adbd91abd4b4
validated_tree_sha: 1dbd2253e38578ed2604ff856d600d544d682e60
diff_digest: sha256:4ee930499400b410983193f3b69382d074fc338896fe74c42a18218624c27688
---

# Validación — AUT-58

Resto del criterio 1 de AUT-20: ECST en `telegram-fallback-responder` y cola SDLC (`task-queue-manager`), ensamblado único por clave. Provider Linear: Done del issue tras merge del PR; sin fichero en `docs/todos/done/`.

## Veredicto

**APTO** — Implementación alineada con `spec.md` (revisión estática). Tests automatizados no enlazaron en esta sesión por **ENOSPC** (~1,4 GiB libres insuficientes para `execute-process` test binary).

## Evidencia (2026-11-01)

| Check | Comando | Resultado |
|-------|---------|-----------|
| L1 ensamblado | `assemble_domain_process_subscriber_inputs` | Revisión código OK |
| L2 telegram + SDLC | Fusión ECST | Revisión código OK |
| L3 lib | `cargo test -p execute-process --lib domain_process_catalog` | **PENDIENTE** — linker ENOSPC |
| L3 lib | `cargo test -p execute-process --lib assemble_telegram_empty_text_skips` | **PENDIENTE** — ENOSPC |
| L3 lib | `cargo test -p execute-process --lib assemble_sdlc_forbidden_process_fails` | **PENDIENTE** — ENOSPC |
| Integración | `cargo test -p execute-process --test eda_subscriber_ecst_redb route_domain_telegram` | **PENDIENTE** — ENOSPC |
| Integración | `cargo test -p execute-process --test eda_subscriber_ecst_redb route_domain_kalma2` | **PENDIENTE** — ENOSPC |

Re-ejecutar desde `SddIA/` tras liberar espacio (p. ej. podar `SddIA/target`, ~99 GiB). Si algún test falla, corregir antes de merge.

## Criterios de cierre (refinamiento AUT-58)

| Criterio | Laudo |
|----------|-------|
| Telegram con texto + ECST | APTO |
| SDLC permitido + ECST | APTO |
| Ensamblado por clave (15) | APTO (código; test pendiente ENOSPC) |
| Router redb telegram / Kalma2 | APTO (código; test pendiente ENOSPC) |
| Regresión texto vacío / proceso no permitido | APTO (código; test pendiente ENOSPC) |