---
document_id: PBI-FIX-FRACTURE-4c01d65b972f
title: "[FIX] delivery-close-cycle — push non-fast-forward escalado a Kintsugi + falso positivo Mayeuta"
format: markdown
version: "1.1.0"
created: "2026-09-26"
refined: "2026-09-27"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: 4c01d65b972f
fracture_process: delivery-close-cycle
friction_id: F-DCC-PUSH-NON-FAST-FORWARD
incident_ref: "System_Fracture_Detected — 4c01d65b972f"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
related_pbis:
  - id: PBI-KAIZEN-MAYEUTA-PRECISION-DIAGNOSTICA
    rol: "C-RC2 es un especimen de F-MAYEUTA-EVIDENCE-SCOPE. Si el Kaizen se ejecuta antes, C-RC2 se resuelve allí (K2) y este FIX se limita a C-RC1."
precedents:
  - SddIA/evolution/add08452-fbff-4768-b906-9b0eb2baa9e3.md   # F-DCC-DNS-UNRESOLVED
  - SddIA/evolution/bcb10a45-5cda-4e3e-9839-e0b912538003.md   # F-DCC-HOOK-EVOL-OVERESCALATION
---

# [FIX] delivery-close-cycle — push non-fast-forward escalado a Kintsugi + falso positivo Mayeuta

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` |
| Emisor | `execute-process` |
| Acción intentada | `Publicación remota` |
| Rama | `refactor/sddia-installer-v3-io-contract` |
| Estado operativo | Resuelto fuera de banda: rama integrada en `main` (`9346332`, PR #303, 2026-09-26). La deuda es del **motor**, no de la entrega. |

## Traza de error

```
SddIA pre-push: SKIPPED (delivery-close-cycle guard)
To https://github.com/racso80es/SddIA.git
 ! [rejected]        refactor/sddia-installer-v3-io-contract -> refactor/sddia-installer-v3-io-contract (non-fast-forward)
