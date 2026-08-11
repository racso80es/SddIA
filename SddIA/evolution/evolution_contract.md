---
id: evolution-contract
uuid: 6e2a9c41-8f3b-4d7e-9a1c-5b0d8e4f2a73
type: norm
version: 1.1.0
contrato_version: "1.1.0"
created: "2026-08-11"
source_feature: evolution-contract-index-v11
normative_key: normative_documents.evolution_contract
---

# Contrato evolution v1.1.0

SSOT del protocolo de registro bajo `directories.evolution`. Clave Cúmulo: `normative_documents.evolution_contract`. Índice: `normative_documents.evolution_log`.

## 1. Jurisdicción de filename (excepción atómica)

Bajo `directories.evolution`, el nombre físico canónico de un registro es **`{id_cambio}.md`** donde `id_cambio` es un **UUID v4**.

Esta jurisdicción **exceptúa** el estándar atómico global de entidades genoma (`{name}.md` + frontmatter `id`/`uuid`/`type`/`version`). El identificador lógico kebab (`id`) es opcional en frontmatter; **no** se exige que el filename sea `{name}.md`.

## 2. Esquema canónico (registros nuevos)

Todo registro **nuevo** debe incluir frontmatter YAML con:

| Campo | Obligatorio | Regla |
|-------|-------------|--------|
| `contrato_version` | sí | SemVer del contrato aplicado; registros nuevos: `1.1.0`. |
| `id_cambio` | sí | UUID v4; **igual** al stem del filename. |
| `fecha` | sí | `YYYY-MM-DD` o datetime ISO-8601. Prohibido inventar. |
| `tipo_operacion` | sí | Enum: `alta` \| `baja` \| `modificacion`. |
| `descripcion_breve` | sí | String no vacío. |
| `hash_integrity` | sí | Huella no vacía (`sha256:…` u otra política documentada). |
| `relacionado` | sí | Lista con ≥1 referencia (path lógico, PBI, entidad, commit). |

### Condicionales

- Si `tipo_operacion: baja` → exigir `rutas_eliminadas` y `commit_referencia_previo` cuando existan artefactos retirados.

### Opcionales canónicos

`autor`, `impacto`, `cambios_realizados`, `id` (kebab), `proyecto_origen_cambio`, `contexto`, `replicacion`, `source_feature`, `document_id`.

## 3. Compatibilidad legacy (solo lectura / clasificación)

Los históricos **no** se reescriben por este contrato. El validador y el índice los clasifican:

| Señal | Clase / mapeo |
|-------|----------------|
| Esquema `contrato_version` + `id_cambio` + `fecha` + `tipo_operacion` (tipología no canónica) | `INV-L` |
| Cabecera atómica parcial (`uuid`/`type`/`date`/`version` sin contrato v1.1 completo) | `INV-A` |
| Filename no UUID v4 | `NOMBRE` |
| Sin UUID v4 en frontmatter ni filename | `UUID-INV` |
| `fecha`/`date`/`created` ausente o inválida | `SIN_FECHA` |
| Filename `*-temp*` · tipología análisis-temporal · `estado: borrador` | `BORRADOR` |
| `hash_integrity` vacío o ausente | no conforme canónico (no rellenar) |

### Alias de lectura

| Alias | Campo canónico |
|-------|----------------|
| `uuid` | identidad (`id_cambio`) |
| `date`, `created` | `fecha` |
| `type`, `tipo`, `operation` | tipología (legacy si ≠ enum canónico) |
| `related_entities`, `artefactos_afectados` | `relacionado` |
| `descripcion`, `descripcion_breve`, título H1 | descripción |

### Tipologías legacy → enum canónico (migración futura)

`feature` / `bug-fix` / `refactorizacion` / `feature-milestone` / `process` / `type` atómico → destino típico `modificacion` o `alta` según laudo del PBI de migración; **este contrato no muta**.

## 4. Índice (`Evolution_log.md`)

- Una fila por registro oficial del universo indexado.
- Columnas mínimas: `id_cambio`, `fecha`, `resumen`, `clase_formato`, `ruta_relativa`.
- Cabecera con `contrato_version` y vínculo a este contrato.
- El índice referencia; no reescribe cuerpos de detalle.

## 5. Validador

Herramienta de solo lectura: `sddia-qa validate-evolution-contract`. No constituye gate CI bloqueante en v1.1.0.

## 6. Semántica de borradores

Artefactos bajo `directories.evolution` con señal `BORRADOR` permanecen clasificados en el universo si pertenecen al corte indexado; su extracción física pertenece a un ciclo de migración distinto.
