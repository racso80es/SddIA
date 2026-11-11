---
id: man-32-lineage-man-43-remediation-20261111
uuid: "02f88f2d-4891-44af-8d7e-735f0c6c2b3d"
type: audit
version: 1.0.0
created: "2026-11-11"
updated: "2026-11-11"
tracker_ref: MAN-43
parent_ref: MAN-32
scope: "Implementación MAN-32 y linaje MAN-35..MAN-43; énfasis en hito H1 de MAN-43"
verdict: NO_APTO_TOTAL
milestone_h1: APTO_CONDICIONAL
audited_merge: f60ae1d461b4a0b00e15cc85c01c0636f3ac0583
audited_pr: https://github.com/racso80es/SddIA/pull/453
---
# Auditoría MAN-32 → MAN-43 — desacople EventManager

## Dictamen

La implementación de MAN-32 creó correctamente una frontera aditiva y su linaje materializó el cutover productivo, la configuración de instancia, los arneses de prueba y el sello lógico. Sin embargo, el objetivo final de MAN-43 —`EventManagerPort` 2.0 como frontera única y cero runtime EDA sobre filesystem— **no está alcanzado**.

El PR #453 es válido como **hito H1 parcial**: introdujo `EventUri`, `PublishSeal` v2 y el gate SemVer `^2.0.0`, y sus checks remotos fueron verdes. No acredita H1 completo ni H2–H6. El veredicto del PBI completo permanece **`NO_APTO_TOTAL`**.

## Linaje auditado

```text
MAN-32
└─ MAN-33
   └─ MAN-35
      ├─ MAN-36
      │  ├─ MAN-39
      │  └─ MAN-40
      │     └─ MAN-43
      ├─ MAN-37
      └─ MAN-38
         └─ MAN-42
```

MAN-33 no dispone de bundle documental local; solo puede auditarse por las referencias de MAN-35. MAN-37, MAN-39 y MAN-42 son ramas laterales del linaje, pero afectan pruebas, lifecycle y contención del mismo runtime.

## Hallazgos

| ID | Severidad | Hallazgo | Evidencia | Impacto |
|----|-----------|----------|-----------|---------|
| **F-01** | P0 | Redb purga y consulta entregas/trazas mediante `contains(event_id)` | `infrastructure/adapters/redb_event_adapter/src/lib.rs` (`purge_event`, `list_deliveries`) | Un ID opaco corto puede seleccionar o borrar registros de otro evento. |
| **F-02** | P0 | Runtime exige contrato `2.0.0`, pero taxonomía, bindings, schema, README y Constitución permanecen en `1.2.0`/v1 | `contract.rs`; `capability-bindings.md`; `capability-taxonomy.md`; `CONSTITUTION_CORE.md` | SSOT contractual fracturado; resolución DI y consumidores pueden discrepar. |
| **F-03** | P0 | Continúa la doble materialización puerto + `.events/` | `engine/actions.rs::emit_pr_presented`; watcher; sweeper; workspace-server | MAN-43 AC-4/5/6/9 no cumplidos; coexistencia de dos fuentes de verdad. |
| **F-04** | P1 | `PublishEcstRequest` no transporta `EventFamily`/`EventUri` y Redb no persiste familia | `sddia-core/.../contract.rs`; adapter Redb | `list_events(family)` no puede ser correcto. |
| **F-05** | P1 | `list_events` ignora familia, estado y cursor | default de `EventManagerPort` y adapter Redb | API v2 declara filtros que no aplica. |
| **F-06** | P1 | `list_terminal_events` solo devuelve DLQ y ningún adapter implementa terminales completos | `sddia-core/.../port.rs` | El sweeper no puede abandonar filesystem. |
| **F-07** | P1 | La suite contractual no cubre entregas, terminales, familia, estado ni paginación | `sddia-core/.../contract_suite.rs` | Verde falso: el contrato puede pasar sin cumplir L-6. |
| **F-08** | P1 | Idempotencia de publicación depende del texto `contains("duplicate")` | `runtime_bridge.rs::publish_ecst` | Contrato frágil; errores no relacionados pueden ocultarse. |
| **F-09** | P1 | Watcher combina `claim_pending` con descubrimiento `read_dir` y payload `event_file_path` | `daemons/event-watcher/src/main.rs` | Riesgo de carrera/doble despacho y dependencia física residual. |
| **F-10** | P1 | Periféricos mantienen rutas o escrituras FS | Kalma2, workspace-server telemetry, GitHub bridge | AC-8/9 abiertos. |
| **F-11** | P1 | `sddia-event-store`, `eda_port_legacy` y allowlists siguen vivos | workspace Cargo; `gate_event_manager_port.rs` | AC-10/11 abiertos. |
| **F-12** | P2 | El cierre documental mezcla sello H1 con estado del feature completo | `validacion.md`, `implementation.md`, `execution.md` | Riesgo de interpretar el merge H1 como `APTO_TOTAL`. |

