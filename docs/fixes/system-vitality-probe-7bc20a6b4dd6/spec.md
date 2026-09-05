---
feature_name: system-vitality-probe-7bc20a6b4dd6
created: "2026-09-05"
process: bug-fix
base: main
branch: fix/system-vitality-probe-7bc20a6b4dd6
uuid: db46c34e-4c2d-42dd-b2e1-36230853f23c
scope: laudo-b-documental-no-regresion
document_id: PBI-FIX-FRACTURE-7bc20a6b4dd6
fracture_hash: 7bc20a6b4dd6
verdict: B-documentary-debt
execution_id: "777857be-0814-4923-ad64-dd29f7942962"
physical_fix_commit: "ab272346cd77a9cb9fd320d177179086409ae6ce"
physical_fix_pr: "https://github.com/racso80es/SddIA/pull/251"
related_fixes:
  - docs/fixes/dcc-sddia-qa-lab
  - docs/fixes/centinelas-fracture-ola-20260901
  - docs/features/latido-ontologico-vitalidad-organos
---

# Spec — system-vitality-probe-7bc20a6b4dd6

## Decisión

Laudo **(B) deuda documental**. La sonda `cumulo.tools_index` operó según diseño: ELF `sddia-qa` ausente. La ignición omitía `-p sddia-qa`. Ese diff ya está en `main` vía commit `ab27234` y PR **#251** (`fix/ignition-pre-push-guard`, feature `dcc-sddia-qa-lab`).

Este ciclo no reimplementa ignición. Archiva `PBI-FIX-FRACTURE-7bc20a6b4dd6`, certifica no-regresión y deja correlato evolution.

`plan.md` **no** se emite: no hay blueprint de proceso.

## Fuera de alcance

- Mutar `SddIA/process/system-vitality-probe.md`, `system_vitality.rs`, eventos, `start-sddia.sh`.
- Fusionar este `document_id` con Ola 2 DCC (`ca3d901fdc9a-OLA2`) o con `centinelas-fracture-ola-20260901`.
- Atribuir la resolución física a PR #248 o #249 (H6/H7 del PBI v1.2.0).

## Discriminación A vs B

| Hecho | Evidencia | Lectura |
|-------|-----------|---------|
| Causa en ignición | `ab27234` añade `sddia-qa` a `release_pkgs` y lote debug | Ya en `main` |
| PR canónico | [#251](https://github.com/racso80es/SddIA/pull/251) merge `2026-09-04T09:19:16Z` | No #248/#249 |
| Segregación | `centinelas-fracture-ola-20260901` CA5: este PBI intacto en `pending/` | Ciclo aparte |
| Runtime | ELF debug+release; `verify-tools-index: OK`; estado `cumulo.tools_index` green | Sin colapso activo |

Gate empírico (CA1) en ejecución: si tumba el laudo → pivot (A) y detener archivo.

## Cambios (Tekton)

1. Re-verificar CA1 (binarios, QA, `system-vitality-probe`, estado de sonda).
2. Archivar PBI `pending/` → `done/`, `status: cerrado`, `fix_ref: docs/fixes/system-vitality-probe-7bc20a6b4dd6`.
3. `implementation.md` + `execution.md` + `validacion.md`.
4. Evolution `db46c34e-4c2d-42dd-b2e1-36230853f23c` vía `sddia-qa evolution-register`.
5. Cierre de entrega: `delivery-close-cycle`. CA6 (CI) no es `APTO` sin run verde.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| VITALITY-DOC-CA1 | No-regresión: ELF `sddia-qa` debug+release; `verify-tools-index` OK; probe `verdict: ok`; `cumulo.tools_index` green |
| VITALITY-DOC-CA2 | Linaje PR #251 + `ab27234` + `dcc-sddia-qa-lab`; cero #248/#249 como resolución |
| VITALITY-DOC-CA3 | PBI en `docs/todos/done/` con `document_id` intacto y `fix_ref` de este ciclo |
| VITALITY-DOC-CA4 | `validacion.md` `global: APTO`, `pbi_archived: true` (CA6 no cuenta hasta run CI) |
| VITALITY-DOC-CA5 | Diff sin genoma ni `start-sddia.sh` |
| VITALITY-DOC-CA6 | Checks GitHub del PR verdes (`PENDIENTE-CI` hasta `run_id`) |
