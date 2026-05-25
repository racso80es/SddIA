---
document_id: PBI-KAIZEN-EDA-AUDIT-NO-BUS-DEPENDENCY
title: "[Kaizen] Validación genómica EDA sin dependencia del bus — correlación durable"
format: markdown
version: "1.0.0"
created: "2026-05-25"
status: cerrado
priority: alta
process: feature
closed_via: PBI-KAIZEN-EDA-COVERAGE-SSOT-BUS-ISOLATION
closed: "2026-05-25"
superseded_by: PBI-KAIZEN-EDA-COVERAGE-SSOT-BUS-ISOLATION
superseded_by_path: docs/todos/done/[Kaizen] EDA cobertura durable, aislamiento bus y smoke e2e — SSOT eda-coverage.md
branch_name: feat/eda-coverage-ssot-bus-isolation
feature_ref_target: docs/features/eda-coverage-ssot-bus-isolation
upstream:
  - docs/todos/pending/[Kaizen] deuda EDA orphan_count — correlación processed y backfill pre-commit.md
  - docs/features/eda-orphan-debt-precommit/
related:
  - SddIA/scripts/qa/audit-entity-eda-coverage.py
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/library/norms/features-documentation-pattern.md
  - docs/features/eda-domain-entities-splus/spec.md
blocks: "Cierre arquitectónico de aduana genómica desacoplada del ciclo de vida efímero del bus EDA"
---

# [Kaizen] Validación genómica EDA sin dependencia del bus — correlación durable

> **Cerrado** vía consolidación e implementación en [`PBI-KAIZEN-EDA-COVERAGE-SSOT-BUS-ISOLATION`](../done/[Kaizen]%20EDA%20cobertura%20durable,%20aislamiento%20bus%20y%20smoke%20e2e%20—%20SSOT%20eda-coverage.md) (rama `feat/eda-coverage-ssot-bus-isolation`).

**Estatus:** Cerrado (consolidado)  
**Jurisdicción:** Yunque Operativo · Aduana Argos / pre-commit genómico  
**Precedencia:** fix `eda-orphan-debt-precommit` (workaround F1 + retención cabeceras `Domain_Entity_Created`)

---

## 1. Casuística (por qué existe este PBI)

Durante el cierre de **PBI-KAIZEN-EDA-ORPHAN-DEBT** se confirmó empíricamente:

| Fase | Comportamiento | Consecuencia |
|------|----------------|--------------|
| Baseline con eventos en `pending/` | `--scan` → `orphan_count: 0` | Enmascara deuda real |
| Fix C2 (leer `processed/` + `processing/`) | Correlación ampliada | Insuficiente si el bus purga cabeceras |
| `event-watcher --once` post-backfill | `archive_event_after_sweep` elimina instancias ECST | `--scan` → `orphan_count: 43` |
| Workaround opción 2 | Retener cabecera `processed/` solo para `Domain_Entity_Created` | Restaura V4 en 0, pero **acopla la aduana al bus** |

**Laudo:** la validación genómica (`pre-commit`, `delivery-close-cycle` fase Aduana EDA) **no debe depender** de que eventos ECST sigan presentes en carpetas efímeras del bus (`pending` / `processing` / `processed`). El bus es transporte y orquestación; la correlación entidad ↔ sello debe ser **durable e independiente del barrido del watcher**.

---

## 2. Declaración de propósito

Diseñar e implementar una fuente de verdad de correlación genómica que permita a `audit-entity-eda-coverage.py --scan` (y gates derivados) verificar cobertura EDA **sin** exigir instancias vivas en el bus local.

### Principio rector

```text
Aduana genómica  →  consulta correlación durable (índice / manifiesto / DLT)
Bus EDA          →  orquestación reactiva; NO fuente única de verdad para orphan_count
```

---

## 3. Opciones de diseño (evaluar en clarify)

| Opción | Descripción | Pros | Contras |
|--------|-------------|------|---------|
| **A** | Registro versionado `SddIA/core/eda-entity-correlation-index.json` (o tabla en índices existentes) | Determinista; git-auditable | Mutación explícita en emit/sync |
| **B** | Manifiesto Merkle + acta como prueba de cobertura; scan lee actas ancladas | Ya existe Fase C | Múltiples actas; merge de lotes |
| **C** | Consulta DLT/IOTA como SSOT post-anclaje | Soberanía inmutable | Latencia; lab vs prod |
| **D** | Híbrido: índice local + digest Merkle opcional | Balance operativo / auditoría | Más touchpoints |

**Criterio de elección:** `--scan` y pre-commit deben pasar tras `event-watcher --once` **sin** retener cabeceras en `processed/` por workaround.

---

## 4. Backlog atómico (borrador)

| Hito | Objetivo | Criterio |
|------|----------|----------|
| **H1** | Laudo arquitectónico en `clarify.md` | Opción elegida documentada; retención cabeceras marcada como transitoria |
| **H2** | SSOT correlación durable | Artefacto(s) bajo `SddIA/core/` o norma explícita |
| **H3** | Refactor `audit-entity-eda-coverage.py` | `--scan` usa SSOT; bus opcional como hint, no gate |
| **H4** | Integración emit/sync | `emit-domain-mutation` / `sync-entity-index` actualizan correlación |
| **H5** | Poda workaround | Eliminar retención forzada en `archive_event_after_sweep` tras migración |
| **H6** | V4 sostenido sin bus | watcher + scan → `orphan_count: 0` sin cabeceras retenidas |

---

## 5. Relación con fix en curso

| Artefacto | Rol |
|-----------|-----|
| `eda-orphan-debt-precommit` | Mitigación táctica (C2 + retención + backfill Merkle) |
| **Este PBI** | Solución estructural: aduana no dependiente del bus |

