---
document_id: PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA
uuid: "85287f67-30e7-4ffc-b83f-cc7562bd47df"
title: "[KAIZEN] Fan-out de fractura sin idempotencia real — PBI cerrados resucitados y Mayeuta en dead-letter"
format: markdown
version: "1.2.0"
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
  - F-FRACTURE-IDENTIDAD-FUERA-DEL-GENOMA
architectural_constraints:
  - A-INMUTABILIDAD-DONE
  - A-BUS-ESTADO-CERO
  - A-RESOLUTOR-EN-CORE
  - A-CEGUERA-NOMINAL
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
  - docs/todos/pending/[DEUDA] Escaneo lineal de docs-todos en el resolutor de fractura — umbral de indexación.md
source_audit: "Conteo directo sobre ./.events (processed + dead-letter) y lectura de los dos handlers nativos del fan-out System_Fracture_Detected"
review_notes: "Filtro A aplicado 2026-08-28 — tres colisiones de topología corregidas en §5 (CA2, CA3/CA4, CA6). Refinado v1.2.0: doctrina de ceguera nominal (§5.4), identidad en genoma y backfill (CA11/CA12), tabla de precedencia en CA1"
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
5. **Identidad del PBI fuera del genoma** — causa raíz de todas las anteriores. El
   frontmatter que emite `build_pbi_body` **no contiene el hash de traza como propiedad**:
   sólo aparece embebido en cadenas (`document_id: PBI-FIX-FRACTURE-<hash>`,
   `incident_ref: "System_Fracture_Detected — <hash>"`) y en el nombre del fichero
   (`[FIX] {slug} — fractura sistémica ({hash}).md`). Tampoco existe propiedad alguna que
   declare el proceso fracturado; el slug vive únicamente en el nombre. Al no haber
   identidad declarada, cualquier resolución degenera en parseo de cadenas: `target.is_file()`
   compara rutas construidas y `find_open_fracture_pbi` filtra por prefijo literal
   `[FIX] {slug} — fractura sistémica (`. El motor no lee el genoma porque el genoma no lo
   declara.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| FPBI-CA1 | **Deduplicación estructural.** `materialize-fracture-pbi` resuelve la existencia de un homólogo leyendo **exclusivamente el frontmatter YAML** de `docs/todos/pending/` y `docs/todos/done/`. Nunca recrea ni resucita un PBI ya materializado: toda salida es `success: true` con `reason` explícito y la ruta del canónico. Precedencia determinista en §4.1 |
