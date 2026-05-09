# Contrato de evolución SddIA (YAML en frontmatter)

**Versión del contrato:** `1.1`  
**Persistencia:** `paths.sddiaEvolutionPath` (Cúmulo) + `paths.sddiaEvolutionContractFile` (este documento como referencia humana).  
**Norma:** `SddIA/norms/sddia-evolution-sync.md`

## Propósito

Definir el esquema del **frontmatter YAML** de cada fichero de detalle `{id_cambio}.md` y las reglas de **índice** (`paths.sddiaEvolutionLogFile`).

## Identificador

- **`id_cambio`:** UUID v4 en forma canónica (minúsculas, con guiones).
- **Nombre de fichero:** `{id_cambio}.md` en `paths.sddiaEvolutionPath`.
- El formato legible tipo `SSDD-LOG-YYYYMMDD-HHMM` **no** sustituye al GUID como id machine-readable.

## Campos obligatorios (frontmatter del detalle)

| Campo | Tipo | Descripción |
| :--- | :--- | :--- |
| `contrato_version` | string | Debe ser `1.1`. |
| `id_cambio` | string | UUID v4. |
| `fecha` | string | ISO 8601 (fecha/hora). |
| `autor` | string | Autor del cambio. |
| `proyecto_origen_cambio` | string | Proyecto o contexto de origen. |
| `contexto` | string | Contexto operativo o técnico. |
| `descripcion_breve` | string | Resumen en una línea. |
| `tipo_operacion` | string | `alta` \| `baja` \| `modificacion`. |
| `cambios_realizados` | lista | Cada ítem: `anterior` y `nuevo` (strings). |
| `impacto` | string | `Bajo` \| `Medio` \| `Alto`. |
| `replicacion.instrucciones` | string | Cómo replicar el cambio en otro entorno. |
| `replicacion.hash_integridad` | string | SHA-256 en hex minúsculas del bloque YAML canónico, o `SHA-256-PENDIENTE`. |

## Campos condicionales (operación `baja`)

| Campo | Tipo | Descripción |
| :--- | :--- | :--- |
| `rutas_eliminadas` | lista de strings | Rutas retiradas o deprecadas. |
| `commit_referencia_previo` | string | SHA corto o referencia donde el artefacto aún existía. |

## Hash de integridad

- Calcular SHA-256 sobre el **bloque YAML canónico** del frontmatter (sin el delimitador `---` exterior), con orden de claves fijo y normalización acordada en el binario `sddia_evolution_register`.
- Salida: 64 caracteres hexadecimales en **minúsculas**.

## Índice `Evolution_log.md`

Tabla recomendada: **ID (GUID)** | **Fecha** | **Descripción breve**. Toda fila debe corresponder a un `{id_cambio}.md` existente.
