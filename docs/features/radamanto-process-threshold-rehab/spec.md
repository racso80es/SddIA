---
feature_name: radamanto-process-threshold-rehab
created: "2026-08-16"
process: refactorization
phase: design
agents: dedalo
base: main
scope: umbrales-radamanto-por-tipo + rehab-revoked PPR/DCC (olas 174+177)
branch_name: refactor/radamanto-process-threshold-rehab
persist_ref: docs/features/radamanto-process-threshold-rehab
pbi_ref: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
uuid: ba900e95-1a47-4185-b86c-bc7a251b4fe6
version_spec: "1.0.0"
status: dedalo_locked
olas:
  - ola-1
  - ola-2
correlation_id: ""
---

# Spec — radamanto-process-threshold-rehab

## 1. Misión técnica

Unificar anti-recurrencia y rehabilitación de `pull-request-review` (ola 1 / PPR #174) y `delivery-close-cycle` (ola 2 / PPR #177): ontología `entity_type: process`, umbrales Radamanto por tipo, fail-soft por ola, un `persist_ref`, un PR.

Consumir `objectives.md` + `clarify.md` como `refined_requirements` (Mayeuta).

## 2. Diagnóstico (evidencia 2026-08-16)

| Vector | Hecho |
|--------|--------|
| Umbral SSOT | `SddIA/agents/radamanto.thresholds.json` · `success_rate_min: 0.85` **plano** (sin tabla por tipo) |
| Tipología motor | `entity_type_from_id` en `radamanto_batch_core.rs`: sin prefijo `type:` → default **`tool`** |
| Cerbero instancia | `delivery-close-cycle` ∈ `revoked` · `entity_type: tool` · `reason: success_rate_below_threshold` · `since: 2026-08-16T16:11:08Z` |
| PPR instancia | `pull-request-review` **ausente** de `revoked` · stats root `healthy` · `rehab_laudo` legado #124+#125 |
| DCC stats root | `pending_redemption` · `recovery_attempts: 3` · `structure_valid: true` · ventana ~20 samples con ≥4 `exit_code=1` → rate ≈ **0.80 < 0.85** |
| Exención latency | `LATENCY_THRESHOLD_EXEMPT` ⊇ solo `pull-request-review` — **no** cubre `success_rate_below_threshold` |
| Laterales | `permanent.feature`; `revoked.bug-fix`; `revoked.emit-pr-audited-event` — **fuera de alcance** |

Causa raíz: procesos multi-fase indexados por nombre bare se miden y revocan como `tool` atómico bajo umbral 0.85; fallos de fricción (sub-fase / telemetría secundaria) contaminan `exit_code` global → re-revocación post-rehab.

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-TYPE-RESOLVE** | Resolver `entity_type` con precedencia: (1) prefijo `type:id` válido; (2) si `resolve_process_path(repo, id)` OK → `process`; (3) default `tool`. Aplicar en `governance_payload` / telemetría / Degraded. |
| **L-THRESH-TABLE** | Versionar umbrales a **1.1.0** con `success_rate_min_by_entity_type`. Plano `success_rate_min` permanece como **fallback** (= tool). |
| **L-NUMBERS** | `tool`/`skill`/`action`/`norm`/`codex`/`event` = **0.85**; `agent` = **0.75**; `process` = **0.70**. Justificación: rate empírico DCC ≈0.80 pasa umbral process y falla tool; conserva degradación ante colapso real (<0.70 con `batch_min_events`). |
| **L-RATE-LOOKUP** | En batch healthy: `rate_min = by_type[entity_type] ?? success_rate_min`. Mismo lookup para abrupt drop. |
| **L-LATENCY-PROCESS** | Eximir latency cuando `entity_type == process` (wall-clock multi-fase). Mantener allowlist `LATENCY_THRESHOLD_EXEMPT` como overlay aditivo (no único mecanismo). Incluir `delivery-close-cycle` vía tipología, no hardcode suelto. |
| **L-INSTR-R41** | Alinear texto R4.1 en `radamanto.instructions.json`: umbral efectivo = lookup por tipo. |
| **L-REHAB-INST** | Instancia (no git): retirar `delivery-close-cycle` de `revoked`; verificar PPR ausente; buckets DCC root (+ nested si degradado) → `healthy`, limpiar `degraded_at`, reset `recovery_attempts`/`consecutive_success_count` según redención, `rehab_laudo: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS`, `rehabilitated_at` ISO. Prohibido tocar `feature`/`bug-fix`/`emit-pr-audited-event`. |
| **L-FAILSOFT-OLA1** | En `pull-request-review`: fricción de sub-fase no causal (API externa, lectura puntual, evidencia git soft con bridge) → `fail_soft: true` en phase report antes de `aggregate_execution_terminal`. F2/F4/F5 duros intactos. |
| **L-FAILSOFT-OLA2** | En `delivery-close-cycle`: si Snapshot + Publicación remota cruzaron umbral físico (commit/push OK) y existe `pr_url`, timeout/error no crítico en `telemetry_receipt` / validación secundaria de repo **no** impide `capsule_delivery_emit_presented` ni marca éxito global falso; marcar esos reports `fail_soft: true`. |
| **L-GENOME** | Motor Rust (`radamanto_batch_core.rs`, fail-soft handlers) = parche engine. Umbrales/instructions = asset agente vía cadena autorizada. Instancia `.SddIA/` = evidencia en `execution.md`, no diff de PR. |
| **L-SCOPE** | Sin faros Kaizen (troceo EDA PPR; centinela EDA `RBAC_EMITTER_NOT_REVOKED`). Sin merge `accept-pr` de #174/#177. Sin residual Kalma2 Shell. |

### Números canónicos (SSOT post-cambio)

```json
{
  "version": "1.1.0",
  "success_rate_min": 0.85,
  "success_rate_min_by_entity_type": {
    "tool": 0.85,
    "skill": 0.85,
    "action": 0.85,
    "agent": 0.75,
    "process": 0.70,
    "norm": 0.85,
    "codex": 0.85,
    "event": 0.85
  },
  "batch_min_events": 10,
  "latency_ms_p95_threshold": 30000,
  "redemption_success_count": 3,
  "max_recovery_attempts": 3,
  "abrupt_drop_min_samples": 3
}
```

## 4. Touchpoints

| Locus (Cúmulo) | Mutación |
|----------------|----------|
| `directories.agents` → `radamanto.thresholds.json` | Schema 1.1.0 + tabla por tipo |
| `directories.agents` → `radamanto.instructions.json` | R4.1 lookup por tipo |
| `SddIA/engine/execute-process/.../radamanto_batch_core.rs` | `entity_type` con repo+catálogo process; `rate_min` por tipo; latency exempt si `process` |
| `SddIA/engine/execute-process/.../phase_capsules.rs` / `delivery_close.rs` | Fail-soft ola 2 (post-push / telemetry secundaria) |
| Handlers / agregación PPR (runtime residual o reports agente) | Fail-soft ola 1 en fricción no causal |
| `.SddIA/cerbero/revoked_entities.json` | Rehab instancia DCC (no PR) |
| `.SddIA/radamanto/stats.json` | Redención DCC → healthy (no PR) |
| `directories.evolution` | Entrada breve UUID ciclo |
| `persist_ref` cascada | `implementation.md` / `execution.md` / `validacion.md` + archive PBI |

Prohibido mutar YAML de proceso `pull-request-review.md` / `delivery-close-cycle.md` salvo nota documental mínima vía `entity-manager` si Argos lo exige; preferir comportamiento en engine.

## 5. Contratos de comportamiento

### 5.1 Lookup umbral

```text
etype = resolve_entity_type(repo, entity_id)
rate_min = thresholds.success_rate_min_by_entity_type[etype]
         ?? thresholds.success_rate_min
if samples.len >= batch_min_events && rate < rate_min:
  degrade(success_rate_below_threshold)
```

### 5.2 Tipología

```text
if id contains ":" and prefix ∈ VALID_ENTITY_TYPES → prefix
else if resolve_process_path(repo, id).is_ok() → "process"
else → "tool"
```

### 5.3 Fail-soft (agregación)

Contrato existente `phase_terminal::aggregate_execution_terminal`: `fail_soft: true` excluye la fase del causal failure → `success` global / `exit_code` no colapsan. Tekton debe **emitir** ese flag en los touchpoints ola 1/2; no redefinir el agregador salvo gap demostrado.

### 5.4 Aduana post-rehab

| Check | Esperado tras merge + instancia alineada |
|-------|------------------------------------------|
| `RBAC_PROCESS_REGISTRY` | APTO (`pull-request-review` ∉ revoked) |
| `RBAC_EMITTER_NOT_REVOKED` | APTO (`delivery-close-cycle` ∉ revoked) |

## 6. Criterios técnicos (mapa AC)

| AC | Verificación |
|----|--------------|
| **AC-OLA1** | PPR ∉ `revoked`; tipología `process` en eventos Degraded futuros; check aduana APTO |
| **AC-OLA2** | DCC ∉ `revoked`; tipología `process`; check aduana APTO |
| **AC-THRESH** | JSON 1.1.0 + unit/smoke: entity process usa 0.70; tool sigue 0.85; anti-recurrencia documentada |
| **AC-TYPE** | Bare `delivery-close-cycle` / `pull-request-review` → `entity_type=process` en payload governance |
| **AC-FAILSOFT** | Ola1: fricción sub-fase con `fail_soft` no fuerza exit global 1; Ola2: sello Presented tras push OK pese a timeout telemetría secundaria |
| **AC-SCOPE** | `feature` / `bug-fix` / `emit-pr-audited-event` intactos en Cerbero |
| **AC-DOC** | Cascada completa; canónico + satélites en `docs/todos/done/`; `validacion.md` `global: APTO`, `pbi_archived: true`, `branch` coherente |

## 7. Pruebas mínimas (Tekton / Argos)

| Caso | Aserción |
|------|----------|
| Lookup process | `success_rate_min_for("process") == 0.70` |
| Lookup tool | `== 0.85` |
| `entity_type_from_id(repo, "delivery-close-cycle")` | `"process"` |
| `entity_type_from_id(repo, "pull-request-review")` | `"process"` |
| Batch healthy process rate 0.80 / n≥10 | **no** Degraded |
| Batch healthy tool rate 0.80 / n≥10 | Degraded `success_rate_below_threshold` |
| Latency process avg > threshold | **no** Degraded por latency |
| Instancia post-rehab | DCC ∉ revoked; status healthy |
| Scope | claves laterales sin cambio |

## 8. Fuera de alcance

- Faros Kaizen (troceo EDA PPR; aislar `RBAC_EMITTER_NOT_REVOKED` en centinela).
- Rehabilitar `feature` / `bug-fix` / `emit-pr-audited-event`.
- Merge / `accept-pr` de PR #174 / #177 históricos.
- Residual Kalma2 Shell / `git-manager` (dedup #136).
- Reabrir #124 / #125 / #136.
- Renombrar `fix-tool-process` ni taxonomía EDA global.

## 9. Handoff Tekton

Ejecutar `plan.md` en orden T0→T5. Git solo `skill:git-manager`. No inventar checks aduana APTO sin evidencia de instancia + umbrales desplegados.
