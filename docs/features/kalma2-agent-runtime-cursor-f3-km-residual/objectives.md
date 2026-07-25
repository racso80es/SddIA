---
feature_name: kalma2-agent-runtime-cursor-f3-km-residual
created: "2026-07-24"
process: feature
branch_name: feat/kalma2-agent-runtime-cursor-f3-km-residual
persist_ref: docs/features/kalma2-agent-runtime-cursor-f3-km-residual
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136).md
document_id: PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL
phase: cierre-documental
agents: mayeuta
purpose: Requisito termodinámico estable — residual R1/R2/R3 aduana PPR en kalma2-agent-runtime-cursor
---

# Objetivos — kalma2-agent-runtime-cursor-f3-km-residual

## Misión

Cerrar la **deuda residual de aduana PPR** cuando el runtime de sesión es `kalma2-agent-runtime-cursor` (CLI/agente), sin reabrir el cierre nativo de PPR #125 (`ppr-tech-triage`, `ppr-prep-branch`, G3 prompt Cumulo-only).

## Requisito estable (qué)

El ecosistema debe permitir que una sesión PPR/agent-runtime demuestre, con evidencia auditable y sin inventar éxito:

1. **R1 — F3 formal:** `TECH_FORMAL_EXECUTE_PROCESS` APTO porque F3 formal es ejecutable desde el runtime agente **o** porque la evidencia nativa (`formal_execute_process` / equivalente) se inyecta al handoff del agente y Argos la consume.
2. **R2 — Evidencia git:** `GIT_EVIDENCE_VIA_GIT_MANAGER` APTO porque existe materialización física de `skill:git-manager` en la sesión agent-runtime (preferente `./sddia-run.sh --tool git-manager` JSON stdin), **o** evidencia nativa PPR inyectada al handoff, **o** bypass soberano documentado en norma/contrato. Queda prohibido el bypass raw git destructivo y el APTO sin stdout/artefacto.
3. **R3 — Política KM:** `RBAC_AUTHORING_KM_POLICY` APTO en agent-runtime cuando no hay autoría ilegítima bajo `docs/todos/`. Semillas Kaizen solo vía `agent:cumulo` (Cosecha) o evento `Kaizen_Alert_Required`. Sin falso `NO_APTO` por cumplimiento correcto de esa frontera.

## Por qué

PPR #125 cerró los handlers **nativos** Prep/F3 y la regla de prompt KM; la aduana en path **kalma2-agent-runtime-cursor** sigue marcando R1/R2 (y R3 operativo) como NO_APTO — p. ej. Shell/Auto-review rechaza `git-manager`, F3 no aparece en handoff agente. Esa asimetría nativo≠agente es la deuda de este PBI (`PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL`), deduplicada en Cosecha Kaizen desde `inyeccion-dependencias-envelope-homologacion`.

## Alcance

| Incluye | Excluye |
|---------|---------|
| Path `kalma2-agent-runtime-cursor` + handoff agente → Argos | Revalidar como deuda los handlers nativos G1/G2 #125 (salvo regresión) |
| Checks R1, R2, R3 en aduana PPR agent path | PBI-042 (Hito 4 envelope) |
| Ajuste mínimo de runtime/prompt/aduana para paridad de evidencia | Rehabilitación `revoked_entities` / signer `delivery-close-cycle` (seed ARQUITECTURA #136) |
| Cascada documental feature + archivo PBI en rama | Merge histórico PR #136 |

## Criterios de aceptación

- [ ] **AC-R1:** `TECH_FORMAL_EXECUTE_PROCESS: APTO` en path agent-runtime (ejecución F3 o evidencia nativa inyectada al handoff).
- [ ] **AC-R2:** `GIT_EVIDENCE_VIA_GIT_MANAGER: APTO` con stdout/artefacto `git-manager` o bypass soberano documentado.
- [ ] **AC-R3:** `RBAC_AUTHORING_KM_POLICY: APTO` cuando la sesión respeta Cumulo-only en `docs/todos/` (sin falso NO_APTO).
- [ ] **AC-DOC:** `features-documentation-pattern` completo; PBI en `docs/todos/done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`.
- [ ] **AC-NONREG:** smokes/handlers nativos #125 sin regresión.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager` (o evidencia nativa equivalente inyectada).
- KM / `docs/todos/`: solo Cumulo o `Kaizen_Alert_Required`.
- `features-documentation-pattern` v1.2.x — un `.md` por fase, frontmatter obligatorio.
- Jerarquía: Acción → Agente → Skill → Tools.
- Verdad objetiva: no inventar éxito ni fabricar evidencia git/F3.

## Handoff a Dedalo

Consumir este cuerpo como `refined_requirements`. Diseñar blueprint (`spec.md`, `plan.md`) que cierre R1–R3 en el path agente con touchpoints mínimos, jurisdicción disjunta del seed ARQUITECTURA `delivery-close-cycle-revoked-signer`, y prueba de aceptación reproducible sin inventar stdout.
