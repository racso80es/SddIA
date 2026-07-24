---
feature_name: delivery-close-cycle-revoked-signer
created: "2026-07-24"
purpose: Estabilización PBI-PPR-136-DCC-REVOKED-SIGNER — revoked_entities delivery-close-cycle + signer_identity_rbac en ECST PullRequest_Presented
branch_name: feat/delivery-close-cycle-revoked-signer
persist_ref: docs/features/delivery-close-cycle-revoked-signer
pbi_ref: docs/todos/done/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md
document_id: PBI-PPR-136-DCC-REVOKED-SIGNER
execution_id: 00b9e53d-d231-45f5-9685-4d2b86b7ab63
phase: mayeuta-stabilization
agents: mayeuta
---

# Clarificación — PBI-PPR-136-DCC-REVOKED-SIGNER

## D0 — Semilla

- **PBI:** `docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md` (`document_id: PBI-PPR-136-DCC-REVOKED-SIGNER`; `status: abierto`).
- **Ciclo:** feature `delivery-close-cycle-revoked-signer` · rama `feat/delivery-close-cycle-revoked-signer`.
- **Init:** `./sddia-run.sh --process feature` → `workspace-init` **executed** (`execution_id: 00b9e53d-d231-45f5-9685-4d2b86b7ab63`). Fases Mayeuta…Argos agent-runtime **failed** (SSL EPROTO cursor-cli); estabilización materializada en IDE (relay).
- **Incidente origen:** aduana PPR #136 — checks no bloqueantes `RBAC_EMITTER_NOT_REVOKED` + `RBAC_SIGNER_PRESENT` sobre emisor `delivery-close-cycle` / ECST `PullRequest_Presented`.
- **Precedente:** rehabilitación `pull-request-review` PPR #124/#125 (`docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125).md`).
- **Fuente auditora:** `docs/features/inyeccion-dependencias-envelope-homologacion/validacion.md` · correlation `e3079c94-2a40-4f60-b9c4-b4ade1ca031b`.
- **Normas / SSOT:** `.SddIA/cerbero/revoked_entities.json`, `SddIA/process/delivery-close-cycle.md`, `SddIA/actions/emit-pr-presented-event.md`, `SddIA/events/domain/pull-request-presented.md`, `features-documentation-pattern`.
- **Fuera:** rehabilitación de `feature` / `bug-fix` en el mismo registry (salvo laudo agrupado); residual Kalma2-agent-runtime-cursor (seed OPERATIVO PPR #136).

## D1 — Matriz de validación (E1 × E2)

| ID | Check / residual | Estado empírico (inicio ciclo) | Evidencia |
|----|------------------|--------------------------------|-----------|
| **E1** | `RBAC_EMITTER_NOT_REVOKED` | **NO_APTO** | `revoked_entities.json` → clave `delivery-close-cycle` (`abrupt_success_rate_drop`, since `2026-07-23T10:05:15Z`; PBI citaba 2026-07-13) |
| **E2** | `RBAC_SIGNER_PRESENT` | **NO_APTO** | ECST `PullRequest_Presented` sin `signer_identity_rbac` (payload tip. `branch`/`pr_url`/`status`) |
| Dedup | `RBAC_PROCESS_REGISTRY` («PPR revoked») | **Cerrado** | seeds PPR #124/#125; `pull-request-review` ausente de revoked |
| Registry lateral | `feature` / `bug-fix` en revoked | **Fuera de alcance** | mismas claves presentes; no tocar salvo laudo |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-PROCESS** | PBI frontmatter decía `process: bug-fix`; mandato operador: ciclo **`feature`**. Se opera como feature; alinear `process` del PBI a `feature` en este ciclo. |
| **L-E1-LAUDO** | E1 exige **laudo Cerbero/Radamanto** explícito: rehabilitar `delivery-close-cycle` **o** justificar retención con evidencia. Precedent PPR #125 favorece rehabilitación si la causa es wall-clock/outlier telemetría, no fallo de aduana. Dedalo fija el camino en `spec.md` tras leer telemetría/motivo. |
| **L-E2-SIGNER** | E2 exige que `emit-pr-presented-event` (vía `delivery-close-cycle`) rellene `signer_identity_rbac` no nulo en el ECST, alineado a contrato `pull-request-presented`. Referencia positiva: features con signer `Vertice_Biologico_Relay`. |
| **L-SCOPE-HARD** | Prohibido rehabilitar `feature`/`bug-fix` en este PR salvo laudo Racso que los agrupe con E1. |
| **L-KALMA2** | Residual SSL/agent-runtime Kalma2 = PBI OPERATIVO paralelo; no bloquea este ciclo documental/genómico. |
| **L-GENOME** | Mutaciones a process/action/event vía `entity-manager` / cadena autorizada; `revoked_entities.json` es instancia Cerbero (`.SddIA/`), no genoma indexado — Dedalo confirma vía Cúmulo. |

## D3 — Criterios de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-E1** | Laudo documentado + estado coherente de `delivery-close-cycle` en `revoked_entities` (rehabilitado **o** retención justificada con evidencia). |
| **AC-E2** | Emisión `PullRequest_Presented` con `signer_identity_rbac` ≠ null. |
| **AC-ADUANA** | En PPR posterior: `RBAC_EMITTER_NOT_REVOKED` y `RBAC_SIGNER_PRESENT` → **APTO**. |
| **AC-DOC** | Cascada `features-documentation-pattern` completa; PBI → `docs/todos/done/` + `validacion.md` `pbi_archived: true` en la rama del PR. |

## D4 — Handoff Dedalo

1. Auditar causa `abrupt_success_rate_drop` de `delivery-close-cycle` (telemetría / Radamanto) → proponer rehabilitación vs retención.
2. Localizar path de emisión `signer_identity_rbac` en `emit-pr-presented-event` / `phase_capsules` / bridge.
3. `spec.md` + `plan.md`: touchpoints mínimos, pruebas de aduana, sin expandir a `feature`/`bug-fix`.
