---
feature_name: dcc-simulated-barrier-c51acf014c0f
created: "2026-08-30"
process: bug-fix
base: main
scope: simulated-barrier-dcc-fracture-taxonomy
branch_name: fix/dcc-simulated-barrier-c51acf014c0f
persist_ref: docs/fixes/dcc-simulated-barrier-c51acf014c0f
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — barrera de fase simulated (c51acf014c0f).md
document_id: PBI-FIX-FRACTURE-c51acf014c0f
execution_id: "04d2fdeb-697d-4868-af44-f68840f6a5ca"
---

# Especificación — fractura `c51acf014c0f` (simulated ≠ cierre)

## Problema

En relevo IDE / sin `SDDIA_AGENT_RUNTIME_COMMAND`, las fases de agente de `bug-fix`/`feature`/`refactorization` resuelven `status: simulated`. `agent_phase_blocks_downstream` **no** incluye `simulated` → el orquestador encadena `Cierre documental` y `delivery-close-cycle` sobre un ciclo **sin cascada documental**.

Efectos encadenados:

| ID | Efecto |
|----|--------|
| F1 | Causa raíz: no hay punto de corte entre Diseño y Cierre |
| F2 | `Cierre documental` skip (`validacion.md ausente`) no corta el pipeline |
| F3 | `Apertura en forja` puede fallar sin `pr_url` y sin `gh_stdout`/`gh_stderr` |
| F4 | CI `wasi-runtime-smoke` corre sobre snapshot incompleto → `EVOL_MATERIAL_UNREGISTERED` |
| F4b | `emit_dcc_phase_fractures` escala `blocked` de aduana a `System_Fracture_Detected` (`c339de406e29`) |

## Reproducción (esta sesión)

`./sddia-run.sh --process bug-fix` + `SDDIA_AGENT_RELAY_IDE=1` + `SDDIA_LAB_ALLOW_DIRTY=1`:

