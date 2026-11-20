---
feature_name: man-44-rechazar-apto-incoherente
created: 2026-11-20
updated: 2026-11-20
process: feature
branch_name: feat/man-44-rechazar-apto-incoherente
persist_ref: docs/features/man-44-rechazar-apto-incoherente
pbi_ref: MAN-44
parent_ref: MAN-43
document_id: PBI-8205539f
phase: verificacion
agent: argos
global: APTO
norm_ref: features-documentation-pattern@1.3.1
execution_id: e049f4db-4e0e-41d0-b651-e9b9de645f71
validated_base_sha: b1376f215188352fc74b3db31fb6cf52e85eb29f
validated_head_sha: c0f3f1129b4c5c474e908076c2ef5623807cee2d
validated_tree_sha: 7c542764a5ac47d2a152b1248fcea17525f82fea
diff_digest: sha256:fe98f372df0bc50a013e27e220b9193eaf9ca64cb30ee8437b7080678191ddbe
---

# Validación — man-44-rechazar-apto-incoherente

## global

**`APTO`** — Aduana de coherencia APTO (`apto_coherence::validate_apto_coherence`) implementada e integrada en `delivery-close-cycle` (pre-sello) y `accept-pr` / `diff_seal` (pre-fusión). Todos los tests unitarios de los módulos afectados, `clippy -D warnings` y `verify-process-integrity` pasan tras la última edición.

## checks

| Check / AC | Resultado | Evidencia |
|------------|-----------|-----------|
| **AC-1 (KA-CLOSE-1)** | **APTO** | `ka_close_1_execution_blocked_rejects_apto` + `diff_seal::validate_for_accept_pr_merge` + `accept_pr` `Fusión Soberana` abortan si `execution.md` está `blocked` o ausente. |
| **AC-2 (KA-CLOSE-2)** | **APTO** | `ka_close_2_open_phases_rejects_apto` + `dcc_fracture_suppressed_on_apto_coherence_block` rechazan fases abiertas en `plan.md` sin emitir `System_Fracture_Detected`. |
| **AC-3 (KA-CLOSE-3)** | **APTO** | `ka_close_3_resolution_denylist_rejects_apto` + parche en `features-documentation-pattern.md` bloquean resoluciones fuera de patrón (`PASS_H1_MILESTONE`, `*-only`). |
| **AC-4 (KA-CLOSE-4)** | **APTO** | Contrafactual documentado en `execution.md` citando `docs/audits/man-43-h1-milestone-razonamiento-20261111.md` (KZ-1). |
| **AC-5 (Doble puerta)** | **APTO** | `apto_coherence::validate_apto_coherence` invocado en `delivery_close::capsule_diff_seal_validacion` y `diff_seal::validate_for_accept_pr_merge`. |
| **Tests unitarios** | **APTO** | `apto_coherence` (7/7), `diff_seal` (14/14), `delivery_close::tests` (38/38), `accept_pr` (21/21). |
| **Clippy & Integrity** | **APTO** | `cargo clippy -p execute-process -- -D warnings` (0 errores/warnings); `./sddia-run.sh --verify-process-integrity` (`OK`). |

## git_changes

- `SddIA/engine/execute-process/src/engine/apto_coherence.rs`
- `SddIA/engine/execute-process/src/engine/mod.rs`
- `SddIA/engine/execute-process/src/engine/delivery_close.rs`
- `SddIA/engine/execute-process/src/engine/diff_seal.rs`
- `SddIA/engine/execute-process/src/engine/accept_pr.rs`
- `SddIA/library/norms/features-documentation-pattern.md`
- `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md`
- `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md`
- `docs/audits/MAN-44.md`
- `docs/features/man-44-rechazar-apto-incoherente/{clarify,objectives,spec,plan,implementation,execution,validacion}.md`
- `docs/features/man-44-rechazar-apto-incoherente/_fixtures/ka-close-{1,2,3}/`

## branch

- **Rama:** `feat/man-44-rechazar-apto-incoherente`
- **Persist:** `docs/features/man-44-rechazar-apto-incoherente`
- **execution_id:** `e049f4db-4e0e-41d0-b651-e9b9de645f71`
- **pbi_ref:** `MAN-44`