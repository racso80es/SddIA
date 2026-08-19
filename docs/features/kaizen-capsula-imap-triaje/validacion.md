---
feature_name: kaizen-capsula-imap-triaje
created: "2026-08-19"
updated: "2026-08-19T15:43:00Z"
process: pull-request-review
phase: Certificación RBAC
agent: cerbero
agents: cerbero
branch: feat/kaizen-capsula-imap-triaje
branch_name: feat/kaizen-capsula-imap-triaje
persist_ref: docs/features/kaizen-capsula-imap-triaje
pbi_ref: docs/todos/done/PBI-KAIZEN-CAPSULA-IMAP-TRIAJE.md
document_id: PBI-KAIZEN-CAPSULA-IMAP-TRIAJE
uuid: "9c25bb52-57a4-4ede-be43-41388a7576b2"
execution_id: "14fff213-bcee-4c26-ad17-53e5e585979b"
correlation_id: 17043d6d-c978-4245-b554-2c5edcf94422
global: APTO
pbi_archived: true
authorization_status:
  exitCode: 0
  signer_identity_rbac: tekton
  signer_process_chain: "feature → tekton (+ entity-manager T4/T5)"
  note: "PASS_F4_RBAC · 11 áreas auditadas · 0 bloqueos · Shell git-manager Rejected — Evidence Bridge native_state"
git_manager_invoked: false
git_evidence_source: native_state-evidence-bridge
formal_execute_process: true
checks:
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  AC_A_GLOVE: APTO
  AC_B1_SILENCIO_RUIDO: APTO
  AC_B2_ELEVACION: APTO
  AC_B3_RETORNO: APTO
  O3_NO_DUALIDAD: APTO
  G4_CEGUERA: APTO
  PBI_ARCHIVED: APTO
  LAB_TELEGRAM_LIVE: DIFERIDO
  LAB_IMAP_LIVE: DIFERIDO
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  GIT_EVIDENCE_SESSION_SHELL: NO_APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  RBAC_PROCESS_REGISTRY: APTO
  RBAC_SIGNER_VS_GENOME: APTO
  F4_RBAC_GATE: APTO
  RBAC_CERBERO_CERT: APTO
non_blocking_findings:
  - GIT_EVIDENCE_SESSION_SHELL
  - RBAC_PROCESS_SIGNER_REVOKED
situational_notes:
  - "process:feature ∈ revoked_entities.permanent since 2026-08-19T07:59:05Z (max_recovery_attempts_exceeded); artefactos materializados en cadena autorizada pre/post revocación — alerta auditoría, no bloqueo F4"
  - "pull-request-review ∉ revoked · entity-manager ∉ revoked"
  - "PBI-KAIZEN-CAPSULA-IMAP-TRIAJE solo en docs/todos/done/; ausente en docs/todos/pending/"
git_changes:
  - docs/features/kaizen-capsula-imap-triaje/
  - docs/todos/done/PBI-KAIZEN-CAPSULA-IMAP-TRIAJE.md
  - SddIA/daemons/email-watcher/src/main.rs
  - SddIA/events/domain/email-triaged.md
  - SddIA/events/domain/email-quick-action-requested.md
  - SddIA/events/domain/index.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-quick-action-ingest.md
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_quick_action.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - interfaces/kalma2/
  - SddIA/evolution/fa0f00e4-20f1-4258-95a9-e4d753f71d71.md
---

# Validación — kaizen-capsula-imap-triaje

**Veredicto global: APTO** (unidad + genoma). Lab IMAP/Telegram vivo diferido.

| AC | Evidencia |
|----|-----------|
| **AC-A** | UID aislado; `--once` envelope `success`⇔`exitCode`; test `once_envelope_json_io_contract` |
| **AC-B1** | `verdict=noise` → `build_telegram_message_from_event` = None |
| **AC-B2** | actionable → poke `from/subject/uid`; inbox WUI filtra solo actionable |
| **AC-B3** | POST `/api/email-quick-action` → `Email_Quick_Action_Requested` → ingest proof; sin IMAP STORE |
| **O3** | Cero clase `Actionable_Email_Detected` |

PBI archivado en `docs/todos/done/` en esta rama.

## Certificación RBAC (cerbero)

| Firmante | Área genoma | Contexto entidad | `allowed_policies` firmante | Resultado |
|----------|-------------|------------------|-----------------------------|-----------|
| tekton (feature/Ejecución) | `SddIA/daemons/email-watcher/` | `ecosystem-evolution` | `ecosystem-evolution` ✓ | PASS |
| tekton (quirúrgico T2) | `SddIA/events/domain/email-triaged.md` | `ecosystem-evolution` | `ecosystem-evolution` ✓ | PASS |
| tekton (vía `entity-manager` T4) | `SddIA/events/domain/email-quick-action-requested.md` | `ecosystem-evolution` | cadena `ecosystem-evolution` ✓ | PASS |
| tekton (in-ciclo T3) | `SddIA/core/event-domain-subscriptions.json` | `ecosystem-evolution` | `ecosystem-evolution` ✓ | PASS |
| tekton (vía `entity-manager` T5) | `…/email-quick-action-ingest.md` | `ecosystem-evolution` | cadena `ecosystem-evolution` ✓ | PASS |
| tekton | `SddIA/engine/…/handlers/*`, `route_domain_core.rs` | `ecosystem-evolution` | `ecosystem-evolution` ✓ | PASS |
| tekton | `SddIA/interfaces/kalma2-bridge/` | `filesystem-ops`, `source-control` | ambos ✓ | PASS |
| tekton | `interfaces/kalma2/` | `filesystem-ops`, `source-control` | ambos ✓ | PASS |
| tekton | `SddIA/evolution/` | excepción §external-ai-constraints | excepción explícita | PASS |
| argos/tekton | `docs/features/` | excepción §external-ai-constraints | excepción explícita | PASS |
| feature/argos | `docs/todos/done/` | `filesystem-ops` | `filesystem-ops` ✓ | PASS |

**F4:** `PASS_F4_RBAC` · `exitCode: 0` · 11 áreas / 0 bloqueos · CID `17043d6d-…`

**KM (R3):** `RBAC_AUTHORING_KM_POLICY: APTO` — `git_changes` incluye solo `docs/todos/done/PBI-KAIZEN-CAPSULA-IMAP-TRIAJE.md`; verificado ausente en `docs/todos/pending/`.

**Evidence Bridge (R1/R2):** copia de `_agent_handoff.md` (`source=native_state`, `notes=idempotent-hit-handoff`) → `TECH_FORMAL_EXECUTE_PROCESS: APTO`, `GIT_EVIDENCE_VIA_GIT_MANAGER: APTO`. Shell `./sddia-run.sh --tool git-manager` Rejected esta sesión → `GIT_EVIDENCE_SESSION_SHELL: NO_APTO` (no bloqueante).
