---
feature_name: event-bus-audit-store-signals
created: 2026-10-31
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/event-bus-audit-store-signals
tracker_ref: AUT-27
execution_id: 6cd0a99f-1d58-43ee-8e30-31e9e13a090b
global: APTO
pbi_archived: false
branch: Fix_Events_Multiple
approval_status: aprobado
verdict: aprobado
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
checks:
  AUT27-T1-STORE-UNAVAILABLE: APTO
  AUT27-T2-CATALOG-CROSS: APTO
  AUT27-T3-EFECTO-RESIDUO: APTO
  AUT27-T4-REGRESSION: APTO
  AUT27-LIVE-REDB-PROBE: APTO
git_changes:
- SddIA/tools/event-bus-audit/src/main.rs
- docs/fixes/event-bus-audit-store-signals/
- docs/audits/AUT-27.md
validated_head_sha: 5694e1f6348313cf9d2ed480fe78e19541bc826c
validated_tree_sha: d50b36af50fc198da776db33287c70ba14896560
diff_digest: sha256:c0b4d4f5ea07ed2983bcd69e199a4ec738633dcd36ba72ab4163d223ace5ffb2
---

# Validación — AUT-27

## Veredicto

**APTO** — En modo `redb`, `event-bus-audit` deja de devolver un escaneo mudo: señala `store_unavailable`, cruza catálogo ↔ `subscriber_delivery`, marca `success_sin_efecto` y inventaría pending residual con fractura asociada.

## Evidencia

| Check | Medio | Resultado |
|-------|--------|-----------|
| T1 | Test `store_unavailable_emits_anomaly_not_silent_empty_scan` + sección informe | OK |
| T2 | Test `redb_audit_catalog_mismatch_on_presented` (ids `argos`/`cumulo` + `suscriptor_sin_fila`) | OK |
| T3 | Test `redb_audit_marks_success_sin_efecto_and_tracker_outbound_shield` + `redb_audit_residual_pending_with_fracture_trace` | OK |
| T4 | `cargo test -p event-bus-audit` | 13/13 OK |
| Live redb | `SDDIA_EVENT_STORE=redb`, store `.SddIA/runtime/events.redb` abre sin `store_unavailable`; señales `delivery_fuera_de_catalogo` (18) y `residual_filesystem` (2) | OK |

## Casos históricos del PBI

Los `event_id` de la descripción de AUT-27 (`3803aeae-7a1e-4c15-b02f-8c72d16fd8ff`, `BVLgK63NjKvYhH1Zt4LGPE2KbrSCAhQaBTnJmfng8xCU`) **no** están en el store actual; la regresión los fija en fixtures. En el store vivo hay `PullRequest_Merged` / `PullRequest_Presented` recientes con ids de catálogo en `delivery_state` (p. ej. `16ad7b2f-aaae-486f-875e-6653d3542ae2`); la auditoría no inventa falsos positivos cuando las filas de entrega ya coinciden con el plan.

## Cierre tracker

Provider `linear`: **AUT-27** → Done con el PR que incluya este diff; sin fichero en `docs/todos/done/`.