## Evidencia crítica

### F-01 — selección por substring

`purge_event` acepta claves que empiecen por `{event_id}:` **o que contengan** el ID; las trazas se seleccionan exclusivamente con `contains`. `list_deliveries` repite el patrón para evento y suscriptor. Con identidad opaca, `abc` y `xabcx` son valores válidos y no equivalentes.

La corrección debe reemplazar inferencias textuales por registros tipados:

```text
DeliveryRecord {
  event_id,
  subscriber_id,
  delivery_id,
  status,
  attempt
}
```

La clave física puede ser compuesta, pero los filtros y la purga deben comparar campos por igualdad exacta.

### F-02 — split contractual

Estado observado:

```text
runtime kernel:       eda.event_manager 2.0.0 / ^2.0.0
bindings/taxonomía:   eda.event_manager 1.2.0
schema materializado: eda.event_manager.v1.schema.json
schema referenciado:  eda.event_manager.v2.schema.json
```

H1 no puede declararse completo hasta que la mutación gobernada alinee el Genoma y la suite pruebe rechazo explícito de v1.

### F-03 — doble fuente

`emit_pr_presented` publica en el puerto y después escribe `.events/domain/{event_id}.json`. El watcher reclama por puerto, pero también recorre directorios. El sweeper descubre terminales por FS. Workspace-server emite telemetría con `fs::write`. La coexistencia es deuda activa, no una excepción histórica.

## Matriz MAN-43 AC

| AC | Estado auditado |
|----|-----------------|
| AC-1 | PARCIAL — núcleo v2 presente; schema/suite/ops incompletos |
| AC-2 | PARCIAL — `EventUri` existe; familia no persiste en el evento almacenado |
| AC-3 | PARCIAL — `PublishSeal` limpio; APIs productivas conservan `event_file_path`/paths |
| AC-4 | NO_APTO |
| AC-5 | NO_APTO |
| AC-6 | NO_APTO |
| AC-7 | PARCIAL |
| AC-8 | NO_APTO |
| AC-9 | NO_APTO |
| AC-10 | NO_APTO |
| AC-11 | NO_APTO |
| AC-12 | PARCIAL — CI H1 verde; no existe evidencia del alcance total |
| AC-13 | NO_APTO |
| AC-14 | PARCIAL — evolution H1 presente; convergencia constitucional pendiente |

## Olas de remediación obligatorias

| Ola | Propósito | Hallazgos | Gate de salida |
|-----|-----------|-----------|----------------|
| **O0 — Contención de integridad** | Corregir claves/filtros Redb e idempotencia tipada antes de ampliar consumidores | F-01, F-08 | Tests de colisión de IDs y purga exacta verdes |
| **O1 — Contrato v2 íntegro** | Persistir familia/estado, completar operaciones, schema/bindings/taxonomía y suite | F-02, F-04…F-07 | Contract suite mock/Redb/externo + rechazo v1 |
| **O2 — Estado y testigos** | Llevar entregas, processing/processed/DLQ detrás del puerto | F-03, F-06 | Cero escritura de testigos FS |
| **O3 — Cutover centinelas** | Watcher y sweeper solo por puerto | F-03, F-09 | Cero `read_dir`/`is_file`/`event_file_path` en ambos |
| **O4 — Periferia limpia** | Actions, Kalma2, workspace-server y GitHub sin fallback FS ni paths físicos | F-03, F-10 | Gates AC-8/9 verdes |
| **O5 — Demolición** | Retirar store/legacy/allowlists y migrar matrices | F-11 | Cero dependencia o allowlist legacy |
| **O6 — Convergencia y cierre** | README, Constitución, Cúmulo, evolution y auditorías | F-02, F-12 | AC-1…14 con evidencia y `APTO_TOTAL` |

## Reglas de ejecución

1. O0 bloquea O2–O5: no extender el uso de una persistencia con selección ambigua.
2. O1 bloquea O3: watcher/sweeper no cortan FS hasta disponer de consultas y terminales completas.
3. O2 y O4 pueden avanzar en paralelo después de O1.
4. O5 solo comienza cuando no quedan consumidores de `sddia-event-store`.
5. El sello histórico del PR #453 no se revoca; se clasifica como **milestone H1 parcial**.
6. `APTO_TOTAL` pertenece a O6 y exige evidencia de todos los AC, no solo checks verdes del PR H1.

## Veredicto final

```yaml
man_32_lineage: APTO_CON_DEUDA_EVOLUTIVA
man_43_h1_milestone: APTO_CONDICIONAL
man_43_full_feature: NO_APTO_TOTAL
next_wave: O0_CONTENCION_INTEGRIDAD
```
