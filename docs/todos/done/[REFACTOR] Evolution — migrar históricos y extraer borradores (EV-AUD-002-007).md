---
document_id: 7bb37ff1-decd-4ec5-968b-344a5334f9eb
title: Evolution — migrar históricos y extraer borradores
type: refactorization
status: done
priority: high
created: "2026-08-11"
closed: "2026-08-14"
suggested_branch: refactor/evolution-history-normalization
persist_ref: docs/features/evolution-history-normalization
execution_id: 63062872-e707-496e-b1b3-1ea736e256f0
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-002
  - EV-AUD-007
depends_on:
  - 4feb4ea2-b1ca-41c6-bc57-75457840eabf
---

# Evolution — migrar históricos y extraer borradores

## Problema

Ninguno de los registros del corte cumple íntegramente un formato atómico común; nueve carecen de UUID v4 válido y dos borradores temporales contaminan el universo oficial.

## Objetivo

Normalizar los históricos al contrato restaurado, preservando identidad, fecha, contenido y trazabilidad de origen.

## Alcance

1. Generar manifiesto determinista `old_path → new_path/uuid`.
2. Migrar por lotes los tres esquemas detectados.
3. Corregir fechas, tipos y UUID inválidos sin perder referencias históricas.
4. Extraer `*-analisis-temp.md` a territorio documental no normativo.
5. Actualizar `Evolution_log.md` y referencias internas.
6. Verificar hashes e idempotencia del migrador.

## Criterios de aceptación

- 100 % de registros oficiales valida contra el contrato vigente.
- Cero colisiones UUID, duplicados o entradas sin índice.
- Los borradores quedan fuera de `directories.evolution`.
- Cada renombre conserva alias o mapa de redirección auditable.
- Segunda ejecución del migrador produce diff vacío.
- La auditoría periódica reporta conformidad formal completa.

## Restricción

Prohibida una sustitución masiva sin manifiesto reversible y pruebas por lote.

## Cierre

Rama `refactor/evolution-history-normalization`. Universo official 65/65 CANONICO (64 migrados + hito `63062872-…`). Manifiesto `docs/features/evolution-history-normalization/migration-manifest.json`.
