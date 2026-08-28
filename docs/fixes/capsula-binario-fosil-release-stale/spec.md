---
feature_name: capsula-binario-fosil-release-stale
created: "2026-08-28"
process: bug-fix
branch_name: fix/capsula-binario-fosil-release-stale
persist_ref: docs/fixes/capsula-binario-fosil-release-stale
pbi_ref: docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md
document_id: PBI-FIX-FRACTURE-6a49e0ad310e-R1
uuid: 7c3e9a12-4b8f-4d2e-9a61-0e5f8c2d1b47
scope: capsule-freshness-gate
base: main
execution_id: "13161205-2a2a-4320-9953-554e18a1f7c5"
fracture_hash: 6a49e0ad310e
laudo_freshness: fail-stale
inputs_note: "objectives.md ausente en persist_ref; refined_requirements = bug_summary orquestador + cuerpo PBI R1"
---

# Spec — Gate de frescura de cápsulas compiladas (binario fósil)

## Problema

`resolve_capsule_native` itera `compiled_capsules.profiles` (`release` → `debug`) y devuelve el **primer binario que existe**, sin comparar frescura con fuentes. Runtime sirve `target/release/iota-immutable-publisher` (2026-07-20, pre-`d78cafb`) que rechaza `payload` array; el fuente/`debug` actuales lo aceptan. `route_domain_core` atribuye el fallo a relay (`F-DLT-RELAY-SIN-SUPERVISOR`), encola reanclaje y emite fractura especulativa. Deriva medida: **17 cápsulas** release más viejas que debug.

Repro determinista (PBI §1.1): release → `Campo obligatorio ausente o inválido: payload`; debug → `merkle_root` + pruebas.

## Causa raíz

| ID | Hecho | Trabajo |
|----|-------|---------|
| D1 | Precedencia ciega a `mtime` en `capsule_paths.rs` | Gate frescura + política SSOT (CA3/CA4) |
| D2 | `start-sddia.sh` solo `cargo build -p execute-process` | Build reproducible de cápsulas indexadas (CA8) |
| D3 | `emit_dlt_batch_fracture` fija `friction_id` relay ante cualquier causa | Traza factual + IDs propios (CA6) |
| D4 | Cola `.SddIA/dlt/reanchor-queue/` (10) reintenta contra fósil | Drenaje post-fix (CA2) |
| D5 | `#region agent log` con ruta absoluta host en Core | Purga (CA9) |
| D6 | Artefactos release obsoletos en disco | Inventario + recompile/purge (CA7/CA1) |

## Laudo de diseño (semilla + PBI §6 opción A)

**Política `fail-stale` (opción A).** Primer perfil cuyo binario exista:

1. Si `mtime(binario) < max(mtime fuentes)` → `Err` con prefijo canónico  
   `capsule-stale: <name> <perfil> <mtime_bin> < fuente <mtime_src>` — **no** se ejecuta, **no** se cae al siguiente perfil.
2. Si fresco → servir ese path.
3. Si ningún perfil tiene fichero → `NotFound` (comportamiento actual de ausencia).

Prohibido opción B (servir el más reciente ignorando orden) y C (sellado de versión en respuesta) en este PBI.

## Solución

### 1. SSOT — `compiled_capsules` en `cumulo.paths.json` (CA4)

Ampliar bloque existente (sin hardcode de perfiles en handlers):

```json
"compiled_capsules": {
  "native_root": "SddIA/target",
  "wasm_root": "SddIA/target/wasm32-wasip1",
  "profiles": ["release", "debug"],
  "freshness": {
    "policy": "fail-stale",
    "source_roots": ["tools", "skills", "daemons"]
  }
}
```

- `source_roots`: claves lógicas bajo `execution_capsules` (`tools`/`skills`/`daemons`); rutas físicas solo vía Cúmulo.
- Ausencia de `freshness` → defaults: `policy=fail-stale`, `source_roots=["tools","skills","daemons"]`.
- Ausencia de `profiles` → defaults actuales `["release","debug"]` (ya en `default_roots`).

### 2. Core — `capsule_paths.rs` (CA3/CA5)

API:

```text
enum CapsuleResolveError { NotFound, Stale { name, profile, bin_mtime, src_mtime, message } }
fn resolve_capsule_native(repo, name) -> Result<PathBuf, CapsuleResolveError>
```

(o `Result` equivalente; `Option` actual queda **retirada** en el path nativo invocado por runtime).

Algoritmo:

1. Cargar roots + `freshness` desde Cúmulo.
2. Para cada `profile` en orden: si `native_root/profile/name` es fichero:
   - Localizar fuentes: `{execution_capsules[root]}/{name}/src/**` (primer root que exista con `src/`).
   - Sin fuentes localizables → **degradación documentada**: servir binario (no panic); telemetría/log `freshness-skipped: no-sources` (CA5c).
   - Con fuentes: `src_mtime = max mtime` de ficheros bajo `src/` (no dirs). Si `bin_mtime < src_mtime` → `Stale`.
   - Else → `Ok(path)`.
