---
feature_name: kalma2-agent-runtime-cursor-f3-km-residual
created: "2026-07-24"
purpose: Estabilización PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL — residual aduana PPR R1/R2/R3 en path kalma2-agent-runtime-cursor (no handlers nativos #125)
branch_name: feat/kalma2-agent-runtime-cursor-f3-km-residual
persist_ref: docs/features/kalma2-agent-runtime-cursor-f3-km-residual
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md
document_id: PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL
phase: cierre-documental
agents: mayeuta
---

# Clarificación — PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL

## D0 — Semilla

- **PBI:** `docs/todos/pending/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md` (`document_id: PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL`; `status: abierto`).
- **Ciclo:** feature `kalma2-agent-runtime-cursor-f3-km-residual` · rama `feat/kalma2-agent-runtime-cursor-f3-km-residual`.
- **Init:** orquestador nativo `execute-process` / `workspace-init` materializó `objectives.md` mínimo + `.tmp/feature-init.json` (`document_id` coherente). Estabilización Mayeuta en relay IDE (path agent-runtime).
- **Incidente origen:** aduana PPR #136 — findings no bloqueantes `TECH_FORMAL_EXECUTE_PROCESS`, `GIT_EVIDENCE_VIA_GIT_MANAGER`, residual `RBAC_AUTHORING_KM_POLICY` cuando el runtime de sesión es `kalma2-agent-runtime-cursor`.
- **Precedente cerrado:** PPR #125 / `docs/fixes/kalma2-ppr-runtime-gaps-ppr-125/` — handlers nativos `ppr-tech-triage` + `ppr-prep-branch` + prompt Cumulo-only (G1–G4 APTO). Este PBI **no** reabre ese cierre; cierra el **hueco residual del path agente**.
- **Fuente auditora:** `docs/features/inyeccion-dependencias-envelope-homologacion/validacion.md` · correlation `e3079c94-2a40-4f60-b9c4-b4ade1ca031b` · dedup Cosecha → este OPERATIVO (R1/R2).
- **Paralelo PPR #136:** seed ARQUITECTURA `delivery-close-cycle-revoked-signer` (E1/E2 revoked/signer) — jurisdicción disjunta.
- **Normas / artefactos de lectura:** `features-documentation-pattern`, `external-ai-constraints`, `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py`, `SddIA/skills/git-manager.md`, `SddIA/process/pull-request-review.md`.
- **Fuera:** PBI-042 Hito 4 envelope; rehabilitación `revoked_entities` / signer DCC; merge histórico PR #136.

## D1 — Matriz de residuales (R1 × R2 × R3)

| ID | Check | Estado empírico (aduana agent path) | Qué ya cubrió #125 (nativo) | Residual a estabilizar |
|----|-------|-------------------------------------|-----------------------------|------------------------|
| **R1** | `TECH_FORMAL_EXECUTE_PROCESS` | **NO_APTO** en sesiones Kalma2-agent | `ppr-tech-triage` · `formal_execute_process: true` | F3 formal **visible/ejecutable** desde agent-runtime **o** evidencia nativa **inyectada** al handoff del agente |
| **R2** | `GIT_EVIDENCE_VIA_GIT_MANAGER` | **NO_APTO** — `./sddia-run.sh --tool git-manager` rechazado (Shell/Auto-review); sin stdout físico | `ppr-prep-branch` · `git_manager_invoked: true` | Evidencia `git-manager` **materializada** en sesión agent-runtime **o** bypass soberano **documentado** y aceptado por aduana |
| **R3** | `RBAC_AUTHORING_KM_POLICY` | **NO_APTO** recurrente en dictámenes Argos agent path (falso negativo operativo) | G3: prompt Cumulo-only para `docs/todos/` | Política KM Cumulo-only **enforceable** en agent-runtime **sin** falso `NO_APTO` cuando el agente no escribe KM |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-PROCESS** | Frontmatter PBI declara `process: bug-fix`; mandato operador y rama = ciclo **`feature`**. Se opera como feature; alinear `process` del PBI a `feature` en este ciclo (mismo laudo que seed ARQUITECTURA #136). |
| **L-PATH** | Alcance = path **`kalma2-agent-runtime-cursor`** (CLI/agente + handoff). Prohibido reabrir o revalidar como deuda los handlers nativos G1/G2 de #125 salvo regresión demostrada. |
| **L-R1-F3** | Criterio R1 satisfecho si, en una sesión PPR/agent-runtime: (a) F3 formal se invoca vía cápsula/`execute-process` desde el runtime agente, **o** (b) el handoff agente recibe evidencia nativa inyectada (`formal_execute_process` / equivalente auditable) y Argos puede marcar `TECH_FORMAL_EXECUTE_PROCESS: APTO` sin inventar éxito. |
| **L-R2-GIT** | Criterio R2 satisfecho si hay stdout/artefacto físico de `skill:git-manager` (preferente `./sddia-run.sh --tool git-manager` JSON stdin) **o** evidencia ya materializada por handler nativo PPR **inyectada** al handoff, **o** bypass soberano documentado en norma/contrato del runtime. Prohibido declarar APTO sin materialización. Prohibido bypass raw git destructivo. |
| **L-R3-KM** | Criterio R3: autoría bajo `docs/todos/` solo `agent:cumulo` (Cosecha) o evento `Kaizen_Alert_Required`. Tekton/Argos/Mayeuta/Dedalo **no** escriben semillas KM. El check `RBAC_AUTHORING_KM_POLICY` debe resultar **APTO** cuando el diff/sesión agent-runtime respeta esa frontera; un `NO_APTO` por forja genómica legítima fuera de `docs/todos/` no es el residual de este PBI (distinguir forja Core vs KM). Dedalo acota el touchpoint de enforcement (prompt, aduana Argos, o bridge handoff) sin expandir a semillas nuevas. |
| **L-SCOPE-HARD** | Fuera de alcance innegociable: PBI-042; `revoked_entities` / signer de `delivery-close-cycle`; merge histórico PR #136; reescritura del peaje F2–F4 ya APTO. |
| **L-TRUTH** | «No inventes éxito»: si Shell/Auto-review bloquea `git-manager` o F3 no materializa, el check permanece NO_APTO y se reporta explícitamente — no se fabrica evidencia. |
| **L-SIBLING** | Seed ARQUITECTURA PPR #136 (`delivery-close-cycle-revoked-signer`) es paralelo; no acoplar E1/E2 a R1–R3 en el mismo PR salvo laudo Racso. |

## D3 — Criterios de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-R1** | En path agent-runtime (o handoff con evidencia nativa inyectada), `TECH_FORMAL_EXECUTE_PROCESS` → **APTO** con traza auditable. |
| **AC-R2** | En path agent-runtime, `GIT_EVIDENCE_VIA_GIT_MANAGER` → **APTO** con stdout/artefacto `git-manager` o bypass soberano documentado. |
| **AC-R3** | Sesión agent-runtime que no autorice KM fuera de Cumulo/evento → `RBAC_AUTHORING_KM_POLICY` **APTO** (sin falso NO_APTO). |
| **AC-DOC** | Cascada `features-documentation-pattern`; PBI → `docs/todos/done/` + `validacion.md` `pbi_archived: true` en la rama del PR. |
| **AC-NONREG** | Smokes/handlers nativos #125 (`ppr-prep-branch`, `ppr-tech-triage`) no regresan. |

## D4 — Ambigüedades resueltas / no abiertas

| Tema | Resolución |
|------|------------|
| ¿Nativo vs agente? | Residual = **agente**; nativo ya cerrado en #125. |
| ¿Obliga invocación F3 en toda sesión IDE? | No: basta evidencia formal **o** inyección soberana al handoff que Argos pueda auditar. |
| ¿Shell IDE como evidencia git? | No; preferir cápsula / evidencia nativa / bypass documentado. |
| ¿R3 = prohibir forja `SddIA/actions/`? | No; R3 = frontera **KM/`docs/todos/`**. Forja genómica tiene su propia aduana. |

## D5 — Handoff Dedalo

1. Auditar gap concreto entre handlers nativos (#125) y lo que `kalma2-agent-runtime-cursor` / `_agent_handoff.md` expone a Argos en PPR.
2. Proponer en `spec.md`+`plan.md` el mínimo touchpoint para R1/R2/R3 (inyección handoff vs invocación runtime vs ajuste dictamen) sin reabrir G1–G4 nativos.
3. Definir prueba de aceptación reproducible (smoke o sesión PPR lab) que demuestre APTO en agent path sin inventar stdout.
4. Mantener jurisdicción disjunta respecto a `delivery-close-cycle-revoked-signer`.
