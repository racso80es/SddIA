---
feature_name: dcc-simulated-barrier-c51acf014c0f
created: "2026-08-30"
process: bug-fix
phases:
  - barrier-simulated
  - skip-doc-cuts-dcc
  - emit-dcc-fracture-taxonomy
  - forge-telemetry
  - verify-unit
  - document-and-stop-for-laudo
branch_name: fix/dcc-simulated-barrier-c51acf014c0f
persist_ref: docs/fixes/dcc-simulated-barrier-c51acf014c0f
---

# Plan — fractura `c51acf014c0f`

Corte de esta entrega: **Diseño (spec + plan) + commit**. Sin implementación de código. **No** re-lanzar `delivery-close-cycle` (PR #231 ya abierto por F1; DCC falló en Aduana evolution).

## Fase 1 — Barrera `simulated` (F1, CA-1/CA-2/CA-6)

Motor `executor.rs` (no genoma DA-2):

1. Detectar proceso con cierre encadenado (`bug-fix` / `feature` / `refactorization`).
2. Si una fase `agent:` reporta `simulated` y `{persist_ref}/validacion.md` no existe:
   - marcar `barrier_prior` (ampliar `agent_phase_blocks_downstream` **o** guarda local al bucle);
   - fases `Cierre documental en rama` y `Cierre de entrega` → `skipped` / `awaiting_agents`, `reason: prior_agent_phase_not_executed` (o `simulated_relay_no_validacion`).
3. Envelope raíz `success: true` (relevo IDE no es colapso).
4. Test: fixture `bug-fix` con tres fases simulated + cierre → DCC **no** invocado; `delivery_close` ausente en `data`.

No aplicar `simulated` como barrera global a procesos sin cascada documental.

## Fase 2 — Skip documental (F2, CA-3)

`phase_capsules.rs` / `feature-pbi-archive`:

- Tras skip `validacion.md ausente`, el estado debe ser barrera para `Cierre de entrega` si F1 no cubrió (DCC invocado a pelo).
- Preferible: F1 hace redundante el camino; F2 es red de seguridad.
- Sin `System_Fracture_Detected`.

## Fase 3 — Taxonomía de fractura DCC (F4b, CA-5/CA-7)

`delivery_close.rs` `emit_dcc_phase_fractures`:

1. Si `phase_name` ∈ {`Aduana evolution`, `Aduana EDA genómica`} y `status == blocked` → **return** (no emitir dominio).
2. Conservar emisión para `failed` de forja/cápsula y otros `blocked` no-aduana.
3. Test: reutilizar patrón `dcc_fracture_emits_on_blocked_phase` — caso evolution blocked → pending **sin** `System_Fracture_Detected`; caso `Apertura en forja` failed → sí emite.

## Fase 4 — Telemetría `gh` (F3, CA-4)

`capsule_delivery_gh_pr`: concatenar stdout/stderr truncados al `Err` de `pr_url`. Cambio acotado.

## Fase 4.5 — Aduana local activa (F4a/F4c, CA-9…CA-12)

Refinado tras auditoría de la recaída `EVOL_HASH_MISMATCH`. Sin mutación de genoma `.md`.

1. **F4a — bit ejecutable versionado (D1):** `git update-index --chmod=+x SddIA/scripts/qa/git-hooks/{pre-commit,pre-push,post-merge}`. Verificar con `git ls-files --stage` → `100755`.
2. **F4a — verify-hooks activo (D2/D3):** `SddIA/tools/sddia-qa/src/verify_hooks.rs`:
   - comprobar bit `+x` de cada dispatcher (metadata `st_mode & 0o111`), añadir finding si falta;
   - modo `--fix`: `git config core.hooksPath SddIA/scripts/qa/git-hooks` + `chmod +x` dispatchers; idempotente.
   - `main.rs`: enrutar `verify-hooks --fix`.
   - Test unitario: fixture sin `+x` → finding; con `--fix` → exit 0 en segunda pasada.
3. **F4c — gate evolution en pre-push (D4):** `SddIA/scripts/qa/git-hooks/pre_push_gate.sh`: correr `run_evolution_gate` (ya definido) sobre `origin/main...HEAD` **antes** del bucle de ramas DCC, cuando el rango toque `SddIA/evolution/` o material sin correlato. Conservar guarda `in_delivery_close_cycle` para el push del propio orquestador (no duplicar con AEL-CA9).

Verificación:

```text
cd SddIA && cargo test -p sddia-qa verify_hooks
git ls-files --stage SddIA/scripts/qa/git-hooks/pre-push   # 100755
SddIA/target/debug/sddia-qa verify-hooks --json            # findings=[]
```

## Fase 5 — Verificación

```text
cd SddIA && cargo test -p execute-process agent_phase_blocks
cd SddIA && cargo test -p execute-process emit_dcc_phase_fractures
cd SddIA && cargo test -p execute-process evolution_phase_blocks
```

Argos posterior: `validacion.md` CA-1…CA-8. PBI a `docs/todos/done/` en la misma rama.

## Fase 6 — Cierre de entrega

Solo tras F1 en código + `validacion.md` APTO. **No** tratar `b0a4bde` / PR #231 como Done. Push incremental a `fix/dcc-simulated-barrier-c51acf014c0f`. Higienizar snapshot de instancia/genoma ajeno **antes** del merge (commit revert o no incluir esos paths en el PR final).

## Diferido (laudo / ciclo propio)

- AEL-CA9: extraído a `PBI-KAIZEN-AEL-CA9-DCC-EVOLUTION-PHASE`.
- Higiene operativa de #231 (`systemd` instance, logs, tools/daemons en el snapshot) — resuelta vía revert en rama.

## Orden

```text
spec/plan (este commit)
  → laudo Vértice Biológico
    → Fase 1 (barrera)  [bloquea F4]
      → Fase 3 (F4b emit)
        → Fase 2 (red de seguridad)
          → Fase 4 (telemetría)
            → Fase 5 tests + validacion + PBI done/
              → Fase 6 DCC con agentes ejecutados / validacion presente
```
