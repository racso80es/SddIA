---
feature_name: dcc-revoked-registry-rehab-ppr187
created: "2026-08-21"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-dcc-cerbero + adjudicacion-retroactiva-eda-post-umbral
branch_name: refactor/dcc-revoked-registry-rehab-ppr187
persist_ref: docs/features/dcc-revoked-registry-rehab-ppr187
pbi_ref: docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187).md
document_id: PBI-PPR-187-DCC-REVOKED-REGISTRY
uuid: c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14
version_spec: "1.0.0"
status: dedalo_locked
olas:
  - A1
  - A2
correlation_id: ""
source_correlation_id: 4gKBTRCyZzvEFQcbDWFnBmdC3ZjvqTJmauHiYgTWwj32
source_pr_url: https://github.com/racso80es/SddIA/pull/187
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
---

# Spec — dcc-revoked-registry-rehab-ppr187

## 1. Misión técnica

Rehabilitar `delivery-close-cycle` en Cerbero/Radamanto (re-revocación `abrupt_success_rate_drop` @ `2026-08-20T12:04:10Z`) **y** cortar re-muerte: adjudicación **retroactiva** de `fail_soft` sobre el report `"Aduana EDA genómica"` cuando el umbral físico (`pr_url` / `delivery_push`) ya cruzó pese a huérfanos EDA **preexistentes**. Un `persist_ref`, un PR.

Consumir `objectives.md` + `clarify.md` como `refined_requirements` (Mayeuta).

## 2. Diagnóstico (evidencia código + instancia)

| Vector | Hecho |
|--------|--------|
| Cerbero instancia | `revoked.delivery-close-cycle` · `entity_type: process` · `reason: abrupt_success_rate_drop` · `since: 2026-08-20T12:04:10Z`. Ausente de `permanent`. |
| Radamanto bucket **raíz** | `degraded` · `recovery_attempts: 2` · 5 samples (3 OK / 2 KO · rate 0,60 < 0,70) · n≥`abrupt_drop_min_samples: 3` → vector abrupto, no #177. |
| Fósil | `entities.delivery-close-cycle` healthy lab — **no** es el revocado; no mutar. |
| Orden fases DCC | 1 Snapshot → 2 Impacto → **3 Aduana EDA** → 4 Push → 5 gh PR → 6 ECST → 7 Higiene. |
| Cuello A2 | `mark_fail_soft_if_secondary` exige `(has_pr \|\| delivery_push)` **en el momento** de la llamada. EDA es fase 3; umbral aparece en 4–5 → EDA blocked **nunca** recibe `fail_soft` inline. |
| `is_dcc_secondary_phase` hoy | Solo `"Impacto SddIA condicional"` \| `"Higiene local"`. Ampliar la lista **solo** no basta. |
| Gate EDA | `capsule_eda_genomic_audit_gate`: `orphan_count > 0` → `status: blocked`, `argos_verdict: block`. **No** debilitar a pass. |
| Agregador | `aggregate_execution_terminal`: `blocked` sin `fail_soft` → `exit_code: 1`. **Prohibido mutar.** |
| Path residual | `residual_runner` Ok(gate) EDA **no** llama `mark_fail_soft_if_secondary`; post-bucle tampoco adjudica. Simetría A2 obligatoria vía helper compartido. |
| Umbrales | `radamanto.thresholds.json` **1.1.0** intactos. |
| Laterales | `revoked.bug-fix`, `revoked.refactorization`, `revoked.emit-pr-audited-event` — fuera. |

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un PR. Prohibido `bug-fix` satélite. |
| **L-WAVES** | A1 instancia + A2 motor en el mismo ciclo. Rehab Cerbero sola = reabrir vector. |
| **L-REHAB-INST** | A1 = instancia Cúmulo (`radamanto.revoked_entities`, `radamanto.stats`). Evidencia en `execution.md`. Prohibido versionar esas rutas en el diff del PR. |
| **L-CERBERO** | Eliminar nodo `revoked.delivery-close-cycle`. Assert `permanent.delivery-close-cycle` ausente. Cerbero no tiene `healthy`. |
| **L-STATS** | Reset **solo** bucket raíz `delivery-close-cycle`. No tocar fósil `entities.delivery-close-cycle`. |
| **L-RESET-ABS** | `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-187-DCC-REVOKED-REGISTRY`; `rehabilitated_at` ISO UTC de A1. |
| **L-SAMPLES** | Vaciar `samples` **o** conservar ≤3 últimos OK runtime (`exit_code: 0`). Eliminar KO `d7310496…` / `19391b9f…`. |
| **L-ONTOLOGY** | Conservar `entity_type: process`. |
| **L-FAILSOFT-RETRO** | Extensión **L-FAILSOFT-OLA2**: helper puro `adjudicate_eda_fail_soft_post_physical(phase_reports, state)` invocado **tras** el bucle de fases y **antes** de `aggregate_execution_terminal`. |
| **L-PRED-EDA** | Predicado: umbral físico `(non_empty(pr_url) \|\| delivery_push present)` **y** report con `phase_name == "Aduana EDA genómica"` **y** `status ∈ {blocked, failed}` **y** `orphan_count > 0` **y** `argos_verdict == "block"` → inyectar `fail_soft: true` en ese report. |
| **L-EDA-SIGNAL** | Prohibido mutar `capsule_eda_genomic_audit_gate` a pass silencioso. Argos sigue registrando block. |
| **L-AGGREGATOR** | `phase_terminal.rs` **intacto**. |
| **L-SECONDARY-LIST** | **No** sustituir el post-pass ampliando solo `is_dcc_secondary_phase`. Opcional: añadir `"Aduana EDA genómica"` a la lista **solo** como coherencia documental; el post-pass permanece obligatorio. Preferencia Dedalo: **no** ampliar la lista en este ciclo (evita falsa sensación de cobertura inline); el post-pass es la única mutación semántica. |
| **L-RESIDUAL-SYM** | En `residual_runner::run` (path DCC u orquestación residual que ejecute las mismas fases): invocar el **mismo** helper exportado `pub(crate)` desde `delivery_close.rs` tras el bucle, antes del agregador. No reimplementar predicado. |
| **L-CAUSAL** | Sin umbral físico → EDA blocked permanece causal (`exit_code: 1`). Fallo snapshot / push / apertura PR / block Argos post-umbral por deuda **del diff actual** → causal (sin `fail_soft` en esas fases). |
| **L-NO-HOLLOW** | Prohibido tocar `survival_hollow` / `LIFECYCLE_PROCESSES` / `radamanto_batch_core` bajo pretexto A3. DCC ∉ esa lista. |
| **L-THRESH** | Umbrales 1.1.0 bit-idénticos en el PR. |
| **L-YAML** | No inyectar `fail_soft` estático en YAML `{name}.md` del proceso. Runtime en `phase_report`. |
| **L-GENOME** | Motor = parche `SddIA/engine/execute-process/`. Genoma process solo nota vía `entity-manager` si Argos lo exige. |
| **L-DOC** | Cascada patrón + PBI → `docs/todos/done/` + `validacion.md` APTO `pbi_archived: true` en la misma rama. |