error: falló el empuje de algunas referencias a 'https://github.com/racso80es/SddIA.git'
ayuda: Updates were rejected because the tip of your current branch is behind
ayuda: its remote counterpart. If you want to integrate the remote changes,
ayuda: use 'git pull' before pushing again.
ayuda: See the 'Note about fast-forwards' in 'git push --help' for details.
```

## Mandato

Corregir la clasificación del fallo en el motor. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Diagnóstico refinado (Tekton, 2026-09-27)

### Refutación de la síntesis Mayeuta original

La síntesis automática concluyó *"Violación de jurisdicción delegada: terminal raw usada para evadir cápsula"* (`new_norm`). **Falso.**

- El push lo ejecutó la cápsula oficial: `capsule_delivery_remote_push` → `skill:git-manager` `push` `{force:false}`.
- La guarda anti-recursión funcionó (`SKIPPED (delivery-close-cycle guard)`).
- Disparador del falso positivo: `enrich_fracture_pbi_kaizen.rs` evalúa `has_any(["git push", "gh ", "bypass", …])` sobre `blob = error_trace + attempted_action + process_name`. El token `git push` aparece en el **texto de ayuda de git** (`'git push --help'`), no en ninguna acción del operador.
- La norma propuesta (`obediencia-procesos.md` § Jurisdicción Delegada) ya existe y no aplica. **No se crea ni se endurece ninguna norma.**

### Causa raíz 1 — Motor DCC: non-fast-forward no clasificado (C-RC1)

- `origin/<branch>` tenía commits ausentes en la rama local (local *behind*). Git rechaza con `force:false`, que es lo correcto.
- `Publicación remota` devuelve `failed` y `emit_dcc_phase_fractures` no tiene una aduana para esta firma. Resultado: emite `System_Fracture_Detected`, aunque es un estado **accionable por el operador**, no un colapso del Core.
- Es la misma clase de defecto que `F-DCC-DNS-UNRESOLVED`, `F-DCC-HOOK-EVOL-OVERESCALATION` y `F-DCC-WORKFLOW-SCOPE`: sobre-escalado de un bloqueo operativo a Kintsugi.

### Causa raíz 2 — Mayeuta: detector de bypass contaminado por stderr de git (C-RC2)

- El detector de bypass raw recorre `error_trace`, que es la salida de la propia cápsula oficial. Cualquier stderr de git o gh que cite comandos (`git push --help`, `use 'git pull'`, `gh auth …`) produce un diagnóstico de violación de jurisdicción.
- Tampoco existe una rama de diagnóstico para non-fast-forward.

## Alcance

### Dentro

1. **`delivery_close.rs`**
   - Añadir `dcc_non_fast_forward_trace(trace)`: firma independiente del locale, `non-fast-forward` **o** `(fetch first)`, junto con `[rejected]`.
   - Añadir `dcc_non_ff_block_suppresses_fracture(phase_name, status, trace)`, que solo aplica a `Publicación remota`.
   - Añadir `stamp_dcc_non_ff_block(entry, …)`: fija `status: blocked`, `friction_id: F-DCC-PUSH-NON-FAST-FORWARD` y un `operator_hint` que indique sincronizar vía `skill:git-manager` (`fetch` + `pull` de `origin <branch>`), resolver conflictos si los hay y relanzar DCC. Prohibido `git pull`/`git push` raw.
   - Enganchar la aduana en `emit_dcc_phase_fractures` y la estampa en el mismo punto que `stamp_dcc_network_block` y sus hermanas.
   - Mantener el halt existente de `Apertura en forja` / `Sello` / `Higiene` (`prior_push_not_ok`).
2. **`enrich_fracture_pbi_kaizen.rs`**
   - Excluir el detector de bypass raw cuando el token solo aparece dentro de stderr de git o gh. Mínimo: no disparar si `attempted_action` es una fase DCC y el trace procede de la cápsula (`git-manager`/`shell-executor`). Alternativa: evaluar los tokens de bypass sobre metadatos de acción del operador, no sobre `error_trace`.
   - Añadir una rama `is_non_fast_forward_trace` con la causa *"rama local detrás de origin; sincronizar vía git-manager"* y la propuesta `process_fix`. Sin `new_norm`.
3. **Genoma** (vía `execute-process.py` → `entity-manager`, sin edición manual)
   - `delivery-close-cycle.md` § Fase Publicación remota: documentar `F-DCC-PUSH-NON-FAST-FORWARD` (`blocked`, sin Kintsugi) y la remediación.
   - `obediencia-procesos.md`: añadir la entrada de escalado non-fast-forward, en paralelo a `F-DCC-WORKFLOW-SCOPE`, con la prohibición explícita de `git pull`/`git push --force` raw.
4. **Evolution**: entrada en `SddIA/evolution/` que vincule el UUID `5417c92c-da7f-4d46-b245-55cf1b17961a` (DCC) y este PBI, más su fila en `Evolution_log.md`.

### Fuera

- Auto-sincronización (fetch/rebase/merge) dentro de DCC antes del push. Integra cambios remotos sin laudo humano y puede producir conflictos o commits de merge no revisados. Si se quiere, va en un PBI propio.
- Cualquier uso de `force: true`.
- Averiguar qué introdujo commits en `origin/refactor/sddia-installer-v3-io-contract`: la rama ya está integrada y el origen no cambia el fix.

## Criterios de aceptación

- [ ] **CA1:** El trace de este PBI en `Publicación remota` produce `status: blocked`, `friction_id: F-DCC-PUSH-NON-FAST-FORWARD` y `operator_hint` presente. **No** se crea `System_Fracture_Detected` en `eda_bus.pending`. Test: `dcc_fracture_suppressed_on_push_non_fast_forward`.
- [ ] **CA2:** La variante `[rejected] … (fetch first)` se clasifica igual. Test dedicado.
- [ ] **CA3:** El non-fast-forward en una fase distinta de `Publicación remota` **no** se suprime. Test negativo.
- [ ] **CA4:** Tras el bloqueo, `Apertura en forja`, `Sello Presentación ECST` e `Higiene local` quedan `skipped` con `reason: prior_push_not_ok`. Esta regresión debe seguir pasando.
- [ ] **CA5:** La enriquecedora Kaizen, con el trace de este PBI, **no** emite *"Violación de jurisdicción delegada"* ni la propuesta `new_norm`, y **sí** emite la causa non-fast-forward. Test en `enrich_fracture_pbi_kaizen.rs`.
- [ ] **CA6:** La enriquecedora sigue detectando un bypass real cuando el token proviene de la acción del operador, no del stderr de la cápsula. Test positivo que mantiene la cobertura existente.
- [ ] **CA7:** `cargo test -p execute-process` en verde; `sddia-qa gate-evolution --json --range --sync-base` en verde; `verify-process-integrity` y `verify-tools-index` en verde.

## Criterio de cierre

- [ ] C-RC1 y C-RC2 resueltas (CA1–CA7)
- [ ] Argos APTO en `validacion.md` del fix (`pbi_archived: true`)
- [ ] Este TODO movido a `docs/todos/done/` en la misma rama del PR
