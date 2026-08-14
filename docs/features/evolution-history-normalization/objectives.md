---
feature_name: evolution-history-normalization
created: "2026-08-14"
process: refactorization
branch_name: refactor/evolution-history-normalization
persist_ref: docs/features/evolution-history-normalization
pbi_ref: docs/todos/done/[REFACTOR] Evolution — migrar históricos y extraer borradores (EV-AUD-002-007).md
document_id: 7bb37ff1-decd-4ec5-968b-344a5334f9eb
correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-002
  - EV-AUD-007
depends_on: 4feb4ea2-b1ca-41c6-bc57-75457840eabf
---

# Objetivos — evolution-history-normalization

## Objetivo

Normalizar el universo oficial bajo `directories.evolution` al contrato vigente (`normative_documents.evolution_contract` v1.1.1+), preservando identidad usable, fecha evidenciada, cuerpo Markdown y trazabilidad de origen; extraer los borradores `*-analisis-temp.md` fuera de ese directorio.

## Alcance

1. Generar y congelar un manifiesto determinista y reversible `old_path → new_path/id_cambio` **antes** de mutar ficheros.
2. Migrar por lotes los tres esquemas del corte: INV-A, INV-L, NOMBRE/UUID-INV (más delta post-corte no indexado).
3. Corregir fechas, `tipo_operacion` y UUID inválidos según laudos L-DATE / L-TIPO / L-MANIFEST, sin inventar evidencia.
4. Extraer `entity-manager-eda-propuesta-analisis-temp.md` y `emit-domain-mutation-analisis-temp.md` a territorio documental bajo `paths.auditsPath` (fuera de `directories.evolution`).
5. Actualizar `Evolution_log.md` (cobertura 100 % de oficiales post-migración) y referencias internas a paths/UUID viejos.
6. Calcular `hash_integrity` canónico y verificar idempotencia: segunda ejecución → diff vacío.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-CANON | 100 % de registros oficiales valida como `CANONICO` contra el contrato vigente. |
| AC-INDEX | Cero colisiones UUID, cero duplicados, cero oficiales sin fila en `Evolution_log.md`, cero filas huérfanas. |
| AC-DRAFT | Cero borradores / `*-temp*` bajo `directories.evolution`; contenido conservado en `paths.auditsPath`. |
| AC-ALIAS | Cada renombre tiene alias o fila de redirección auditable (manifiesto + origen en el registro). El mapa no es un fichero no-UUID bajo `directories.evolution`. |
| AC-IDEM | Segunda ejecución del migrador sobre el mismo manifiesto produce diff vacío. |
| AC-AUDIT | El validador/auditoría periódica sobre el universo oficial reporta conformidad formal completa (0 clases legacy en oficiales). |
| AC-PR | Manifiesto + migración por lotes con pruebas + índice + cascada documental + PBI archivado en un único PR. |

## Fuera de alcance

- Reabrir o alterar el gate evolution (reason-codes, fail-hard delta). Ya entregado en `70f78d23-…`.
- Mutar `cumulo.paths.json` o añadir `paths.evolutionPath`.
- Formalizar borradores como registros oficiales.
- Reescribir el significado del cuerpo Markdown.
- Inventar fechas, UUIDs no congelados en manifiesto, o hashes.

## Restricciones

- Git solo vía `skill:git-manager`. Rama canónica: `refactor/evolution-history-normalization`.
- Prohibida sustitución masiva sin manifiesto reversible y pruebas por lote.
- UUID v4 nuevos: una generación, freeze en manifiesto; pasadas siguientes solo lectura del mapa.
- Fecha SIN_FECHA: solo desde evidencia git; si no hay evidencia, el ítem bloquea.
- Tipología legacy → `modificacion` por defecto; `baja` solo con evidencia de retirada; nunca inferir `alta`.
- CANONICO ya válido: no reescribir hash ni cuerpo.
- Mutación genoma indexado solo vía cápsulas autorizadas cuando el blueprint lo exija; docs de tarea bajo `persist_ref` son jurisdicción documental.
- El cuerpo de este documento es el `refined_requirements` de entrada a Dedalo.

## Ley aplicada

- `features-documentation-pattern` v1.2.x (frontmatter + un `.md` por fase).
- `normative_documents.evolution_contract` v1.1.1 (esquema canónico; este PBI es la mutación autorizada de históricos).
- `SddIA/norms/sddia-evolution-sync.md` (`{id_cambio}.md`, índice, enum `alta`\|`baja`\|`modificacion`).
- `SddIA/norms/paths-via-cumulo.md` / `cumulo.paths.json` 1.6.1 (`directories.evolution`, `paths.featurePath`, `paths.auditsPath`).
- `SddIA/norms/external-ai-constraints.md` (soberanía de rutas; forja gobernada).
