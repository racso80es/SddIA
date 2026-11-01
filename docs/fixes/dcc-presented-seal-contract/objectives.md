---
feature_name: dcc-presented-seal-contract
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-presented-seal-contract
pbi_ref: AUT-48
tracker_ref: AUT-48
document_id: PBI-4a0c629e
execution_id: "9e85f016-c9be-46de-af20-92455e7817b8"
---

# Objetivos — dcc-presented-seal-contract

## Misión

Cerrar el residuo de AUT-48 en HEAD `31ec7ac4`: el genoma de `delivery-close-cycle` sigue diciendo que `pr_url` del Sello Presentación es opcional. El handler ya exige URL y, si falta, queda `blocked`.

## Alcance

- `delivery-close-cycle.md` (códice software-engineering), fase Sello Presentación ECST, vía `entity-manager`.
- Nota en `SddIA/evolution/` vinculada a AUT-48.

## Fuera

- `capsule_delivery_emit_presented`, `resolve_pr_url_for_presented_seal` y la firma `186c785103d8` (AUT-31).
- Reinyectar el DCC que originó la fractura. Inventar `pr_url`.

## Ley aplicada

- Git vía `skill:git-manager` en la entrega. Esta sesión no hace checkout: `SDDIA_LAB_SKIP_GIT=1`. HEAD es `Fix_Events_Multiple`.
- El motor nombra `fix/dcc-presented-seal-contract`. Esa rama no se abre.
- El proceso es genoma (`process_domain_roots`). Prohibido `Write` IDE sobre el `.md`.
- Provider `linear`: cierre en el issue, sin fichero nuevo en `docs/todos/pending/` ni `docs/todos/done/`.
