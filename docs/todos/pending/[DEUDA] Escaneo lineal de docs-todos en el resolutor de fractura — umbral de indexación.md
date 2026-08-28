---
document_id: PBI-DT-FRACTURE-RESOLVER-SCAN-LINEAL
uuid: "927fd039-17ae-4e2f-baba-b85329f58311"
title: "[DEUDA] Escaneo lineal de docs/todos/ en el resolutor de fractura — umbral de indexación"
format: markdown
version: "1.0.0"
created: "2026-08-28"
updated: "2026-08-28"
status: pending
priority: baja
process: refactorization
type: deuda
dispatch: false
suggested_branch: refactor/fracture-resolver-index-threshold
persist_ref_suggested: docs/refactors/fracture-resolver-index-threshold
depends_on:
  - PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA
tech_debt_ids:
  - DT-RESOLUTOR-ESCANEO-LINEAL
  - DT-INDICE-SIN-UMBRAL-MEDIDO
related_pbis:
  - id: PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA
    rol: "Introduce el escaneo lineal de frontmatter (§4.1). Este PBI hereda su coste; no altera su semántica ni su tabla de precedencia."
  - id: PBI-OPER-DEUDA-TECNICA-KINTSUGI-001
    rol: "Define el portador canónico de deuda no-fractura (type: deuda + tech_debt_ids en pending/). Este PBI es el primer habitante de ese formato."
architectural_constraints:
  - A-INDICE-ACELERADOR-NO-AUTORIDAD
  - A-BUS-ESTADO-CERO
  - A-CEGUERA-NOMINAL
  - A-AGNOSTICISMO-CORE
related:
  - SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/core/paths.rs
  - SddIA/engine/execute-process/src/main.rs
  - SddIA/tools/index.md
  - SddIA/core/eda-coverage.json
  - SddIA/core/cumulo.paths.json
  - docs/todos/pending/[KAIZEN] Fan-out de fractura sin idempotencia real — PBI cerrados resucitados y Mayeuta en dead-letter.md
source_audit: "Medición directa 2026-08-28 sobre docs/todos/done (205 documentos, 944.980 bytes); benchmark de lectura de cabecera 2 KB; censo de `created:` por mes; inspección del modelo de proceso en main.rs y del precedente de índice en SddIA/tools/index.md"
review_notes: "Origen: apreciación táctica de Tormentosa sobre §4.1 del PBI de fan-out. Tres afirmaciones del planteamiento corregidas en §4."
---

# [DEUDA] Escaneo lineal de docs/todos/ en el resolutor de fractura — umbral de indexación

## 1. La disyuntiva

`PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA` §4.1 obliga al resolutor de Core a leer el
frontmatter de **todos** los documentos de `docs/todos/pending/` y `docs/todos/done/` en
cada evento `System_Fracture_Detected`, para decidir entre `already_open`,
`deduped_by_process`, `regression_opened` y `materialized`.

Es una operación **lineal en el número de documentos archivados**, ejecutada en una ruta
de alta frecuencia, sobre un directorio que crece de forma **monótona por diseño**: nada
sale nunca de `done/`.

La disyuntiva es de temporización, no de dirección:

- **Escanear** mantiene una única fuente de verdad (el genoma de cada documento) y cero
  estado derivado, al precio de un coste que crece sin techo.
- **Indexar** (`fracture_hash → path`) acota la latencia, al precio de introducir un
  artefacto derivado que puede desincronizarse de la verdad que pretende acelerar.

Este PBI no resuelve la disyuntiva construyendo el índice. La resuelve **fijando el umbral
medido a partir del cual construirlo deja de ser optimización prematura**, y la doctrina
que ese índice deberá respetar cuando llegue.

## 2. Medición empírica (estado a 2026-08-28)

| Métrica | Valor |
|---------|-------|
| Documentos en `docs/todos/done/` | 205 |
| Documentos en `docs/todos/pending/` | 9 |
| Peso total de `done/` | 944.980 bytes (≈ 0,9 MB) |
| Bytes leídos en un barrido de cabeceras (2 KB por documento) | 363.174 |
| Latencia de un barrido completo | **2,04 ms** |

