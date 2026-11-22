---
generated_by: kalma2-agent-runtime-cursor
persist_ref: docs/features/aut-97-project-git-invalid-lab-prefix
---

# Agent handoff log

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-11-22T13:13:19Z"
source: prosthesis_subprocess
git_manager_invoked: true
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
formal_evidence_detail: "verify-process-integrity: OK"
```

## 2026-11-22T13:14:54Z — Triaje documental
- process: `pull-request-review`
- agents: `argos`
- execution_id: `17427fd2-d7dd-423f-8b97-33548496889c`
- correlation_id: `2232c895-15ab-4340-83e7-015a30da61a6`
- pbi_ref: ``
- runtime: kalma2-agent-runtime-cursor
- backend: `cli`
- status: `executed`
- message: - Downstream F3/F4/Veredicto pendiente

### Transcript (tail)

```
**Veredicto: ok** (`PASS_F2_DOC` · `global: APTO`)

- Archivo tocado: `docs/features/aut-97-project-git-invalid-lab-prefix/validacion.md`
- R1/R2: copia Evidence Bridge `prosthesis_subprocess` → TECH/GIT **APTO** (sin stdout inventado; Shell git-manager Rejected)
- R3 KM: **APTO** — 0 writes ilegítimos en `docs/todos/**`; `pending/` vacío
- F2: cascada doc + CA-1…CA-4 + evolution `ab3365ad-…` **APTO**; workspace PPR ausente (no bloquea)
- Residuales no bloqueantes: `GIT_EVIDENCE_SESSION_SHELL`, `GIT_EVIDENCE_NATIVE_PPR`, `WSPACE_MATERIALIZED`
- Downstream F3/F4/Veredicto pendiente
```

### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "2026-11-22T13:14:54Z"
source: native_state
git_manager_invoked: true
formal_execute_process: true
TECH_FORMAL_EXECUTE_PROCESS: APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
notes: "idempotent-hit"
```
