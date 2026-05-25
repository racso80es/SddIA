---
feature_name: eda-orphan-debt-precommit
created: "2026-05-25"
process: bug-fix
branch_name: fix/eda-orphan-debt-precommit
persist_ref: docs/features/eda-orphan-debt-precommit
pbi_ref: docs/todos/pending/[Kaizen] deuda EDA orphan_count — correlación processed y backfill pre-commit.md
document_id: PBI-KAIZEN-EDA-ORPHAN-DEBT
---

# Objetivos — Deuda EDA orphan_count (pre-commit)

## Misión

Eliminar el bloqueo sistemático del hook **pre-commit** (`Argos orphan_count=N`) y la inconsistencia de la **Aduana EDA genómica** en `delivery-close-cycle`, cerrando la deuda EDA preexistente al backfill Fase C de `eda-domain-entities-splus` y corrigiendo la regresión **C2**: el audit solo correlaciona eventos en `pending/`, no en `processed/` ni cabeceras `processing/`.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| 46 entidades indexadas en genoma Core | Gate `--scan` evalúa cobertura ECST por `entity_uuid` |
| Forja histórica sin `entity-manager` | Deuda **C1** — entidades sin `Domain_Entity_Created` |
| `iter_bus_event_files()` omite `processed/` y `processing/` | Regresión **C2** — tras `event-watcher --once`, huérfanas reaparecen |
| Backfill Fase C ejecutado en lab (vanguardia) | Eventos en `pending/` enmascaran C2 hasta procesar el bus |
| Kaizen higiene ficheros temporales (PR #44) | Expuso la deuda al activar pre-commit sobre mutaciones `SddIA/` |
| Incidente documentado: `orphan_count=43` | Síntoma en commit con hook activo; 8 clases afectadas |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Correlación EDA estable** | `find_existing_domain_event` considera `pending` ∪ `processed` ∪ `processing` ∪ legacy |
| **O2** | **orphan_count = 0 sostenido** | `--scan` en 0 tras F1 **y** tras `event-watcher --once` (V4); backfill solo si persiste deuda real |
| **O3** | **Pre-commit desbloqueado** | Hook pasa en commit de prueba que toque `SddIA/` sin `SDDIA_SKIP_HOOKS` |
| **O4** | **Fase C cerrada** | Manifiesto + acta Merkle **solo si Track 2 se ejecuta** (`orphan_count > 0` post F1+V4) |
| **O5** | **Prevención forward** | Norma/documentación: no repetir protocolo frágil «solo pending» |

## No objetivos

- Reescribir el backfill histórico de `eda-domain-entities-splus` (solo consolidar cierre en este `persist_ref` si F2 aplica).
- Gate pre-commit por diff incremental (O5 F4.3 — futuro).
- Cerrar deuda arquitectónica bus-independiente (derivada a PBI `[Kaizen] validación genómica EDA sin dependencia del bus`).
- Cerrar `Argos_Eda_Emision` ni `Kaizen_Alert_Required` EDA v2 (PBIs separados).

## Ley aplicada

- `features-documentation-pattern` v1.2.0
- Proceso `bug-fix` v1.4.0
- Cierre documental en rama (`.cursor/rules/task-closure-documental.mdc`)
- Protocolo backfill Fase C: `eda-domain-entities-splus/spec.md` §6
