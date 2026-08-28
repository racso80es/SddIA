---
feature_name: capsula-binario-fosil-release-stale
created: "2026-08-28"
updated: "2026-08-28"
process: bug-fix
branch_name: fix/capsula-binario-fosil-release-stale
persist_ref: docs/fixes/capsula-binario-fosil-release-stale
pbi_ref: docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md
document_id: PBI-FIX-FRACTURE-6a49e0ad310e-R1
uuid: 7c3e9a12-4b8f-4d2e-9a61-0e5f8c2d1b47
scope: execution-anchor-pattern
base: main
execution_id: "13161205-2a2a-4320-9953-554e18a1f7c5"
fracture_hash: 6a49e0ad310e
laudo_anchor: source-sha256-plus-local-elf-witness
laudo_digest: crate-plus-local-deps-A-with-B-lockfile-fallback
laudo_freshness: revoked-mtime
---

# Spec — Patrón de Anclaje de Ejecución

Sustituye el spec de frescura por `mtime` (Dedalo, ciclo abortado). SSOT de criterios: PBI R1 v1.3.0.

## Problema

`resolve_capsule_native` itera `compiled_capsules.profiles` (`release` → `debug`) y sirve el **primer ELF que existe**. No comprueba identidad. En esta instancia, `target/release/iota-immutable-publisher` rechaza `payload` array (`Campo obligatorio ausente o inválido: payload`); el fuente vigente (`d78cafb` / `43f8bf3`) y el binario `debug` lo aceptan. El motor etiqueta el fallo como `F-DLT-RELAY-SIN-SUPERVISOR` sin haber tocado red (repro bajo `SDDIA_LAB_SIMULATE_IOTA=1`).

El gate `L-DIRTY-INIT` no desescapa quoting octal de `git status --porcelain`, así que un `pbi_ref` con `[REGRESIÓN]` nunca entra en scope. El ciclo `bug-fix` de este PBI aborta en Inicialización.

## Causa raíz (hechos, no hipótesis)

| ID | Hecho | Trabajo |
|----|-------|---------|
| D1 | Resolución ciega a identidad del artefacto | Aduana `source_sha256` + testigo local (CA5/CA6) |
| D2 | Cicatriz `L-BUNDLE-STALE` solo en bundle y solo `CONSUMER_BINS` | Promover al motor; cobertura indexada incl. publisher (CA8) |
| D3 | Digest de fuente incluye `Cargo.lock` completo → invalidación global | Granularidad A + fallback B (CA9) |
| D4 | `version:` del genoma no se mueve con el crate | Ancla automática vía `entity-manager`, nunca a mano (CA7) |
| D5 | Porcelain octal vs `pbi_ref` UTF-8 | Desescape o `status -z` (CA1/CA2) |
| D6 | Colapso de fase sin `System_Fracture_Detected` | Emitir o excluir por norma (CA3) |
| D7 | Traza de batch atribuye relay por defecto | IDs factuales (CA12) |
| D8 | Log de debug con ruta absoluta de host en Core | Purga (CA13) |
| D9 | Cola reanchor reintenta el mismo ELF | Inventario + rebuild **antes** del fallo duro; drenaje (CA14–CA16) |

**Correcciones de magnitud (PBI §1.2 / §8.2):** no afirmar «17 fósiles» ni «746 fracturas todas de `route-domain-event`». Fosilidad **probada** = 1 (`iota-immutable-publisher`). Eventos únicos de fractura = 380; de `route-domain-event` = 11; de `bug-fix` = 0.

## Laudos de diseño

| ID | Decisión |
|----|----------|
| L-MTIME-REVOKED | Gate por `mtime` **prohibido**. El tiempo no es identidad. |
| L-ANCHOR-SPLIT | Genoma declara `source_sha256` (portable). Testigo junto al ELF declara `elf_sha256` + `source_sha256` (local, no versionado: `target/` está en `.gitignore`). Cadena: genoma → testigo → ELF. |
| L-NO-ELF-IN-GENOME | Prohibido `compiled_target_hash` / hash de ELF en el `{name}.md`. Compilación no reproducible bit a bit (sin `rust-toolchain.toml`). |
| L-PROFILE-BLIND | `profiles` = orden de **búsqueda**. Se sirve el primer artefacto que **cumple** el contrato; si release no cumple y debug sí → se sirve debug (CA11e). Si ninguno cumple → `capsule-stale-hash`. |
| L-DIGEST-A-B | Digest de fuente = crate + cierre transitivo de crates **locales** + recorte del lockfile a esas dependencias (**A**). Si `cargo metadata` no está disponible → lockfile+workspace manifests completos (**B**), documentado. Prohibido C (solo crate) como política permanente. |
| L-CACHE | Caché de aduana en `.SddIA/` (p. ej. bajo claves de instancia ya SSOT). Invalidación `(path, mtime, size)` del ELF. **Prohibido** mutar `cumulo.paths.json` como almacén de hashes. |
| L-ATTACK-ORDER | 1) CA1–CA3. 2) CA14 (rebuild + inventario). 3) anclas + aduana con fallo duro. Invertir 2 y 3 deja el runtime inoperante. |
| L-NO-MANUAL-HASH | `source_sha256` en genoma solo vía `entity-manager` (o proceso de cierre que lo invoque). Write directo sobre `SddIA/tools/*.md` = violación DA-2. |
| L-REUSE-BUNDLE | No inventar un segundo algoritmo. Extraer `_sddia_source_digest` / testigo a Core Rust; el script de bundle **delega** o comparte el mismo contrato de bytes. |