Prohibido declarar cerrada la deuda arquitectónica cuando solo exista el workaround de retención de cabeceras.

---

## 6. Criterios de aceptación (Definition of Done)

| ID | Criterio |
|----|----------|
| KA-CA1 | `--scan` correlaciona vía SSOT durable, no solo `iter_bus_event_files()` |
| KA-CA2 | `pre-commit` PASS tras watcher sin retener cabeceras `processed/` |
| KA-CA3 | `delivery-close-cycle` Aduana EDA sin excepción `_backfill_manifest_active` frágil |
| KA-CA4 | Workaround retención podado o detrás de flag deprecado |
| KA-CA5 | `validacion.md` APTO + PBI en `done/` (un PR) |

---

## 7. Referencias

- Ejecución empírica: `docs/features/eda-orphan-debt-precommit/execution.md` § V4 / impasse
- Norma Ruido de Sistema: `features-documentation-pattern` § Cobertura EDA genómica
- Backfill Fase C: `eda-domain-entities-splus/spec.md` §6


## 8. PROPUESTA A REFINAR
[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. NO EXPLANATIONS, ONLY CODE.]

CONTEXTO ESTRATÉGICO:
Ejecución del PBI: "Validación genómica EDA sin dependencia del bus — correlación durable".
Misión: Desacoplar la auditoría (Aduana genómica) del canal de eventos efímero (.SddIA/events/), implementando un Índice de Cobertura Genómica inmutable (SSOT) alojado en la jurisdicción del Core.

ARTEFACTOS Y MUTACIONES REQUERIDAS:

1. NUEVO SSOT: `SddIA/core/eda-coverage.json`
   - Crea este archivo inicial con la estructura de un Mapa Topológico de Estado (diccionario clave-valor, no un array cronológico).
   - Estructura estricta requerida:
     {
       "coverage_matrix": {
         "<entity_id>": {
           "is_covered": true,
           "last_emitted_event": "<event_type>",
           "last_hash": "<hash>",
           "correlation_timestamp": "<iso_8601_date>"
         }
       }
     }

2. MOTOR DE EMISIÓN: Acción `emit-domain-mutation` (y/o `sync-entity-index`)
   - Modifica el flujo para que ejecute una Doble Fase Atómica:
     * Fase A: Operación Upsert (crear o sobrescribir, nunca append) en `SddIA/core/eda-coverage.json` para el `entity_id` correspondiente con el hash del nuevo evento.
     * Fase B: Inyección física de la chispa (archivo del evento) en `.SddIA/events/pending/`.

3. ADUANA GENÓMICA: `audit-entity-eda-coverage.py` (ejecutado en pre-commit)
   - ELIMINAR la dependencia tóxica de lectura física del bus: purgar `iter_bus_event_files()` o cualquier lectura hacia `.SddIA/events/processed/`.
   - Implementar la validación escaneando ÚNICAMENTE el bloque de la entidad dentro de `SddIA/core/eda-coverage.json` para comprobar que `is_covered` es `true` y el hash está anclado.

4. BARRENDERO: `event-sweeper.py` (o función `archive_event_after_sweep`)
   - Eliminar el workaround actual que retiene cabeceras forzosamente.
   - El barrido debe ser absoluto (Sweep vacío): el archivo se destruye/archiva sin dejar rastro en `processed/` para la aduana.

RESTRICCIONES DURAS:
- Mantenimiento estricto de nomenclatura internacional (inglés) para el código y el JSON.
- No alteres la lógica de negocio ajena al ecosistema EDA.
- Presenta únicamente las modificaciones de código (diffs o bloques completos reescritos).


  [EXECUTE AS RAW KERNEL. NO EXPLANATIONS. ONLY CODE.]

CONTEXTO: Desacoplamiento de Auditoría EDA y Aislamiento de Entornos.

1. SSOT DE CORRELACIÓN (Durable):
   - Implementa `SddIA/core/eda-coverage.json` con la estructura de Mapa Topológico (diccionario <entity_id>:<data>).
   - Este archivo será la única fuente para la Aduana.

2. PARAMETRIZACIÓN DE RUTA:
   - Refactoriza todos los módulos que acceden a eventos (`emit-domain-mutation`, `event-sweeper`, `audit-entity-eda-coverage.py`) para que utilicen la variable de entorno `EVENT_BUS_PATH`.
   - Por defecto (producción), esta variable es `./events/`.
   - En el entorno de pruebas, esta variable será inyectada mediante la carga de `.env.test` apuntando a `./.tmp/events_test/`. El resto de parametros de test pueden ser una copia de .env.

3. ADUANA GENÓMICA (`audit-entity-eda-coverage.py`):
   - Eliminar obligatoriamente cualquier lectura a `./events/processed/`.
   - Validación atómica: leer exclusivamente el `entity_id` en `SddIA/core/eda-coverage.json`. Si el hash coincide, el commit es válido.

4. COMPORTAMIENTO DEL BUS (Sweeper):
   - Configura el `event-sweeper` para realizar "Sweep Vacío" (borrado total de los archivos tras el procesamiento) en ambos entornos (producción y test). 
   - Al no existir ya dependencia de retención para la auditoría, la limpieza debe ser absoluta.

5. INTEGRACIÓN DE CONFIGURACIÓN:
   - Asegura que el script de inicio (`run-eda-e2e-lab.py`) sea el responsable de cargar `.env.test` antes de invocar cualquier acción, aislando así el ecosistema de pruebas del flujo principal.

6. ADECUACIONES POR PARAMETRIZACION DE LA RUTA.
- Adecuar como corresponda a cumulo.
- Adecuar como corresponda a readme.
- Analiza el resto de posibles implicaciones sobre este cambio.


