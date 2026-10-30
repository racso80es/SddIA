---
feature_name: fracture-provider-sink
created: "2026-10-30"
process: refactorization
branch: refactor/fracture-provider-sink
pbi_ref: AUT-30
tracker_ref: AUT-30
execution_id: "478e05bc-92e4-47e8-a473-fe6a4ac1c4db"
---

# Objetivos — fracture-provider-sink

## Misión

`materialize-fracture-pbi` elige el sumidero del manifiesto ya resuelto. Sin `project_slug`, el índice auto-hospedado (`project_root` `.`) aporta el proveedor. `linear` crea la issue por `tracker:lifecycle` con el cuerpo `build_pbi_body`, título igual al H1, etiquetas `pbi` y `fix`, estado backlog. El dedupe remoto es `fracture_hash` en la descripción. Un markdown local no veta el alta. Hash solo en Done abre regresión. `Unconfigured` no escribe fichero.

## Alcance

- Puerto: `Unconfigured` deja de ser alias de fichero.
- Alta remota y hecho `Fracture_Pbi_Materialized` para que `enrich-fracture-pbi-kaizen` comente esa referencia.
- Route: una acción o tool declarada se ejecuta aunque el agente sea cognitivo y el evento viva en redb.
- El deny no ignora el fallo de depósito. El watcher deja constancia si `events.redb` no abre y suelta el store antes de `route-domain-event`.

## Fuera

Manifiesto sin objeto `tracker` sigue `Unconfigured` (test vigente). Título de `forge-pbi` (AUT-28). Candado post-`create_issue` (AUT-29). Renombrar issues ya abiertas.

## Ley aplicada

- `todos-jurisdiction` v1.1.1: el sumidero lo elige el provider.
- Un commit de entrega vía `git-manager` cuando se cierre la ejecución.
