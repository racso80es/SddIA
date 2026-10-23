---
status: temporal
created: "2026-10-23"
subject: inyección de cápsula proveedora en el ciclo documental
---

# Inyección de cápsula proveedora — puntos decididos

Documento de trabajo. No es entidad, no es PBI, no es norma.

## Corte

El proceso deja de decidir el soporte. Una sola llamada, `create_issue` de `tracker:lifecycle`. La cápsula inyectada materializa el ítem.

Hoy el handler hace las dos cosas: `create_issue` si el manifiesto es remoto, y después escribe siempre el markdown. Esa escritura sale del proceso.

## Contrato

La capacidad sigue siendo `tracker:lifecycle`. El método de alta sigue siendo `create_issue`. No hay verbo nuevo.

Petición que el proceso puede enviar:

- `operation`: `create_issue`
- `title`
- `description`
- `labels`
- `parent_ref` opaco (el `tracker_ref` de la historia)

Respuesta que el proceso puede leer:

- `tracker_ref` opaco (`issue_ref` en el adaptador actual)

Fuera del proceso: `team_key`, `state_name`, `project_id`, rutas de `docs/todos`, y la elección `file` / `linear`. Eso lo lee la cápsula del manifiesto que la inyección le entrega.

`historia_ref` deja de ser una ruta que el proceso abre para sacar el `tracker_ref`. El invocante pasa el identificador opaco del padre. Si solo existe como fichero de historia, lo resuelve el proveedor de fichero.

## Inyección

La fase **Registro en Tracker** conserva `requires_capability: tracker:lifecycle`. El binding del proyecto elige la cápsula.

| Manifiesto | Cápsula | `create_issue` |
| :--- | :--- | :--- |
| `provider: linear` | `linear-tracker-adapter` | Issue en el tracker. Devuelve el identificador opaco. |
| `provider: file` | Cápsula nueva del mismo contrato | Markdown en el bucket que Cúmulo declara (`paths.todos`). Devuelve un `tracker_ref` opaco. |
| Otro proveedor | Su cápsula | Su persistencia. El mismo sobre. |

El Core, con `provider: linear` en `.SddIA/project.md`, deja de generar el fichero en `docs/todos/pending/`. El fichero aparece solo cuando el binding apunta al proveedor de fichero.

## Qué queda en el proceso

`forge-pbi` arma el título, el cuerpo y las etiquetas (`pbi`, y `fix` o `kaizen` según `process` / `tipo`) y hace una llamada.

**Sellado** emite `PBI_Forged` con ese `tracker_ref`. No hay `fs::write` en el handler.

Si `create_issue` falla, no hay alta local de compensación: `Tracker_Sync_Failed` y el proceso no finge un artefacto.

`PBI_Forged` deja de publicar `artifact_path`. El suscriptor `tracker-stamp` sigue sellando por `tracker_ref`.

## Mismo cuerpo

El `description` de `create_issue` es el fichero entero, cabecera YAML incluida: el mismo texto que persiste el proveedor de fichero. Quedan fuera solo los campos que el tracker añade y el fichero no tiene: `team_key`, `state_id` y `labelIds`. El título es el mismo en los dos.

## Procesos a refactorizar con el mismo patrón

El patrón es uno: el proceso llama un verbo de `tracker:lifecycle` y la cápsula inyectada persiste. La bifurcación a quitar es `tracker_lifecycle_port::substrate()` dentro de cada handler. El motor deja de implementar el sumidero fichero.

| Proceso | Qué hace el handler ahora | Corte |
| :--- | :--- | :--- |
| `forge-pbi` | `create_issue` si es remoto y siempre escribe el markdown | Ya decidido arriba. |
| `refine-hu` | Lee la historia del disco, la sella con `fs::write` y solo llama `create_issue` si no hay `tracker_ref` y el sustrato es remoto | Alta y sello por el proveedor. El proceso no abre la ruta. |
| `refine-pbi` | Sella el fichero. El YAML atribuye la persistencia a `tracker:lifecycle`, pero el handler no llama `update_issue_description` | El sello definitivo es una llamada al proveedor. |
| `tracker-markdown-apply` | El YAML atribuye la réplica al contrato. El handler escribe el markdown y lo mueve a `done` | Aplicar el cambio y cerrar la réplica es método del proveedor de fichero. Con `linear` la cápsula no escribe disco. |
| `tracker-stamp` | Estado y comentario ya van por la cápsula. `archive_cancelled_pbi` mueve el fichero a `todos_done`. Si falta `tracker_ref`, lee el PBI del disco | El sello de estado se queda. El movimiento de fichero y la resolución por ruta pasan al proveedor. |
| `tracker-backlog-query` | Remoto: `list_issues`. Local: `backlog_items` del puerto en el motor | Las dos ramas son `list_issues` de la cápsula inyectada. |
| `tracker-description-sync` | Remoto: `update_issue_description` leyendo `markdown_path`. Local: sale con «sumidero local» | El cuerpo lo carga el proveedor. El proceso no elige el sumidero ni abre el bucket. |
| `tracker-sync-replay` | Remoto: `fetch_issue` y reaplicación. Local: `discarded` | El mismo verbo sobre la cápsula. Descartar por ser fichero deja de ser una rama del proceso. |
| `materialize-fracture-pbi` | Escribe el PBI en disco y además puede llamar `create_issue` | Mismo defecto, aunque no es un proceso de ciclo. El alta pasa por `create_issue`. |

## Fuera de este corte

`feature`, `bug-fix`, `refactorization` y `delivery-close-cycle` no entran. Su documentación vive en `persist_ref` (`docs/features`, `docs/fixes`).
