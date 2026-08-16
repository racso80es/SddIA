---
feature_name: radamanto-process-threshold-rehab
created: "2026-08-16"
process: refactorization
phase: blueprint
agents: dedalo
phases: T0-T5
branch_name: refactor/radamanto-process-threshold-rehab
persist_ref: docs/features/radamanto-process-threshold-rehab
pbi_ref: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
document_id: PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS
uuid: ba900e95-1a47-4185-b86c-bc7a251b4fe6
olas:
  - ola-1
  - ola-2
---

# Plan — radamanto-process-threshold-rehab

Blueprint ejecutable para Tekton. Contratos: `spec.md` laudos L-* + AC-*.

## T0 — Umbrales SSOT (AC-THRESH)

1. Mutar `SddIA/agents/radamanto.thresholds.json` → version `1.1.0` + `success_rate_min_by_entity_type` (números L-NUMBERS).
2. Ajustar R4.1 en `SddIA/agents/radamanto.instructions.json` (lookup por tipo).
3. Cadena autorizada si la aduana de forja lo exige para assets bajo `directories.agents`; si no, parche atómico del JSON companion.

## T1 — Motor Radamanto (AC-TYPE / AC-THRESH)

1. `radamanto_batch_core.rs`:
   - `resolve_entity_type(repo, entity_id)` per L-TYPE-RESOLVE (usar `resolve_process_path`).
   - `success_rate_min_for(thresholds, etype)` per L-RATE-LOOKUP.
   - Latency: skip si `etype == "process"` **o** allowlist (L-LATENCY-PROCESS).
2. Defaults en `load_radamanto_config` / merge: incluir tabla vacía-safe (fallback plano 0.85).
3. Unit tests en crate `execute-process` (casos §7 spec).

## T2 — Fail-soft olas (AC-FAILSOFT)

1. **Ola 2:** `phase_capsules.rs` / `delivery_close.rs` — tras push+`pr_url`, errores de telemetría/validación secundaria → `fail_soft: true`; no abortar `capsule_delivery_emit_presented`.
2. **Ola 1:** touchpoint mínimo en cadena PPR / residual runner / phase reports: fricción no causal marca `fail_soft` antes de agregación; no debilitar F2/F4/F5.
3. Smoke o unit que demuestre agregación con `fail_soft` (reusar contrato `phase_terminal`).

## T3 — Rehab instancia (AC-OLA1 / AC-OLA2 / AC-SCOPE)

1. Retirar `revoked.delivery-close-cycle`.
2. Verificar `pull-request-review` ausente; si reaparece pre-merge, misma rehab.
3. Redención stats DCC → `healthy` + `rehab_laudo` canónico.
4. Assert laterales intactos. Evidencia en `execution.md` (paths instancia no van al PR).

## T4 — Documental Tekton + evolution

1. `implementation.md` + `execution.md` (frontmatter patrón).
2. Entrada `directories.evolution` vinculando UUID `ba900e95-1a47-4185-b86c-bc7a251b4fe6`.
3. Compilar/tests: `cargo test -p execute-process --lib` (filtros radamanto/failsoft si aplica).

## T5 — Verificación / cierre documental (Argos + archive)

1. Argos → `validacion.md` (`global`, `checks` mapeando AC-*, `pbi_archived: true`, `branch`).
2. Mover a `docs/todos/done/` en la **misma rama**:
   - PBI canónico `PBI-PPR-174-177-REVOKED-PROCESS-THRESHOLDS`
   - satélite ola 1 `PBI-PPR-174-REVOKED-REGISTRY`
   - satélite ola 2 `PBI-PPR-177-DCC-REVOKED-REGISTRY`
3. `delivery-close-cycle` (cierre de este refactor) — no despachar bug-fix satélite.

## Orden innegociable

```text
T0 → T1 → T2 → T3 → T4 → T5
```

T3 puede paralelizarse tras T1 (umbrales ya en disco local) pero **no** antes de T1 si se quiere anti-recurrencia inmediata en el mismo host.

## Delegaciones canónicas (RBAC)

| Fase plan | Cápsulas / agentes |
|-----------|-------------------|
| Mutación engine + tests | Tekton (filesystem bajo engine; no Shell raw destructivo) |
| Git | `skill:git-manager` exclusivamente |
| Cierre PR | `action:execute-process` → `delivery-close-cycle` |
| Archive PBI | Tekton en rama (norma cierre documental); Cúmulo solo si evento Kaizen |

`target_executor_rbac` esperado: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `quality-assurance`.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Re-revocación por rate real <0.70 | Aceptable; fail-soft reduce falsos; no inventar exención success_rate |
| `resolve_process_path` I/O en hot path batch | Cache por proceso de batch o set de nombres process conocidos en el tick |
| Instancia CI ≠ local | AC-OLA* verificables en host; aduana remota depende de rehab en ese entorno |
| Forja agentes bloqueada | Abortar T0 con causa; no bypass raw del genoma `{name}.md` |

## Fuera de este plan

Faros Kaizen; rehab `feature`/`bug-fix`/`emit-pr-audited-event`; accept-pr históricos #174/#177.
