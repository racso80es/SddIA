---
id: evolution-contract
uuid: 6e2a9c41-8f3b-4d7e-9a1c-5b0d8e4f2a73
type: norm
version: 1.1.1
contrato_version: "1.1.1"
created: "2026-08-11"
updated: "2026-08-14"
source_feature: evolution-registry-gate
normative_key: normative_documents.evolution_contract
---

# Contrato evolution v1.1.1

SSOT del protocolo de registro bajo `directories.evolution`. Clave Cúmulo: `normative_documents.evolution_contract`. Índice: `normative_documents.evolution_log`.

## 1. Jurisdicción de filename (excepción atómica)

Bajo `directories.evolution`, el nombre físico canónico de un registro es **`{id_cambio}.md`** donde `id_cambio` es un **UUID v4**.

Esta jurisdicción **exceptúa** el estándar atómico global de entidades genoma (`{name}.md` + frontmatter `id`/`uuid`/`type`/`version`). El identificador lógico kebab (`id`) es opcional en frontmatter; **no** se exige que el filename sea `{name}.md`.

## 2. Esquema canónico (registros nuevos)

Todo registro **nuevo** debe incluir frontmatter YAML con:

| Campo | Obligatorio | Regla |
|-------|-------------|--------|
| `contrato_version` | sí | SemVer del contrato aplicado; registros nuevos: `1.1.1`. |
| `id_cambio` | sí | UUID v4; **igual** al stem del filename. |
| `fecha` | sí | `YYYY-MM-DD` o datetime ISO-8601. Prohibido inventar. |
| `tipo_operacion` | sí | Enum: `alta` \| `baja` \| `modificacion`. |
| `descripcion_breve` | sí | String no vacío. |
| `hash_integrity` | sí | `sha256:` + hex del payload canónico (frontmatter **sin** este campo + cuerpo Markdown, UTF-8, LF). Vacío inválido. |
| `relacionado` | sí | Lista con ≥1 referencia (path lógico, PBI, entidad, commit). |

### Condicionales

- Si `tipo_operacion: baja` → exigir `rutas_eliminadas` y `commit_referencia_previo` cuando existan artefactos retirados.

### Opcionales canónicos

`autor`, `impacto`, `cambios_realizados`, `id` (kebab), `proyecto_origen_cambio`, `contexto`, `replicacion`, `source_feature`, `document_id`.

## 3. Universo oficial post-migración y arqueología legacy

Tras `evolution-history-normalization` (manifiesto `docs/features/evolution-history-normalization/migration-manifest.json`, PBI `7bb37ff1-…`), el **universo oficial** bajo `directories.evolution` es **CANONICO** v1.1.1. Verificación: `sddia-qa validate-evolution-contract --universe official --manifest <manifiesto>`.

La tabla siguiente es **arqueología de lectura** (clasificador `--universe audit-cut` y manifiesto reversible). No describe el estado post-PR. El mapa `old_path → new_path` no vive bajo `directories.evolution`.

Los históricos **ya migrados** no se reescriben de nuevo por este contrato. El gate `gate-evolution` certifica el **delta** inyectado, no re-audita el universo completo.

| Señal | Clase / mapeo |
|-------|----------------|
| Esquema `contrato_version` + `id_cambio` + `fecha` + `tipo_operacion` (tipología no canónica) | `INV-L` |
| Cabecera atómica parcial (`uuid`/`type`/`date`/`version` sin contrato v1.1 completo) | `INV-A` |
| Filename no UUID v4 | `NOMBRE` |
| Sin UUID v4 en frontmatter ni filename | `UUID-INV` |
| `fecha`/`date`/`created` ausente o inválida | `SIN_FECHA` |
| Filename `*-temp*` · tipología análisis-temporal · `estado: borrador` | `BORRADOR` |
| `hash_integrity` vacío o ausente | no conforme canónico (no rellenar) |
| Contrato 1.1.1 completo | `CANONICO` |

### Alias de lectura

| Alias | Campo canónico |
|-------|----------------|
| `uuid` | identidad (`id_cambio`) |
| `date`, `created` | `fecha` |
| `type`, `tipo`, `operation` | tipología (legacy si ≠ enum canónico) |
| `related_entities`, `artefactos_afectados` | `relacionado` |
| `descripcion`, `descripcion_breve`, título H1 | descripción |

### Tipologías legacy → enum canónico

`feature` / `bug-fix` / `refactorizacion` / `feature-milestone` / `process` / `type` atómico → `modificacion` salvo evidencia de `baja`/`alta`. Aplicado por el migrador del PBI `7bb37ff1-…`; este contrato no autoriza mutaciones posteriores fuera de `gate-evolution`.

## 4. Índice (`Evolution_log.md`)

- Una fila por registro oficial del universo indexado.
- Columnas mínimas: `id_cambio`, `fecha`, `resumen`, `clase_formato`, `ruta_relativa`.
- Cabecera con `contrato_version` y vínculo a este contrato.
- El índice referencia; no reescribe cuerpos de detalle.

## 5. Validador (solo lectura)

`sddia-qa validate-evolution-contract`. Clasifica el universo. **No** es gate bloqueante.

## 6. Semántica de borradores

Artefactos con señal `BORRADOR` **no** permanecen bajo `directories.evolution`. Extracción autorizada: `docs/audits/evolution/drafts/` (`paths.auditsPath`). El clasificador `audit-cut` sigue leyendo el inventario histórico del corte; `--universe official` los excluye vía manifiesto (`lote: L4` / `accion: extract`).

## 7. Exclusiones de correlación material (L-EXCL / L-SELF)

| Path lógico Cúmulo | Efecto |
|--------------------|--------|
| `directories.evolution/**` | Fuera de correlación material. Un diff ⊆ este árbol no exige segundo registro. |

Listas ad hoc en hooks **prohibidas**. Untracked / `compiled_capsules` no entran en el diff versionado.

## 8. Gate y reason-codes

Aduana Universal: `sddia-qa gate-evolution` (CLI nativo). Captura el árbol, inyecta JSON (`request.diff` + `request.registry`) en la cápsula WASI `sddia-evolution-register` vía `capsule-json-io` v2.0. La cápsula coteja y emite el veredicto. El CLI **no** delega el cotejo al hook.

Fail-hard **solo sobre el delta inyectado**. El universo legacy no se certifica aquí.

| Código | Significado | Exit |
|--------|-------------|------|
| `EVOL_OK` | Veredicto apto (incl. L-SELF, idempotencia) | 0 |
| `EVOL_MATERIAL_UNREGISTERED` | Path material sin correlato en registros del diff | ≠0 |
| `EVOL_RECORD_INVALID` | Registro del diff no canónico | ≠0 |
| `EVOL_NOT_INDEXED` | Detalle del diff sin fila en índice | ≠0 |
| `EVOL_HASH_MISMATCH` | `hash_integrity` ≠ recompute | ≠0 |
| `EVOL_DUPLICATE` | `id_cambio` ya existe en alta | ≠0 |
| `EVOL_ATOMICITY` | Persistencia detalle/índice inconsistente | ≠0 |
| `EVOL_CUMULO` | Falta clave/ruta Cúmulo o cápsula | ≠0 |

Coherencia: `exitCode === 0` ⟺ `success === true`.
