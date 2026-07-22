---
feature_name: inyeccion-dependencias-envelope-homologacion
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-envelope-homologacion
persist_ref: docs/features/inyeccion-dependencias-envelope-homologacion
document_id: PBI-042-ENVELOPE-HOMOLOGACION
execution_id: e7a4b2c3-8f1d-4e6a-9b2c-1d3e5f7a9b0c
phases: 6
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
scope: "Hito 4 — Envelope Cerbero + homologación catálogo (R9–R10)"
---

# Plan / Blueprint — DI envelope Cerbero + homologación catálogo (Hito 4)

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md`, `spec.md`.

`target_executor_rbac.allowed_policies`: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula / acción | ¿Permitida? |
|------------------|-------------|
| `skill:filesystem-manager` | sí |
| `skill:git-manager` | sí |
| `skill:shell-executor` | sí (`cargo test`) |
| `action:execute-process` | sí (smokes) |

Ninguna fase exige cápsula fuera de `allowed_policies`. Homologación R10 es mutación genoma vía `entity-manager` (no bypass manual).

---

## Fases declarativas

```yaml
phases:
  - name: "R9 Schema envelope + cerbero_di_envelope"
    intent: "Crear di.binding.schema.json; implementar cerbero_di_envelope.rs; cablear post-RBAC en executor/residual/reactor; DLQ + tests AC-R9."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "R10 Homologación catálogo ED"
    intent: "Anotar ≥4 ED nuevas §4.6 spec vía entity-manager; fase cierre refactorization; evolution por ED."
    delegates_to: ["skill:filesystem-manager"]
  - name: "Documentación contrato I/O"
    intent: "Patch capsule-json-io.md con nota R9; verificar coherencia process-contract."
    delegates_to: ["skill:filesystem-manager"]
  - name: "Regresión H3 + H2 + MVP"
    intent: "cargo test capability_di + cerbero_di + envelope + di_reactor + di_output; AC-R5/R6/R7/R8 + AC-R1/R2 + AC-P1/P2/P3 verdes."
    delegates_to: ["skill:shell-executor"]
  - name: "Documentación ejecución"
    intent: "implementation.md + execution.md; handoff Argos."
    delegates_to: ["skill:filesystem-manager"]
  - name: "Sellado documental parcial"
    intent: "Actualizar handoff; validacion.md pendiente Argos."
    delegates_to: ["skill:filesystem-manager"]
```

---

## R9 — Schema envelope + Cerbero

| # | Entregable | Detalle |
|---|------------|---------|
| R9.1 | `di.binding.schema.json` | Schema §4.3 spec bajo `capability_contracts` |
| R9.2 | `cerbero_di_envelope.rs` | `validate_packaged_bindings`; códigos `CERBERO_ENVELOPE_SCHEMA_MISMATCH`, `CERBERO_DI_BINDING_INCOHERENT` |
| R9.3 | `executor.rs` | Post-`validate_di_rbac`, pre-`execute_phase_body`: validar `entry["di_binding"]` vs `resolved_bindings` |
| R9.4 | `residual_runner.rs` | Paridad cadena |
| R9.5 | `capability_di_reactor.rs` | Envelope en `run_sync_chain`; `cerbero_envelope_di_code` en `CapabilityDi_Resolved` |
| R9.6 | Tests AC-R9 | Fixture tamper: gate APTO + RBAC allow + envelope inválido → abort; DLQ con código envelope |

**Salida:** AC-R9 demostrable; orden `resolve → gate → rbac → envelope → inject` intacto.

**Restricción:** no mover ni debilitar `capability_di_gate` (**L-GATE-PRESERVE**).

---

## R10 — Homologación catálogo ED

| # | ED | Mutación |
|---|-----|----------|
| R10.1 | `refactorization.md` | Nueva fase «Cierre documental en rama» con `requires_capability` `doc:closure` (ciego) antes de «Cierre de entrega» |
| R10.2 | `delivery-close-cycle.md` | `requires_capability` `proc:git-sync` en «Publicación remota» (ciego) |
| R10.3 | `accept-pr.md` | `requires_capability` `proc:git-sync` en «Fusión Soberana» (ciego) |
| R10.4 | `pull-request-review.md` | `requires_capability` `proc:git-sync` en «Preparación de rama» (ciego) |
| R10.5 | Evolution | Entrada `SddIA/evolution/` por cada ED + entrada feature Hito 4 |

**Salida:** AC-R10 — conteo ≥8 ED homologadas verificable.

**Restricción:** sin altas en `capability-taxonomy.catalog` (**L-R10-NO-INVENT**); sin filas nuevas en `capability-bindings.md`.

---

## Regresión obligatoria

| Bloque | Tests / evidencia |
|--------|-------------------|
| AC-REG-H3 | `cerbero_di_rbac` deny; EDA piloto non-blocking; `proc:git-sync` en taxonomía; output validator |
| AC-REG-H2 | Resolver ciego; `di_binding` en stdin |
| AC-REG-MVP | Gate pre-ignición P1/P2/P3 |

Ejecutar con flag EDA **ausente** salvo tests AC-R6 explícitos.

---

## Handoff Argos

Criterios en `spec.md` §5. `validacion.md` con `global: APTO` solo si AC-R9, AC-R10 y regresión verdes. PBI-042 padre permanece en `pending/` (**L-PBI-LOC**).
