---
feature_name: kaizen-paciente0-redeploy-20260825
created: "2026-08-25"
process: feature
items_applied: "T1-T7"
branch_name: feat/kaizen-paciente0-redeploy-20260825
persist_ref: docs/features/kaizen-paciente0-redeploy-20260825
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
execution_id: "7fd0a353-d2fe-4895-8abe-d7f5b34f652c"
---

# Execution — kaizen-paciente0-redeploy-20260825

## Laboratorio T1–T5

| ID | Resultado |
|----|-----------|
| tests `instance_creator` | 3 passed |
| Resolver sin pin (14:40) | release |
| entity-manager UPDATE | v1.2.0 `4eccbf39…` |

## T6 Paciente 0

| Paso | Resultado |
|------|-----------|
| Bundle `--out SddIA_AP` | `20260825T124331Z` 7 bins, 0 `.rs`, `PY_LEAK=no` |
| Precondición overlay | `{}` plantado (no unlink) |
| `instance-creator` sin pin | `success:true` cid `37890eec-…` vault 6; `local_qa_emitted:false` |
| Overlay / ExecStart | starter-kit; `…/SddIA_AP/SddIA/daemons/email-watcher.sh` `active` |
| Ignición + pin forja | F-DEP-09 CONFIG; ELF instancia; 0 cargo |
| WUI | HTTP 200 `:8766` |
| Auditoría | `docs/audits/kaizen-paciente0-redeploy-20260825-residual.md` |

DLQ `Local_QA_Requested` `afc03462` / `7688b280`: emisión **previa**, no T6.
