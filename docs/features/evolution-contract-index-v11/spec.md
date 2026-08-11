---
feature_name: evolution-contract-index-v11
created: "2026-08-11"
process: feature
branch_name: feat/evolution-contract-index-v11
persist_ref: docs/features/evolution-contract-index-v11
pbi_ref: docs/todos/pending/[ARQUITECTURA] Evolution — restaurar contrato e índice canónico (EV-AUD-001).md
document_id: 4feb4ea2-b1ca-41c6-bc57-75457840eabf
execution_id: c906d516-f708-48bc-87b3-19980a9a11ab
phase: blueprint
agents: dedalo
base: main
scope: "Restaurar evolution_contract + Evolution_log + validador lectura (EV-AUD-001)"
---

# Spec — evolution-contract-index-v11

## Problema

Cúmulo 1.6.1 declara `normative_documents.evolution_contract` → `SddIA/evolution/evolution_contract.md` y `normative_documents.evolution_log` → `SddIA/evolution/Evolution_log.md`, ambos ausentes. `sddia-evolution-sync` ya exige contrato v1.1 + índice; sin ellos la trazabilidad no es ejecutable.

## Decisiones de diseño (laudos Dedalo)

| ID | Laudo |
|----|--------|
| **L-RESTORE** | Materializar SSOT en rutas Cúmulo; **no** mutar `cumulo.paths.json`. |
| **L-CONTRACT-V11** | `evolution_contract.md` **v1.1.0** (SemVer documento), migrable; esquema canónico único para registros **nuevos** y destino de migración futura. |
| **L-JURISDICTION** | Bajo `directories.evolution`, filename canónico = `{id_cambio}.md` (UUID v4). Excepción explícita al estándar atómico `{name}.md` de genoma. |
| **L-INDEX-CUT** | `Evolution_log.md` materializa **exactamente 61 filas** = inventario del corte `docs/audits/evolution/2026-08-11.md`. Registros post-corte presentes en disco (delta observado) **no** entran en este índice; se listan en docs de migración del PR. |
| **L-NO-MUTATE** | Cero escrituras a ficheros de detalle históricos. |
| **L-VALIDATOR-HOST** | Validador = subcomando **`sddia-qa validate-evolution-contract`** (Rust, solo lectura, `--json`). Sin wire a CI bloqueante. |
| **L-UNIVERSE** | Modo AC: `--universe audit-cut --audit-ref paths.auditsPath/evolution/2026-08-11.md` (o path resuelto equivalente). Clasifica los 61 del manifiesto del informe; no inventa filas. |
| **L-FORGE** | `evolution_contract.md` / `Evolution_log.md` = jurisdicción `directories.evolution` (excepción DA-2/EDA). Mutación `SddIA/tools/sddia-qa/**` bajo topología feature activa (DA-4); sin creator nuevo. |

## Contrato v1.1.0 — esquema canónico (registros nuevos)

Frontmatter obligatorio:

| Campo | Tipo | Regla |
|-------|------|--------|
| `contrato_version` | string SemVer | Debe ser `1.1.0` (o compatible documentada). |
| `id_cambio` | UUID v4 | Igual al stem del filename. |
| `fecha` | date ISO (`YYYY-MM-DD` o datetime ISO-8601) | Obligatorio; no inventar. |
| `tipo_operacion` | enum | `alta` \| `baja` \| `modificacion` |
| `descripcion_breve` | string no vacío | Resumen máquina/humano. |
| `hash_integrity` | string | Huella declarada (sha256:… o política del contrato); vacío inválido en canónico. |
| `relacionado` | lista string / objetos | ≥1 referencia a artefacto, PBI, entidad o path lógico. |

Campos opcionales canónicos: `autor`, `impacto`, `cambios_realizados`, `rutas_eliminadas`, `commit_referencia_previo` (obligatorio si `baja`), `id` kebab lógico, `proyecto_origen_cambio`, `contexto`, `replicacion`.

### Alias legacy → clasificación (sin reescritura)

