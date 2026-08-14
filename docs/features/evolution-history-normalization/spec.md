---
feature_name: evolution-history-normalization
created: "2026-08-14"
process: refactorization
phase: Diseño de refactor
agents: dedalo
branch_name: refactor/evolution-history-normalization
persist_ref: docs/features/evolution-history-normalization
pbi_ref: docs/todos/pending/[REFACTOR] Evolution — migrar históricos y extraer borradores (EV-AUD-002-007).md
document_id: 7bb37ff1-decd-4ec5-968b-344a5334f9eb
correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
source_audit: docs/audits/evolution/2026-08-11.md
findings:
  - EV-AUD-002
  - EV-AUD-007
depends_on: 4feb4ea2-b1ca-41c6-bc57-75457840eabf
status: dedalo_locked
version_spec: "1.0.0"
---

# Spec — evolution-history-normalization

## Problema

0/61 registros del corte cumplen conformidad atómica completa; nueve carecen de UUID v4 válido; dos borradores `*-analisis-temp.md` contaminan `directories.evolution`. El contrato v1.1.1 clasifica legado pero prohíbe mutación autónoma. Este PBI es la **excepción autorizada** de migración física (dependencia `4feb4ea2-…` cerrada).

## Decisiones de diseño (laudos Dedalo)

| ID | Laudo |
|----|--------|
| **L-BRANCH** | Rama canónica: `refactor/evolution-history-normalization`. |
| **L-UNIVERSE** | Universo oficial post-migración = todos los `*.md` bajo `directories.evolution` **excepto** `evolution_contract.md`, `Evolution_log.md` y borradores extraídos. Corte actual: **64** registros (= 61 corte − 2 borradores + 3 delta post-corte + 2 altas canónicas posteriores `0bceeb41-…`, `c906d516-…`). |
| **L-MANIFEST** | Manifiesto JSON determinista y reversible **congelado antes** de cualquier mutación. Ancla: `persist_ref/migration-manifest.json`. Segunda ejecución **lee** el manifiesto; no regenera UUID. |
| **L-MANIFEST-HOST** | Subcomando Rust `sddia-qa migrate-evolution-history` (modos `manifest` \| `apply` \| `verify`). Reutiliza `canonical_hash` de `sddia-evolution-register` vía dependencia de crate (paridad test obligatoria). |
| **L-MAP-TERRITORY** | El mapa global **no** vive bajo `directories.evolution`. Copia de auditoría opcional en `paths.auditsPath/evolution/migration/` (solo documental). |
| **L-LOT-ORDER** | L1 (INV-A) → L2 (INV-L) → L3 (NOMBRE/UUID-INV/SIN_FECHA) → L4 (BORRADOR). Cada lote: apply + tests + commit atómico vía `git-manager` antes del siguiente. |
| **L-SKIP-CANON** | Registros ya `CANONICO` v1.1.1 (`0bceeb41-…`): no reescribir cuerpo ni hash; solo asegurar fila de índice coherente. |
| **L-DATE** | `fecha`/`date`/`created` parseable → conservar día; normalizar a `YYYY-MM-DD` salvo datetime ISO ya válido. SIN_FECHA → `git log --diff-filter=A --follow --format=%aI -- <old_path>` (primera aparición); registrar derivación en manifiesto + `relacionado`. Sin evidencia git → **bloquear ítem**. |
| **L-TIPO** | Enum legacy → `modificacion` por defecto; `baja` solo con evidencia de retirada; **prohibido** inferir `alta`. `analisis-temporal`/`borrador` → L4, no enum oficial. |
| **L-DRAFT** | Extraer a `docs/audits/evolution/drafts/{stem}.md` (bajo `paths.auditsPath`). Contenido intacto; frontmatter no canónico permitido. Eliminar filas de índice oficial. |
| **L-ALIAS** | Cada renombre deja: (a) fila manifiesto `old_path↔new_path`; (b) entrada en `relacionado` del registro: `origen:<old_path>`; (c) campo opcional `origen_migracion` en frontmatter canónico. |
| **L-INDEX** | Reconstruir `Evolution_log.md`: 64 filas, `clase_formato: CANONICO`, `contrato_version: 1.1.1`, orden fecha desc / SIN_FECHA al final / empate `ruta_relativa` asc. |
| **L-CONTRACT-§3** | Actualizar `evolution_contract.md` §3: universo oficial post-PR es CANONICO; tabla legacy permanece como arqueología / lectura del manifiesto. Bump documento a coherencia v1.1.1 si procede. |
| **L-VALIDATOR** | Extender `validate-evolution-contract` con `--universe official`: escanea universo oficial, exige `by_class.CANONICO == classified_total` y `log_matches_universe`. |
| **L-IDEM** | `migrate-evolution-history verify --manifest …` → diff vacío en registros, índice y paths extraídos; exit ≠0 si hay drift. |
| **L-GIT** | Mutaciones git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-NO-CUMULO** | Prohibido mutar `cumulo.paths.json`. |

