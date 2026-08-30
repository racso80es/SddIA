---
feature_name: dcc-dns-unresolved-d0cfd5b66ff1
created: "2026-08-30"
process: bug-fix
phases:
  - classify-transient-network
  - suppress-dcc-net-fracture
  - stamp-friction-id
  - fix-kaizen-blob
  - verify-unit
  - document-and-stop-for-laudo
branch_name: fix/dcc-dns-unresolved-d0cfd5b66ff1
persist_ref: docs/fixes/dcc-dns-unresolved-d0cfd5b66ff1
---

# Plan — fractura `d0cfd5b66ff1`

Corte de **esta** entrega: **Diseño (spec + plan) + commit**. Sin implementación de código. **No** `delivery-close-cycle` (DNS F1 sigue activo en este host; DCC reventaría otra vez en push).

La implementación (fases 1–5) queda para el siguiente estímulo Tekton sobre esta rama.

## Fase 1 — Clasificador de red transitoria (F2, CA-1/CA-2)

`delivery_close.rs`:

```text
fn dcc_transient_network_trace(trace: &str) -> bool
```

`trace.to_lowercase()` contiene alguno de: `could not resolve host`, `temporary failure in name resolution`, `name or service not known`, `network is unreachable`, `connection timed out`.

Test unitario del predicado: positivos DNS/ES/EN; negativo `no se pudo resolver pr_url desde gh`.

## Fase 2 — Supresión F4c (F2, CA-1/CA-2/CA-4)

```text
fn dcc_net_block_suppresses_fracture(phase_name, status, error_trace) -> bool
```

True iff:

- `phase_name` ∈ {`Publicación remota`, `Apertura en forja`}
- `status` ∈ {`failed`, `blocked`}
- `dcc_transient_network_trace(error_trace)`

En `emit_dcc_phase_fractures`: `continue` si F4b **o** F4c. No tocar F4b.

Tests junto a `dcc_fracture_suppressed_on_evolution_gate_block` / `dcc_fracture_emits_on_failed_forge_phase`:

| Fixture | Pending `System_Fracture_Detected` |
|---------|-------------------------------------|
| Publicación remota + `Could not resolve host: github.com` | vacío |
| Apertura en forja + `Could not resolve host` | vacío |
| Apertura en forja + `no se pudo resolver pr_url desde gh` | **emite** (regresión CA-4) |
| Aduana evolution blocked | vacío (F4b) |

## Fase 3 — Sello accionable (F2, CA-3)

Tras `Err(e)` en el handler de fase DCC (~L188), si F4c aplica:

- `entry["status"] = "blocked"` (no `fail_soft`)
- `entry["friction_id"] = "F-DCC-DNS-UNRESOLVED"`
- conservar `entry["error"]`

Agregador terminal: DCC sigue `success: false` (sin `pr_url` / `delivery_push`). Test de envelope o de `aggregate_execution_terminal` si ya hay patrón; si no, aserción sobre el report.

No mutar `unwrap_git_manager_body` ni emitir `offline` en push.

## Fase 4 — Mayeuta matcher (F3, CA-5/CA-6)

`enrich_fracture_pbi_kaizen.rs` `analyze_fracture_kaizen`:

1. Blob de la regla hook = `error_trace` + `attempted_action` (sin `process_name`).
2. Quitar `"delivery-close"` del array unario.
3. Otras reglas pueden seguir usando el blob completo si no colisionan; no ampliar alcance.

Tests:

- Nuevo: `analyze_fracture_kaizen_dns_not_hook_recursion` — process `delivery-close-cycle`, traza DNS, acción `Publicación remota` → sección **sin** «Recursión o re-entrada».
- Existente `analyze_fracture_kaizen_recursion_verdict` sigue verde (`pre-push hook blocked`).

## Fase 5 — Verificación

```text
cd SddIA && cargo test -p execute-process dcc_fracture
cd SddIA && cargo test -p execute-process analyze_fracture_kaizen
cd SddIA && cargo test -p execute-process dcc_transient
```

No `sddia-qa` extra. No genoma. Evolution se registra en el ciclo de implementación, no en este corte.

## Fase 6 — Este corte (hecho al commit)

1. `objectives.md` / `spec.md` / `plan.md` bajo `persist_ref`.
2. Commit vía `git-manager` (`files` = esos tres paths).
3. Detener. Sin `implementation.md`, sin tests aplicados, sin DCC, sin push.

## Fuera de este corte

Código motor, tests, evolution, validacion, archivo PBI, PR.
