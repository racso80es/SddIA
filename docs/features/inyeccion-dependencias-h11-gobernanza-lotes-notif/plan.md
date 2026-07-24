---
feature_name: inyeccion-dependencias-h11-gobernanza-lotes-notif
created: "2026-07-23"
process: feature
branch_name: feat/inyeccion-dependencias-h11-gobernanza-lotes-notif
persist_ref: docs/features/inyeccion-dependencias-h11-gobernanza-lotes-notif
pbi_ref: docs/todos/pending/[ARQUITECTURA] PBI-045 — DI para Gobernanza, Lotes y Notificaciones (Hito 11).md
document_id: PBI-045-DI-GOBERNANZA-LOTES-NOTIFICACIONES
execution_id: 881f8cf6-6a4c-48aa-9f76-d84df5641db8
agent_planificador: dedalo
target_executor: tekton
phases: "H11-A,H11-B,H11-C-gated,H11-D-gated,regresion,docs"
---

# Plan — H11 Gobernanza / Lotes / Notificaciones (PBI-045)

## Fase 0 — Precondiciones

- intent: Confirmar rama `feat/inyeccion-dependencias-h11-gobernanza-lotes-notif`, `objectives`/`clarify`/`spec` coherentes, recount 35/7.
- delegates_to:
  - agent:tekton
- gate: none

## Fase 1 — H11-A Reuso sonda/LLM (0 altas)

- intent: Homologar `capsule-invoke-smoke` (`qa:probe`) y fases Filtro C + Síntesis de `telegram-fallback-responder` (`llm:interact`) vía `entity-manager` update + sellos EDA + evolution.
- delegates_to:
  - agent:tekton
  - action:execute-process
- touchpoints:
  - SddIA/process/capsule-invoke-smoke.md
  - SddIA/process/telegram-fallback-responder.md
- exclude: fase Materialización fallback; taxonomía; bindings; runtime
- gate: GO (reuso catálogo vigente)

### Checklist H11-A

- [ ] `capsule-invoke-smoke` fase io-choke → `requires_capability` `qa:probe`
- [ ] `telegram-fallback-responder` Filtro C + Síntesis → `requires_capability` `llm:interact`
- [ ] Materialización **sin** `requires` (defer canal)
- [ ] `Domain_Entity_Updated` ×2 + evolution
- [ ] orphan_count == 0

## Fase 2 — H11-B Reuso fs:persist (0 altas)

- intent: Homologar `memory-evolution-ingest`, `radamanto-batch` y fase Resolución de `execute-suite` con `fs:persist` (densidad híbrida; conservar agents/action).
- delegates_to:
  - agent:tekton
  - action:execute-process
- touchpoints:
  - SddIA/process/memory-evolution-ingest.md
  - SddIA/process/radamanto-batch.md
  - SddIA/process/execute-suite.md
- exclude: altas Códice; otras fases execute-suite; runtime
- gate: GO (precedente H7 / telemetry-batch-stub / suite-creator)

### Checklist H11-B

- [ ] Tres process updates con `requires_capability` `fs:persist` (suite solo Resolución)
- [ ] `delegates_to` no-FS preservados
- [ ] Sellos EDA ×3 + evolution
- [ ] orphan_count == 0

## Fase 3 — H11-C Gobernanza (gated)

- intent: Tras countersign Racso: (A) alta `gov:rbac` + schema + binding + provider + `cerbero-governance-react` DI **o** (B) defer documentado sin mutación.
- delegates_to:
  - agent:tekton
  - action:execute-process
- gate: **L-TEKTON-GATE** — bloqueado hasta Racso
- candidates:
  - id: gov:rbac
  - contract: gov.rbac

### Checklist H11-C

- [ ] Countersign Racso (alta \| defer) registrado en frontmatter/spec
- [ ] Si alta: taxonomía bump + schema + bindings + provider `provides` + process DI + sellos
- [ ] Si defer: entrada explícita en `validacion.md` / clarify sin genoma

## Fase 4 — H11-D Canal Telegram (gated)

- intent: Tras countersign: (A) forjar `tool:telegram-gateway` `{name}.md` + `provides: channel:ingest` + alta Códice/binding + process DI **o** (B) defer.
- delegates_to:
  - agent:tekton
  - action:execute-process
- gate: **L-TEKTON-GATE** — bloqueado hasta Racso
- candidates:
  - id: channel:ingest
  - contract: channel.ingest
- entropy: crate sin `.md` bajo directories.tools

### Checklist H11-D

- [ ] Countersign Racso (alta+forge \| defer)
- [ ] Si alta: tool.md via cadena autorizada + taxonomía + binding + process DI + sellos
- [ ] Opcional laudoado: `provides` en `send-telegram-notification` / fase Materialización
- [ ] Si defer: documentar Entropía tool.md + residual en validacion

## Fase 5 — Regresión DI

- intent: Suites `capability_di` / `cerbero_di` verdes post mutaciones A+B (+C/D si aplican).
- delegates_to:
  - agent:tekton
- commands_ref:
  - cargo test -p execute-process capability_di
  - cargo test -p execute-process cerbero_di
- gate: AC-REG

## Fase 6 — Documentación + cierre

- intent: `implementation.md` / `execution.md` / Argos `validacion.md`; archivo PBI-045 en `done/` + `pbi_archived: true` en rama; `delivery-close-cycle`.
- delegates_to:
  - agent:tekton
  - agent:argos
  - action:execute-process
- gate: AC-H11 + task-closure-documental

## Orden de ejecución Tekton

```text
F0 → F1(H11-A) → F2(H11-B) → [Racso] → F3(H11-C) → F4(H11-D) → F5 → F6
```

Sin countersign: F3/F4 = defer documentado; F5/F6 con 5/7 + defer C/D si Racso lo acepta como umbral Done.
