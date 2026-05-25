---
document_id: PBI-KAIZEN-EDA-COVERAGE-SSOT-BUS-ISOLATION
title: "[Kaizen] EDA cobertura durable, aislamiento bus y smoke e2e — SSOT eda-coverage"
format: markdown
version: "1.0.0"
created: "2026-05-25"
status: listo_para_merge
priority: alta
process: feature
branch_name: feat/eda-coverage-ssot-bus-isolation
feature_ref_target: docs/features/eda-coverage-ssot-bus-isolation
planning_completed: "2026-05-25"
implementation_completed: "2026-05-25"
consolidates:
  - PBI-KAIZEN-EDA-AUDIT-NO-BUS-DEPENDENCY
  - PBI-KAIZEN-EDA-BUS-E2E-SMOKE-LOCAL-TOPOLOGY
upstream:
  - docs/todos/done/[Kaizen] deuda EDA orphan_count — correlación processed y backfill pre-commit.md
  - docs/features/eda-orphan-debt-precommit/
  - docs/todos/done/[Kaizen] higiene ficheros temporales — .tmp fuera de control y limpieza post-uso.md
  - docs/features/kaizen-higiene-ficheros-temporales/
related:
  - SddIA/core/eda-coverage.json
  - SddIA/core/cumulo.paths.json
  - SddIA/scripts/qa/audit-entity-eda-coverage.py
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/scripts/qa/run-eda-e2e-lab.py
  - SddIA/scripts/qa/env_loader.py
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.py
  - SddIA/core/event-subscriptions.json
  - .github/workflows/sddia-index-qa.yml
  - .dev/.env.example
blocks: "Aduana genómica acoplada al bus efímero; CI eda-bus-e2e-smoke en rojo sostenido"
---

# [Kaizen] EDA cobertura durable, aislamiento bus y smoke e2e — SSOT eda-coverage

**Estatus:** Listo para merge  
**Persist ref:** `docs/features/eda-coverage-ssot-bus-isolation/` — implementación F0–F8 local ✅ (2026-05-25)  
**Precedencia:** mitigación táctica `eda-orphan-debt-precommit` (C2 + retención cabeceras); Kaizen higiene ficheros temporales (`scope: local` en lab)

> **Consolidación:** sustituye a `PBI-KAIZEN-EDA-AUDIT-NO-BUS-DEPENDENCY` y `PBI-KAIZEN-EDA-BUS-E2E-SMOKE-LOCAL-TOPOLOGY`. Los dos problemas comparten la misma deuda arquitectónica: tratar el bus de archivos como fuente de verdad para correlación y barrido.

---

## 1. Casuística unificada

### 1.1 Aduana genómica acoplada al bus (orphan_count)

Durante el cierre de **PBI-KAIZEN-EDA-ORPHAN-DEBT** se confirmó empíricamente:

| Fase | Comportamiento | Consecuencia |
|------|----------------|--------------|
| Baseline con eventos en `pending/` | `--scan` → `orphan_count: 0` | Enmascara deuda real |
| Fix C2 (leer `processed/` + `processing/`) | Correlación ampliada | Insuficiente si el bus purga cabeceras |
| `event-watcher --once` post-backfill | `archive_event_after_sweep` elimina instancias ECST | `--scan` → `orphan_count: 43` |
| Workaround opción 2 | Retener cabecera `processed/` solo para `Domain_Entity_Created` | Restaura V4 en 0, pero **acopla la aduana al bus** |

**Laudo parcial:** la validación genómica (`pre-commit`, `delivery-close-cycle` fase Aduana EDA) **no debe depender** de instancias ECST en carpetas efímeras del bus.

### 1.2 CI `eda-bus-e2e-smoke` en rojo (topología local)

| Campo | Valor |
|-------|--------|
| **Síntoma** | Job CI `eda-bus-e2e-smoke` falla con exit code 1 |
| **Comando** | `python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json` |
| **Salida típica** | `parent_still_pending: true`, `success: false`, `cleaned: true` |

