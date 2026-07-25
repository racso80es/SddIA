---
feature_name: kalma2-agent-runtime-cursor-f3-km-residual
created: "2026-07-24"
process: feature
base: main
scope: "Residual R1/R2/R3 aduana PPR en path kalma2-agent-runtime-cursor (Evidence Bridge)"
version_spec: "1.0.0"
uuid: f3a91c2e-8b47-4d6e-a1c5-9e0d7b4f2a68
status: dedalo_locked
document_id: PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL
pbi_uuid: 3d9bb1de-e45d-49fe-99f7-9b0b31d79c1d
branch_name: feat/kalma2-agent-runtime-cursor-f3-km-residual
persist_ref: docs/features/kalma2-agent-runtime-cursor-f3-km-residual
pbi_ref: docs/todos/pending/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md
phase: blueprint
agents: dedalo
depends_on:
  - docs/fixes/kalma2-ppr-runtime-gaps-ppr-125
sibling_disjoint:
  - docs/features/delivery-close-cycle-revoked-signer
---

# Spec — kalma2-agent-runtime-cursor-f3-km-residual

## 1. Problema (gap nativo ≠ agente)

PPR #125 cerró handlers **nativos** (`ppr-prep-branch`, `ppr-tech-triage`) y reglas de prompt KM. En path **`kalma2-agent-runtime-cursor`** (fases `agent:` vía prótesis) la aduana sigue NO_APTO:

| ID | Check | Causa raíz agent path | Ya cerrado nativo (#125) |
|----|-------|----------------------|---------------------------|
| **R1** | `TECH_FORMAL_EXECUTE_PROCESS` | F3 formal no corre en sesión agente; handoff no expone `formal_execute_process` | `ppr-tech-triage` |
| **R2** | `GIT_EVIDENCE_VIA_GIT_MANAGER` | Agente pide `./sddia-run.sh --tool git-manager` vía **Shell IDE** → Auto-review Rejected; sin stdout físico | `ppr-prep-branch` · `invoke_git_manager` |
| **R3** | `RBAC_AUTHORING_KM_POLICY` | Dictamen Argos confunde forja genómica (`SddIA/actions/`, etc.) con frontera KM (`docs/todos/`) → falso NO_APTO | G3 prompt Cumulo-only |

**Asimetría:** nativo escribe flags en `state`; `agent_runtime` pasa `inputs`/`state` parciales al CLI pero **no** materializa evidencia legible por Argos ni invoca cápsulas fuera del Shell IDE.

## 2. Laudos Dedalo

| ID | Laudo |
|----|-------|
| **L-BRIDGE** | Cierre R1/R2 = **Evidence Bridge** en la prótesis `kalma2-agent-runtime-cursor.py`: materializar evidencia por **subprocess** del propio runtime (no Shell IDE del agente). |
| **L-STATE-FWD** | `agent_runtime.rs` reenvía al payload flags ya presentes en `state` (`git_manager_invoked`, `tech_triage_formal` / `formal_execute_process`, `tech_checks`) para reutilizar #125 en ciclos PPR mixtos. |
| **L-SCHEMA** | Evidencia machine-readable bajo `persist_ref/_agent_handoff.md` (bloque YAML acotado) + eco en JSON stdout del AGENT_PHASE. Argos **consume** ese bloque; no inventa stdout. |
| **L-R1** | `TECH_FORMAL_EXECUTE_PROCESS: APTO` si (a) flag nativo inyectado, **o** (b) prótesis ejecuta sensor formal (`execute-process --verify-process-integrity` o equivalente documentado) y sella `formal_execute_process: true` con traza. |
| **L-R2** | `GIT_EVIDENCE_VIA_GIT_MANAGER: APTO` si (a) flag nativo inyectado, **o** (b) prótesis invoca `skill:git-manager` (`operation_type: status`) vía `./sddia-run.sh --tool git-manager` JSON stdin y persiste digest/`gitStdout` (truncado) en el bloque. Prohibido APTO sin materialización. |
| **L-R3** | `RBAC_AUTHORING_KM_POLICY` audita **solo** autoría bajo `docs/todos/**`. Cumulo / `Kaizen_Alert_Required` = vía legítima. Sin writes KM ilegítimos → **APTO**. Forja Core ≠ este check (aduana genómica aparte). Ajuste: prompt Argos + criterio en `build_prompt` / handoff. |
| **L-TRIGGER** | Materializar evidencia al menos en fase **Verificación** (`agent:argos`). Si el payload ya trae flags nativos APTO, no re-invocar cápsulas (idempotencia). Opcional: cache por `persist_ref` en la misma sesión. |
| **L-MOCK** | Con `SDDIA_AGENT_RUNTIME_MOCK=1`: **no** fabricar APTO. Sellar `evidence_materialized: false` + `reason: mock`. |
| **L-TRUTH** | Si subprocess falla / Auto-host sin binario: check permanece NO_APTO y se reporta; sin inventar éxito. |
| **L-NO-REOPEN** | No mutar handlers `ppr-prep-branch` / `ppr-tech-triage` salvo regresión demostrada. No reabrir PBI-042. |
| **L-SIBLING** | Jurisdicción disjunta de `delivery-close-cycle-revoked-signer` (E1/E2). |
| **L-GENOME** | Preferir touchpoints **no indexados** (prótesis Python + `agent_runtime.rs`). Si hace falta norma/prompt genómico → `entity-manager`; fuera del mínimo si el prompt de la prótesis basta. |

## 3. Topología de solución

```text
execute-process (fase agent:)
  → agent_runtime.rs
       ├─ payload += runtime_evidence_from_state (L-STATE-FWD)
       └─ spawn SDDIA_AGENT_RUNTIME_COMMAND
            → kalma2-agent-runtime-cursor.py AGENT_PHASE
                 ├─ (Argos / Verificación) materialize_runtime_evidence()
                 │     ├─ prefer state/native flags
                 │     ├─ else: sddia-run --tool git-manager (status)
                 │     └─ else: execute-process --verify-process-integrity
                 ├─ append ### Runtime evidence (machine) → _agent_handoff.md
                 ├─ prompt agente (+ criterio KM scoped para Argos)
                 └─ CLI/SDK Cursor
  → Argos lee handoff + diff → checks R1/R2/R3
```

## 4. Contrato — bloque de evidencia

Ubicación: `{persist_ref}/_agent_handoff.md` (append).

Bloque Markdown a appendear:

~~~markdown
### Runtime evidence (machine)

```yaml
schema: kalma2-agent-runtime-evidence/v1
materialized_at: "<ISO-8601 UTC>"
source: native_state | prosthesis_subprocess | none
git_manager_invoked: true|false
formal_execute_process: true|false
TECH_FORMAL_EXECUTE_PROCESS: APTO|NO_APTO
GIT_EVIDENCE_VIA_GIT_MANAGER: APTO|NO_APTO
git_evidence_digest: "<sha256|trunc stdout|omit if none>"
formal_evidence_detail: "<ok|errors summary|omit>"
notes: "<mock|error|idempotent-hit>"
```
~~~

Reglas:

- `source: native_state` solo si flags vienen del payload/state sin re-ejecutar.
- `source: prosthesis_subprocess` exige exitCode 0 de la cápsula invocada para marcar APTO del check correspondiente.
- `source: none` + ambos false → Argos **debe** emitir NO_APTO en R1/R2 (verdad objetiva).

## 5. Touchpoints Tekton (mínimo)

| # | Artefacto | Mutación |
|---|-----------|----------|
| T1 | `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` | `materialize_runtime_evidence`; append schema v1; enriquecer `build_prompt` Argos (leer bloque; KM = solo `docs/todos/`); invocar en `run_agent_phase` cuando fase/agente = Verificación/argos (y cuando flags nativos ausentes) |
| T2 | `SddIA/engine/execute-process/src/engine/agent_runtime.rs` | Inyectar `runtime_evidence` / flags desde `state` al JSON stdin; test unitario |
| T3 | Tests / smoke lab | Prótesis: mock negativo no inventa APTO; path feliz con git-manager status → bloque APTO; KM: diff sin `docs/todos/` → criterio APTO |
| T4 | Cascada documental | `implementation.md`, `execution.md`; evolution breve UUID ciclo |

**Prohibido en este ciclo:** mutar `pull-request-review.md` genoma; rehabilitar `revoked_entities`; merge histórico #136; semillas KM por Tekton.

## 6. Criterios técnicos de aceptación

| AC | Verificación |
|----|--------------|
| **AC-R1** | Sesión agent-runtime Verificación: bloque evidencia con `formal_execute_process: true` **o** flag nativo → Argos `TECH_FORMAL_EXECUTE_PROCESS: APTO` |
| **AC-R2** | Mismo path: `git_manager_invoked: true` + digest/artefacto → `GIT_EVIDENCE_VIA_GIT_MANAGER: APTO` |
| **AC-R3** | Sesión sin writes ilegítimos a `docs/todos/` → `RBAC_AUTHORING_KM_POLICY: APTO` aunque el diff toque forja Core |
| **AC-DOC** | Cascada `features-documentation-pattern`; PBI → `done/`; `validacion.md` `global: APTO`, `pbi_archived: true` |
| **AC-NONREG** | Smokes/handlers #125 Prep/Triaje sin regresión |
| **AC-TRUTH** | MOCK o fallo subprocess → no APTO fabricado |

## 7. Fuera de alcance

- PBI-042 Hito 4 envelope.
- E1/E2 `delivery-close-cycle-revoked-signer`.
- Bypass raw git destructivo / Shell IDE como fuente de evidencia.
- Reescritura peaje F2–F4 ya APTO en nativo.

## 8. Viabilidad RBAC (ejecutor Tekton / feature)

`target_executor_rbac` del proceso `feature`: `ecosystem-evolution`, `filesystem-ops`, `source-control`.

| Cápsula | context | ¿Permitido? |
|---------|---------|-------------|
| `skill:git-manager` | source-control | sí |
| `action:execute-process` | (orquestación feature) | sí vía runtime |
| prótesis scripts (no cápsula indexada) | filesystem-ops | sí |
| `agent:tekton` / `agent:argos` | fases proceso | sí |

Sin cápsulas fuera de catálogo. Sin KM write por Dedalo/Tekton/Argos.