## Universo actual (inventario Dedalo)

### Oficiales a normalizar (64)

| Origen | Conteo | Notas |
|--------|-------:|-------|
| Corte audit 2026-08-11 | 59 | 61 − 2 borradores L4 |
| Delta post-corte | 3 | `0c19403d-…`, `83bbfdeb-…`, `a7c3e91f-2b4d-4e8a-…` |
| Altas canónicas post-corte | 2 | `0bceeb41-…` (skip rewrite), `c906d516-…` (normalizar a v1.1.1) |

### L4 — extracción (2, fuera del universo oficial)

| old_path | destino |
|----------|---------|
| `SddIA/evolution/entity-manager-eda-propuesta-analisis-temp.md` | `docs/audits/evolution/drafts/entity-manager-eda-propuesta-analisis-temp.md` |
| `SddIA/evolution/emit-domain-mutation-analisis-temp.md` | `docs/audits/evolution/drafts/emit-domain-mutation-analisis-temp.md` |

### Clasificación por lote (corte + delta, excl. L4 y skip)

| Lote | Clase(s) | Conteo aprox. | Criterio de inclusión |
|------|----------|--------------:|------------------------|
| L1 | INV-A | ~35 | Cabecera atómica parcial sin contrato v1.1 completo |
| L2 | INV-L | ~18 | `contrato_version`+`id_cambio`+`fecha`+`tipo_operacion` legacy |
| L3 | NOMBRE, UUID-INV, SIN_FECHA | ~9 + 1 | Filename no UUID v4 y/o identidad inválida; incluye `migracion-execute-process-rust-p14-p15.md` |
| skip | CANONICO | 1 | `0bceeb41-64d1-4920-af9d-46a11c0455a2.md` |

## Manifiesto (`migration-manifest.json`)

### Metadatos

```json
{
  "manifest_version": "1.0.0",
  "feature": "evolution-history-normalization",
  "correlation_id": "4b9de6b3-c400-49c8-86f2-55f08ec64ce4",
  "contrato_version_target": "1.1.1",
  "frozen_at": "<ISO-8601, set at manifest freeze>",
  "universe_total_official": 64,
  "draft_extractions": 2,
  "repo_commit_at_freeze": "<git rev-parse HEAD via git-manager>"
}
```

### Fila de entrada (`entries[]`)

| Campo | Obligatorio | Regla |
|-------|-------------|-------|
| `seq` | sí | Orden determinista de aplicación |
| `lote` | sí | `L1` \| `L2` \| `L3` \| `L4` \| `SKIP` |
| `old_path` | sí | Path relativo repo |
| `new_path` | sí* | *Igual a old si solo normaliza FM; distinto si renombra/extrae |
| `id_cambio` | sí* | UUID v4; null en L4 |
| `accion` | sí | `normalize_fm` \| `rename` \| `normalize_and_rename` \| `extract` \| `skip` |
| `uuid_source` | no | `filename` \| `frontmatter_uuid` \| `frontmatter_id_cambio` \| `manifest_v4` |
| `fecha_source` | no | `fecha` \| `date` \| `created` \| `git_first_add` |
| `fecha_derivacion` | no | Obligatorio si `fecha_source=git_first_add` |
| `tipo_operacion` | no | Destino enum; default `modificacion` |
| `classes_detected` | sí | Clases del validador pre-migración |
| `hash_action` | sí | `compute` \| `preserve` \| `none` |