## Solución

### 1. Bloque I — Aduana de apertura (CA1–CA3)

`workspace_init.rs` `porcelain_path_from_line`:

- Desescapar `\NNN` octal y escapes C (`\\`, `\"`, `\t`, `\n`) **antes** de comparar con `pbi_ref` / `persist_ref`.
- Preferible: `git-manager` `status` con `-z` si el contrato frozen lo admite en este ciclo; si no, desescape sobre porcelain citado.
- Dejar de aplicar `replace('\\', "/")` sobre la ristra **aún citada**.

Test: línea literal ` M "docs/todos/pending/[REGRESI\303\223N] … sist\303\251mica …-R1.md"` + `pbi_ref` UTF-8 → in-scope.

CA3: `dirty-worktree` (y homólogos de precondición de `workspace_init`) emiten `System_Fracture_Detected` con `process_name` del proceso vivo y `error_trace` literal, **o** una norma en `obediencia-procesos.md` los excluye explícitamente del bus. Default de diseño: **emitir** (el silencio actual es el defecto). Genoma del evento: sin Write directo; si hace falta campo nuevo → `entity-manager`.

### 2. Bloque II — Tres piezas del ancla (CA4–CA11)

**Declaración.** Frontmatter de cada cápsula indexada (`tools` / `skills` / `daemons` / crates de `engine` con binario invocable):

```yaml
source_sha256: "sha256:<hex>"
```

Formato idéntico al testigo actual (`sha256:` + hex). Ausencia del campo (CA11d): `capsule-stale-hash` con razón `genome-missing-source-sha256` — **sin panic**, sin servir ELF. Backfill inicial = mismo pipeline que CA7, no edición humana.

**Testigo local.** `{native_root}/{profile}/{name}.sha256`:

```
source_sha256: sha256:…
elf_sha256: sha256:…
```

Escritura: línea de montaje (build runtime / bundle / paso post-`cargo build`). No se commitea (`target/`).

**Aduana** en `resolve_capsule_native` (y WASM por simetría si el path se spawnea):

1. Leer `source_sha256` del genoma `{execution_capsules|directories}/{class}/{name}.md`.
2. Para cada perfil, si existe ELF: verificar testigo + `elf_sha256(ELF)==testigo` + `testigo.source == genoma.source`.
3. Primer match → `Ok(path)`.
4. Ninguno → `Err(capsule-stale-hash: {name} — genoma {sha} / artefacto {sha|ausente})`.

Call sites (`capsules.rs`): propagar el `Err`; no degradar a python/script salvo el `NotFound` histórico de cápsula **sin** binario en ningún perfil.

**Digest (A).** Entrada: `Cargo.toml` + `build.rs` + `src/**` del crate; crates path-dep locales transitivos; entradas de `Cargo.lock` **solo** de ese cierre. Algoritmo de hasheo: lista `relpath TAB file_sha256` ordenada `LC_ALL=C`, luego SHA-256 del listado (paridad con L147–180 del bundle). Fallback B: incluir `SddIA/Cargo.toml` + `Cargo.lock` completos si metadata falla.

Test CA9: mutar un fichero de crate A no cambia digest de crate B (bajo política A).

**Coste (CA10):** medir en `execution.md`; caché `.SddIA/` con invalidación de ELF.

### 3. Bloque III — Higiene y físico (CA12–CA16)

Traza batch: matriz factual

| Hecho medido | `friction_id` |
|--------------|---------------|
| `iota-relay-unreachable` / health relay caído | `F-DLT-RELAY-SIN-SUPERVISOR` |
| `capsule-stale-hash` | `F-CAPSULA-BINARIO-FOSIL` |
| Rechazo de contrato de entrada (p.ej. payload) | `F-CAPSULA-CONTRATO-ENTRADA` + path/perfil si conocido |
| Otro | causa literal; **prohibido** prefijo relay |

Purga `// #region agent log` en `route_domain_core.rs` (~L1136–1159).

CA14 **antes** de activar el `Err` duro en runtime de producción: rebuild de cápsulas indexadas, escribir testigos, sellar genomas vía EM, tabla en `execution.md`. Luego CA15/CA16 (pre-sellado + drenaje cola instancia; la cola `.SddIA/` no viaja en el PR).

`start-sddia.sh` / bundle: no dejar solo `-p execute-process`. El build debe producir ELF+testigo de las cápsulas invocables, **incluido** `iota-immutable-publisher`.

### 4. Cierre (CA17–CA18)

Patrón documental v1.2.0: `implementation.md` + `execution.md` + `validacion.md` APTO `pbi_archived: true`; PBI R1 a `docs/todos/done/` en la **misma** rama; anotar padre sin reabrir.

## Criterios de aceptación (Argos)

CA1–CA18 = PBI §5. Numeración del PBI es la canónica (no la del spec abortado).

## Fuera de alcance

- Reimplementar relay / centinela (Kaizen #208).
- DL Mayeuta fan-out.
- `rust-toolchain.toml` / reproducibilidad bit a bit.
- WASI.
- Reabrir `PBI-FIX-FRACTURE-6a49e0ad310e`.
- Hash de ELF en git.

## Riesgo operativo

Activar aduana dura sin CA14 → parada del parque. Mitigación = L-ATTACK-ORDER. Política A mal implementada (olvidar path-deps) → falsos conformes; tests CA9 obligatorios.
