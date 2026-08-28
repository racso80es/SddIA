---
document_id: PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA
uuid: "85287f67-30e7-4ffc-b83f-cc7562bd47df"
title: "[KAIZEN] Fan-out de fractura sin idempotencia real — PBI cerrados resucitados y Mayeuta en dead-letter"
format: markdown
version: "1.1.0"
created: "2026-08-28"
updated: "2026-08-28"
status: "pendiente"
priority: "alta"
process: bug-fix
type: kaizen
dispatch: false
suggested_branch: fix/kaizen-fracture-fanout-idempotencia
incident_ref: "Auditoría 2026-08-28 08:54 — PBI-FIX-FRACTURE-6a49e0ad310e reaparece en pending/ tras cierre en PR #210"
friction_ids:
  - F-FRACTURE-PBI-IGNORA-DONE
  - F-FRACTURE-ENRICH-PATH-DESALINEADO
  - F-FRACTURE-EVENT-STORM-MISMA-TRAZA
  - F-FRACTURE-PBI-UNTRACKED-BLOQUEA-DONE
architectural_constraints:
  - A-INMUTABILIDAD-DONE
  - A-BUS-ESTADO-CERO
  - A-RESOLUTOR-EN-CORE
related:
  - SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/core/paths.rs
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/core/cumulo.paths.json
  - SddIA/actions/materialize-fracture-pbi.md
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/events/domain/system-fracture-detected.md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (6a49e0ad310e).md
  - docs/fixes/route-domain-event-fracture-6a49e0ad/validacion.md
source_audit: "Conteo directo sobre ./.events (processed + dead-letter) y lectura de los dos handlers nativos del fan-out System_Fracture_Detected"
review_notes: "Filtro A aplicado 2026-08-28 — tres colisiones de topología corregidas en §5 (CA2, CA3/CA4, CA6)"
---

# [KAIZEN] Fan-out de fractura sin idempotencia real — PBI cerrados resucitados y Mayeuta en dead-letter

## 1. Falla Estructural y Contexto

El fan-out de `System_Fracture_Detected` delega en dos handlers nativos que **derivan la
ruta del PBI de forma independiente** y con criterios distintos:

- `materialize-fracture-pbi` (Cúmulo) escribe en `docs/todos/pending/` y deduplica con
  dos reglas: fichero exacto existente, o *cualquier* PBI abierto del mismo
  `process_name`.
- `enrich-fracture-pbi-kaizen` (Mayeuta) **recalcula** la ruta a partir del hash de la
  traza y falla si ese fichero concreto no está en `pending/`.

Ninguno de los dos consulta `docs/todos/done/`. El resultado es que el ciclo de vida
documental de la deuda (abrir → cerrar → archivar) es invisible para el bus: un PBI
cerrado y mergeado vuelve a materializarse en `pending/` en la siguiente emisión de la
misma traza.

### 1.1. Cronología verificada (caso `6a49e0ad310e`)

Traza: `F-DLT-RELAY-SIN-SUPERVISOR: merkle-batch-preseal failed: Campo obligatorio ausente o inválido: payload`
Hash SHA-256 (12) = `6a49e0ad310e`. Causa física ya remediada por Kaizen DLT #208.

| Hora (local) | Hecho |
|--------------|-------|
| 27/08 20:25 | `e0beb9c` — PBI archivado en `docs/todos/done/` (resuelto por Kaizen DLT) |
| 27/08 20:31 | `882cc0e` — copia reintroducida en `pending/` (regresión documental) |
| 28/08 08:02 | `5f5d054` — purga de `pending/`, regla Kintsugi, cascada del fix |
| 28/08 08:03 | `2c9dc60` — cierre post-merge PR #210, `validacion.md` APTO |
| 28/08 08:29 | 7 eventos `System_Fracture_Detected` con la **misma** traza en 18 s |
| 28/08 08:29 | `cumulo.materialize-fracture-pbi` → `success`: recrea el PBI en `pending/` |