Determinismo: ordenar `entries` por `(lote, old_path)` antes de congelar.

### Reglas de identidad (`id_cambio`)

1. Filename stem = UUID v4 RFC 4122 → `id_cambio` = stem; acción mínima `normalize_fm`.
2. Filename no UUID + frontmatter `uuid`/`id_cambio` UUID v4 válido → `rename` al UUID existente (p. ej. `event-bus-audit-process.md` → `8d577a50-055a-40b9-b7e2-93e2d2415796.md`; `capsules-bridge-rust-port.md` → `a7b3c4d5-e6f7-4890-a1b2-c3d4e5f67890.md`).
3. Filename UUID v4 + frontmatter UUID v4 distintos → **bloquear** en manifiesto hasta laudo explícito; no elegir heurísticamente.
4. Sin UUID v4 válido en filename ni FM → generar **una vez** `manifest_v4`; persistir en manifiesto congelado (7× NOMBRE PR, `git-hooks-ca3-ola-b-contract.md`, `pull-request-automation-dlt-oraculo-20260523.md`, `pull-request-review-v2*.md`, `migracion-execute-process-rust-p14-p15.md`, `e1f2a3b4-…` si nibble versión inválido).
5. `id_cambio` en frontmatter legacy no UUID (p. ej. `a7c3e891-2b4f-…-pull-review-v2`) → descartar; aplicar regla 4.

## Transformación por lote

### L1 — INV-A

Entrada: registros con `INV-A` sin `CANONICO`.

Salida frontmatter canónico v1.1.1:

| Origen | Destino |
|--------|---------|
| `uuid` | `id_cambio` (eliminar `uuid` suelto) |
| `date`, `created` | `fecha` |
| `type`, `tipo`, `record_kind`, `operation` | mapeo L-TIPO → `tipo_operacion` |
| `related_entities`, `artefactos_afectados` | `relacionado` (unión, dedupe) |
| título H1 / `descripcion` | `descripcion_breve` si falta |
| — | `contrato_version: "1.1.1"` |
| — | `hash_integrity` = `canonical_hash(payload)` |

Cuerpo Markdown: **intacto**. Campos atómicos legacy (`id`, `type`, `version`) eliminados del frontmatter salvo `id` kebab si aporta trazabilidad.

### L2 — INV-L

Entrada: `contrato_version` presente + esquema legacy.

Acciones:

- `contrato_version` → `"1.1.1"`.
- `tipo_operacion` legacy → enum L-TIPO.
- `hash_integrity` vacío o en `replicacion.hash_integrity` → **calcular** canónico del payload normalizado (no copiar hex legacy suelto).
- Normalizar `fecha` a representación permitida.
- Filename ya UUID v4 → mantener stem = `id_cambio`.

### L3 — NOMBRE / UUID-INV / SIN_FECHA

Combinación de reglas identidad + L1/L2 según FM detectado.

Caso especial `migracion-execute-process-rust-p14-p15.md`:

- `uuid: c8f2a1b3-…` inválido (versión ≠ 4) → nuevo v4 en manifiesto.
- Fecha ausente → derivar vía git (L-DATE); si falla → bloqueo.

### L4 — BORRADOR

- `git mv` a `docs/audits/evolution/drafts/`.
- Sin conversión a canónico.
- Actualizar referencias cruzadas (p. ej. puntero entre los dos borradores; menciones en docs/features si existen).

### SKIP

- `0bceeb41-64d1-4920-af9d-46a11c0455a2.md`: verificar índice; no mutar archivo.

## Hash canónico

Reutilizar algoritmo `sddia-evolution-register::canonical_hash`:

1. Raw file UTF-8, normalizar CRLF → LF.
2. Eliminar línea(s) `hash_integrity:` del bloque frontmatter.
3. `sha256:` + hex SHA-256 del resultado.

Calcular **después** de normalizar frontmatter y **antes** de escribir `hash_integrity` final.

## Evolution_log.md (post-migración)

