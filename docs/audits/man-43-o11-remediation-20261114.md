# MAN-43 O11 — Remediation forja O10 (2026-11-14)

**Veredicto:** `APTO_PARCIAL` (genoma recuperado; C-4 trazabilidad parcial; C-3/C-5 abiertos).

## Ejecutado

- Motor: anti-sobrescritura tool/skill `update`; `implementation_source_git_apply` en norm-forge; delete tool/skill elimina fila en `index.md`.
- `entity-manager`: restauración 3 `.md` desde `c60ae233`; delete 2 stubs; `hash_refresh` 4 daemons; 16× reaplicación Rust O9; `hash_refresh` taxonomía (contenido 2.0.0 ya presente desde O10).

## UUIDs falsos O10 (residuo bus)

Entidades sobrescritas por forja obsoleta (referencia para compensación humana si aplica):

| Entidad | UUID falso (O10) | UUID canónico (`c60ae233`) |
|---------|------------------|----------------------------|
| `shell-executor` | `c1f79172-b746-4e30-9eec-7abbcc2dd53b` | `93d23720-d79a-412f-a85d-ab9b2d9862bd` |
| stubs eliminados | `69582707-…` / `179ab3e1-…` | (sin `.md` en `c60ae233`) |

Sello O11: `Domain_Entity_Updated` con UUID canónico en restauraciones; `Domain_Entity_Deleted` en stubs.

## C-4

El parche bifásico (`em-c4-forge-v2.json`) falló porque el catálogo ya estaba en 2.0.0. Cierre operativo: `hash_refresh_only` + `Domain_Entity_Updated`. **Pendiente laudo:** prosa/Directriz 2.0 sellada solo parcialmente respecto al story «dos updates por entity-manager».

## Pendiente

- C-3 schema JSON `eda.event_manager.v1` vs 2.0.0 puerto
- C-5 `route-domain-event.outputs`
- Suite `cargo test -p execute-process --lib` completa
- Clippy estructural ~85 en execute-process