**Causas encadenadas:**

| # | Causa | Ubicación |
|---|--------|-----------|
| **C1** | Lab forja con `scope: "local"` (higiene Kaizen) | `run-eda-e2e-lab.py` |
| **C2** | Suscriptores `Domain_Entity_Created` con `applies_to_origin_topology: ["core"]` | `event-subscriptions.json` |
| **C3** | `origin_topology: local` → suscriptores vacíos tras filtro | `eda_bus_utils.subscriber_applies_to_topology` |
| **C4** | `route_domain_event` retorna `success: true` **sin** `try_sweep_event` cuando `subscribers == []` | `route_domain_event_core.py` L457–468 |
| **C5** | Criterio E2E exige `not pending.is_file()` y `sweep.status == "purged"` | `run-eda-e2e-lab.py` |

### 1.3 Convergencia arquitectónica

```text
Problema A (aduana)     →  correlación no durable; workaround retención cabeceras
Problema B (smoke CI)   →  sweep incompleto + lab comparte bus prod sin aislamiento
Solución unificada      →  SSOT eda-coverage.json + sweep vacío + EVENT_BUS_PATH + router no-subscribers
```

---

## 2. Declaración de propósito

Implementar un **Índice de Cobertura Genómica** durable bajo jurisdicción Core, desacoplar la Aduana Argos del ciclo de vida efímero del bus, parametrizar la ruta del bus por entorno, y restaurar el smoke E2E en CI **sin** revertir `scope: local` ni reintroducir workarounds opacos.

### Principio rector

```text
Aduana genómica  →  consulta SddIA/core/eda-coverage.json (SSOT)
Bus EDA          →  orquestación reactiva; barrido absoluto (sweep vacío)
Lab / CI         →  bus aislado vía EVENT_BUS_PATH (.dev/.env.test)
```

---

## 3. Laudo de diseño (cerrado para implementación)

Las opciones abiertas en los PBI originales quedan resueltas así:

| Decisión | Opción elegida | Notas |
|----------|----------------|-------|
| Fuente correlación `--scan` | **A refinada** — `SddIA/core/eda-coverage.json` | Mapa topológico `{entity_id: {...}}`, no array cronológico |
| Retención cabeceras `processed/` | **Poda** tras migración | Eliminar `retain_processed` en `archive_event_after_sweep` |
| Ruta del bus | **`EVENT_BUS_PATH`** | Precedencia: env → `cumulo.paths.json` → default `.events` |
| Perfil lab/CI | **`.dev/.env.test`** | Cargado por `run-eda-e2e-lab.py` antes de cualquier acción |
| Sweep sin suscriptores | **Router A** | Si `subscribers == []` tras filtro topológico → invocar `try_sweep_event` |
| Scope lab | **Mantener `local`** | No revertir higiene Kaizen |
| Merkle / DLT | **Fuera de alcance v1** | `eda-coverage.json` es SSOT operativo local; anclaje Merkle sigue en Fase C existente |

**Prohibido:** declarar cerrada la deuda cuando solo exista retención de cabeceras o lectura de `processed/` en la aduana.

---

## 4. Especificación técnica (refinada desde propuesta §8)

### 4.1 Nuevo SSOT: `SddIA/core/eda-coverage.json`

Archivo versionado en git. Estructura estricta:

```json
{
  "version": "1.0.0",
  "coverage_matrix": {
    "<entity_uuid>": {
      "is_covered": true,
      "last_emitted_event": "Domain_Entity_Created",
      "last_hash": "<sha256_hex>",
      "correlation_timestamp": "<iso_8601_utc>"
    }
  }
}
```

| Regla | Detalle |
|-------|---------|
| Claves | `entity_uuid` (no nombre kebab) |
| Mutación | **Upsert** por entidad; nunca append cronológico |
| Inicial | `{}` en `coverage_matrix` + backfill one-shot desde entidades indexadas |