Condiciones del benchmark: caché de página caliente, lector en Python (intérprete, no el
binario Rust). Es por tanto una **cota superior floja**: la implementación real en
`execute-process` leerá menos —corta en el `---` de cierre del frontmatter— y en un
lenguaje compilado. Con caché fría el orden de magnitud sigue siendo de milisegundos de un
solo dígito.

Contraste con la carga real observada: la ráfaga peor documentada fueron **7 eventos en
18 segundos**. El escaneo aporta ~14 ms de los 18.000 disponibles.

### 2.1. Proyección de crecimiento

Censo de `created:` en `done/` (19 documentos sin el campo, excluidos del cómputo):

| Mes | Documentos archivados |
|-----|----------------------|
| 2026-05 | 33 |
| 2026-06 | 31 |
| 2026-07 | 57 |
| 2026-08 (28 días) | 65 |

Ritmo reciente ≈ **61 documentos/mes**, con tendencia al alza. Extrapolación lineal a ese
ritmo, y latencia proyectada asumiendo escalado proporcional al benchmark:

| Volumen de `done/` | Horizonte temporal | Latencia estimada del barrido |
|--------------------|--------------------|-------------------------------|
| 205 (hoy) | — | 2 ms |
| 1.000 | ≈ 13 meses | ≈ 10 ms |
| 2.000 | ≈ 2,5 años | ≈ 20 ms |
| 5.000 | ≈ 6,5 años | ≈ 50 ms |

La conclusión que arrojan los números es incómoda para ambas posturas: la preocupación es
**estructuralmente legítima** —el crecimiento es monótono y la ruta es caliente— y
**operativamente remota**: el punto de dolor está a años vista, mientras que el coste de
un índice prematuro (un segundo SSOT desincronizable) se paga desde el primer commit.

## 3. Veredicto: diferir con umbral, no diferir con intención

Rechazadas las dos salidas fáciles:

- **Construir el índice ahora** es optimización prematura sobre una medición de 2 ms. Peor
  que ineficiente: introduce una clase de fallo (índice stale) en el mismo componente cuya
  razón de existir es acabar con los PBI fantasma. Curar la resurrección de deuda con un
  caché que puede mentir sobre qué está cerrado es reintroducir el problema por otra
  puerta.
- **Anotar «lo miraremos en el futuro»** en las notas de implementación del PBI padre es
  precisamente la forma de deuda que el protocolo Kintsugi existe para erradicar: una
  intención sin portador, sin identidad y sin condición de disparo, que sobrevive solo
  mientras alguien la recuerde.

La resolución es un **gate cuantitativo**: la deuda queda registrada con identidad propia,
inerte (`dispatch: false`), y con una condición de activación verificable por telemetría,
no por apreciación.

## 4. Auditoría del planteamiento (correcciones)

Tres precisiones sobre la formulación original de la apreciación. Ninguna invalida el
diagnóstico de fondo; las tres alteran la solución.

### 4.1. «Índice inverso **en memoria**» — inviable en este runtime

`execute-process` es un **binario de una sola invocación**: `main()` procesa y termina con
`process::exit`. Cada evento del bus es un proceso nuevo. Un índice en memoria moriría con
la invocación que lo construyó y nunca amortizaría su coste de construcción —que es
exactamente el escaneo lineal que pretende evitar—, dejando el sistema estrictamente peor:
mismo O(N), más complejidad.

Además chocaría con `A-BUS-ESTADO-CERO`, ya laudado en el PBI padre §5.2: el enrutador
debe ser tejido nervioso inerte, sin memoria transaccional entre invocaciones.

Cualquier índice viable en esta arquitectura es un **artefacto materializado en disco**,
regenerable y versionado.

### 4.2. «Base vectorial» — inadecuada por categoría

