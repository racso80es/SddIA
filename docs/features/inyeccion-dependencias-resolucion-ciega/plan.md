---
feature_name: inyeccion-dependencias-resolucion-ciega
created: "2026-07-22"
process: feature
branch_name: feat/inyeccion-dependencias-resolucion-ciega
persist_ref: docs/features/inyeccion-dependencias-resolucion-ciega
document_id: PBI-042-RESOLUCION-CIEGA-INYECCION
execution_id: a7e3c9f2-4b1d-4e8a-9c5f-2d6b8e1a0f47
phases: 6
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
scope: "Hito 2 — Resolución ciega e inyección (R1–R4)"
---

# Plan / Blueprint — DI resolución ciega e inyección (Hito 2)

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md`, `spec.md`.

`target_executor_rbac.allowed_policies`: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula | ¿Permitida? |
|---------|-------------|
| `skill:filesystem-manager` | sí |
| `skill:shell-executor` | sí (`cargo test`) |
| `skill:git-manager` | sí (commits bajo orden) |
| `action:execute-process` | sí (smokes proceso; sin mutar genoma fuera de DA-4) |

Ninguna fase del blueprint exige cápsula fuera de `allowed_policies`.

---

## Fases declarativas

```yaml
phases:
  - name: "R3 Binding table + Cúmulo"
    intent: "Forjar capability-bindings.md (fila doc:closure→filesystem-manager) y registrar capability_di.bindings en Cúmulo."
    delegates_to: ["skill:filesystem-manager"]
  - name: "R1 Resolver + R3 adapt gate"
    intent: "Implementar capability_di_resolver; cablear resolve→gate; adaptar capability_di_gate a proveedor efectivo."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "R2 Inject di_binding"
    intent: "Merge di_binding en invoke_capsule_json / agent_runtime / entry de fase; documentar capsule-json-io."
    delegates_to: ["skill:filesystem-manager"]
  - name: "R4 Piloto genoma + contratos"
    intent: "feature.md y bug-fix.md ciegos; process-contract modo ciego; sin tocar Library_Codex."
    delegates_to: ["skill:filesystem-manager"]
  - name: "Verificación"
    intent: "Tests AC-R1..R4 + AC-REG (P1–P3); implementation.md + execution.md + evolution."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager"]
  - name: "Sellado documental parcial"
    intent: "Actualizar handoff; no delivery-close hasta Argos APTO."
    delegates_to: ["skill:filesystem-manager"]
```

---

## R3 — Binding table + Cúmulo

| # | Entregable | Detalle |
|---|------------|---------|
| R3.1 | `SddIA/core/capability-bindings.md` | Frontmatter §4.1 spec; UUID v4 nuevo; fila única `doc:closure` → `skill:filesystem-manager` |
| R3.2 | `cumulo.paths.json` | Bloque `capability_di.bindings`; bump patch version |
| R3.3 | Evolution | Entrada vinculando `document_id` / execution_id de este feature |

**Salida:** runtime puede resolver path del mapa solo vía Cúmulo.

**Restricción:** prohibido escribir en `SddIA/library/codexes/` o sobrecargar `capability-taxonomy` como router.

---

## R1 — Resolver + adapt gate

| # | Entregable | Detalle |
|---|------------|---------|
| R1.1 | `capability_di_resolver.rs` | Pasos 1–6 de `spec.md` §4.3; códigos `CAPABILITY_BINDING_MISSING`, `CAPABILITY_PROVIDER_AMBIGUOUS` |
| R1.2 | Cableado `executor.rs` (+ residual si aplica) | Antes del gate: resolve → fase efectiva |
| R1.3 | Adapt `capability_di_gate.rs` | Validar `provides` del proveedor efectivo; conservar DLQ P1–P3 |
| R1.4 | Tests resolver | Fila OK; missing; ambiguous; coherencia con `delegates_to` dual |

**Salida:** path ciego pasa aduana sin `delegates_to` literal en genoma.

---

## R2 — Inject `di_binding`

| # | Entregable | Detalle |
|---|------------|---------|
| R2.1 | Helper merge | Construye objeto `di_binding` (§4.5) desde `ResolvedBinding` |
| R2.2 | `capsules.rs` | Inyectar en stdin JSON pre-subprocess |
| R2.3 | `agent_runtime` / entry fase | Propagar binding en envelope de handoff / auditoría |
| R2.4 | `capsule-json-io.md` | Documentar campo opcional |
| R2.5 | Test | Assert presencia y forma de `di_binding` en payload sintetizado |

**Salida:** AC-R2 demostrable en test sin depender de LLM-native side-effects.

---

## R4 — Piloto genoma + contratos

| # | Entregable | Detalle |
|---|------------|---------|
| R4.1 | `feature.md` | Fase cierre: solo `requires_capability`; **sin** `delegates_to` |
| R4.2 | `bug-fix.md` | Misma anotación ciega (`requires_capability` + sin `delegates_to`) |
| R4.3 | `process-contract.md` | § DI: `delegates_to` opcional bajo binding table |
| R4.4 | Hash / versión ED | Recalcular `hash_signature` / bump patch de process si la norma del repo lo exige al mutar frontmatter |

**Salida:** ≥2 consumidores ciegos + 1 fila mapa + 1 proveedor MVP intacto.

---

## Orden de ejecución Tekton

1. R3 (mapa + Cúmulo) — sin romper runtime hasta cablear resolver.
2. R1 (resolver + gate) + tests unitarios intermedios.
3. R2 (inject + norma I/O).
4. R4 (genoma piloto) — **después** de que resolve+gate soporten path ciego (si no, fases residuales fallarían).
5. Verificación completa + `implementation.md` / `execution.md` / evolution.
6. Argos en ciclo posterior.

## Criterios de salida del blueprint

- AC-R1, AC-R2, AC-R3, AC-R4, AC-REG según `spec.md` §5.
- Sin R5–R8, sin GesFer, sin F1, sin altas al Códice.
- Genoma Core bajo DA-4 (feature activa en rama); Git solo `skill:git-manager` bajo orden.
- PBI-042 permanece en `docs/todos/pending/` (L-PBI-LOC).