| FPBI-CA2 | **Trazabilidad de regresión.** Si la fractura reaparece estando cerrada (homólogo sólo en `done/`), se materializa un **PBI nuevo** en `pending/` que conserva el mismo `fracture_hash` e inyecta `regression_of: <document_id del predecesor>`. El PBI de `done/` es **estrictamente inmutable**: no se edita, no se reabre, no se mueve. El prefijo `[REGRESIÓN]` en el título es afordancia humana, sin carga semántica para el motor (CA10) |
| FPBI-CA3 | La derivación de la ruta del PBI de fractura reside en la **capa Core** (`execute-process/src/core/`, sobre `cumulo.paths.json`), no en `engine/`. Cúmulo y Mayeuta la consumen; ningún handler la recalcula ni deriva rutas por su cuenta |
| FPBI-CA4 | Mayeuta localiza el PBI aunque Cúmulo haya deduplicado por clase de proceso, resolviendo **vía Core** con la misma precedencia de §4.1 sobre `fracture_hash` / `fracture_process`. Sin importar `engine::materialize_fracture_pbi` desde `engine::enrich_fracture_pbi_kaizen`: cero dependencia entre cápsulas de agentes |
| FPBI-CA5 | Cuando no hay PBI que enriquecer, `enrich-fracture-pbi-kaizen` **no** va a dead-letter: retorna `success: true` con motivo (`no_target`), reservando el DL para fallos reales |
| FPBI-CA6 | La deduplicación de eventos idénticos ocurre por **colisión física en el sistema de archivos** al escribir el evento: nombre derivado del hash de contenido + `create_new(true)`. `AlreadyExists` = duplicado descartado. Prohibida cualquier caché temporal, ventana configurable o estado transaccional en el enrutador. Ámbito: ficheros de `./.events`, **no** PBIs (ver §5.4 para la asimetría) |
| FPBI-CA7 | Tests unitarios: (a) homólogo cerrado en `done/` → no recrea el canónico, abre regresión; (b) homólogo abierto en `pending/` → `already_open`, cero escrituras; (c) ráfaga de 7 eventos idénticos tras cierre → exactamente **un** PBI de regresión; (d) misma clase de fractura con traza distinta (ola heartbeat) → dedup por `fracture_process`, un solo PBI; (e) PBI renombrado a mano en disco → sigue deduplicando por genoma; (f) Mayeuta enriquece el PBI real cuando Cúmulo dedujo por `fracture_process` |
| FPBI-CA8 | Barrido de los 351 dead-letters de `mayeuta.enrich-fracture-pbi-kaizen`: reproceso o cierre documentado, con `orphan_count` estable en `event-bus-audit` |
| FPBI-CA9 | Purga del stale actual `docs/todos/pending/[FIX] route-domain-event — fractura sistémica (6a49e0ad310e).md` y no-regresión verificada tras una nueva emisión de la misma traza |
| FPBI-CA10 | **Prohibición de entropía visual.** Queda prohibida toda deduplicación de PBI basada en el nombre físico del fichero: comparación de rutas construidas, prefijos, sufijos o búsqueda de cadenas. La capa Core resuelve la existencia de un homólogo mapeando únicamente propiedades del genoma YAML (`fracture_hash`, `fracture_process`, `status`). El motor es **ciego a la nomenclatura**: renombrar un PBI a mano no altera su resolución. Implica eliminar el atajo `target.is_file()` de `materialize_fracture_pbi::run` y el filtrado por prefijo de `find_open_fracture_pbi` |
| FPBI-CA11 | **Identidad en el genoma.** El frontmatter emitido declara `fracture_hash: <12 hex>`, `fracture_process: <slug>` y, en regresiones, `regression_of`. El contrato `SddIA/actions/materialize-fracture-pbi.md` documenta estos campos y los `reason` de salida; `enrich-fracture-pbi-kaizen.md` deja de describir la resolución "por hash de nombre" |
| FPBI-CA12 | **Backfill de los históricos.** Los **55** PBI de fractura ya archivados en `docs/todos/done/` (más el stale de `pending/`) reciben `fracture_hash` y `fracture_process` derivados de su `document_id` actual, en un paso de migración único y auditable. Sin esto CA1 pasa en tests y falla en producción: el canónico `6a49e0ad310e` seguiría siendo invisible para el motor. La migración es la **única** operación autorizada a leer nombre o `document_id` como fuente del hash, y no vulnera A-INMUTABILIDAD-DONE por ser anotación de identidad, no de estado |
| FPBI-CA13 | **Unicidad de `document_id`.** El PBI de regresión conserva `fracture_hash` pero nunca el `document_id` del cerrado: se emite `PBI-FIX-FRACTURE-<hash>-R<n>`. La colisión de `document_id` rompería `task-closure-documental` (mover `pending/` → `done/` con el mismo id) y la auditoría de PBI. El nombre de fichero incorpora el mismo discriminante por mera unicidad del filesystem, sin valor semántico |

### 4.1. Precedencia de resolución (contrato de CA1)

Orden determinista, primera coincidencia gana. Todas las lecturas son sobre frontmatter.

| # | Condición sobre el genoma | Desenlace | `reason` |
|---|---------------------------|-----------|----------|
| 1 | `pending/` contiene PBI con mismo `fracture_hash` y `status: abierto` | No escribe. Devuelve su ruta | `already_open` |
| 2 | `pending/` contiene PBI abierto con mismo `fracture_process` (traza distinta, misma clase) | No escribe. Devuelve su ruta | `deduped_by_process` |
| 3 | `done/` contiene PBI con mismo `fracture_hash` (cerrado) | Abre regresión en `pending/` (CA2/CA13). Devuelve ruta nueva + `canonical_ref` al de `done/` | `regression_opened` |
| 4 | Sin coincidencias | Materializa PBI nuevo | `materialized` |

El paso 1 precede al 3 y acota la cadena de regresiones: en la ráfaga real de 7 eventos en
18 s, el primero abre la regresión y los seis restantes caen en `already_open`. Una
recurrencia posterior sólo puede abrir una regresión nueva si la anterior ya fue cerrada,
que es exactamente la señal que se quiere conservar.

El paso 2 preserva la defensa anti-ola que hoy existe y que una resolución *sólo* por
`fracture_hash` destruiría (§5.5).