Un índice vectorial resuelve **similitud semántica aproximada** sobre embeddings. La
operación aquí es la contraria: búsqueda **exacta y determinista** de una clave de 12
caracteres hexadecimales. Aplicarlo supondría:

- sustituir una respuesta exacta por una aproximada en el componente que decide si una
  deuda está cerrada —inaceptable bajo el Principio de Evidencia Determinista de Argos;
- introducir una dependencia externa con estado en el Core, violando el agnosticismo del
  repositorio base (`.cursorrules` §5);
- añadir latencia de red o de carga de modelo muy superior a los 2 ms que pretende
  ahorrar.

El precedente correcto ya vive en el genoma y no requiere inventar nada: los índices de
familia `{familia}/index.md` mantenidos por Cúmulo (`SddIA/tools/index.md`, con
`index_version`, `indexed_at`, `maintained_by_agent`) y los artefactos derivados tipo
`SddIA/core/eda-coverage.json`. Estructura plana, legible, versionada en git, reconstruible
desde la verdad.

### 4.3. «O(1)» — la ganancia real no es asintótica

Un índice en disco es O(1) en consulta, pero traslada el O(N) a la **reconstrucción**, que
alguien debe pagar y, sobre todo, **disparar en el momento correcto**. En números de hoy la
mejora es de ~2 ms a ~0,2 ms: irrelevante frente al arranque del propio binario. La
notación asintótica describe bien el futuro y engaña sobre el presente; el umbral de §6
existe para separar ambos.

### 4.4. Precisión de alcance

El escaneo no afecta solo a `done/`: la precedencia §4.1 del PBI padre recorre también
`pending/`. Es una diferencia sin importancia práctica —9 documentos frente a 205— pero
`pending/` no crece de forma monótona (se drena al archivar), así que **el único vector de
crecimiento real es `done/`**. La apreciación acierta al señalarlo.

## 5. Invariante de diseño: el índice es acelerador, jamás autoridad

Restricción que cualquier implementación futura debe respetar, y que se registra aquí para
que no dependa de la memoria de quien la implemente:

| Regla | Motivo |
|-------|--------|
| La SSOT es el frontmatter del documento. El índice es caché derivada | Un índice autoritativo puede declarar cerrada una deuda abierta, o viceversa; es el fallo original con otra máscara |
| Todo fallo de lookup en el índice degrada al escaneo lineal, nunca a un veredicto negativo | Un `miss` significa «índice incompleto», no «no existe el PBI» |
| El índice es reconstruible desde cero en cualquier momento y su reconstrucción es idempotente | Permite borrarlo ante la menor sospecha, sin ceremonia |
| El índice se regenera en el mismo commit que archiva un PBI, o se marca stale | Un humano que mueva un fichero a mano no puede corromper la resolución |
| El índice no contiene nombres de fichero como clave de identidad | `A-CEGUERA-NOMINAL` (PBI padre §5.4): la clave es `fracture_hash`; la ruta es solo el valor devuelto |
| Sin dependencias externas con estado (bases de datos, servicios) | Agnosticismo del Core; el artefacto vive en el repositorio |

## 6. Umbral de activación

La deuda se activa cuando la telemetría —no la estimación— cruce **cualquiera** de estos
límites:

| Disparador | Límite | Fuente de verdad |
|-----------|--------|------------------|
| Volumen | `docs/todos/done/` supera **2.000 documentos** | Censo del directorio |
| Latencia | p95 de la resolución de fractura supera **50 ms** | Telemetría emitida por el resolutor (CA1) |
| Frecuencia | El coste agregado del resolutor supera el **1 %** del tiempo de CPU de `route-domain-event` en una ventana de 7 días | Telemetría del bus |

Mientras no se cruce ninguno, **construir el índice está explícitamente prohibido** por
este PBI: cualquier PR que lo introduzca antes del gate se rechaza en revisión citando §3.

