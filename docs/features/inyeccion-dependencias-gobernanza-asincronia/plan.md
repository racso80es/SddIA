---
feature_name: inyeccion-dependencias-gobernanza-asincronia
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-gobernanza-asincronia
persist_ref: docs/features/inyeccion-dependencias-gobernanza-asincronia
document_id: PBI-042-GOBERNANZA-ASINCRONIA
execution_id: f8b2c4d1-6e3a-4f7b-9c2d-1a0e5f8b3c7d
phases: 7
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
scope: "Hito 3 — Gobernanza Cerbero, piloto EDA, Códice y schema salida (R5–R8)"
---

# Plan / Blueprint — DI gobernanza y asincronía (Hito 3)

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md`, `spec.md`.

`target_executor_rbac.allowed_policies`: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula / acción | ¿Permitida? |
|------------------|-------------|
| `skill:filesystem-manager` | sí |
| `skill:git-manager` | sí |
| `skill:shell-executor` | sí (`cargo test`) |
| `skill:git-manager` (forja genoma) | sí |
| `action:execute-process` | sí (smokes; sin bypass genoma) |

Ninguna fase exige cápsula fuera de `allowed_policies`.

---

## Fases declarativas

```yaml
phases:
  - name: "R7 Códice + contrato + binding"
    intent: "Alta proc:git-sync en capability-taxonomy vía entity-manager; schema proc.git_sync; fila binding; provides git-manager; evolution."
    delegates_to: ["skill:filesystem-manager"]
  - name: "R5 Cerbero DI RBAC"
    intent: "Implementar cerbero_di_rbac; cablear post-gate en executor/residual; DLQ + tests AC-R5."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "R8 Validador salida runtime"
    intent: "capability_di_output_validator + jsonschema; hook post-cápsula; tests AC-R8."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "R6 Piloto EDA DI"
    intent: "CapabilityDi_Requested/Resolved; capability_di_reactor; flag SDDIA_DI_EDA_PILOT; suscripción piloto; tests AC-R6."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "Regresión H2 + MVP"
    intent: "cargo test capability_di + cerbero_di + di_output + di_reactor; AC-R1/R2 + AC-P1/P2/P3 verdes sin flag EDA."
    delegates_to: ["skill:shell-executor"]
  - name: "Documentación ejecución"
    intent: "implementation.md + execution.md; patch capsule-json-io; handoff Argos."
    delegates_to: ["skill:filesystem-manager"]
  - name: "Sellado documental parcial"
    intent: "Actualizar handoff; validacion.md pendiente Argos."
    delegates_to: ["skill:filesystem-manager"]
```

---

## R7 — Códice + contrato + binding

| # | Entregable | Detalle |
|---|------------|---------|
| R7.1 | `proc.git_sync.schema.json` | Schema §4.5 spec bajo `capability_contracts` |
| R7.2 | `capability-taxonomy.md` | Bump patch; fila `proc:git-sync` vía `entity-manager` update |
| R7.3 | `git-manager.md` | Añadir `provides` proc:git-sync |
| R7.4 | `capability-bindings.md` | Segunda fila canónica proc:git-sync → git-manager |
| R7.5 | Evolution | UUID feature en `SddIA/evolution/` |

**Salida:** AC-R7 demostrable en diff norma + binding.

**Restricción:** taxonomía ≠ binding table (L-CODEX-ROLE); no Library_Codex.

---

## R5 — Cerbero DI RBAC

| # | Entregable | Detalle |
|---|------------|---------|
| R5.1 | `cerbero_di_rbac.rs` | API §4.3; códigos `CERBERO_RBAC_DENIED`, `CERBERO_ENTITY_REVOKED` |
| R5.2 | `executor.rs` | Tras gate OK: `validate_di_rbac` → fail abort pre-inject |
| R5.3 | `residual_runner.rs` | Paridad si aplica fases DI |
| R5.4 | DLQ helper | Reutilizar topología `eda_bus.dead_letter` |
| R5.5 | Tests | Fixture políticas restrictivas: gate pasa, Cerbero niega (**AC-R5**) |

**Salida:** orden `resolve → gate → Cerbero → inject` cableado; ignición bloqueada en deny.

---

## R8 — Validador salida runtime

| # | Entregable | Detalle |
|---|------------|---------|
| R8.1 | `Cargo.toml` | Dependencia `jsonschema` |
| R8.2 | `capability_di_output_validator.rs` | Validar stdout JSON vs schema contrato |
| R8.3 | `capsules.rs` | Hook post-`invoke_capsule_json*` |
| R8.4 | Tests | stdout sin `required` → `CONTRACT_OUTPUT_SCHEMA_MISMATCH` + DLQ (**AC-R8**) |
| R8.5 | `capsule-json-io.md` | Nota validación post-ejecución |

**Salida:** payload real validado; gate pre-ignición intacto.

---

## R6 — Piloto EDA DI

| # | Entregable | Detalle |
|---|------------|---------|
| R6.1 | `capability_di_reactor.rs` | Consume `CapabilityDi_Requested`; ejecuta resolve→gate→Cerbero; emite `CapabilityDi_Resolved` |
| R6.2 | `executor.rs` | Rama `SDDIA_DI_EDA_PILOT=1` o `di_composition: eda_pilot`: emit evento, skip sync DI, no await |
| R6.3 | `event-domain-subscriptions.json` | Entrada piloto `CapabilityDi_Requested` |
| R6.4 | Helper test | `drain_di_reactor_once(repo)` para aserciones deterministas |
| R6.5 | Tests | Evento en pending + fase no bloqueada (**AC-R6**) |

**Salida:** traza observable en `./.events/`; path sync default sin regresión.

---

## Regresión obligatoria

| Comando / suite | Criterio |
|-----------------|----------|
| `cargo test -p execute-process capability_di` | AC-P1, AC-P2, AC-P3, AC-R1, AC-R2 |
| `cargo test -p execute-process cerbero_di` | AC-R5 |
| `cargo test -p execute-process di_output` | AC-R8 |
| `cargo test -p execute-process di_reactor` | AC-R6 |
| Sin `SDDIA_DI_EDA_PILOT` | Path H2 idéntico pre-Hito 3 |

---

## Orden de ejecución Tekton

1. **R7** (norma + schema + binding) — prerequisito homologación para tests nuevos.
2. **R5** (Cerbero) — depende de resolver/gate H2; independiente de R8/R6.
3. **R8** (output validator) — depende de contratos; independiente de R6.
4. **R6** (EDA piloto) — último; evita interferir regresión sync durante R5/R8.
5. Regresión completa + docs + handoff Argos.

## Criterios de salida del blueprint

- AC-R5, AC-R6 (producto) + AC-R7, AC-R8 + AC-REG-H2 + AC-REG-MVP según `spec.md` §5.
- PBI-042 permanece en `docs/todos/pending/` (L-PBI-LOC).
- Genoma Core bajo DA-4; Git solo `skill:git-manager`.
- Sin GesFer, F1, migración masiva ED, Cerbero-only aduana.