| Propiedad | Valor |
|-----------|-------|
| `contrato_version` | `"1.1.1"` |
| `universe_total` | `64` |
| `source_audit` | referencia corte + nota migración |
| `migration_manifest` | `docs/features/evolution-history-normalization/migration-manifest.json` |
| Columnas | `id_cambio` \| `fecha` \| `resumen` \| `clase_formato` \| `ruta_relativa` |
| `clase_formato` | `CANONICO` en todas las filas oficiales |
| Excluidos | borradores L4; filas `UUID-INV` placeholder |

Regeneración: subcomando `migrate-evolution-history` modo `apply` actualiza índice atómicamente tras último lote, o paso dedicado `reindex` en `verify`.

## CLI — `sddia-qa migrate-evolution-history`

```bash
# 1. Generar borrador manifiesto (stdout + persist_ref)
sddia-qa migrate-evolution-history manifest \
  [--json] [--write persist_ref/migration-manifest.json]

# 2. Aplicar lote(s)
sddia-qa migrate-evolution-history apply \
  --manifest persist_ref/migration-manifest.json \
  [--lote L1|L2|L3|L4] [--dry-run]

# 3. Idempotencia + conformidad
sddia-qa migrate-evolution-history verify \
  --manifest persist_ref/migration-manifest.json [--json]
```

| Modo | Mutación | Exit 0 cuando |
|------|----------|---------------|
| `manifest` | no | manifiesto generado; 0 ítems bloqueados |
| `apply` | sí | lote aplicado sin errores |
| `verify` | no | diff vacío + validador `--universe official` conforme |

## Validador extendido

Nuevo universo:

```bash
sddia-qa validate-evolution-contract --json --universe official
```

Comportamiento:

- Enumerar `directories.evolution/*.md` excluyendo contrato, log y paths listados como `extract` en manifiesto congelado.
- Clasificar cada registro; **fallar** (`success: false` en JSON) si existe fila sin clase `CANONICO` exclusiva (sin INV-* paralelas).
- Verificar `evolution_log_rows == classified_total` y cero `missing`.

Modo `audit-cut` existente: **sin cambio** (regresión).

## Referencias internas (cascada)

Actualizar punteros a paths/UUID antiguos en:

- `SddIA/evolution/*.md` (`relacionado` cruzado)
- `docs/features/**`, `docs/audits/**` (grep `SddIA/evolution/` y stems legacy)
- Borradores L4: referencias mutuas → nuevas rutas bajo `docs/audits/evolution/drafts/`
- `docs/features/evolution-contract-index-v11/migration-notes.md`: nota de cierre (referencia, no reescritura histórica)

## Criterios técnicos ↔ AC

| AC | Verificación |
|----|--------------|
| AC-CANON | `validate-evolution-contract --universe official` → 64/64 `CANONICO` |
| AC-INDEX | 64 filas; cero huérfanas; cero duplicados `id_cambio` |
| AC-DRAFT | 0 `*-temp*` bajo `directories.evolution`; 2 ficheros en `docs/audits/evolution/drafts/` |
| AC-ALIAS | `migration-manifest.json` completo + `origen:` en `relacionado` por renombre |
| AC-IDEM | `migrate-evolution-history verify` → diff vacío |
| AC-AUDIT | Informe JSON en `persist_ref/_qa-validate-evolution-official.json` |
| AC-PR | Manifiesto + lotes + índice + cascada + PBI `done/` + `validacion.md` APTO en un PR |

## Fuera de alcance

- Reabrir gate evolution / reason-codes (`70f78d23-…`).
- Mutar `cumulo.paths.json`.
- Formalizar borradores como registros oficiales.
- Reescribir narrativa del cuerpo Markdown.
- Fail-hard del gate sobre universo legacy completo.

## Handoff Tekton

1. Implementar `migrate-evolution-history` en `SddIA/tools/sddia-qa` con tests por lote.
2. Congelar manifiesto; revisión humana opcional del mapa UUID antes de `apply`.
3. Ejecutar L1→L4 en orden; evidencia git vía `git-manager`.
4. Extender validador `--universe official`.
5. Actualizar contrato §3 e índice.
6. Auto-registrar hito vía `gate-evolution` / `evolution-register` en el mismo PR.
7. Emitir `implementation.md` + `execution.md`; **no** ejecutar migración en fase Dedalo.
