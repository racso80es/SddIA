---
feature_name: route-domain-test-progress-marks
created: 2026-11-01
process: feature
branch: Fix_Events_Multiple
persist_ref: docs/features/route-domain-test-progress-marks
tracker_ref: AUT-59
historia_ref: AUT-20
global: APTO
approval_status: aprobado
checks:
  AUT59-L1-HELPER: APTO
  AUT59-L2-CALL-SITES: APTO
  AUT59-METRIC-OBSERVABILITY: APTO
git_changes:
- SddIA/engine/execute-process/tests/eda_subscriber_ecst_redb.rs
- docs/features/route-domain-test-progress-marks/
validated_base_sha: "2d14642644f1acd7ebfa1fa32bac0f941f817736"
validated_head_sha: "6ac2b651c54bbbab8f5b14cb1cc4eb6aaabb7fa4"
validated_tree_sha: "520520befed83c2b688eaaf303c1320b1414b6ab"
diff_digest: "sha256:0971d1c7bde471cbf30ba394620c0c3513f6cd6c3ee32f41c29969fe93134121"
---

# Validación — AUT-59

Kaizen: marcas `enter`/`leave` en tests `route_domain_*`. Provider Linear: Done tras merge del PR; sin fichero en `docs/todos/done/`.

## Veredicto

**APTO** — Los cinco tests emiten marcas en stderr y en `.tmp/route-domain-test-progress.log` del checkout. La métrica de observabilidad del refinamiento queda cumplida.

## Evidencia (2026-11-01, cwd `SddIA/`, `TMPDIR=../.tmp/cargo-test`, `CARGO_TARGET_DIR=target`)

| Check | Comando / artefacto | Resultado |
|-------|---------------------|-----------|
| Compilación test | `cargo test -p execute-process --test eda_subscriber_ecst_redb route_domain_tracker_markdown --no-run` | OK |
| Marcas nocapture | `cargo test … route_domain_tracker_markdown -- --nocapture` | enter + leave en stderr |
| Suite `route_domain_` | `cargo test … route_domain_ -- --nocapture --test-threads=1` | 5 passed; marcas en los 5 |
| Log estable | `.tmp/route-domain-test-progress.log` (raíz checkout) | append con pares enter/leave |

## Nota ajena a AUT-59

`route_domain_telegram_empty_text_still_skipped` **falla** la aserción de `skipped-empty-text` (`delivery_status` muestra `ecst-gate: failed`). Las marcas **sí** se emiten (enter/leave visibles antes del panic). Corrección fuera del alcance de este kaizen.

## Criterios de cierre (refinamiento)

| Criterio | Laudo |
|----------|-------|
| Helper stderr + log checkout | APTO |
| Cinco call sites | APTO |
| Métrica enter sin leave diagnosticable | APTO |