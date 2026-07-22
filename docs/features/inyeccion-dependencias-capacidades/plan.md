---
feature_name: inyeccion-dependencias-capacidades
created: "2026-07-21"
process: feature
branch_name: feat/inyeccion-dependencias-capacidades
persist_ref: docs/features/inyeccion-dependencias-capacidades
document_id: PBI-042-INYECCION-DEPENDENCIAS-CAPACIDADES
execution_id: 9120e3da-6ba9-4a93-9735-34486383c7de
phases: 6
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
scope: "MVP — Metadatos Activos + Códice de la Lengua + Aduana Temprana"
---

# Plan / Blueprint — DI por capacidades (MVP)

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md`, `spec.md`.

`target_executor_rbac.allowed_policies`: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula | ¿Permitida? |
|---------|-------------|
| `action:execute-process` → `entity-manager` / `norm-creator` | sí |
| `skill:filesystem-manager` | sí |
| `skill:shell-executor` | sí (cargo test) |
| `skill:git-manager` | sí (commits bajo orden) |

---

## Fases declarativas

```yaml
phases:
  - name: "M2 Códice de la Lengua"
    intent: "Forjar Library_Norm capability-taxonomy + schema doc.closure + registro Cúmulo."
    delegates_to: ["action:execute-process", "skill:filesystem-manager"]
  - name: "M1 Metadatos Activos — contratos"
    intent: "Documentar provides/requires_capability en process/actions/skills-contract."
    delegates_to: ["skill:filesystem-manager"]
  - name: "M1 Metadatos Activos — piloto"
    intent: "Anotar feature.md (requires) y filesystem-manager.md (provides)."
    delegates_to: ["skill:filesystem-manager"]
  - name: "M3 Aduana Temprana"
    intent: "Implementar capability_di_gate en execute-process + cableado pre-ignición."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "Verificación"
    intent: "Tests AC-P1..P3 + AC-M*; implementation.md + execution.md + evolution."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager"]
  - name: "Sellado documental parcial"
    intent: "Actualizar handoff; no delivery-close hasta Argos APTO."
    delegates_to: ["skill:filesystem-manager"]
```

---

## M2 — Códice de la Lengua

| # | Entregable | Detalle |
|---|------------|---------|
| M2.1 | `entity-manager` create `norm` | `entity_name: capability-taxonomy`; friction = Taxonomía Universal; alta `doc:closure` |
| M2.2 | `capability-contracts/doc.closure.schema.json` | JSON Schema mínimo del contrato piloto |
| M2.3 | `cumulo.paths.json` | `normative_documents.capability_taxonomy` + path `capability_contracts`; bump patch version |

**Salida:** norma indexada en `library_norms/index.md`; Cúmulo resuelve path.

---

## M1 — Metadatos Activos

| # | Entregable | Detalle |
|---|------------|---------|
| M1.1 | `process-contract.md` | § Metadatos DI: `requires_capability` en fase/proceso |
| M1.2 | `actions-contract.md` | § `provides` opcional MVP |
| M1.3 | `skills-contract.md` | § `provides` opcional MVP |
| M1.4 | `feature.md` | Fase «Cierre documental en rama» + `requires_capability` |
| M1.5 | `filesystem-manager.md` | `provides` `doc:closure` |

**Salida:** genoma piloto declara DI; contratos describen schema.

---

## M3 — Aduana Temprana

| # | Entregable | Detalle |
|---|------------|---------|
| M3.1 | `capability_di_gate.rs` | Pasos 1–6 de `spec.md` §4.4 |
| M3.2 | Cableado | Llamada desde `executor.rs` / residual path **antes** de handler de fase cuando `requires_capability` no vacío |
| M3.3 | DLQ | Escribir fallo en `eda_bus.dead_letter` con código `CAPABILITY_*` / `CONTRACT_SCHEMA_MISMATCH` |
| M3.4 | Tests | Unitarios P1/P2/P3 en el crate |
| M3.5 | Opt-out lab | `SDDIA_LAB_SKIP_CAPABILITY_DI=1` documentado |

**Salida:** `cargo test -p execute-process` verde en tests del gate.

---

## Orden de ejecución Tekton

1. M2 (norma + schema + Cúmulo) — sin gate aún no rompe runtime.
2. M1 contratos (docs) → M1 piloto (genoma).
3. M3 gate + tests.
4. Verificación + `implementation.md` / `execution.md` / evolution.
5. Argos en ciclo posterior (no en esta pasada si tests fallan).

## Criterios de salida del blueprint

- AC-M1, AC-M2, AC-M3, AC-P1–P3 según `spec.md` §5.
- Sin mutación GesFer / sin archivo PBI kitchen.
- Genoma vía `entity-manager` para la norma; contratos/piloto/engine bajo topología feature activa (DA-4).
