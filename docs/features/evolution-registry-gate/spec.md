---
feature_name: evolution-registry-gate
created: "2026-08-13"
process: feature
branch_name: feat/evolution-registry-gate
persist_ref: docs/features/evolution-registry-gate
pbi_ref: docs/todos/pending/[FEATURE] Evolution — gate automático de registro y coherencia (EV-AUD-001-002).md
document_id: 70f78d23-e209-4e41-9292-cb7421a934f6
execution_id: 0bceeb41-64d1-4920-af9d-46a11c0455a2
phase: blueprint
agents: dedalo
base: main
scope: "Cápsula sddia-evolution-register + gate sddia-qa + aduana pre-commit/CI (EV-AUD-001/002)"
---

# Spec — evolution-registry-gate

## Problema

`sddia-evolution-sync` obliga a registrar alta/baja/modificación material bajo `./SddIA/`. El contrato v1.1.0 y el validador `validate-evolution-contract` son **solo lectura**. No existe aduana que falle commit/PR ante diff material sin evolution correlacionada, ni cápsula de escritura atómica detalle+índice.

## Decisiones de diseño (laudos Dedalo)

| ID | Laudo |
|----|--------|
| **L-CUMULO-KEYS** | Cúmulo 1.6.1 **no** declara `paths.skillCapsules` ni `paths.skillsRustPath`. Resolución: `directories.skills/{name}.md`; crate `execution_capsules.skills/{name}/`; WASI `compiled_capsules.wasm_root`; CLI `compiled_capsules.native_root`. Prohibido hardcodear; prohibido inventar claves Cúmulo. |
| **L-SPLIT** | Dominio WASI = skill `sddia-evolution-register` (veredicto + cálculo de nuevo estado). Orquestación nativa = `sddia-qa gate-evolution` (captura, inyección, persistencia). `validate-evolution-contract` permanece intacto. |
| **L-ATOMIC** | Cápsula emite `{ detail, index }` canónicos. CLI: backup índice → write detalle → patch índice → verify → rollback si falla. |
| **L-FORGE** | Alta de skill vía `entity-manager` → `skill-creator` (`skill_context: ecosystem-evolution`, ya citado en `execution-contexts.md`). Extensión `sddia-qa` bajo topología feature (DA-4); sin tool-creator. |
| **L-CONTRACT-111** | Bump `evolution_contract.md` **1.1.0 → 1.1.1**: exclusiones, reason-codes, política `hash_integrity`, semántica gate vs validador. Compatibilidad lectora del validador 1.1.0 intacta. |
| **L-ENFORCE-DELTA** | Fail-hard **solo sobre el diff inyectado**. No certifica los 61 históricos como canónicos. Universo legacy sigue en `validate-evolution-contract` (no bloqueante). |
| **L-INJECT** | Captura del árbol = **CLI nativo** (`sddia-qa`). Inyección stdin `capsule-json-io` v2.0: `request.diff` + `request.registry`. Cápsula **prohibido** Git / working tree / cálculo de diff. |
| **L-HOOK-INERT** | Pre-commit = detonador inerte: solo invoca `sddia-qa`. Abort iff sobre `success: false` ∧ `exitCode > 0`. Prohibido inventario de paths, ephemeral de diff o cotejo en el hook. |
| **L-CLI-ARGOS** | `sddia-qa gate-evolution` captura árbol, carga registro vía Cúmulo, invoca WASI (`wasmtime`), echa el sobre stdout, propaga `exitCode`. CI = mismo CLI. |
| **L-EXCL** | Exclusiones **solo** las del contrato §7. Conjunto inicial: todo path bajo `directories.evolution`. Lista ad hoc en hooks **prohibida**. |
| **L-SELF** | Diff ⊆ `directories.evolution` → `EVOL_OK` (el protocolo es el registro). No exigir segundo correlato. |
| **L-CORRELATE** | Path material `P` cubierto iff algún registro **añadido o modificado** en el mismo diff (no `BORRADOR`) lista `P` o un prefijo directorio de `P` en `relacionado` (alta/modificacion) o `rutas_eliminadas` (baja). |
| **L-HASH** | `hash_integrity` = `sha256:` + hex del payload canónico del registro (frontmatter **sin** el campo `hash_integrity` + cuerpo Markdown, UTF-8, newlines LF). Lo calcula la cápsula sobre JSON inyectado/emitido; el CLI no reinterpreta la fórmula. |
| **L-CODES** | Enum estable en contrato §8 y JSON `reason_code`. Prohibidos mensajes solo narrativos. |
| **L-NO-BYPASS** | Ningún env nuevo de skip para IA obrera. `SDDIA_SKIP_HOOKS=1` permanece humano-soberano (norma). |
| **L-WASI-DOMAIN** | Sustrato de dominio = `wasm32-wasip1` (`compiled_capsules.wasm_root`). Persistencia host (write detalle/índice) = CLI nativo aplicando el JSON emitido. Sin excepción WASI→nativo para Git: Git no existe en la cápsula. |

