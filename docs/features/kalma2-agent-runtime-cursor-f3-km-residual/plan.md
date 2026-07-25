---
feature_name: kalma2-agent-runtime-cursor-f3-km-residual
created: "2026-07-24"
process: feature
phases: [T0, T1, T2, T3, T4]
document_id: PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL
branch_name: feat/kalma2-agent-runtime-cursor-f3-km-residual
persist_ref: docs/features/kalma2-agent-runtime-cursor-f3-km-residual
pbi_ref: docs/todos/pending/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md
phase: blueprint
agents: dedalo
---

# Plan — kalma2-agent-runtime-cursor-f3-km-residual

Blueprint Tekton: Evidence Bridge R1/R2 + alcance KM R3. Sin reabrir #125 nativo. Sin E1/E2 DCC.

## Fases

### T0 — Documentación Dedalo
- [x] Consumir `objectives.md` / `clarify.md` (L-PATH…L-SIBLING)
- [x] `spec.md` laudos + contrato evidencia v1
- [x] este `plan.md`

### T1 — Prótesis Evidence Bridge (R1/R2/R3 prompt)
- [x] `materialize_runtime_evidence(repo, persist, doc) -> dict`
  - Preferir `doc.runtime_evidence` / flags nativos del payload
  - Else: subprocess `./sddia-run.sh --tool git-manager` stdin JSON `status` + `repository_path`
  - Else/además (formal): `execute-process --verify-process-integrity` (o binario documentado en `execution.md`)
  - Append bloque `### Runtime evidence (machine)` schema v1
- [x] Gate: fase Verificación / `agent:argos`; skip re-invoke si flags ya APTO
- [x] MOCK: `evidence_materialized: false`, `notes: mock` — sin APTO inventado
- [x] `build_prompt`: Argos — leer bloque evidencia; `RBAC_AUTHORING_KM_POLICY` solo `docs/todos/**`

### T2 — Forward state (`agent_runtime.rs`)
- [x] Copiar al payload: `git_manager_invoked`, `tech_triage_formal` / `formal_execute_process`, `tech_checks` si existen en `state`
- [x] Campo `runtime_evidence` agregado (objeto)
- [x] Test unitario: state con flags → payload los incluye

### T3 — Prueba de aceptación reproducible
- [x] Smoke prótesis (lab): script `kalma2-evidence-bridge-smoke.sh` (MOCK/native/KM); ejecución host en sesión Tekton = blocked Auto-review Shell
- [x] Caso KM: criterio documentado — sin paths `docs/todos/` ilegítimos → APTO
- [x] Non-reg: no tocar/romper tests `pull_request_review` Prep/Triaje

### T4 — Cierre documental
- [x] `implementation.md` / `execution.md`
- [x] Evolution breve (UUID feature / PBI)
- [x] Argos: `validacion.md` APTO + checks AC-R1..R3 + smoke host
- [x] PBI → `docs/todos/done/` + `pbi_archived: true`
- [ ] Handoff `delivery-close-cycle`

## Orden de forja

```text
T1 (prótesis) → T2 (agent_runtime forward) → T3 (smokes) → T4 (docs/cierre)
```

T1 y T2 pueden compartir commit si el smoke exige flags nativos + prótesis.

## Delegación / RBAC

| Fase | delegates_to / capacidad | Notas |
|------|--------------------------|-------|
| T1–T3 | `agent:tekton` + filesystem-ops | Prótesis + motor |
| Evidencia git | `skill:git-manager` vía `./sddia-run.sh --tool git-manager` | Subprocess prótesis; no Shell IDE |
| Formal F3 | `action:execute-process` (`--verify-process-integrity`) | Paridad peaje #125 |
| Verificación | `agent:argos` | Consume handoff schema v1 |
| KM semillas | Solo `agent:cumulo` / `Kaizen_Alert_Required` | Tekton/Argos/Dedalo no escriben `docs/todos/` semillas |
| Git commits | `skill:git-manager` | Prohibido bypass raw destructivo |

## Criterios de salida por fase

| Fase | Done local |
|------|------------|
| T1 | Handoff schema v1 en Verificación; prompt KM scoped |
| T2 | Test forward state verde |
| T3 | Smoke R1/R2 materializado o NO_APTO explícito; AC-R3 criterio claro |
| T4 | Cascada + PBI archivado en rama |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Shell/Auto-review bloquea al **agente** | Evidencia la genera la prótesis, no el Shell IDE |
| `sddia-run` / binario ausente en host | L-TRUTH: NO_APTO + notes; no fabricar |
| Argos ignora bloque handoff | Prompt explícito + AC en `validacion.md` checks |
| Acoplar E1/E2 DCC | L-SIBLING: PR disjunto |

## Explícitamente no planificado

Handlers nativos G1/G2 #125 · PBI-042 · revoked/signer DCC · merge histórico PR #136 · mutación genoma `pull-request-review.md`.
