---
document_id: PBI-FIX-FRACTURE-6a49e0ad310e-R1
uuid: "e7edb590-3193-4709-a1a2-e863a79842e4"
title: "[REGRESIÓN] route-domain-event — fractura sistémica"
format: markdown
version: "1.3.0"
created: "2026-08-28"
updated: "2026-08-28"
status: cerrado
closed: "2026-08-28"
resolution_ref: docs/fixes/capsula-binario-fosil-release-stale/
cycle_blocked_by: null
priority: alta
process: bug-fix
type: regression
dispatch: false
suggested_branch: fix/capsula-binario-fosil-release-stale
fracture_hash: 6a49e0ad310e
fracture_process: route-domain-event
incident_ref: "System_Fracture_Detected — 6a49e0ad310e"
regression_of: PBI-FIX-FRACTURE-6a49e0ad310e
friction_ids:
  - F-CAPSULA-BINARIO-FOSIL
  - F-PERFIL-PRECEDENCIA-CIEGA-A-IDENTIDAD
  - F-BUILD-DEV-DESALINEADO-CON-RUNTIME
  - F-TRAZA-FRACTURA-ATRIBUCION-FALSA
  - F-DLT-REANCHOR-COLA-SIN-DRENAJE
  - F-CORE-RUTA-ABSOLUTA-DEBUG
  - F-GATE-DIRTY-PORCELAIN-SIN-DESESCAPE
  - F-FASE-COLAPSA-SIN-EVENTO-DE-FRACTURA
  - F-GENOMA-VERSION-ANCLA-MUERTA
  - F-CICATRIZ-VIVE-FUERA-DEL-MOTOR
  - F-DIGEST-GRANULARIDAD-WORKSPACE
architectural_constraints:
  - A-ANCLA-POR-CONTENIDO-NO-POR-TIEMPO
  - A-BINARIO-COHERENTE-CON-FUENTE
  - A-CEGUERA-DE-PERFILES
  - A-SSOT-EN-GENOMA
  - A-CORE-AGNOSTICO
  - A-TRAZA-NO-ESPECULATIVA
  - A-RUTA-GIT-DESESCAPADA