El fichero resultante queda **untracked**, con `status: abierto` y `created: 2026-08-28`,
mientras el canónico en `done/` mantiene `status: cerrado`, `merged_pr: 210` y
`pbi_archived: true`.

### 1.2. Magnitud medida sobre `./.events`

| Métrica | Valor |
|---------|-------|
| Eventos únicos con la traza `6a49e0ad310e` | 10 |
| Ráfaga concentrada 28/08 06:29:08–06:29:26 UTC | 7 eventos |
| Dead-letters de `mayeuta.enrich-fracture-pbi-kaizen` | **351** |
| Motivo único de esos DL | `PBI de Cúmulo no encontrado: … — ejecutar materialize-fracture-pbi antes` |

Los 351 DL no son un fallo de Mayeuta: son la consecuencia directa de que Cúmulo
deduplique por `process_name` y devuelva una ruta (`existing_rel`) que Mayeuta nunca
recibe, porque el fan-out entrega a cada suscriptor el `payload` del evento y no la
salida del suscriptor anterior. Procesos más castigados: `github-bridge-watcher`,
`telegram-watcher`, `email-watcher` — cada nueva traza del mismo daemon genera un DL.

## 2. Impacto

- **Deuda fantasma**: PBI cerrados reaparecen como abiertos. El inventario de
  `docs/todos/pending/` deja de ser fiable como cola de trabajo.
