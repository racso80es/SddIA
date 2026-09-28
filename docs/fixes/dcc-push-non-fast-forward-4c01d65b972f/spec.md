---
feature_name: dcc-push-non-fast-forward-4c01d65b972f
created: "2026-09-28"
process: bug-fix
base: main
scope: dcc-non-ff-kintsugi-overescalation
branch_name: fix/dcc-push-non-fast-forward-4c01d65b972f
persist_ref: docs/fixes/dcc-push-non-fast-forward-4c01d65b972f
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (4c01d65b972f).md
document_id: PBI-FIX-FRACTURE-4c01d65b972f
fracture_hash: 4c01d65b972f
execution_id: "55be0f6b-b30f-47b4-a8df-5853e2b5d0cf"
---

# Especificación — fractura `4c01d65b972f` (push non-fast-forward ≠ Kintsugi)

## Problema

`delivery-close-cycle` fase **Publicación remota** con push oficial (`skill:git-manager`, `force: false`) rechazado por `non-fast-forward` escala a `System_Fracture_Detected`. Es bloqueo operativo (rama local detrás de `origin`), misma clase que DNS / evolution-gate / workflow-scope.

Mayeuta (C-RC2) quedó resuelto en Kaizen `PBI-KAIZEN-MAYEUTA-PRECISION-DIAGNOSTICA` (`fracture-signatures.json` + corpus `4c01d65b972f`). Este fix cierra **C-RC1**: supresión DCC + sello `F-DCC-PUSH-NON-FAST-FORWARD`.

## Reproducción

Trace canónico del PBI (Publicación remota, `failed`):

```text
SddIA pre-push: SKIPPED (delivery-close-cycle guard)
 ! [rejected]        branch -> branch (non-fast-forward)
error: falló el empuje de algunas referencias
```

## Cambio requerido

| ID | Entrega |
|----|---------|
| F1 | Catálogo `F-DCC-PUSH-NON-FAST-FORWARD`: `dcc_suppress` + `fracture_policy: suppress` + `operator_hint` |
| F2 | `delivery_close.rs`: suppress + `stamp_dcc_non_ff_block` en ruta de fase y `emit_dcc_phase_fractures` |
| F3 | Tests CA1–CA4 + regresión Kaizen CA3 (`fracture_corpus_regression`) |
| F4 | Genoma DCC + `obediencia-procesos.md` § escalado non-FF (vía `entity-manager` / forja aplicable) |
| F5 | Evolution UUID DCC `5417c92c-da7f-4d46-b245-55cf1b17961a` + PBI |

## Fuera de alcance

Auto fetch/rebase en DCC; `force: true`; re-diseño del detector bypass Mayeuta (ya en catálogo).

## Criterios de aceptación

Ver PBI `PBI-FIX-FRACTURE-4c01d65b972f` CA1–CA7.