related:
  - SddIA/engine/execute-process/src/engine/capsule_paths.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/scripts/build-release-bundle.sh
  - SddIA/tools/iota-immutable-publisher.md
  - SddIA/tools/iota-immutable-publisher/src/main.rs
  - SddIA/core/cumulo.paths.json
  - SddIA/.gitignore
  - start-sddia.sh
  - SddIA/library/codexes/codex-software-engineering/process/bug-fix.md
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (6a49e0ad310e).md
  - docs/todos/done/[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch.md
  - docs/todos/done/[KAIZEN] Fan-out de fractura sin idempotencia real — PBI cerrados resucitados y Mayeuta en dead-letter.md
source_audit: "Ejecución directa de ambos binarios de iota-immutable-publisher con payload array bajo SDDIA_LAB_SIMULATE_IOTA=1; barrido mtime release/debug y contraste con la fecha del último commit por crate; auditoría de los 8 testigos .sha256 de SddIA/target/release contra _sddia_source_digest y sha256sum del ELF; recuento por event_id único sobre las ocho carpetas de ./.events; historia git del genoma iota-immutable-publisher.md frente a la de su crate"
review_notes: "v1.1.0 causa raíz aislada. v1.2.0 dirty-worktree. v1.3.0 anclaje por contenido; mtime revocado; magnitud corregida. Diseño sellado 2026-08-28 en docs/fixes/capsula-binario-fosil-release-stale/{spec,plan}.md (spec abortado de fail-stale sustituido). Código = fase Ejecución."
cycle_blocked_by: F-GATE-DIRTY-PORCELAIN-SIN-DESESCAPE
design_ref: docs/fixes/capsula-binario-fosil-release-stale/spec.md
---

# [REGRESIÓN] route-domain-event — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `route-domain-event` |
| Emisor | `execute-process` |
| Acción intentada | `merkle-batch-preseal` |

## Traza de error

```
F-DLT-RELAY-SIN-SUPERVISOR: merkle-batch-preseal failed: Campo obligatorio ausente o inválido: payload
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## 1. Causa raíz verificada — el motor ejecutó un binario que no corresponde a su fuente

La causa **no** es el relay IOTA ni el comportamiento del operador IA. El fallo ocurre en la
validación de entrada de la cápsula `iota-immutable-publisher`, **antes de cualquier rama de
transporte** (simulación, mock HTTP o relay), porque el motor invoca un artefacto compilado
que no corresponde al código fuente vigente.

Cadena causal:

1. `route_domain_core.rs` (pre-sellado batch, ~L2100) invoca `iota-immutable-publisher`
   con `payload` como **array de strings** (una hoja Merkle por evento).
2. `capsules::invoke_tool_capsule_json` resuelve el binario vía
   `capsule_paths::resolve_capsule_native`, que itera `compiled_capsules.profiles` de
   `cumulo.paths.json` en orden `["release", "debug"]` y devuelve **el primero que existe**.
   No comprueba **ninguna** propiedad del artefacto: ni identidad, ni versión, ni frescura.
3. El `release` presente en disco solo acepta `payload` como string; con array devuelve
   `Campo obligatorio ausente o inválido: payload`. El fuente vigente sí soporta array desde
   `d78cafb` (*Ocean DPP V2 y Merkle Batching*, 2026-08-07), refinado en `43f8bf3` (2026-08-10).
4. `route_domain_core.rs` interpreta el fallo como caída del relay, sella
   `last_batch_anchor_error`, encola en `.SddIA/dlt/reanchor-queue/` y emite
   `System_Fracture_Detected` con el prefijo `F-DLT-RELAY-SIN-SUPERVISOR`.

El código está bien; el artefacto en ejecución no. Es **desalineación entre fuente y binario**,
no defecto de lógica.

### 1.1. Reproducción determinista

```bash
cd SddIA
echo '{"action":"publish_immutable_data","network":"testnet","payload":["{\"a\":1}","{\"b\":2}"]}' \
  | SDDIA_LAB_SIMULATE_IOTA=1 ./target/release/iota-immutable-publisher   # FALLA
echo '{"action":"publish_immutable_data","network":"testnet","payload":["{\"a\":1}","{\"b\":2}"]}' \
  | SDDIA_LAB_SIMULATE_IOTA=1 ./target/debug/iota-immutable-publisher     # OK
```

| Binario | `payload` array | `payload` string |
|---------|-----------------|------------------|
| `target/release/iota-immutable-publisher` | **`Campo obligatorio ausente o inválido: payload`** | success |
| `target/debug/iota-immutable-publisher` | success (`merkle_root` 64 hex + 2 pruebas) | success |

La traza reproducida es **idéntica, carácter a carácter**, a la del incidente. Que el fallo se
reproduzca con `SDDIA_LAB_SIMULATE_IOTA=1` prueba que ocurre antes de tocar la red: el estado
del relay es irrelevante para esta fractura.

### 1.2. Magnitud: qué está medido y qué no

Tres criterios distintos dan tres respuestas distintas sobre el mismo parque de binarios. Esto
**no** es un detalle metodológico: es la prueba de que el tiempo no sirve como ancla.

| Criterio | Resultado | Fiabilidad |
|----------|-----------|------------|
| `mtime(release)` < `mtime(debug)` | **17 binarios** | **Ninguna.** Solo indica que alguien recompiló `debug` después; no dice si el código cambió. |
| Último commit del crate posterior a `mtime(release)` | **5 binarios**: `iota-immutable-publisher`, `github-bridge-watcher`, `io-choke`, `sandbox-breacher`, `execute-process` | **Baja.** Con resolución de día, `io-choke` y `sandbox-breacher` comparten fecha binario/commit; `execute-process` se recompiló hoy y aparece marcado por horas. Un commit puede no tocar el código compilado. |
| **Prueba funcional** (contrato de entrada) | **1 binario**: `iota-immutable-publisher` | **Total**, pero no escalable: exige conocer y ejercitar el contrato de cada cápsula. |

**Corrección de la v1.1.0 de este PBI:** afirmé que «17 cápsulas ejecutan código fósil». Es
falso. Diecisiete presentan divergencia de perfiles; una sola tiene fosilidad probada y cinco
son sospechosas por un criterio con ruido demostrado en ambos sentidos. La cifra real de
cápsulas comprometidas **no es determinable** con la instrumentación actual — y esa
indeterminación es, en sí misma, el defecto estructural que este PBI debe cerrar.

### 1.3. Estado de la cola de reanclaje

`.SddIA/dlt/reanchor-queue/` acumula **10 entradas** (27/08 18:21 → 28/08 12:00), todas con
`error_trace: "Campo obligatorio ausente o inválido: payload"`. `try_drain_dlt_reanchor_queue`
las reintenta contra el mismo binario: bucle de fallo indefinido y reemisión periódica de la
fractura.

## 2. Por qué el PBI padre no cerró la fractura

`PBI-FIX-FRACTURE-6a49e0ad310e` (cerrado, PR #210) atribuyó dos causas:

| Causa declarada en el cierre | Estado | Evidencia |
|------------------------------|--------|-----------|
| **Física**: relay IOTA sin centinela → publisher sin payload válido | **Refutada** | El fallo se produce en la validación de entrada local. Se reproduce con `SDDIA_LAB_SIMULATE_IOTA=1`, ruta que jamás alcanza `publish_via_relay`. |
| **Operativa**: operador IA continuó sin escalado Kintsugi (`prompt_adjustment`) | **Válida pero ortogonal** | Corrige la conducta ante el colapso; no elimina el colapso. |
| «La fractura es histórica (pre-relay); no reproduce» | **Falsa** | Reproducible de forma determinista con un comando. |

Lección estructural: el mensaje de error del motor (`F-DLT-RELAY-SIN-SUPERVISOR`) **nombra una
hipótesis, no un hecho medido**, y esa etiqueta especulativa dirigió dos ciclos de diagnóstico
hacia la capa equivocada.

## 3. Impacto

- **DLT sin anclaje**: ningún evento se sella en batch; la cadena de custodia inmutable queda
  interrumpida desde el 27/08.
- **Storm de fracturas**: cada barrido reemite `System_Fracture_Detected`; el gasto se contiene
  solo gracias a la idempotencia por contenido del PR #217.
- **Superficie de confianza indeterminada**: no existe forma de saber, hoy, qué cápsulas están
  sirviendo comportamiento antiguo. Cualquier fix mergeado puede ser inerte en runtime sin que
  nadie lo note hasta que un contrato rompa.
- **Diagnóstico envenenado**: la etiqueta de fricción falsa consume ciclos y produce cierres
  documentales que no cierran nada.

## 4. El ancla ya existe en el repositorio, pero fuera del motor

`SddIA/scripts/build-release-bundle.sh` implementa desde `L-BUNDLE-STALE v2` exactamente el
patrón que este PBI necesita:

| Función | Qué hace |
|---------|----------|
| `_sddia_source_digest` (L147-180) | SHA-256 sobre la lista ordenada (`LC_ALL=C`) de pares `ruta<TAB>sha256` de `Cargo.toml`, `build.rs` y `src/**` del crate, más `SddIA/Cargo.toml` y `SddIA/Cargo.lock`. **Determinista y portable**: depende del contenido versionado, no de la máquina. |
| `_sddia_elf_digest` (L182-184) | SHA-256 del ELF. Identidad **local** del artefacto. |
| `_sddia_write_witness` (L186-194) | Escribe `{binario}.sha256` con `source_sha256` + `elf_sha256`. |
| `_sddia_verify_witness` (L196-220) | Exige coincidencia de ambos; si no, aborta con `L-BUNDLE-STALE: cicatriz divergente`. |

Comprobado: `SDDIA_BUNDLE_DIGEST_ONLY=iota-immutable-publisher` devuelve hoy
`sha256:4b5ec550a0b3f0c52c65ce4b12171db056d4bca466394de3dba81f0053eb367e`. La primitiva
funciona y está disponible para cualquier cápsula.

Tres huecos la dejan inoperante donde importa:

1. **Solo cubre `CONSUMER_BINS`** (8 binarios: `kalma2-bridge`, `telegram-watcher`,
   `telegram-gateway`, `event-sweeper`, `event-watcher`, `send-telegram-notification`,
   `execute-process`, `email-watcher`). `iota-immutable-publisher` —la cápsula que colapsó—
   **no está en la lista**.
2. **Solo se ejecuta en el circuito de bundle** (`--skip-build`). `resolve_capsule_native`, que
   es el camino real de ejecución de todas las cápsulas, jamás consulta el testigo.
3. **Granularidad de workspace.** Auditados hoy los 8 testigos existentes: **los 8 divergen**,
   y en 6 de ellos el ELF coincide (`elf_ok`) mientras el digest de fuente no. La causa no es
   que cambiara su código: `telegram-gateway` no se toca desde `1051cd5` (2026-06-02), pero
   `SddIA/Cargo.lock` (28/08) y `SddIA/Cargo.toml` (27/08) entran en el digest de **todas** las
   cápsulas. Un bump de dependencia invalida el parque completo.

El hueco 3 es la trampa del diseño: promover esta primitiva al motor **tal cual**, con política
de fallo duro, detendría hoy el sistema entero.

## 5. Criterios de aceptación

### Bloque I — Aduana de apertura de ciclo (prerrequisito, §8)

Sin esto no puede abrirse el ciclo `bug-fix` de este PBI ni el de ninguna otra fractura.

- **CA1** — `workspace_init::porcelain_path_from_line` **desescapa** el quoting de git antes de
  comparar: secuencias octales (`\303\223`) y escapes C (`\t`, `\n`, `\"`, `\\`) resueltos a
  UTF-8. Alternativa preferible: `status -z` (terminador `NUL`, inmune a `core.quotePath`) vía
  `skill:git-manager`. La conversión `replace('\\', "/")` deja de aplicarse sobre rutas citadas,
  porque corrompe los escapes.
- **CA2** — Test unitario con la línea literal de §8.1: un PBI llamado `[REGRESIÓN] … sistémica`
  declarado en `pbi_ref` **no** aparece como dirty fuera de scope.
- **CA3** — Un colapso de fase por precondición del motor emite `System_Fracture_Detected` o
  queda excluido por norma explícita. Hoy no hace ninguna de las dos cosas.

### Bloque II — Patrón de Anclaje de Ejecución (núcleo)

- **CA4** — El genoma `{name}.md` de cada cápsula indexada declara `source_sha256` en su
  frontmatter YAML. Es el **SSOT de qué debe ejecutarse**. Viaja en el mismo commit que el
  código. No se declara hash de ELF en el genoma (ver §6.2).
- **CA5** — `resolve_capsule_native` deja de devolver «el primero que existe». Selecciona el
  artefacto que **satisface el contrato**: testigo local presente, `elf_sha256` del testigo igual
  al SHA-256 del ELF en disco, y `source_sha256` del testigo igual al declarado en el genoma. El
  orden de `profiles` deja de ser criterio de verdad y pasa a ser mero orden de búsqueda.
- **CA6** — Si ninguna candidata satisface el contrato: `Err("capsule-stale-hash: <name> — genoma
  <sha> / artefacto <sha o ausente>")`. Sin fallback silencioso a otro perfil, sin degradación.
- **CA7** — La escritura del testigo y la actualización de `source_sha256` en el genoma son
  **automáticas**, integradas en la línea de montaje (build + `entity-manager` en el cierre del
  ciclo). Prohibida la mutación manual del campo: sería un ancla muerta más (§6.3).
- **CA8** — Cobertura: todas las cápsulas indexadas (`SddIA/{tools,skills,daemons,engine}`), no
  solo `CONSUMER_BINS`. `iota-immutable-publisher` incluida.
- **CA9** — Granularidad del digest resuelta según laudo de §6.4, con test que demuestre que un
  cambio en una cápsula **no** invalida a las demás.
- **CA10** — Coste medido: sobrecoste por invocación documentado en `execution.md`, con caché
  en `.SddIA/` (jamás en `cumulo.paths.json`, que es SSOT de rutas y no almacén de estado
  mutable) e invalidación por `(ruta, mtime, tamaño)` del ELF.
- **CA11** — Tests unitarios en tempdir: (a) artefacto conforme → se sirve; (b) ELF alterado un
  byte → `capsule-stale-hash`; (c) testigo ausente → `capsule-stale-hash`; (d) genoma sin
  `source_sha256` → política declarada y verificada, sin panic; (e) release no conforme y debug
  conforme → se sirve **debug**, probando la ceguera de perfiles.

### Bloque III — Higiene y trazabilidad

- **CA12** — La traza de fractura del anclaje batch declara el hecho medido: ruta del binario
  invocado, perfil y causa literal de la cápsula. `F-DLT-RELAY-SIN-SUPERVISOR` se reserva para
  fallos verificados de relay; el rechazo de contrato de entrada usa `friction_id` propio.
- **CA13** — Eliminado el bloque `// #region agent log` de `route_domain_core.rs` (L1136-1159),
  que escribe en `/home/racso/Proyectos/SddIA/.cursor/debug-478d0f.log`: ruta absoluta de la
  máquina del Vértice Biológico dentro del Core (viola `A-CORE-AGNOSTICO`).
- **CA14** — Recompilación conforme del parque y publicación del inventario resultante en
  `execution.md`: por cápsula, `source_sha256` del genoma y veredicto de la aduana. Este paso
  **precede** a la activación del gate.
- **CA15** — El pre-sellado batch ancla con éxito (`merkle_root` + `merkle_proofs`) y ningún
  evento nuevo queda con `last_batch_anchor_error`.
- **CA16** — Las 10 entradas de `.SddIA/dlt/reanchor-queue/` se drenan o se cierran con motivo
  documentado.

### Bloque IV — Cierre

- **CA17** — Argos `APTO` en `docs/fixes/{persist_ref}/validacion.md`, con `pbi_archived: true`.
- **CA18** — Este PBI movido a `docs/todos/done/` **en la misma rama del PR**, y el padre
  `PBI-FIX-FRACTURE-6a49e0ad310e` anotado con la corrección de su causa raíz declarada (sin
  reabrirlo).

## 6. Laudo del Vértice Biológico — Patrón de Anclaje de Ejecución

### 6.1. Revocación del laudo anterior

La v1.2.0 de este PBI adoptó un gate de frescura por comparación de `mtime`. **Queda revocado.**
La evidencia de §1.2 lo desautoriza: sobre el mismo parque, el criterio temporal produce 17, 5 o
1 según cómo se mire, con falsos positivos (`execute-process`, recompilado hoy) y falsos
negativos estructurales (un `touch` reordena el veredicto sin cambiar una línea de código). El
tiempo no es identidad.

### 6.2. Arquitectura adoptada

Tres piezas, en el orden en que operan:

| # | Pieza | Dónde vive |
|---|-------|-----------|
| 1 | **Declaración** — `source_sha256` de la cápsula | Genoma `{name}.md`, frontmatter YAML. Versionado, viaja con el código. |
| 2 | **Aduana** — verificación previa al `spawn` | `resolve_capsule_native`. Colapsa con `capsule-stale-hash` si el artefacto no acredita su origen. |
| 3 | **Orquestación** — actualización del ancla | Línea de montaje: build escribe el testigo; el cierre del ciclo actualiza el genoma vía `entity-manager`. Nunca a mano. |

El orquestador queda **ciego a los perfiles**: no busca «el release» ni «el debug», busca un
artefacto que acredite descender de la fuente declarada. Si no existe, no se ejecuta.

**Corrección al planteamiento original.** La propuesta declaraba `compiled_target_hash` (hash del
ejecutable) en el genoma y afirmaba que «el código fuente, el binario y el hash viajan juntos en
el mismo commit». Dos hechos del repositorio lo impiden:

1. **El binario no viaja.** `SddIA/.gitignore:1` ignora `target/`. En cualquier clon, CI o
   instancia consumidora el ELF se produce localmente.
2. **La compilación no es reproducible bit a bit.** No hay `rust-toolchain.toml`, de modo que ni
   siquiera la versión del compilador está fijada; a eso se suman rutas absolutas embebidas,
   información de depuración y orden de símbolos. Dos máquinas honestas producen ELF distintos a
   partir del mismo fuente.

Un `compiled_target_hash` en el genoma bloquearía, por tanto, toda ejecución fuera de la máquina
que lo generó. Por eso el reparto es: **el genoma ancla la fuente** (portable, versionable) y **el
testigo local ancla el artefacto** (`elf_sha256`, no versionado, junto al binario). La cadena
`genoma → testigo → ELF` da la garantía completa sin exigir compilación reproducible.

### 6.3. Por qué el ancla no puede ser `version` ni mantenerse a mano

El genoma `SddIA/tools/iota-immutable-publisher.md` declara `version: "1.0.0"` desde `0295cfb`
(2026-05-14). Desde entonces el contrato de entrada cambió dos veces (`d78cafb`, `43f8bf3`) y la
versión **no se movió**: `d78cafb` incluso editó ese mismo `.md` sin tocarla, y `43f8bf3` cambió
el crate sin abrir el `.md`. Un campo que el humano debe recordar actualizar es un ancla muerta
desde el primer despiste. De ahí CA7: el valor lo escribe la línea de montaje o no vale nada.

### 6.4. Decisión pendiente — granularidad del digest de fuente

`_sddia_source_digest` incluye hoy `SddIA/Cargo.toml` y `SddIA/Cargo.lock` completos, de modo que
cualquier bump de dependencia invalida el parque entero (evidencia: los 8 testigos divergen hoy,
6 de ellos con el ELF intacto). Con política de fallo duro, eso es una parada total.

| Opción | Entrada del digest | Efecto |
|--------|--------------------|--------|
| **A** | Crate + cierre transitivo de dependencias **locales** + entradas del lockfile correspondientes **solo** a sus dependencias | Invalidación mínima y correcta. Requiere resolver el grafo (`cargo metadata`). Mayor coste de implementación. |
| **B** | Crate + lockfile completo (comportamiento actual) | Trivial de implementar; recompilación global ante cualquier bump. Honesto pero caro. |
| **C** | Solo crate (`Cargo.toml`, `build.rs`, `src/**`) | Barato y estable, pero ciego a un cambio de dependencia que sí altera el comportamiento compilado. |

Recomendación: **A**, con **B** como comportamiento transitorio si el grafo no está disponible. (clarificado por Racso como OK)
Sin este laudo, CA9 queda bloqueado.

### 6.5. Secuencia de ataque (obligatoria)

1. **Bloque I** — desescape del gate `dirty-worktree`. Sin él no se puede abrir el ciclo.
2. **CA14** — recompilar el parque y levantar el inventario conforme.
3. **Bloque II** — declarar anclas, implementar la aduana y **entonces** activar el fallo duro.
4. **Bloque III / IV** — higiene, drenaje del DLT y cierre.

Invertir 2 y 3 deja el sistema inoperante: la aduana rechazaría todo artefacto aún no sellado.

## 7. Fuera de alcance

- Reimplementar el relay IOTA o su centinela (Kaizen DLT #208, ya en `main`).
- Barrido de los dead-letters de `mayeuta.enrich-fracture-pbi-kaizen` (PBI Kaizen fan-out).
- Compilación reproducible bit a bit y fijación de toolchain: deseable, pero el patrón adoptado
  está diseñado precisamente para no depender de ella.
- Migración WASI de las cápsulas afectadas.
- Reapertura del PBI padre: se corrige su causa declarada, no su estado.

## 8. Bloqueo de apertura de ciclo — gate `dirty-worktree` inoperable

Registrado el 2026-08-28 al intentar abrir el ciclo `bug-fix` de este PBI. **Dos intentos, ambos
abortados en la fase «Inicialización de Espacio de Trabajo»**; ciclo no abierto por laudo del
Vértice Biológico (detención sin bypass).

| Intento | `execution_id` | Inputs | Resultado |
|---------|----------------|--------|-----------|
| 1 | `e6b44206-68b2-4381-ae15-86dbf25980d0` | sin `pbi_ref` | `dirty-worktree` — no concluyente: habría fallado igual con él |
| 2 | `13161205-2a2a-4320-9953-554e18a1f7c5` | **con** `pbi_ref` exacto | `dirty-worktree` — **defecto del gate** |

### 8.1. Causa: quoting octal de git no desescapado

`core.quotePath` está activo por defecto, de modo que git cita y escapa todo byte no ASCII:

```console
$ git status --porcelain
 M "docs/todos/pending/[REGRESI\303\223N] route-domain-event \342\200\224 fractura sist\303\251mica (6a49e0ad310e)-R1.md"

$ git -c core.quotePath=false status --porcelain
 M "docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md"
```

`porcelain_path_from_line` (`workspace_init.rs` L82-105) retira las comillas pero **no
desescapa**, y a continuación `replace('\\', "/")` convierte las barras del escape en separadores
de ruta. La ristra que llega a `path_in_scope` (L107-120) es
`docs/todos/pending/[REGRESI/303/223N] route-domain-event /342/200/224 fractura sist/303/251mica (…)-R1.md`,
que jamás iguala el `pbi_ref` en UTF-8.

**Alcance real:** ningún artefacto con carácter no ASCII puede declararse en scope. Por
construcción, **todos** los PBI de fractura contienen `[REGRESIÓN]`/`[FIX]`, el guion largo `—` y
`sistémica`. El gate `L-DIRTY-INIT`, concebido como higiene, **impide abrir el ciclo de corrección
de cualquier fractura** salvo mediante su escape `SDDIA_LAB_ALLOW_DIRTY=1`, que desactiva la
higiene por completo en vez de reconocer el scope declarado.

### 8.2. El colapso no produjo evento de dominio

Recuento por `event_id` único sobre las ocho carpetas de `./.events`: **748 ficheros, 380 eventos
`System_Fracture_Detected` únicos** — 364 aparecen a la vez en `processed` y `dead-letter`, 14
solo en `processed` y 2 además en `processing`. Reparto por proceso emisor:

| Proceso | Eventos únicos |
|---------|----------------|
| `email-watcher` | 102 |
| `telegram-watcher` | 102 |
| `github-bridge-watcher` | 93 |
| `event-watcher` | 39 |
| `event-sweeper` | 28 |
| `route-domain-event` | 11 |
| `kalma2-bridge` | 5 |
| **`bug-fix`** | **0** |

*(Corrección de la v1.2.0: allí escribí «746 fracturas, todas de `route-domain-event`». La cifra
era de ficheros, no de eventos, y la atribución era una extrapolación indebida de una muestra
ordenada por fecha: la mayoría son centinelas reportando ciclos omitidos.)*

El fallo del gate viaja como `Err` de precondición dentro del `execution_report` y muere ahí.
Cúmulo no materializó PBI y Mayeuta no enriqueció nada: el escalado Kintsugi quedó enteramente a
cargo del operador. Un defecto que bloquea la apertura de todos los ciclos de fractura es, hoy,
invisible para el bus.

### 8.3. Residuo del intento

`docs/fixes/capsula-binario-fosil-release-stale/` fue creado por `workspace_init` antes del aborto
(`objectives.md`, `_agent_handoff.md`). Se conserva intacto como evidencia y como `persist_ref`
reservado. La rama `fix/capsula-binario-fosil-release-stale` **no** llegó a crearse: el aborto
precede al `checkout`.

### 8.4. Nota de entorno (no es defecto)

En el intento 2, tras el aborto de fase 1, la fase «Ejecución» falló con
`ENOTFOUND api2.cursor.sh`: el runtime remoto de agentes no es alcanzable en este entorno. La vía
prevista es el relevo IDE (`SDDIA_AGENT_RELAY_IDE=1`, laudo `L-RELAY-FLAG`), que deja las fases de
agente en `simulated` para materialización local. No se aplicó, por laudo.

## 9. Conclusión Analítica y Propuesta Evolutiva

_Síntesis Mayeuta pendiente. El diagnóstico físico de §1 está cerrado con evidencia reproducible y
la arquitectura de §6 tiene laudo. Lo que Mayeuta debe dictaminar es el veredicto evolutivo sobre
§2 y §8.2: por qué el sistema aceptó como «causa raíz resuelta» una hipótesis nunca verificada
contra el artefacto en ejecución, y por qué un defecto que bloquea la apertura de todos los ciclos
de fractura no genera señal alguna en el bus._

## Criterio de cierre

- [x] Bloque I completo: el ciclo `bug-fix` de este PBI abre sin `SDDIA_LAB_ALLOW_DIRTY`
- [x] Parque recompilado e inventariado (CA14) antes de activar la aduana
- [x] Patrón de Anclaje operativo: genoma declara, motor verifica, línea de montaje actualiza
- [x] DLT anclando en batch y cola de reanclaje drenada
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/` en la misma rama del PR