## 4. Touchpoints

| Locus (Cúmulo / repo) | Mutación |
|-----------------------|----------|
| `SddIA/engine/execute-process/.../delivery_close.rs` | Añadir `adjudicate_eda_fail_soft_post_physical`; llamar en `run` tras bucle / antes de `aggregate_execution_terminal`. Tests unitarios §7. **No** cambiar semántica de higiene/snapshot existentes. |
| `SddIA/engine/execute-process/.../residual_runner.rs` | Tras bucle de fases, si el proceso es `delivery-close-cycle` (o siempre es idempotente sobre reports EDA): invocar el mismo helper. Path Ok(gate) EDA puede quedar sin `mark_fail_soft_if_secondary` inline — el post-pass cubre. |
| `SddIA/engine/execute-process/.../phase_capsules.rs` | **Prohibido** debilitar `capsule_eda_genomic_audit_gate`. |
| `SddIA/engine/execute-process/.../phase_terminal.rs` | **Prohibido mutar.** |
| `SddIA/engine/execute-process/.../radamanto_batch_core.rs` | **Prohibido mutar** (hollow fuera). |
| `directories.agents` → `radamanto.thresholds.json` | **Prohibido mutar.** |
| `radamanto.revoked_entities` / `radamanto.stats` (instancia) | A1 solo; evidencia `execution.md`. |
| `directories.evolution` | Entrada breve UUID ciclo `c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14`. |
| `persist_ref` | `implementation.md` / `execution.md` / `validacion.md` + archive PBI. |

Prohibido mutar YAML de `delivery-close-cycle.md` salvo nota documental mínima vía cadena autorizada.

## 5. Contratos de comportamiento

### 5.1 Umbral físico

```text
physical = non_empty(state.pr_url) OR state.delivery_push is present
```

### 5.2 Adjudicación retroactiva EDA (A2)

