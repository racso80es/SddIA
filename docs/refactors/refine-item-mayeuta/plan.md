---
feature_name: refine-item-mayeuta
created: "2026-10-27"
process: refactorization
tracker_ref: AUT-19
execution_id: "51caf6b7-c386-447b-a380-edf5e8c97d73"
---

# Plan — refine-item

## Fases

1. [x] T0 Encuadre `refactorization` + documentación Mayeuta/Dedalo (objectives, clarify, spec).
2. [x] T1 Motor `handlers/refine_item` + alias `PROCESS_RENAMED` + timeout refinamiento.
3. [x] T2 Forja genoma (`entity-manager`): norma DoR, `refine-item` códice, baja `refine-pbi`/`refine-hu`, eventos, codex 1.1.0.
4. [x] T3 Consumidores (cursorrules, hooks, gate/ecst) + `hu` `create_issue` en `refine-item`. E2E lab sigue usando `handlers/refine` legacy hasta migración AC.
5. [x] T4 `validacion.md` APTO + DCC + AUT-19 Done (Linear tras merge).

## PR

https://github.com/racso80es/SddIA/pull/404
