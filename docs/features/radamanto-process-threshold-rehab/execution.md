---
feature_name: radamanto-process-threshold-rehab
created: "2026-08-16"
process: refactorization
items_applied:
  - t0-thresholds-json
  - t1-radamanto-batch-core
  - t2-fail-soft-olas
  - t3-instance-rehab
  - t4-docs-evolution
branch_name: refactor/radamanto-process-threshold-rehab
persist_ref: docs/features/radamanto-process-threshold-rehab
document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
uuid: ba900e95-1a47-4185-b86c-bc7a251b4fe6
---

# Execution — radamanto-process-threshold-rehab

## T0

`radamanto.thresholds.json` → `1.1.0` + tabla por tipo. `instructions.json` R4.1/R4.2. Companion JSON (no `{name}.md`).

## T1

`resolve_entity_type`: prefijo válido → `resolve_process_path` → `tool`. Lookup `success_rate_min_for`. Latency skip si `process` **o** allowlist `pull-request-review`.

## T2

- Ola 1: `git status` post-checkout fail-soft; fricción PPR (`timeout`/`bridge`/`network`) en residual; agentes `Triaje documental`/`Cosecha Kaizen` failed → `fail_soft`. F2/F4/F5 (`Veredicto`, `Certificación RBAC`, triaje integrity) intactos.
- Ola 2: `Higiene local` / `Impacto SddIA condicional` failed con `pr_url` → `fail_soft`. Snapshot/push/aduana EDA/sello Presented siguen causales.

## T3 (instancia · no entra al PR)

| Check | Resultado |
|-------|-----------|
| `revoked.delivery-close-cycle` | **ausente** |
| `revoked.pull-request-review` | **ausente** |
| `permanent.feature` | intacto |
| `revoked.bug-fix` / `emit-pr-audited-event` | intactos |
| stats DCC root + `entities.delivery-close-cycle` | `healthy` · `rehab_laudo: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS` · `rehabilitated_at: 2026-08-16T16:37:15Z` |

## T4

Tests: `cargo test -p execute-process --lib` filtros radamanto/fail-soft → **8 passed**. Evolution `ef2b0ef2-b792-4cb7-ac1b-bfea203f4bde`.
