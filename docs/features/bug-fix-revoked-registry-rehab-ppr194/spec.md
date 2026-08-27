---
feature_name: bug-fix-revoked-registry-rehab-ppr194
created: "2026-08-27"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-bug-fix-cerbero-radamanto + ontology-tool-to-process
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr194
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/pending/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY
uuid: 8a4b0d3f-5c2e-4f9b-8d6a-7e8f9a0b1c2d
version_spec: "1.0.0"
status: dedalo_locked
olas:
  - A1
source_correlation_id: "59606407-eed3-4da8-ac13-3cf6205b2147"
source_pr_url: https://github.com/racso80es/SddIA/pull/194
feature_ref: docs/fixes/bundle-consumer-telegram-gateway
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — abrupt_success_rate_drop since 2026-08-16T16:09:32Z; entity_type tool (misclassified)"
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
---

# Spec — bug-fix-revoked-registry-rehab-ppr194

## 1. Misión técnica

Rehabilitar `bug-fix` en Cerbero/Radamanto tras revocación `abrupt_success_rate_drop` (since `2026-08-16T16:09:32Z`, PPR #194) etiquetada erróneamente como `entity_type: tool`: **A1** Yunque Rúnico + corrección ontológica `tool`→`process` (jurisprudencia #174) + cascada documental single-PR. Un `persist_ref`, un PR.

Consumir `objectives.md` + `clarify.md` como `refined_requirements` (Mayeuta).

## 2. Diagnóstico (evidencia código + instancia · 2026-08-27)

| Vector | Hecho |
|--------|--------|
| Cerbero instancia | `revoked.bug-fix` · `entity_type: tool` · `reason: abrupt_success_rate_drop` · `since: 2026-08-16T16:09:32Z`. Ausente de `permanent`. |
| Radamanto | Clave raíz `bug-fix` **ausente** en `radamanto.stats` (Cúmulo). Sin ventana `samples` que podar. |
| SSOT proceso | `process_domain_roots` → `bug-fix.md` (`name: bug-fix`, process-contract). Entidad **process**, no tool. |
| Tipología motor (L-TYPE-VERIFY) | `radamanto_batch_core.rs` · `resolve_entity_type`: bare id → si `resolve_process_path(repo, id).is_ok()` ⇒ `"process"`; else `"tool"`. `bug-fix` resuelve vía domain root → **`process`**. Motor **no** estampa `tool` para esta entidad. |
| Ontología Cerbero | `tool` en `revoked` = **fósil** pre/post #174; no regresión tipológica viva del motor. |
| Umbrales | `directories.agents` → `radamanto.thresholds.json` **1.1.0** (`process: 0.70` / `tool: 0.85`). Intactos. |
| Hollow lifecycle | #185 A3 ya cubre peaje hollow de `bug-fix` (simetría lifecycle). **No** reabrir. |
| Laterales | `revoked.accept-pr`, `revoked.refactorization`, `revoked.emit-pr-audited-event` — **fuera**. |
| Antecesor | #174+#177 (`L-TYPE-RESOLVE` / `L-ONTOLOGY` / `L-SCOPE-HARD` excluyó rehab `bug-fix`). |

**Dictamen Dedalo:** ola A1 sola. **Prohibido** inventar A2/A3 motor: tipología vigente OK; semilla sin vector payload/handoff/EDA nuevo.

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un `persist_ref`, un PR. Prohibido despachar `bug-fix` satélite (meta: este ciclo **rehab** el proceso). |
| **L-WAVES** | Solo **A1**. Hollow #185 y umbrales #174 se **reutilizan**, no se reabren. |
| **L-REHAB-INST** | A1 = instancia `.SddIA/` (Cúmulo `radamanto.revoked_entities` / `radamanto.stats`). Evidencia en `execution.md`. Prohibido versionar mutaciones Cerbero/Radamanto en el diff del PR. |
| **L-CERBERO** | Eliminar nodo `revoked.bug-fix` por completo. Assert `permanent.bug-fix` ausente. Cerbero **no** tiene estado `healthy`. |
| **L-STATS** | Materializar/reset **solo** bucket raíz `bug-fix`. No inventar fósiles `entities.bug-fix` / `process:bug-fix`. |
| **L-RESET-ABS** | `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY`; `rehabilitated_at` ISO UTC de A1; `samples: []` (o ≤3 OK runtime reales si aparecieran antes del reset). |
| **L-ONTOLOGY** | Stats rehab: `entity_type: process`. Prohibido conservar/reintroducir `tool` para esta entidad. |
| **L-TYPE-VERIFY** | **PASS.** Motor `resolve_entity_type` + `resolve_process_path` mapea `bug-fix`→`process`. **No** escalar A2. Tekton/Argos re-assert en ejecución (grep/`cargo test` tipología si existe fixture; lectura código + resolución path). |
| **L-NO-A2** | Prohibido tocar `radamanto_batch_core.rs`, `phase_terminal.rs`, payload/handoff, hollow, frozen I/O bajo pretexto de este PBI. |
| **L-THRESH** | Umbrales 1.1.0 bit-idénticos. Prohibido mutar `radamanto.thresholds.json` / instructions. |
| **L-OUT** | Fuera: rehab laterales; reabrir umbrales; versionar instancia en PR; mutar genoma `{name}.md` de process; semillas Kaizen/`docs/todos/` (jurisdicción Cúmulo). |
| **L-DOC** | Cascada `features-documentation-pattern` + PBI → `docs/todos/done/` + `validacion.md` APTO `pbi_archived: true` en la rama del PR. |

### Shape canónico stats raíz post-A1

```json
{
  "entity_type": "process",
  "status": "healthy",
  "recovery_attempts": 0,
  "consecutive_success_count": 0,
  "degraded_at": null,
  "rehab_laudo": "PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY",
  "rehabilitated_at": "<ISO-UTC-A1>",
  "samples": []
}
```

(Campos adicionales que el schema local ya use en otros buckets sanos: preservar forma homóloga; no inventar historial KO.)

## 4. Touchpoints

| Locus (Cúmulo / repo) | Mutación |
|-----------------------|----------|
| `radamanto.revoked_entities` (`.SddIA/cerbero/revoked_entities.json`) | A1: borrar `revoked.bug-fix`. **Fuera del diff PR.** |
| `radamanto.stats` (`.SddIA/radamanto/stats.json`) | A1: materializar bucket raíz `bug-fix` per **L-RESET-ABS** + **L-ONTOLOGY**. **Fuera del diff PR.** |
| `directories.evolution` | Entrada breve UUID ciclo `8a4b0d3f-5c2e-4f9b-8d6a-7e8f9a0b1c2d`. |
| `persist_ref` | Cascada `implementation.md` / `execution.md` / `validacion.md` + archive PBI. |
| `process_domain_roots` → `bug-fix.md` | **Prohibido.** |
| `directories.agents` → `radamanto.thresholds.json` | **Prohibido.** |
| `SddIA/engine/execute-process/` | **Prohibido** (L-TYPE-VERIFY PASS / L-NO-A2). |

## 5. Contratos de comportamiento

### 5.1 Rehab Cerbero

```text
DELETE revoked["bug-fix"]
ASSERT permanent["bug-fix"] is absent
ASSERT laterales revoked keys unchanged
# No escribir status healthy en Cerbero
```

### 5.2 Materializar Radamanto

```text
IF stats["bug-fix"] missing:
  CREATE bucket per L-RESET-ABS + entity_type=process
ELSE:
  RESET absoluto (mismo shape); samples=[] o ≤3 OK runtime
ASSERT no fossil keys invented
```

### 5.3 Tipología (assert, no mutación)

```text
resolve_entity_type(repo, "bug-fix") == "process"
  <=> resolve_process_path(repo, "bug-fix").is_ok()
```

## 6. Criterios de aceptación (producto)

| AC | Verificación |
|----|--------------|
| **AC-A1** | `bug-fix` ∉ revoked/permanent; stats raíz presentes y `healthy`; `recovery_attempts: 0`; laudo + `rehabilitated_at`; `samples` vacíos o solo OK; evidencia en `execution.md`. |
| **AC-GIT-CLEAN** | Diff PR sin `.SddIA/cerbero/` ni `.SddIA/radamanto/`. |
| **AC-ONTO** | `entity_type: process` en stats rehab; cero `tool` residual para esta entidad post-A1. |
| **AC-TYPE-VERIFY** | Evidencia Dedalo (este spec) + re-assert Tekton/Argos: motor mapea `bug-fix`→`process`. Sin A2. |
| **AC-THRESH** | Umbrales 1.1.0 intactos (sin tocar en el PR). |
| **AC-DOC** | Cascada bajo `persist_ref`; PBI en `done/`; `validacion.md` `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## 7. Tests / asserts de producto (qué, no cómo)

| ID | Caso |
|----|------|
| T-A1-REVOKED | Tras A1: clave `bug-fix` ausente de `revoked` y de `permanent`. |
| T-A1-STATS | Bucket raíz `healthy` + `recovery_attempts: 0` + laudo + `rehabilitated_at` + `samples` vacíos/OK. |
| T-A1-ONTO | `entity_type == "process"`; ninguna ocurrencia `tool` ligada a `bug-fix` en Cerbero/stats post-A1. |
| T-A1-LATERAL | `accept-pr` / `refactorization` / `emit-pr-audited-event` keys revoked **intactas** (mismo since o presencia). |
| T-TYPE | `resolve_process_path` / tipología vigente: `bug-fix`→`process` (lectura código o test existente; **no** exigir nuevo test motor salvo fallo). |
| T-THRESH | `radamanto.thresholds.json` `version == "1.1.0"`; `process == 0.70`; sin diff de umbrales. |
| T-GIT-CLEAN | `git diff` del PR no lista paths instancia Cerbero/Radamanto. |

## 8. Límites / fuera de alcance

- Rehab laterales Cerbero.
- Inventar A2/A3 motor (payload, handoff, fail-soft, hollow).
- Mutar umbrales / agregador / `phase_terminal` / genoma `bug-fix.md`.
- Versionar instancia en el PR.
- Escribir TODOs bajo `docs/todos/` (Cúmulo / Kaizen).
- Usar `bug-fix` como carrier de otro FIX.

## 9. Viabilidad RBAC (Dedalo)

`target_executor_rbac` del proceso `refactorization`: `ecosystem-evolution`, `filesystem-ops`, `source-control`.

| Delegación | Contexto | Cruce |
|------------|----------|-------|
| Mutación instancia + docs (Tekton FS) | filesystem-ops / ecosystem-evolution | OK |
| `skill:git-manager` | source-control | OK |
| `action:execute-process` → `delivery-close-cycle` | cierre | OK |
| Motor / `entity-manager` genoma | — | **No requerido** (sin touchpoints genoma/motor) |

Ninguna fase del blueprint exige política fuera del pack ni cápsula no indexada.
