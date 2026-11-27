---
feature_name: aut-133-remediacion-entidades-anemicas
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-133-remediacion-entidades-anemicas
persist_ref: docs/refactors/aut-133-remediacion-entidades-anemicas
tracker_ref: AUT-133
historia_ref: AUT-125
pbi_ref: PBI-81e91c67
execution_id: "5ea79c56-a610-4223-b2b3-7498a510d239"
purpose: Remediar deuda documental de entidades anémicas en el genoma con inventario trazable, contenido real y promoción del gate global
---
# Clarificación — AUT-133

Origen: auditoría PR #480 ([AUT-125](https://linear.app/oscar-perez/issue/AUT-125/kaizen-auditoria-de-completitud-documental-md-en-entidades-de-dominio), `audit_event_reference` `3899742f-04f5-480b-a1f6-866d93d1d5a3`, resolución **REJECT**). El peaje `gate-entity-no-anemia` endurecido detecta entidades `.md` heredadas de `main` sin secciones contractuales reales, UUID no v4 o cuerpos stub. La remediación masiva con plantilla compartida (`remediate_bulk.py`) y rotación silenciosa de UUIDs fue revertida por *boilerplate gaming* (KA-DOC-1 / KA-DOC-2). Deuda residual: contenido real pendiente en el genoma; CI en `--if-touched`; cualquier relleno con plantilla o rotación silenciosa de UUID reproduciría el REJECT.

**Canal PBI:** `tracker_ref` Linear `AUT-133` / `PBI-81e91c67` (`uuid` `22aef184-e2f3-4861-977c-ac32d9eddc2e`, forge_key `a6caf15c…4454`; outbound `.SddIA/state/tracker-outbound/AUT-133.jsonl`). El cuerpo Markdown del issue no está materializado como fichero local bajo el piloto; el alcance se estabiliza desde semilla de ejecución + propuesta refinada Mayeuta + informe `docs/audits/pr-audit-aut-125-gate-anti-anemia-20261127.md`.

Hermanos (no sustituyen esta deuda): [AUT-132](https://linear.app/oscar-perez/issue/AUT-132/gate-anti-anemia-reglas-contractuales-efectivas-scan-changed-correcto) (oráculo/peaje), [AUT-131](https://linear.app/oscar-perez/issue/AUT-131/forjas-preservar-contratos-y-rollback-atomico-en-update-de-entidades) (integridad de forjas).

## Decisiones de alcance

| ID | Decisión |
| --- | --- |
| D1 | **Inventario pre-remediación versionado (AC-1 / KA-DOC-1).** Antes de mutar cualquier entidad, queda materializado un informe trazable (ruta + familia + códigos de violación) con la salida JSON del gate **global** sobre `main` (o tip de `main` acordado). Prohibido publicar solo un post-remediación vacío (`"violations":[]`) como evidencia de inventario. |
| D2 | **Remediación por contenido real vía `entity-manager` (AC-2 / KA-DOC-2).** Cada entidad remediada recibe cuerpo derivado de su implementación real (handler / cápsula / contrato de familia vigente). Prohibido texto genérico compartido entre entidades; el detector de duplicados del gate debe poder rechazar plantillas repetidas. Toda mutación respeta DA-2 (`entity-manager` / proceso autorizado + evolution cuando corresponda). |
| D3 | **UUID no v4: migración explícita o excepción formal (AC-3 / KA-UUID-1).** Norma de migración explícita (alias + sincronización de índices de familia + `eda-coverage`) **o** excepción formal con registro auditable. Prohibida la rotación silenciosa del campo `uuid` sin actualizar índices y cobertura EDA. |
| D4 | **Promoción del CI a gate global (AC-4 / KA-GATE-GLOBAL-1).** El CI pasa de `--if-touched` a gate global **solo cuando** `violation_count` sea `0` **sin** plantilla (tras D1–D3 verificables). No se declara victoria por `--if-touched` verde con inventario anémico en `main`. |
| D5 | **Un solo PBI de deuda de contenido.** Inventario + remediación real + política UUID + promoción CI conviven en esta entrega `refactorization` (residuo documental de AUT-125 / PR #480 REJECT). |

## Criterios de aceptación (observables)

| ID | Criterio |
| --- | --- |
| AC-1 | Inventario pre-remediación versionado (KA-DOC-1) con salida JSON del gate global sobre `main`. |
| AC-2 | Cada entidad remediada recibe contenido derivado de su implementación real vía `entity-manager`; prohibido texto genérico compartido (detector de duplicados). |
| AC-3 | UUID no v4 con norma de migración explícita (alias + índices + `eda-coverage`) o excepción formal; nunca rotación silenciosa. |
| AC-4 | CI pasa de `--if-touched` a gate global cuando `violation_count` sea `0` sin plantilla. |

## Fuera de alcance

- Rediseño de contratos de familia.
- Corrección del motor del peaje / oráculo → **AUT-132**.
- Bugs de forjas (preservación de contrato, rollback, suite) → **AUT-131**.
- Reescritura de entidades ya conformes.
- Reintroducción de scripts ad-hoc de remediación masiva bajo `docs/` (`remediate_bulk.py` y equivalentes).

## Dependencias / coordinación

- Historia padre: AUT-125; auditoría `docs/audits/pr-audit-aut-125-gate-anti-anemia-20261127.md`.
- Peaje `gate-entity-no-anemia` / `entity_no_anemia`, `entity-manager`, índices de familia y `SddIA/core/eda-coverage.json`.
- Coordinación temporal recomendada con AUT-132 (oráculo fiable) antes o junto a la remediación masiva gobernada.

## Coste de no remediar (contexto estabilizado)

- Oráculo falseable mientras existan stubs o plantilla compartida.
- Gate global bloqueado; deuda enmascarada en aduanas parciales `--if-touched`.
- Agentes sin especificación determinista al ejecutar/componer entidades anémicas.
- Riesgo de reincidencia UUID (índices huérfanos, `eda-coverage` asimétrico).
- Coste de re-auditoría ante intentos cosméticos (REJECT como PR #480).