No existe un desenlace `already_closed` que termine sin escribir nada: encontrar el
homólogo en `done/` es precisamente la condición que dispara la regresión. Lo que CA1
prohíbe no es escribir, sino **recrear la identidad del cerrado**; el motivo emitido en ese
caso es `regression_opened` y lleva `canonical_ref` al documento de `done/`, que satisface
la trazabilidad buscada sin dejar la recurrencia muda.

## 5. Laudo arquitectónico (Filtro A)

Cinco correcciones de topología sobre el borrador inicial. Son **restricciones de
diseño**, no sugerencias: una implementación que las infrinja se rechaza en revisión.

### 5.1. Inmutabilidad del pasado (corrige FPBI-CA2)

Anotar la recurrencia dentro de un PBI ya archivado en `done/` viola la trazabilidad
empírica de los ciclos consolidados: reescribe conocimiento ya destilado y falsea la
correspondencia entre el PBI y el PR que lo cerró.

La topología exige el camino contrario: **PBI nuevo en `pending/`** que conserve el
`fracture_hash` y enlace al predecesor por `regression_of`. Así se preserva la
inmutabilidad sin generar amnesia — la cadena de recurrencias queda navegable hacia
atrás por genoma y el ciclo original conserva su cierre intacto. El prefijo `[REGRESIÓN]`
en el título es señalización para el lector, no mecanismo (§5.4).

Consecuencia derivada: `find_open_fracture_pbi` filtra hoy por el prefijo literal
`[FIX] {slug} — fractura sistémica (`, de modo que un PBI de regresión abierto sería
invisible para el barrido y la siguiente emisión volvería a materializar. La versión
anterior de este PBI resolvía eso homologando el prefijo `[REGRESIÓN]` en el contrato;
esa solución queda **derogada** por §5.4: no se homologa un segundo prefijo, se elimina
la lectura de prefijos.

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

Verificado: `cumulo.paths.json` **no** declara hoy ninguna clave de `todos`; sólo existe
`documentation: "docs"`. Las claves `todos.pending` / `todos.done` son alta nueva, no
reutilización.

### 5.4. Ceguera nominal (corrige FPBI-CA10, deroga su versión anterior)

El nombre de un fichero es **capa de presentación**: lo fija un humano al archivar, lo
altera un renombrado, lo trunca un slug de 48 caracteres. Fundar la identidad de una
entidad en él es entropía visual — el mismo pecado que el Estándar Atómico corrige al
exigir `uuid` en frontmatter en lugar de deducir la entidad de su ruta.

Hoy el handler comete ese pecado dos veces: `target.is_file()` sobre una ruta reconstruida
y `find_open_fracture_pbi` filtrando por prefijo literal. Ambos son *string matching*
disfrazado de resolución.

Doctrina: **el genoma declara, el motor lee**. La identidad de una fractura es
`fracture_hash` (identidad del defecto) más `fracture_process` (clase del defecto), ambas
propiedades YAML de primer nivel. Un PBI renombrado, movido o con el título editado sigue
siendo el mismo PBI para el motor. Corolario operativo: el nombre queda libre para el
lector humano, incluido el prefijo `[REGRESIÓN]`, sin coste de acoplamiento.

Asimetría deliberada con CA6, que sí funda la deduplicación en el nombre del fichero de
evento: no hay contradicción porque los ficheros de `./.events` son **mensajes efímeros
que nadie renombra ni archiva**, escritos y consumidos exclusivamente por el runtime, y su
nombre por hash de contenido es la única forma de dedup sin estado. Los PBI, en cambio,
son documentos vivos con ciclo de vida humano. Regla: identidad por nombre sólo donde el
nombre es propiedad exclusiva de la máquina.

### 5.5. El eje `fracture_process` no es opcional

Una lectura literal de "resolver *exclusivamente* por `fracture_hash`" destruiría una
defensa que hoy funciona. Las trazas de heartbeat incorporan valores variables
(`omitió 13 ciclos… last_heartbeat=…T15:55:03Z` vs `omitió 37 ciclos… T16:08:11Z`), de modo
que **su hash nunca se repite**: la dedup que impide la ola de PBI por daemon no opera por
hash sino por clase de proceso, y está cubierta por el test
`materialize_dedupes_open_pbi_same_process_different_trace`. Suprimirla reintroduciría
exactamente el patrón que produjo las olas de fracturas de julio y agosto (55 PBI de
fractura archivados en `done/` lo atestiguan).

