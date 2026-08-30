---
feature_name: ael-ca9-dcc-evolution-phase
created: "2026-08-30"
process: bug-fix
phases:
  - capsule-sync-base
  - hook-delegate-topology
  - genome-entity-manager
  - verify-unit
  - document-and-stop-for-laudo
branch_name: fix/ael-ca9-dcc-evolution-phase
persist_ref: docs/fixes/ael-ca9-dcc-evolution-phase
---

# Plan — AEL-CA9 residual

Corte de esta entrega: **Diseño (spec + plan) + commit**. Sin implementación de código. **No** re-lanzar `delivery-close-cycle` (PR #232 abierto por init prematuro; snapshot solo `objectives.md`).

## Fase 1 — Paridad `--sync-base` (R2, CA-4)

`SddIA/engine/execute-process/src/engine/phase_capsules.rs` — `capsule_evolution_audit_gate`:

```text
.args(["gate-evolution", "--json", "--range", "--sync-base"])
```

Sin `--if-touched`. Sin `--require-synced-base`. Test: args de spawn o fixture que aserte `--sync-base` en la línea de comando (extender `evolution_audit_ca12_tests` si el spawn es mockeable; si no, test de integración acotado / assert de slice de args extraído a fn `evolution_gate_args()`).

No genoma.

## Fase 2 — Hook delega (R1, CA-1/CA-2/CA-3/CA-7)

`SddIA/scripts/qa/git-hooks/pre_push_gate.sh`:

1. Conservar `skip_hooks` y `in_delivery_close_cycle` al inicio.
2. Parsear stdin + guarda `main` (igual que hoy).
3. **Si** `${#branches[@]} -eq 0` → `run_evolution_gate` → exit.
4. **Si** `${#branches[@]} -gt 0` → **no** `run_evolution_gate`; seguir al bucle DCC.

Hoy el gate está **antes** del `if #branches == 0` (línea 52) → siempre corre. Moverlo **dentro** de esa rama vacía.

Test CA-7: extraer la condición a comentario+función bash testeable, o script de fixture que simule `#branches` y cuente invocaciones a `sddia-qa`. Preferible: test que documente el predicado en `hook_common` / comentario de contrato en `execution.md` si no hay harness bash; no inventar framework.

## Fase 3 — Genoma (R3, CA-5/CA-6) — laudo previo

```text
./sddia-run.sh --process entity-manager --inputs '{... delivery-close-cycle ...}'
```

Domain root: `codex-software-engineering/process/`. Recalc `hash_signature`. Bump 1.3.0 → 1.4.0.

Cuerpo: nota Aduana evolution (`--sync-base`); anti-recursión `SDDIA_HOOK_DELIVERY_CLOSE`; lab skip; Impacto tri-proceso.

**Prohibido** `StrReplace` sobre `delivery-close-cycle.md`.

## Fase 4 — Verificación (tras código, no este commit)

```text
cd SddIA && cargo test -p execute-process evolution_audit
# predicado hook: revisión de pre_push_gate.sh + CA-1/CA-2 en execution.md
SddIA/target/debug/sddia-qa verify-process-integrity
```

Argos: `validacion.md` CA-1…CA-8. PBI → `docs/todos/done/` en la misma rama.

## Fase 5 — Cierre de entrega

Solo tras R1+R2 en código (+ R3 si laudo) y `validacion.md` APTO. **No** tratar `9e24dc8` / PR #232 como Done. Push incremental a `fix/ael-ca9-dcc-evolution-phase`.

## Diferido (fuera de este PR de diseño)

- Binario `execute-process` fósil vs fuente F1/F2 (esta invocación encadenó DCC). Rebuild de sesión, no este PBI.
- Higienizar #232 (snapshot `objectives.md` solo).

## Orden

```text
spec/plan (este commit)
  → laudo Vértice Biológico
    → Fase 1 (R2 cápsula)
      → Fase 2 (R1 hook)
        → Fase 3 (R3 entity-manager)  [requiere laudo]
          → Fase 4 tests + validacion + PBI done/
            → Fase 5 DCC con validacion presente
```
