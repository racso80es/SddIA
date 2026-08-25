---
feature_name: kaizen-paciente0-redeploy-20260825
created: "2026-08-25"
process: feature
branch: feat/kaizen-paciente0-redeploy-20260825
branch_name: feat/kaizen-paciente0-redeploy-20260825
persist_ref: docs/features/kaizen-paciente0-redeploy-20260825
pbi_ref: docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
uuid: "d4f13e9a-5d91-4ab8-a2f5-be2e6b8c4815"
execution_id: "7fd0a353-d2fe-4895-8abe-d7f5b34f652c"
global: APTO
pbi_archived: true
checks:
  O1_RESOLVE: APTO
  O2_OVERLAY_STUB: APTO
  O3_CREATOR_UNICO: APTO
  O4_IGNITION_PIN: APTO
  O5_SMOKE_ECST: APTO
  O6_SYSTEMD_USER: APTO
  O7_VAULT_DEV: APTO
  O8_AUDIT: APTO
  O9_CIERRE: APTO
  G5: NO_APTO
  F_SYS_01: NO_APTO
git_changes:
  - SddIA/scripts/common/sddia_shell_lib.sh
  - start-sddia.sh
  - SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs
  - SddIA/process/instance-creator.md
  - SddIA/process/index.md
  - SddIA/norms/sddia-distribution-protocol.md
  - SddIA/evolution/6b1f97f4-fa9d-48df-a58a-167f5d5e06dc.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/kaizen-paciente0-redeploy-20260825/
  - docs/audits/kaizen-paciente0-redeploy-20260825-residual.md
  - docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md
---

# Validación — kaizen-paciente0-redeploy-20260825

Argos (relevo IDE). `global: APTO`. G5 y F-SYS-01 fuera de alcance (NO_APTO = no gate).

## Checks

| ID | Veredicto | Evidencia |
|----|-----------|-----------|
| O1 | APTO | Debug mtime &lt; release → resolver elige release (14:40). Bundle posterior recompila release. |
| O2 | APTO | Test `replaces_empty_local_paths_stub`; T6 plant `{}` → starter-kit |
| O3 | APTO | Un creator `37890eec-…` sin pin/unlink; ExecStart instancia; overlay no vacío |
| O4 | APTO | `start-sddia` + pin forja → CONFIG F-DEP-09 → ELF `SddIA_AP/SddIA/target/release/execute-process` |
| O5 | APTO | Smoke `local_qa_emitted: false`. Padres DLQ `afc03462`/`7688b280` = emisión previa |
| O6 | APTO | Diferido L-SYS; no bloquea |
| O7 | APTO | Fase E operador previa |
| O8 | APTO | `docs/audits/kaizen-paciente0-redeploy-20260825-residual.md` |
| O9 | APTO | PBI en `done/` este PR; `pbi_archived: true` |

WUI HTTP 200 `:8766`. systemd `%f` `active`. 0 `cargo build` en chunk T6.