Referencia en `cumulo.paths.json` → clave `eda_coverage` apuntando a `SddIA/core/eda-coverage.json`.

### 4.2 Motor de emisión — doble fase atómica

En `emit-domain-mutation` y/o `sync-entity-index` (según handoff de `entity-manager`):

```text
Fase A (durable): upsert en eda-coverage.json — is_covered, last_hash, correlation_timestamp
Fase B (efímero):  inyectar chispa en {EVENT_BUS_PATH}/pending/
```

Si Fase A falla, **no** escribir en pending (fail-fast). Orden obligatorio: A → B.

### 4.3 Aduana genómica — `audit-entity-eda-coverage.py`

| Acción | Detalle |
|--------|---------|
| **Eliminar** | Dependencia gate de `iter_bus_event_files()` / lectura `processed/` para orphan_count |
| **Implementar** | `--scan` recorre entidades indexadas; cada una debe existir en `coverage_matrix` con `is_covered: true` |
| **Validación opcional** | Comparar `last_hash` con hash genómico actual del artefacto (fail si diverge) |
| **Bus** | Solo hint diagnóstico (`--verbose`); nunca gate |

### 4.4 Barrido — sweep vacío

`archive_event_after_sweep` / `event-sweeper`:

- Eliminar bloque `retain_processed = event_type == "Domain_Entity_Created"`.
- Purgar pending, cabeceras processing/processed y testigos sin excepción.

`route_domain_event_core.py`:

- Cuando `subscribers == []` tras filtro topológico, invocar `try_sweep_event` antes de retornar (status `no-subscribers` → purga explícita si aplica).

### 4.5 Parametrización `EVENT_BUS_PATH`