## Contrato I/O — `sddia-evolution-register`

Envelope `capsule-json-io` v2.0. `meta.entityKind: skill`. `meta.entityId: sddia-evolution-register`.

### `request`

| Campo | Tipo | Regla |
|-------|------|--------|
| `operation` | enum | `verdict` \| `alta` \| `modificacion` \| `baja` |
| `diff` | object | **Inyectado por el CLI.** Inventario de paths del árbol (`path`, `status`: A/C/M/R/D). Obligatorio en `verdict`. La cápsula no lo calcula. |
| `registry` | object | **Inyectado por el CLI.** Snapshot JSON de registros + índice bajo `directories.evolution` (vía Cúmulo). Obligatorio en `verdict`. |
| `id_cambio` | UUID v4 | Obligatorio en `modificacion`/`baja`. En `alta`: opcional; si ausente la cápsula genera UUID v4. |
| `fecha` | string | ISO `YYYY-MM-DD` o datetime. Obligatoria en alta/modificacion/baja; la cápsula **no inventa** fecha. |
| `descripcion_breve` | string | No vacío (mutaciones). |
| `relacionado` | string[] | ≥1 path lógico / PBI / entidad (mutaciones). |
| `rutas_eliminadas` | string[] | Obligatorio si `baja` y hay artefactos retirados. |
| `commit_referencia_previo` | string | Obligatorio si `baja` cuando exista commit previo. |
| `dry_run` | bool | Default false. Si true: valida y reporta; CLI **no** persiste. |

Prohibido en cápsula: invocar Git, leer working tree, aportar `diff` propio. Hash: lo sella la cápsula, no el caller.

### `result` (éxito)

| Campo | Contenido |
|-------|-----------|
| `operation` | Eco |
| `reason_codes` | Lista; `verdict` OK → `[EVOL_OK]` o vacía |
| `findings` | `[{ path, reason_code, detail }]` |
| `id_cambio` | UUID v4 efectivo (mutaciones) |
| `detail` | Markdown+frontmatter propuesto (mutaciones); CLI persiste |
| `index` | Índice propuesto (mutaciones); CLI persiste |
| `hash_integrity` | `sha256:…` sellado |
| `idempotent` | `true` si estado propuesto ≡ persistido inyectado |

Fallo: `success: false`, `exitCode ≠ 0`, `reason_code` ∈ enum, `message` accionable. Coherencia `capsule-json-io`: `exitCode === 0` ⟺ `success === true`.

## Reason-codes (L-CODES)

| Código | Cuándo | Exit |
|--------|--------|------|
| `EVOL_OK` | Gate/cápsula OK (incl. L-SELF, idempotencia) | 0 |
| `EVOL_MATERIAL_UNREGISTERED` | Path material en diff sin correlato | ≠0 |
| `EVOL_RECORD_INVALID` | Registro del diff no canónico v1.1.1 | ≠0 |
| `EVOL_NOT_INDEXED` | Detalle del diff sin fila en índice | ≠0 |
| `EVOL_HASH_MISMATCH` | `hash_integrity` ≠ recompute | ≠0 |
| `EVOL_DUPLICATE` | `id_cambio` ya existe (alta) | ≠0 |
| `EVOL_ATOMICITY` | Detalle/índice inconsistentes tras write | ≠0 |
| `EVOL_CUMULO` | Falta clave/ruta Cúmulo | ≠0 |

JSON de gate (stdout `--json`): `{ "success", "reason_codes": [], "findings": [{ "path", "reason_code", "detail" }], "mode": "delta" }`. Exit 0 iff `success` y `reason_codes` ⊆ {`EVOL_OK`} o vacío.

