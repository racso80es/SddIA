---
feature_name: evolution-contract-index-v11
created: "2026-08-11"
process: feature
branch_name: feat/evolution-contract-index-v11
persist_ref: docs/features/evolution-contract-index-v11
pbi_ref: docs/todos/pending/[ARQUITECTURA] Evolution — restaurar contrato e índice canónico (EV-AUD-001).md
document_id: 4feb4ea2-b1ca-41c6-bc57-75457840eabf
execution_id: c906d516-f708-48bc-87b3-19980a9a11ab
source_audit: docs/audits/evolution/2026-08-11.md
finding: EV-AUD-001
---

# Objetivos — evolution-contract-index-v11

## Objetivo

Restaurar la SSOT evolution declarada por Cúmulo (`normative_documents.evolution_contract` y `evolution_log`) con un contrato **v1.1+ migrable**, un índice maestro coherente y un validador **solo lectura** capaz de clasificar los **61** registros del corte `docs/audits/evolution/2026-08-11.md` **sin pérdida ni mutación** de históricos.

## Alcance

1. Materializar `evolution_contract.md` (≥ v1.1) en la ruta Cúmulo, con esquema canónico único, enums, campos obligatorios (identidad, fecha, tipología, hash, referencias), tabla de compatibilidad legacy y semántica explícita de borradores / SIN_FECHA.
2. Declarar en el contrato la **excepción jurisdiccional** de filename: bajo `directories.evolution` el físico canónico es `{id_cambio}.md` (UUID v4), sin exigir el `{name}.md` del estándar atómico global de entidades genoma.
3. Materializar `Evolution_log.md` con cabecera contractual e inventario inicial: **exactamente una fila por cada uno de los 61** registros oficiales del corte; borradores y SIN_FECHA marcados de forma explícita.
4. Añadir validador de contrato en modo lectura que clasifique los 61 sin escribir en registros.
5. Incluir QA y documentación de migración/compatibilidad en el **mismo PR** (sin ejecutar la normalización física).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-PATHS | Existen y son coherentes `normative_documents.evolution_contract` y `normative_documents.evolution_log` respecto a Cúmulo. |
| AC-CONTRACT | El contrato define campos obligatorios, enums de tipología, reglas de fecha, identidad UUID v4, hash y referencias, más alias legacy documentados. |
| AC-JURISDICTION | Queda resuelta por escrito la colisión estándar atómico global vs `{uuid}.md` (excepción bajo `directories.evolution`). |
| AC-VALIDATOR | El validador clasifica 61/61 del corte sin mutar ficheros de detalle. |
| AC-LOG | `Evolution_log.md` tiene exactamente 1 fila por registro oficial del corte; borradores y SIN_FECHA explícitos. |
| AC-PR | QA + docs de compatibilidad/migración + cascada documental en un único PR; PBI archivado en rama según patrón v1.2.x. |

## Fuera de alcance

- Normalización física, renombre o reescritura de frontmatter de históricos (PBI `7bb37ff1-…`).
- Extracción física de borradores `*-temp*` fuera de `directories.evolution` (mismo PBI migración).
- Gate CI bloqueante de registro/coherencia (PBI `70f78d23-…`).
- Retirada o reescritura de claves en `cumulo.paths.json`.

## Restricciones

- Git solo vía `skill:git-manager`.
- Prohibido inventar fechas, UUIDs o hashes en registros existentes.
- Mutación genoma indexado solo vía `entity-manager` / cápsulas autorizadas cuando el blueprint lo exija; docs de feature bajo `persist_ref` son jurisdicción documental.
- El cuerpo de este documento es el `refined_requirements` de entrada a Dedalo.

## Ley aplicada

- `features-documentation-pattern` (frontmatter + un `.md` por fase).
- `SddIA/norms/sddia-evolution-sync.md` (contrato v1.1, `{id_cambio}.md`, índice).
- `SddIA/norms/paths-via-cumulo.md` / `cumulo.paths.json` 1.6.1.
- `SddIA/norms/external-ai-constraints.md` (soberanía de rutas; forja gobernada).