- Diseño/Ejecución/Verificación: `simulated`
- Cierre documental: `executed` + `skipped: true` (`validacion.md ausente`)
- DCC encadenado: snapshot `b0a4bde`, rama empujada, [PR #231](https://github.com/racso80es/SddIA/pull/231)
- DCC `Aduana evolution` **failed** (material sucio de instancia/genoma en el snapshot)
- `bug-fix` terminal: `Cierre de entrega` failed
- Hook pre-push **ignorado** (no ejecutable) — aduana local dormida

El snapshot prematuro arrastró `.SddIA/systemd/*`, logs, cola DLT y mutaciones de genoma (`SddIA/daemons/*`, `SddIA/tools/*`) ajenas al fix.

## Cambio requerido

### F1 — Barrera (motor, no DA-2)

`SddIA/engine/execute-process/src/engine/executor.rs`:

| Condición | Comportamiento |
|-----------|----------------|
| Proceso `bug-fix` / `feature` / `refactorization` | Si alguna fase `agent:` queda `simulated` **y** no existe `{persist_ref}/validacion.md` | **No** ejecutar `Cierre documental` ni `Cierre de entrega` |
| Acuse | `status: awaiting_agents` (o `detached`) en esas fases; envelope `success` limpio | El mandato «detente tras el plan» es representable |
| `simulated` + `validacion.md` presente | Permitir cierre (Argos ya escribió) | No castrar DCC de un ciclo ya verificado |

`agent_phase_blocks_downstream("simulated")` pasa a **true** cuando el proceso exige cascada documental, **o** el bucle de fases aplica la guarda anterior sin ampliar el predicado global a todos los procesos (evitar romper labs que usan `simulated` en procesos sin DCC).

Preferencia: predicado **acotado** a procesos con fase `Cierre de entrega` / `delivery-close-cycle`.

### F2 — Skip documental con corte

Si `feature-pbi-archive` skip por `validacion.md ausente`:

- `status` no debe ser «OK silencioso» hacia DCC.
- Tras F1, DCC no se invoca. Si F1 no aplica (DCC directo): el skip debe **bloquear** `Cierre de entrega` o emitir señal explícita (`awaiting` / evento de telemetría). **No** `System_Fracture_Detected`.

### F3 — Telemetría de forja

`capsule_delivery_gh_pr`: al fallar `pr_url`, el `Err` incluye fragmentos truncados de `gh_stdout`/`gh_stderr`. No cambia la semántica de éxito.

### F4 — Aduana local activa (refinado: implementable este ciclo)

F1 elimina el push prematuro (palanca principal contra la recurrencia de CI). Auditoría de la recaída `EVOL_HASH_MISMATCH` (correlato `a3c51acf`, PR #231) precisa la causa de que la aduana local **no** atrape errores evolution antes de CI:

| # | Defecto | Naturaleza | Clasificación |
|---|---------|-----------|---------------|
| D1 | Dispatchers `pre-commit`/`pre-push`/`post-merge` versionados **sin bit ejecutable** (`-rw-rw-r--`) → git los ignora | Versionado, por-clon | `SddIA/scripts/` (no genoma) |
| D2 | `verify_hooks.rs` comprueba `is_file()`, **no** el bit `+x` → reporta "hooks OK" en falso | Ceguera del verificador | `SddIA/tools/sddia-qa/src/` (motor QA, no entidad `.md`) |
| D3 | `verify-hooks` **reporta** el remedio pero no lo **arma** (armado solo en `start-sddia.sh`) | Punto único de fallo | `SddIA/tools/sddia-qa/src/` |
| D4 | Gate evolution no corre en ruta operador: `pre_commit_gate.sh` no lo invoca; `pre_push_gate.sh` en push de rama delega en DCC (auto-eximido `in_delivery_close_cycle`) | Cobertura nula del gate local | `SddIA/scripts/` (no genoma) |

**Alcance implementable (F4a/F4c) — sin mutación de genoma `.md`:**

- **F4a (D1):** versionar bit ejecutable de los tres dispatchers vía `git update-index --chmod=+x`. Persiste por-clon en el árbol; despierta la aduana de forma versionada.
- **F4a (D2/D3):** `verify_hooks.rs` — comprobar bit ejecutable (no solo `is_file()`); nuevo modo `verify-hooks --fix` que **arma** idempotente: `git config core.hooksPath` + `chmod +x` dispatchers. Sin `--fix` conserva semántica de reporte, pero añade finding si falta `+x`.
- **F4c (D4):** `pre_push_gate.sh` — correr `run_evolution_gate` (rango `origin/main...HEAD`, `--if-touched --sync-base`) **antes** de la rama DCC, para todo push que toque material bajo `SddIA/evolution/` o material sin correlato. Atrapa `EVOL_HASH_MISMATCH` / `EVOL_MATERIAL_UNREGISTERED` localmente, no en CI.

**Diferido (AEL-CA9, requiere `entity-manager`):** declarar `gate-evolution` como **fase** del genoma `delivery-close-cycle.md` (mutación de entidad normada + recálculo `hash_signature`). Complementa F4c en la ruta del propio motor; fuera de este PR.

### F4b — Discriminar gate vs colapso

`emit_dcc_phase_fractures` (`delivery_close.rs`):

| Fase / condición | Emite `System_Fracture_Detected` |
|------------------|----------------------------------|
| `Aduana evolution` `blocked` + `EVOL_MATERIAL_UNREGISTERED` | **No** |
| `Aduana EDA genómica` `blocked` + orphans (gate) | **No** |
| `failed` de cápsula / runtime / forja no-determinista | **Sí** |
| `fail_soft: true` | No (ya excluido) |

Conservar `status: blocked` y `reason_codes`. Opcional: telemetría fractal, no dominio.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | `bug-fix` con fases agente `simulated` y sin `validacion.md` **no** invoca `delivery-close-cycle` |
| CA-2 | Acuse JSON: fases de cierre `skipped`/`awaiting_agents`; `exitCode` 0 (relevo IDE, no colapso) |
| CA-3 | Skip `validacion.md ausente` no permite `Cierre de entrega` |
| CA-4 | Error de `pr_url` incluye stdout/stderr de `gh` (truncado) |
| CA-5 | `emit_dcc_phase_fractures` no escribe `System_Fracture_Detected` ante `EVOL_MATERIAL_UNREGISTERED` en Aduana evolution |
| CA-6 | Test unitario: `agent_phase_blocks_downstream` / bucle — `simulated` barre DCC en `bug-fix` |
| CA-7 | Test unitario: emit fracture — blocked evolution sin fichero pending de dominio |
| CA-8 | Cascada spec/plan/implementation/execution/validacion APTO; PBI en `done/` en el mismo PR |
| CA-9 (F4a) | Dispatchers `pre-commit`/`pre-push`/`post-merge` con bit `+x` versionado (`git ls-files --stage` → modo `100755`) |
| CA-10 (F4a) | `verify-hooks` reporta finding si falta `+x`; `verify-hooks --fix` arma `core.hooksPath` + `chmod +x` idempotente (exit 0 en segunda pasada) |
| CA-11 (F4a) | Test unitario `verify_hooks`: detecta ausencia de `+x` y remedia con `--fix` |
| CA-12 (F4c) | `pre_push_gate.sh` corre `gate-evolution --range --if-touched --sync-base` sobre `origin/main...HEAD` antes de la rama DCC; push con hash evolution inválido → BLOCKED local |

## Fuera de alcance (este PR)

- **AEL-CA9:** mutación de `SddIA/library/codexes/.../delivery-close-cycle.md` vía `entity-manager` (declarar `gate-evolution` como fase de proceso). Requiere forja de genoma + `hash_signature`; ciclo posterior con laudo.
- Higienizar PR #231 / snapshot `b0a4bde` (instancia + genoma ajeno) — resuelto vía revert en la rama; no reintroducir.
- Reabrir DCC sobre este ciclo mientras la cascada documental (`validacion.md`) no esté presente (F1 lo impide por diseño).
- Fractura DLT `b3a715381787` / `701c77ebeab8`.