| Capa | Comportamiento |
|------|----------------|
| `eda_bus_utils.load_eda_bus()` | Si `os.environ["EVENT_BUS_PATH"]` definido, usar como raíz del bus (normalizar `./` y `\`) |
| Default | `./.events` (coherente con `cumulo.paths.json` actual; **no** `./events/`) |
| Lab/CI | `.dev/.env.test` → `EVENT_BUS_PATH=.tmp/events_test` |
| Carga | `run-eda-e2e-lab.py` invoca `load_hierarchical_env` + overlay `.dev/.env.test` al inicio |

Plantilla `.dev/.env.test.example` (commit) + entrada en `.dev/.env.example` documentando la variable.

### 4.6 Touchpoints adicionales (implicaciones)

| Artefacto | Adecuación |
|-----------|------------|
| `cumulo.paths.json` | `eda_coverage`, documentar precedencia `EVENT_BUS_PATH` |
| `pre_commit_gate.py` | Sin cambio de contrato; hereda scan vía audit refactorizado |
| `delivery-close-cycle.md` | Retirar excepción `_backfill_manifest_active` cuando SSOT cubra el lote |
| `test_eda_bus_v3plus.py` | Actualizar tests retención cabecera → sweep vacío |
| `README.md` | Sección EVENT_BUS_PATH + perfiles prod/test |
| `.github/workflows/sddia-index-qa.yml` | Verificar job carga entorno test (o hereda del lab) |

---

## 5. Backlog atómico (orden de implementación)

| Hito | Objetivo | Criterio | Depende de |
|------|----------|----------|------------|
| **H1** | `clarify.md` | Laudo §3 reflejado; trade-offs A/B/C/D del smoke documentados | — |
| **H2** | SSOT + cúmulo | `eda-coverage.json` inicial + ref en `cumulo.paths.json` | H1 |
| **H3** | `EVENT_BUS_PATH` | `load_eda_bus()` respeta env; tests unitarios mínimos | H1 |
| **H4** | Perfil test | `.dev/.env.test.example`; lab carga overlay test | H3 |
| **H5** | Emisión doble fase | Fase A upsert en emit/sync; Fase B pending | H2, H3 |
| **H6** | Backfill one-shot | Poblar `coverage_matrix` desde entidades indexadas + hashes actuales | H2, H5 |
| **H7** | Refactor audit | `--scan` solo SSOT; orphan_count coherente | H6 |
| **H8** | Sweep vacío + router | Poda retención; sweep en rama `subscribers == []` | H7 |
| **H9** | Smoke E2E + CI | `run-eda-e2e-lab.py --json` → `success: true`; job verde | H4, H8 |
| **H10** | Cierre documental | `spec/plan/implementation/execution/validacion.md` APTO; PBI en `done/` | H9 |

---

## 6. Criterios de aceptación (Definition of Done)

| ID | Criterio |
|----|----------|
| **UNI-CA1** | `--scan` correlaciona vía `eda-coverage.json`, no vía bus físico |
| **UNI-CA2** | `pre-commit` PASS tras watcher + sweep vacío sin cabeceras retenidas |
| **UNI-CA3** | `delivery-close-cycle` Aduana EDA sin depender de `_backfill_manifest_active` |
| **UNI-CA4** | `run-eda-e2e-lab.py --entity-class tool --json` → exit 0 local |
| **UNI-CA5** | CI `eda-bus-e2e-smoke` SUCCESS en PR de cierre |
| **UNI-CA6** | Lab mantiene `scope: local`; bus prod no contaminado (`.tmp/events_test`) |
| **UNI-CA7** | Workaround `retain_processed` eliminado |
| **UNI-CA8** | `validacion.md` APTO + PBI en `docs/todos/done/` (un PR) |

---

## 7. Protocolo de validación empírica

1. **Baseline:** `--scan` con SSOT vacío → documentar orphan_count pre-backfill.
2. **Backfill H6:** poblar SSOT → `--scan` → `orphan_count: 0`.
3. **Watcher + sweep:** `event-watcher --once` → `--scan` sigue en 0 **sin** cabeceras en `processed/`.
4. **Lab aislado:** con `.env.test`, ejecutar lab → `success: true`, working tree limpio.
5. **CI:** push PR → `eda-bus-e2e-smoke` SUCCESS; `verify-process-integrity` sin regresión.
6. **Regresión core (opcional):** fixture entidad `scope: core` sigue purgando vía suscriptores.

---

## 8. Inicio formal del proceso Kaizen

| Campo | Valor |
|-------|--------|
| Proceso | `feature` v1.3.0 |
| Rama | `feat/eda-coverage-ssot-bus-isolation` |
| `persist_ref` | `docs/features/eda-coverage-ssot-bus-isolation` |
| `pbi_ref` | este documento |
| `base_branch` | `main` |
| Primer entregable | H1 `clarify.md` + H2 esqueleto SSOT (rama inicial) |

### Payload de arranque (execute-process / operador)

```json
{
  "process": "feature",
  "feature_name": "eda-coverage-ssot-bus-isolation",
  "branch_name": "feat/eda-coverage-ssot-bus-isolation",
  "persist_ref": "docs/features/eda-coverage-ssot-bus-isolation",
  "refined_requirements": "SSOT eda-coverage.json; aduana desacoplada del bus; EVENT_BUS_PATH + .env.test; sweep vacío; router no-subscribers; smoke CI verde con scope local.",
  "pbi_ref": "docs/todos/pending/[Kaizen] EDA cobertura durable, aislamiento bus y smoke e2e — SSOT eda-coverage.md",
  "base_branch": "main"
}
```

---

## 9. Referencias

- Ejecución empírica orphan: `docs/features/eda-orphan-debt-precommit/execution.md` § V4 / impasse
- Norma Ruido de Sistema: `features-documentation-pattern` § Cobertura EDA genómica
- Backfill Fase C: `docs/features/eda-domain-entities-splus/spec.md` §6
- Jerarquía bóvedas: `docs/features/ampliacion-configuracion-entornos/`
- Secuencia fallo smoke: PBI origen `PBI-KAIZEN-EDA-BUS-E2E-SMOKE-LOCAL-TOPOLOGY` §2