La corrección correcta no es eliminar el eje de proceso, sino **sacarlo del nombre del
fichero y meterlo en el genoma** como `fracture_process`. Se conservan los dos ejes de
idempotencia y se cumple la ceguera nominal: ambos se leen del YAML.

## 6. Notas de implementación

Orden obligatorio por dependencia — el paso 1 habilita todo lo demás:

1. **Genoma primero** — emitir `fracture_hash` y `fracture_process` en `build_pbi_body`
   (CA11) y ejecutar el backfill de los 55 históricos de `done/` más el stale de
   `pending/` (CA12). Sin este paso todo lo demás resuelve sobre un vacío: no hay
   propiedad que leer. Es el único paso que puede derivar el hash del `document_id`.
2. **Core** — extraer el resolutor a `core/`, añadir las claves `todos.pending` /
   `todos.done` a `cumulo.paths.json` y migrar ambos handlers a consumirlo. La API pública
   es un lector de frontmatter que devuelve `(fracture_hash, fracture_process, status)` por
   documento, más la resolución de precedencia de §4.1. Desbloquea CA3 y CA4 sin tocar
   comportamiento observable.
3. **Consulta a `done/` y ceguera nominal** — sustituir `target.is_file()` y el filtrado
   por prefijo de `find_open_fracture_pbi` por la resolución de §4.1 sobre ambos
   directorios (CA1, CA10). El barrido lee sólo el bloque frontmatter hasta su `---` de
   cierre e ignora todo documento sin `fracture_hash`, sin mirar el nombre para
   preseleccionar: 205 documentos en `done/` medidos en **2,04 ms**, irrelevante frente a
   una ráfaga de 7 eventos en 18 s. El coste lineal a largo plazo queda acotado por
   `PBI-DT-FRACTURE-RESOLVER-SCAN-LINEAL` (umbral de indexación); aquí se instrumenta la
   telemetría que ese umbral exige, no se indexa.
4. **Rama de regresión** — con la detección en su sitio, emitir el PBI de regresión con
   `document_id` discriminado en lugar de recrear (CA2, CA13).
5. **Fallback de Mayeuta** — cascada vía Core: `cumulo_pbi_path` → `fracture_hash` abierto
   en `pending/` → `fracture_process` abierto en `pending/` → homólogo en `done/` (no
   enriquece un PBI cerrado: se apoya en la regresión ya abierta por Cúmulo) → `no_target`
   sin dead-letter (CA4, CA5).
6. **Colisión física** — nombre de evento por hash de contenido + `create_new` (CA6).

Coordinar con `PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI`, que ataca el mismo patrón (clave de
exclusión insuficiente) en `task_queue_manager.rs`. Ambos comparten la lección de fondo:
**la clave de idempotencia debe ser la identidad del trabajo, no la del mensaje** — y esa
clave se materializa en el disco, nunca en memoria. Dónde se materializa depende del
artefacto: en el **nombre** cuando el fichero pertenece sólo a la máquina (eventos,
locks de single-flight), en el **genoma** cuando el fichero tiene lector y editor humanos
(PBI). Confundir ambos casos es el origen de esta deuda.

Prohibido tratar este PBI como licencia para tocar la causa física del relay IOTA: eso
ya está cerrado en Kaizen DLT #208 y queda fuera de alcance.

## 7. Fuera de alcance

- Causa física `F-DLT-RELAY-SIN-SUPERVISOR` (cerrada en Kaizen DLT #208).
- Single-flight de `task_queue_manager` por `pbi_ref` (dedup: `PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI`).
- Rehabilitación de entidades en Cerbero/Radamanto (ciclos PPR #208 / #210, cerrados).
- Cualquier mutación de PBI archivados en `done/` más allá de la anotación de identidad
  del backfill (CA12): prohibido tocar `status`, `closed`, `merged_pr`, cuerpo o
  conclusiones.
- Migración de PBI no derivados de fractura al esquema `fracture_hash`.
- Indexación del barrido de `done/` para acotar su coste lineal (dedup:
  `PBI-DT-FRACTURE-RESOLVER-SCAN-LINEAL`, que fija el umbral medido de activación). Este
  PBI implementa el escaneo; construir el índice antes del umbral está prohibido allí.