## Gate — `sddia-qa gate-evolution` (CLI nativo)

```text
sddia-qa gate-evolution [--json]
```

Sin `--paths-file` desde el hook. El CLI:

1. Resuelve Cúmulo (`directories.evolution`, `evolution_contract`, `evolution_log`). Falta → sobre `EVOL_CUMULO`, `success: false`, `exitCode > 0`.
2. Captura el árbol (staged en pre-commit; `origin/main...HEAD` en CI).
3. Serializa `request.diff` + `request.registry` + `operation: verdict`.
4. Invoca WASI `sddia-evolution-register` (`wasmtime`, stdin envelope v2.0).
5. Escribe stdout = sobre de la cápsula; `exit` = `exitCode` del sobre.

La **partición material/evo y el cotejo L-CORRELATE viven en la cápsula**, no en el CLI ni en el hook.

**No** clasifica ni falla por schema de registros **fuera** del diff inyectado (universo 61).

## Contrato v1.1.1 — exclusiones (§7)

| Path lógico Cúmulo | Efecto |
|--------------------|--------|
| `directories.evolution/**` | Fuera de correlación material (L-SELF / L-EXCL) |

No se listan `compiled_capsules` ni untracked: no entran en el diff versionado.

## Aduanas

| Superficie | Wiring |
|------------|--------|
| pre-commit | Detonador inerte: invocar `sddia-qa gate-evolution --json`. Abort **solo** si sobre `success: false` ∧ `exitCode > 0`. Cero `git diff`, cero ephemeral de inventario, cero cotejo. |
| CI | Job en `.github/workflows/sddia-index-qa.yml`: build `sddia-qa` + wasm; mismo CLI. |

El CLI puede reutilizar `git` nativo (como `git_run` ya usado en hooks) **dentro de sddia-qa**, no en el hook. No se extiende el enum congelado de `git-manager` en este PBI.

## Criterios técnicos ↔ AC

| AC | Verificación |
|----|--------------|
| AC-ATOMIC | Test: CLI aplica `result.detail`+`result.index`; fallo a mitad deja pre-estado. |
| AC-MATERIAL | Fixture: JSON `diff` con path material sin correlato en `registry` → `EVOL_MATERIAL_UNREGISTERED` **sin** Git en el test de cápsula. |
| AC-INVALID | Fixture: registro inyectado sin hash / UUID inválido / no indexado → código estable. |
| AC-SELF | Fixture: `diff` ⊆ `directories.evolution` → exit 0. |
| AC-TESTS | alta, modificacion, baja, duplicado, hash inválido, idempotente + AC-SELF + AC-MATERIAL + AC-INJECT + AC-HOOK-INERT. |
| AC-CUMULO | CLI lee solo `load_paths_config`; test con claves ausentes → `EVOL_CUMULO`. |
| AC-ADUANA | Hook inerte + job CI invocan el CLI; sin env de bypass obrero. |
| AC-INJECT | Tests WASI: stdin con `diff`/`registry`; crate sin `std::process` Git. |
| AC-HOOK-INERT | `pre_commit_gate.sh` no contiene `diff --cached` para evolution; solo invocación CLI + chequeo sobre. |
| AC-WASI | Artefacto `wasm32-wasip1`; `wasmtime run`. |
| AC-DIAG | Sobre JSON con `reason_code`; `exitCode === 0` ⟺ `success`. |
| AC-DEP | Gate modo `delta` inyectado; 61 legacy no entran. |
| AC-PR | Cascada + PBI `done/` + `validacion.md` en el mismo PR. |

## Fuera de alcance (reafirmado)

Migración física de históricos; extracción `*-temp*`; mutar `cumulo.paths.json`; sustituir `validate-evolution-contract`; fail-hard sobre schema del universo 61.

## Handoff Tekton

Forjar skill WASI + crate `wasm32-wasip1`; CLI `gate-evolution` (captura+inyecta+persiste); contrato 1.1.1; hook **inerte**; CI; tests de la matriz (veredicto con JSON inyectado, sin Git en cápsula); `implementation.md` / `execution.md`; auto-registro del hito vía `operation: alta` (CLI persiste) **antes** del commit de genoma.