- **Bloqueo de Done ajeno**: el fichero recreado dispara `PBI_PENDING_ABSENT: NO_APTO` y
  `AC_DONE_PATH: NO_APTO` en la `validacion.md` de un ciclo que ya estaba cerrado,
  obligando a purgas manuales repetidas (ocurrió en el PR #210 y de nuevo hoy).
- **Mayeuta silenciada**: con 351 DL, el diagnóstico Kaizen no llega al PBI en la
  mayoría de fracturas de daemons. Se materializa el *Qué* sin el *Por Qué*.
- **Ruido en el DLT**: cada evento redundante arrastra su propio anclaje y sus
  suscriptores, inflando `dead-letter` y la cola de reanclaje.
- **Erosión del protocolo Kintsugi**: si el operador IA aprende que el PBI de fractura
  suele ser stale, deja de tratarlo como señal fiable de colapso.

## 3. Hipótesis de causa raíz

1. **Idempotencia parcial por diseño** — `materialize_fracture_pbi::run` solo consulta
   `docs/todos/pending/`; no hay lectura de `docs/todos/done/` ni por `document_id`
   (`PBI-FIX-FRACTURE-<hash>`) ni por nombre de fichero.
2. **Contrato de ruta duplicado en la capa equivocada** — la derivación vive replicada en
   dos handlers de `engine/` en lugar de residir una sola vez en `core/`. Peor: `enrich`
   importa `super::materialize_fracture_pbi` solo para reconstruir un nombre de fichero,
   creando acoplamiento entre cápsulas de agentes que deberían ser mutuamente ciegas.
   Ambos handlers cablean además `docs/todos/pending` como literal, sin pasar por
   `cumulo.paths.json`.
3. **Fan-out sin propagación de salida** — el suscriptor Mayeuta declara el input
   opcional `cumulo_pbi_path`, pero la topología no lo alimenta; el campo es letra
   muerta en producción. Corolario: la resolución no puede depender de que un suscriptor
   herede la salida de otro; debe ser deducible del `payload` más el estado del disco.
4. **Escritura de evento sin identidad de contenido** — el nombre del fichero de evento
   se deriva del `event_id` (UUID nuevo por emisión), de modo que N fracturas idénticas
   producen N ficheros distintos y ninguna colisiona. La identidad del evento está en su
   contenido, pero el nombre no la refleja: la deduplicación que el sistema de archivos
   podría dar gratis se pierde.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| FPBI-CA1 | `materialize-fracture-pbi` no crea el PBI si existe homólogo en `docs/todos/done/` con el mismo `document_id` o el mismo nombre de fichero; retorna `success: true` con motivo explícito (`already_closed`) y la ruta del canónico en `done/` |
| FPBI-CA2 | Si la fractura reaparece estando cerrada, se materializa un **PBI nuevo** en `pending/` con prefijo `[REGRESIÓN]` que enlaza al canónico de `done/` (`regression_of: <document_id>`). El PBI cerrado es **inmutable**: no se edita, no se reabre, no se mueve |
| FPBI-CA3 | La derivación de la ruta del PBI de fractura reside en la **capa Core** (`execute-process/src/core/`, sobre `cumulo.paths.json`), no en `engine/`. Cúmulo y Mayeuta la consumen; ningún handler la recalcula ni deriva rutas por su cuenta |
| FPBI-CA4 | Mayeuta localiza el PBI aunque Cúmulo haya deduplicado por `process_name`, resolviendo **vía Core** por `document_id` / `process_name`. Sin importar `engine::materialize_fracture_pbi` desde `engine::enrich_fracture_pbi_kaizen`: cero dependencia entre cápsulas de agentes |
| FPBI-CA5 | Cuando no hay PBI que enriquecer, `enrich-fracture-pbi-kaizen` **no** va a dead-letter: retorna `success: true` con motivo (`no_target`), reservando el DL para fallos reales |
| FPBI-CA6 | La deduplicación de eventos idénticos ocurre por **colisión física en el sistema de archivos** al escribir el evento: nombre derivado del hash de contenido + `create_new(true)`. `AlreadyExists` = duplicado descartado. Prohibida cualquier caché temporal, ventana configurable o estado transaccional en el enrutador |
| FPBI-CA7 | Tests unitarios: (a) PBI en `done/` → no recrea en `pending/`; (b) dedup por `process_name` → Mayeuta enriquece el PBI real; (c) ráfaga de eventos idénticos → una sola materialización |
| FPBI-CA8 | Barrido de los 351 dead-letters de `mayeuta.enrich-fracture-pbi-kaizen`: reproceso o cierre documentado, con `orphan_count` estable en `event-bus-audit` |
| FPBI-CA9 | Purga del stale actual `docs/todos/pending/[FIX] route-domain-event — fractura sistémica (6a49e0ad310e).md` y no-regresión verificada tras una nueva emisión de la misma traza |
| FPBI-CA10 | El barrido de deduplicación en `pending/` reconoce **ambos** prefijos (`[FIX]` y `[REGRESIÓN]`): un PBI de regresión abierto suprime nuevas materializaciones de la misma traza/proceso. El prefijo queda homologado en el contrato de `materialize-fracture-pbi` |

## 5. Laudo arquitectónico (Filtro A)

Tres correcciones de topología sobre el borrador inicial. Son **restricciones de
diseño**, no sugerencias: una implementación que las infrinja se rechaza en revisión.

### 5.1. Inmutabilidad del pasado (corrige FPBI-CA2)

Anotar la recurrencia dentro de un PBI ya archivado en `done/` viola la trazabilidad
empírica de los ciclos consolidados: reescribe conocimiento ya destilado y falsea la
correspondencia entre el PBI y el PR que lo cerró.

La topología exige el camino contrario: **PBI nuevo en `pending/` con prefijo
`[REGRESIÓN]`** que enlace al original por `regression_of`. Así se preserva la
inmutabilidad sin generar amnesia — la cadena de recurrencias queda navegable hacia
atrás y el ciclo original conserva su cierre intacto.

Consecuencia derivada (FPBI-CA10): `find_open_fracture_pbi` filtra hoy por el prefijo
literal `[FIX] {slug} — fractura sistémica (`, de modo que un PBI `[REGRESIÓN]` abierto
sería invisible para el barrido y la siguiente emisión volvería a materializar. El
reconocimiento de ambos prefijos es parte inseparable de esta corrección, no un extra.

### 5.2. Estado Cero en el enrutador (corrige FPBI-CA6)

Inyectar una ventana de silencio configurable en el core del enrutador amenaza
dogmáticamente la Arquitectura Orientada a Eventos de Estado Cero. El bus debe operar
como **tejido nervioso inerte y sin memoria transaccional**: cualquier caché temporal
convierte el enrutador en un componente con estado, y además es inservible en un script
que sufre Ceguera Espacial absoluta (no puede saber qué ocurrió fuera de su propia
invocación).

La deduplicación temporal debe emerger de una **colisión física de hash en el sistema de
archivos** al intentar escribir el evento: nombre de fichero derivado del hash de
contenido y apertura con `create_new(true)`. Un `AlreadyExists` **es** la deduplicación,
sin que nadie recuerde nada.

Precedente vivo en el repo: `task_queue_manager.rs` ya resuelve su exclusión mutua con
`OpenOptions::new().write(true).create_new(true)` sobre `single_flight_dir`. Replicar ese
patrón, no inventar uno nuevo.

### 5.3. Ubicación del resolutor (corrige FPBI-CA3/CA4)

La función pública única de resolución de rutas se aloja **estrictamente en la capa
Core** (`execute-process/src/core/`, junto a `paths.rs` y su `load_paths_config`), con
las rutas declaradas en `cumulo.paths.json`. Motivos:

- Evita la dependencia circular que hoy existe de facto: `enrich_fracture_pbi_kaizen.rs`
  importa `super::materialize_fracture_pbi` solo para derivar un nombre de fichero.
- Mantiene la **ceguera de ejecución** entre cápsulas de agentes: Mayeuta no debe conocer
  la implementación de Cúmulo para hacer su trabajo.
- Elimina rutas cableadas: `docs/todos/pending` y `docs/todos/done` deben resolverse por
  clave de Cúmulo, no por literal en el código (hoy ambos handlers las cablean).

## 6. Notas de implementación

Orden sugerido, de menor a mayor superficie:

1. **Core primero** — extraer el resolutor a `core/`, añadir las claves de `todos`
   (`pending` / `done`) a `cumulo.paths.json` y migrar ambos handlers a consumirlo.
   Desbloquea CA3 y CA4 sin tocar comportamiento.
2. **Consulta a `done/`** — `materialize_fracture_pbi.rs` ya recorre `pending/` con
   `find_open_fracture_pbi`; el homólogo sobre `done/` (por nombre de fichero y por
   `document_id` en frontmatter) cierra CA1.
3. **Rama de regresión** — con la detección de CA1 en su sitio, emitir el PBI
   `[REGRESIÓN]` en lugar de recrear (CA2).
4. **Fallback de Mayeuta** — cascada vía Core: `cumulo_pbi_path` → ruta por hash en
   `pending/` → PBI abierto del mismo `process_name` → homólogo en `done/` (dispara la
   rama de regresión) → `no_target` sin dead-letter (CA4, CA5).
5. **Colisión física** — nombre de evento por hash de contenido + `create_new` (CA6).

Coordinar con `PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI`, que ataca el mismo patrón (clave de
exclusión insuficiente) en `task_queue_manager.rs`. Ambos comparten la lección de fondo:
**la clave de idempotencia debe ser la identidad del trabajo, no la del mensaje** — y esa
clave se materializa en el sistema de archivos, no en memoria.

Prohibido tratar este PBI como licencia para tocar la causa física del relay IOTA: eso
ya está cerrado en Kaizen DLT #208 y queda fuera de alcance.

## 7. Fuera de alcance

- Causa física `F-DLT-RELAY-SIN-SUPERVISOR` (cerrada en Kaizen DLT #208).
- Single-flight de `task_queue_manager` por `pbi_ref` (dedup: `PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI`).
- Rehabilitación de entidades en Cerbero/Radamanto (ciclos PPR #208 / #210, cerrados).