```text
# tras for phase in phases { reports.push(...) }
adjudicate_eda_fail_soft_post_physical(&mut phase_reports, &state)

for each report where phase_name == "Aduana EDA genómica":
  if physical
     AND status ∈ {blocked, failed}
     AND orphan_count > 0
     AND argos_verdict == "block":
       report.fail_soft = true

verdict = aggregate_execution_terminal(&phase_reports, &state)
# blocked + fail_soft → success / exit_code 0 (agregador intacto)
```

Idempotente: re-llamar no cambia semántica si `fail_soft` ya está.

### 5.3 Causal duro (sin fail_soft)

```text
EDA blocked + !physical                    → exit_code 1
Snapshot / push / apertura PR failed       → exit_code 1 (fases no secundarias)
Argos block post-umbral por deuda del diff → sin fail_soft en esa fase → exit_code 1
```

Huérfanos **preexistentes** (`github-raw-fetcher`, `download-remote-asset`) + umbral cruzado → peaje A2 (no backfill en este ciclo).

### 5.4 A1 instancia (Yunque)

```text
DELETE revoked["delivery-close-cycle"]
ASSERT permanent["delivery-close-cycle"] absent
stats["delivery-close-cycle"] := {
  status: healthy,
  recovery_attempts: 0,
  consecutive_success_count: 0,
  degraded_at: null,
  rehab_laudo: "PBI-PPR-187-DCC-REVOKED-REGISTRY",
  rehabilitated_at: <ISO A1>,
  samples: [] | ≤3 últimos OK runtime
}
# no mutar entities.delivery-close-cycle
```

## 6. Criterios técnicos (mapa AC)

| AC | Verificación |
|----|--------------|
| **AC-A1** | DCC ∉ `revoked` ni `permanent`; stats raíz `healthy`; `recovery_attempts: 0`; laudo + timestamp; samples podados; evidencia en `execution.md` (instancia **no** en diff). |
| **AC-GIT-CLEAN** | `.SddIA/cerbero/` y `.SddIA/radamanto/` ausentes del diff del PR. |
| **AC-ONTO** | `entity_type: process` conservado. |
| **AC-A2** | EDA blocked + huérfanos + umbral → `fail_soft` retroactivo + agregador success / `exit_code: 0`; sin umbral → causal; Argos `block` preservado; agregador sin diff. |
| **AC-TESTS** | Unit §7 verde; regresiones higiene/snapshot intactas. |
| **AC-THRESH** | Umbrales 1.1.0 bit-idénticos. |
| **AC-RBAC** | Aduana PPR posterior: `RBAC_EMITTER_NOT_REVOKED: APTO` con emisor DCC. |
| **AC-DOC** | Cascada; PBI en `done/`; `validacion.md` `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## 7. Pruebas mínimas (producto — qué, no cómo)

| Caso | Aserción |
|------|----------|
| Report EDA `blocked` + `orphan_count: 2` + `argos_verdict: block` + state con `pr_url` → post-adjudicación | `fail_soft: true`; `aggregate_execution_terminal` → `success: true` |
| Mismo report EDA **sin** `pr_url` ni `delivery_push` | sin `fail_soft`; agregador → `exit_code: 1` |
| Umbral solo `delivery_push` (sin `pr_url`) + EDA blocked huérfanos | `fail_soft: true`; success |
| Regresión `dcc_hygiene_failed_is_fail_soft_when_pr_url_present` | intacta |
| Regresión `dcc_snapshot_failed_never_fail_soft` | intacta |
| Regresión `dcc_hygiene_failed_without_pr_stays_causal` | intacta |

Tests en crate `execute-process` (`--lib`, filtro `delivery_close`). Preferir helper puro + fixtures JSON; no exigir spawn orquestador completo para el predicado.

## 8. Fuera de alcance

- Rehab `bug-fix` / `refactorization` / `emit-pr-audited-event` / `feature`.
- Backfill EDA de huérfanos preexistentes.
- Merge / `accept-pr` de PR #187 (ya MERGED).
- Mutar umbrales / `abrupt_drop_min_samples`.
- Poda `survival_hollow` / A3 #185.
- Mutar agregador o castrar señal Argos en gate EDA.
- Versionar instancia Cerbero/Radamanto en el PR.
- Inyectar `fail_soft` estático en YAML de proceso.

## 9. Handoff Tekton

Ejecutar `plan.md` T0→T5. Git solo `skill:git-manager`. No declarar A1 APTO sin evidencia de instancia en `execution.md`. No inventar `pr_url` para forzar fail_soft. No tocar umbrales ni hollow.
