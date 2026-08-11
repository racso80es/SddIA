---
document_id: 4feb4ea2-b1ca-41c6-bc57-75457840eabf
title: Evolution — restaurar contrato e índice canónico
type: architecture
status: done
priority: critical
created: "2026-08-11"
suggested_branch: feat/evolution-contract-index-v11
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-001
blocks:
  - 7bb37ff1-decd-4ec5-968b-344a5334f9eb
  - 70f78d23-e209-4e41-9292-cb7421a934f6
---

# Evolution — restaurar contrato e índice canónico

## Problema

Cúmulo declara `normative_documents.evolution_contract` y `normative_documents.evolution_log`, pero ambos archivos están ausentes. La trazabilidad evolution carece de contrato ejecutable e índice maestro.

## Objetivo

Restaurar una fuente de verdad mínima y compatible con los registros históricos sin reescribirlos todavía.

## Alcance

1. Materializar `evolution_contract.md` v1.1 o una versión superior explícitamente migrable.
2. Definir esquema único, compatibilidad con formatos legacy y semántica de borradores.
3. Materializar `Evolution_log.md` con cabecera contractual e inventario inicial.
4. Resolver la colisión entre el estándar atómico global y el filename `{uuid}.md` propio de evolution.
5. Añadir validador de contrato en modo lectura.

## Criterios de aceptación

- Las dos rutas declaradas por Cúmulo existen y son coherentes.
- El contrato define campos obligatorios, enums, fecha, identidad, hash y referencias.
- El validador clasifica los 61 registros del corte sin pérdida ni mutación.
- `Evolution_log.md` contiene exactamente una fila por registro oficial.
- Borradores y entradas sin fecha se representan de forma explícita.
- QA y documentación de migración quedan en un único PR.

## Fuera de alcance

- Normalización física de los históricos.
- Gate CI bloqueante.