| Alias / señal | Campo canónico / clase |
|---------------|------------------------|
| `uuid` (sin `id_cambio`) | identidad |
| `date` / `created` | fecha |
| `type` / `tipo` / `operation` / `tipo_operacion` ∈ {feature, bug-fix, refactorizacion, feature-milestone, process, …} | tipología **legacy** → clase `INV-L` o `LEGACY_TIPO` |
| `related_entities` / `artefactos_afectados` | referencias |
| filename no UUID | `NOMBRE` |
| UUID ausente/no v4 | `UUID-INV` |
| cabecera atómica parcial (`id`/`uuid`/`type`/`version` sin contrato v1.1) | `INV-A` |
| esquema `contrato_version`/`id_cambio`/`fecha`/`tipo_operacion` | `INV-L` (legacy contractual) |
| `*-temp*` / tipología análisis-temporal / estado borrador | `BORRADOR` |
| fecha ausente/inválida | `SIN_FECHA` |
| `hash_integrity` vacío/`""` | no conforme canónico; no rellenar |

### Jurisdicción filename

El contrato declara verbatim: registros bajo `directories.evolution` **no** están sujetos al filename `{name}.md` del estándar atómico de entidades genoma; el físico canónico es `{id_cambio}.md`.

## Evolution_log.md

1. Frontmatter / cabecera Markdown: `contrato_version: 1.1.0`, vínculo a clave Cúmulo `normative_documents.evolution_contract`, `source_audit`, `universe_total: 61`, `cut_commit` del informe.
2. Tabla columnas: `id_cambio` \| `fecha` \| `resumen` \| `clase_formato` \| `ruta_relativa`.
3. Una fila por cada uno de los 61 nombres del inventario del audit.
4. Orden: fecha descendente; `SIN_FECHA` al final; empate → `ruta_relativa` ascendente.
5. `clase_formato`: unión de etiquetas del audit (`INV-A`, `INV-L`, `NOMBRE`, `UUID-INV`, `BORRADOR`) sin reinterpretar veredictos materiales.

## Validador (`sddia-qa validate-evolution-contract`)

```text
sddia-qa validate-evolution-contract \
  [--json] \
  --universe audit-cut \
  --audit-ref <path-relativo-informe>
```

| Comportamiento | Detalle |
|----------------|---------|
| Resolve | Lee rutas vía Cúmulo (`evolution_contract`, `evolution_log`, `directories.evolution`). |
| Preconditions | Fallo explícito si contrato o log ausentes. |
| Clasificación | Por cada registro del universo: clases de formato + flags (`has_hash`, `tipo_canonico`, …). |
| Mutación | **Prohibida** (abrir solo lectura). |
| Exit | `0` si clasificó N=universo sin I/O error; ≠0 si falta SSOT o no puede leer. **No** falla por legacy/no-conforme (eso es salida JSON, no gate). |
| Evidencia AC | JSON con `classified_total`, `universe_total`, `by_class`, `rows[]`; volcar en `persist_ref` como `_qa-validate-evolution.json`. |

## Criterios técnicos ↔ AC

| AC | Verificación |
|----|--------------|
| AC-PATHS | `test -f` rutas Cúmulo; coherentes con claves. |
| AC-CONTRACT | Contrato v1.1.0 con campos, enums, alias, jurisdicción. |
| AC-JURISDICTION | Sección explícita en contrato. |
| AC-VALIDATOR | `sddia-qa … --universe audit-cut` → `classified_total==61`; `git status` sin diffs en `directories.evolution/*.md` de detalle. |
| AC-LOG | 61 filas; borradores y SIN_FECHA explícitos. |
| AC-PR | Cascada + QA + PBI `done/` + `validacion.md` en un PR. |

## Fuera de alcance (reafirmado)

Normalización física; gate CI; mutación Cúmulo; indexar delta post-corte en este PR.

## Handoff Tekton

Materializar contrato + índice 61 + subcomando QA + docs migración delta; registrar evolution del propio hito **después** del índice del corte (el registro nuevo del ciclo no altera las 61 filas del corte).