3. Si ningún fichero → `NotFound`.

WASM: mismo gate si el path se usa en runtime; si no hay invocación WASM stale en el incidente, aplicar el mismo contrato por simetría mínima o dejar TODO explícito en `execution.md` — **preferencia: mismo gate en `resolve_capsule_wasm`**.

Call sites (`capsules.rs` y tests): propagar `Stale` como error de invocación (`success: false` / `Err`), nunca silenciar.

Tests unitarios tempdir (CA5):

| Caso | Esperado |
|------|----------|
| (a) release fresco vs fuentes | Ok(release) |
| (b) release fósil + debug fresco | `Stale` (no Ok(debug)) |
| (c) binario sin fuentes | Ok(binario) + skip documentado |
| (d) `profiles`/`freshness` ausentes | defaults; sin panic |

### 3. Traza de fractura no especulativa (CA6)

`emit_dlt_batch_fracture` / pre-sellado:

| Condición medida | `friction_id` | `error_trace` |
|------------------|---------------|---------------|
| Relay health caído / `iota-relay-unreachable` | `F-DLT-RELAY-SIN-SUPERVISOR` | hecho relay |
| `capsule-stale:…` | `F-CAPSULA-BINARIO-FOSIL` | ruta, perfil, mtimes |
| Rechazo contrato entrada cápsula (p.ej. `Campo obligatorio…payload`) | `F-CAPSULA-CONTRATO-ENTRADA` | causa literal + path binario invocado si conocido |
| Otro fallo publisher post-validación | ID factual (no relay por defecto) | causa literal |

Prohibido prefijar relay cuando la causa no verifica relay. Tests existentes que asumen `F-DLT-RELAY-SIN-SUPERVISOR` ante causa genérica: actualizar a la matriz.

### 4. Higiene Core (CA9)

Eliminar bloque `// #region agent log` … `#endregion` en `route_domain_core.rs` (~L1136–1159) que escribe `/home/racso/Proyectos/SddIA/.cursor/debug-478d0f.log`. Sustituir por nada (el `skipped-empty-message` ya retorna).

### 5. Remediación física (CA1/CA2/CA7)

Tras gate + binarios frescos:

1. Inventario 17 cápsulas (tabla PBI §1.2) en `execution.md`: por cada una `rebuild-release` | `delete-stale-artifact` | `n/a`.
2. Recompilar al menos `iota-immutable-publisher` release; resto según inventario.
3. Drenar `.SddIA/dlt/reanchor-queue/`: con publisher fresco y relay/sim OK, `try_drain` o cierre documentado; cola vacía o sin causa `payload`.
4. Verificar pre-sellado: array payload → `merkle_root` + sin `last_batch_anchor_error` nuevo por este fallo.

### 6. Build runtime (CA8)

`start-sddia.sh` `_ensure_orchestrator` (o paso hermano previo a ignición): garantizar coherencia de **cápsulas nativas indexadas** usadas por el runtime, no solo `-p execute-process`.

Diseño mínimo aceptable:

- Construir workspace members que correspondan a tools/skills/daemons con binario nativo invocable, **o**
- `cargo build --release` de un manifiesto derivado de `execution_capsules` + índice (lista explícita en script, mantenida vía comentario SSOT “sincronizar con members”).

Prohibido dejar solo `execute-process` como único build de arranque. Documentar comando en `execution.md`.

## Criterios de aceptación (mapeo Argos)

| ID | Criterio |
|----|----------|
| CA1 | Pre-sellado batch OK con payload array; sin `last_batch_anchor_error` por este fallo |
| CA2 | Cola reanchor vacía o sin causa `payload` |
| CA3 | Gate `fail-stale` en `resolve_capsule_native` |
| CA4 | Política en `cumulo.paths.json`; defaults si falta clave |
| CA5 | Tests (a)–(d) en `capsule_paths` |
| CA6 | Fractura factual; relay solo si medido |
| CA7 | Inventario 17 en `execution.md` |
| CA8 | Arranque/build multi-cápsula |
| CA9 | Sin ruta absoluta debug-log en Core |
| CA10 | `validacion.md` APTO + `pbi_archived: true` |
| CA11 | PBI R1 → `docs/todos/done/` en rama del PR; anotación causa en padre (sin reabrir) |

## Fuera de alcance

- Relay IOTA / centinela (Kaizen #208).
- 351 DL Mayeuta fan-out.
- WASI migración.
- Opción B/C de frescura.
- Reabrir `PBI-FIX-FRACTURE-6a49e0ad310e`.

## Riesgo operativo

Primer arranque post-merge con 17 release fósiles → fallos `capsule-stale` hasta rebuild (CA7/CA8). Esperado; no atenuar con fallback silencioso a debug.