El tercer disparador es el que de verdad importa y el que la formulación original no
contemplaba: el coste no depende solo del volumen de `done/`, sino del producto
**volumen × frecuencia de fracturas**. Un repositorio pequeño con tormenta de eventos cruza
el umbral antes que uno grande y sano.

## 7. Criterios de aceptación

| ID | Criterio |
|----|----------|
| DT-CA1 | El resolutor de Core emite telemetría por invocación con `docs_scanned`, `bytes_read` y `duration_ms`, bajo la familia `telemetry` del genoma. Sin este dato el umbral de §6 es inverificable y este PBI no puede cerrarse |
| DT-CA2 | Los tres disparadores de §6 quedan declarados en el contrato del resolutor, no solo en este documento, de modo que sobrevivan a la rotación de operadores |
| DT-CA3 | El invariante «acelerador, jamás autoridad» (§5) queda registrado como restricción arquitectónica `A-INDICE-ACELERADOR-NO-AUTORIDAD`, consultable por quien implemente el índice |
| DT-CA4 | El barrido actual aplica las dos optimizaciones sin estado que no requieren índice: corte de lectura en el `---` de cierre del frontmatter y descarte inmediato de documentos sin `fracture_hash`. Coste marginal, cero superficie nueva |
| DT-CA5 | Hito registrado en `SddIA/evolution/` vinculando el `uuid` de este PBI, con la medición de §2 como línea base para comparar cuando se reevalúe |
| DT-CA6 | Cero solapamiento con `PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA`: este PBI **no** altera la tabla de precedencia §4.1, ni los `reason` emitidos, ni la semántica de regresión. Solo instrumenta y acota |

## 8. Opciones evaluadas (para cuando se abra el gate)

Registro de la evaluación, para no repetirla dentro de dos años:

| Opción | Veredicto | Motivo |
|--------|-----------|--------|
| Escaneo lineal de frontmatter (actual) | **Vigente** | 2 ms con 205 documentos; cero estado derivado |
| Índice `{familia}/index.md` mantenido por Cúmulo | **Candidata principal** | Precedente vivo (`SddIA/tools/index.md`); legible, versionado, reconstruible |
| Sidecar JSON derivado tipo `eda-coverage.json` | **Candidata** | Mismo patrón, menos legible para humanos, más barato de parsear |
| Directorio de anclas por hash (`stat()` sobre ruta derivada) | **Candidata condicionada** | Compatible con `A-CEGUERA-NOMINAL` solo porque sería artefacto exclusivo de máquina (criterio del PBI padre §5.4); replica el precedente `create_new` de `task_queue_manager` |
| Índice en memoria | **Descartada** | Binario de una sola invocación (§4.1) |
| Base vectorial | **Descartada** | Búsqueda aproximada para un problema exacto (§4.2) |

## 9. Fuera de alcance

- Cualquier cambio en la semántica de deduplicación o regresión: pertenece a
  `PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA`.
- Construcción del índice antes de cruzar el umbral de §6.
- Poda, archivado en frío o compactación de `docs/todos/done/`: reduciría N, pero destruye
  el registro histórico que sostiene la trazabilidad de regresiones (`regression_of`).
- Indexación de documentos de `docs/todos/` no derivados de fractura.

## 10. Referencias

| Ref | Uso |
|-----|-----|
| `docs/todos/pending/[KAIZEN] Fan-out de fractura…md` §4.1 | Origen del escaneo lineal |
| `docs/todos/pending/[KAIZEN] Fan-out de fractura…md` §5.2 / §5.4 | `A-BUS-ESTADO-CERO` y `A-CEGUERA-NOMINAL` |
| `SddIA/engine/execute-process/src/main.rs` | Modelo de proceso de una sola invocación |
| `SddIA/tools/index.md` | Precedente de índice de familia mantenido por Cúmulo |
| `SddIA/core/eda-coverage.json` | Precedente de artefacto derivado versionado |
| `SddIA/CONSTITUTION_CORE.md` | Triaje Entrópico; Principio de Evidencia Determinista |
