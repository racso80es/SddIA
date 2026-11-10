---


feature_name: man-39-refactorization-reentry-cli-collapse
created: "2026-11-10"
process: feature
branch_name: feat/man-39-refactorization-reentry-cli-collapse
persist_ref: docs/features/man-39-refactorization-reentry-cli-collapse
pbi_ref: MAN-39
historia_ref: MAN-36
document_id: PBI-70fb886c
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
version_plan: "1.0.0"
phases:
  - T0-salto-forja
  - T1-timeout-boveda
  - T2-aborto-envelope
  - T3-dirty-peaje
  - T4-reanudacion-argos
  - T5-labels-padre
  - T6-tests
rbac_ok: true
scope: "Reentrada sin tren de forja, timeout de bóveda, aborto con envelope, peaje en dirty-gate, labels exactas del padre"
norm_ref: features-documentation-pattern@1.3.1
forge_key: "2e7baef71d4c29dc25c07db567099259c855c06271d36cd0ae854ba35cb5211b"
execution_id: "2783339c-b2b3-4abb-8d80-383a87ad1005"
---
# Plan / Blueprint — man-39-refactorization-reentry-cli-collapse

Blueprint para **Tekton**. Entrada: `objectives.md`, `clarify.md`, `spec.md` **v1.0.0**.
Tracker **MAN-39** / documento **PBI-70fb886c** / historia **MAN-36**.

`target_executor_rbac.allowed_policies`: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `chaos-engineering`, `tracker-operations`.

## Viabilidad RBAC

| Cápsula | Contexto | ¿En Tekton? |
|---------|----------|-------------|
| `skill:filesystem-manager` | `filesystem-ops` | sí |
| `skill:shell-executor` | `system-operations` | sí — `cargo test` |
| `skill:git-manager` | `source-control` | sí — única vía Git |
| `action:execute-process` → `entity-manager` | `ecosystem-evolution` | sí si muta un `.md` de genoma |

**Prohibido:** ficheros nuevos en `docs/todos/`. **Prohibido:** incluir en el commit los sucios ajenos (tools/daemon y docs MAN-40) que viajaron con `SDDIA_LAB_ALLOW_DIRTY`.
**rbac_ok:** `true`.

## Fases

```yaml
phases:
  - name: "T0 Salto de forja"
    intent: "Si implementation.md y execution.md están en persist_ref, Mayeuta/Dedalo/Tekton de feature, bug-fix y refactorization quedan skipped reason forge_already_present."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T1 Timeout de bóveda"
    intent: "Ejecución resuelve timeout con suelo 2400. Clave vacía o no entera cuenta como ausente. El acuse declara timeout_floor_applied cuando el suelo gana."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T2 Aborto con envelope"
    intent: "Muerte del padre termina el grupo del agente y el sddia-workspace-server de ese execution_id, y stdout cierra con success false y agent-runtime-aborted."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T3 Dirty-gate del peaje"
    intent: "path_in_scope admite paths.auditsPath/tracker_ref.md del mismo tracker. El resto de sucios sigue fallando."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T4 Reanudación"
    intent: "Fases saltadas por forge_already_present no bloquean Verificación ni el cierre. STOP_AFTER=design sigue cortando si Dedalo executed y no hay forja."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T5 Labels del padre"
    intent: "Test de contrato: label ausente o typo no crea issue. Catálogo lab MAN incluye Kaizen. Sin alias."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T6 Tests"
    intent: "cargo test -p execute-process y tests del linear-tracker-adapter cubren KA-REENTRY-1..6."
    delegates_to: ["skill:shell-executor"]
```

## Orden y commits

Un commit por tajo, mensaje con el id **MAN-39**, solo paths del tajo. T4 va después de T0 porque reutiliza la razón `forge_already_present`. T6 puede ir en el commit del último tajo que cierre los tests, sin mezclar sucios ajenos.

Contrato `.md` de `linear-tracker-adapter` o de un proceso: solo `entity-manager`. El cambio de comportamiento previsto cabe en Rust del engine y en tests; no hace falta tocar el genoma de proceso si el contrato de I/O no cambia.

## Parada

Esta planificación no ejecuta T0–T6. Siguiente estímulo: el Vértice Biológico.
