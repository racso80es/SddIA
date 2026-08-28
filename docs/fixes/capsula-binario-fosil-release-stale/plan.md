---
feature_name: capsula-binario-fosil-release-stale
created: "2026-08-28"
process: bug-fix
branch_name: fix/capsula-binario-fosil-release-stale
persist_ref: docs/fixes/capsula-binario-fosil-release-stale
pbi_ref: docs/todos/pending/[REGRESIÓN] route-domain-event — fractura sistémica (6a49e0ad310e)-R1.md
document_id: PBI-FIX-FRACTURE-6a49e0ad310e-R1
uuid: a91f2d40-6e3b-4c8a-b7f1-2d9e0c5a84f6
execution_id: "13161205-2a2a-4320-9953-554e18a1f7c5"
phases:
  - l0-ssot-freshness
  - l1-gate-capsule-paths
  - l2-call-sites
  - l3-fracture-trace
  - l4-purge-agent-log
  - tests-unit
  - l5-build-runtime
  - l6-remediate-binaries-drain
  - doc-closure
---

# Plan — Gate fail-stale + remediación fósiles

Orden: L0 → L1 → L2 → L3 → L4 → tests → L5 → L6 → cierre documental. Este artefacto sella Diseño (`spec.md` + `plan.md`). Código = fase Ejecución (Tekton).

## Fase L0 — SSOT frescura (CA4)

Archivo: `SddIA/core/cumulo.paths.json`.

1. Bajo `compiled_capsules`, añadir `freshness.policy: fail-stale` y `freshness.source_roots: ["tools","skills","daemons"]`.
2. No alterar orden `profiles: ["release","debug"]`.
3. Defaults en Rust deben coincidir si la clave falta (documentar en comentario de `load_compiled_capsule_roots`).

## Fase L1 — Gate en `capsule_paths.rs` (CA3)

Archivo: `SddIA/engine/execute-process/src/engine/capsule_paths.rs`.

1. Extender struct de carga con `freshness_policy` + `source_roots`.
2. Helper `max_source_mtime(repo, name, source_roots) -> Option<SystemTime>` vía `execution_capsules` + `src/**` files.
3. `resolve_capsule_native` → `Result<PathBuf, CapsuleResolveError>` con `NotFound` | `Stale {…}`.
4. Misma semántica en `resolve_capsule_wasm` (simetría).
5. Mensaje Stale: `capsule-stale: {name} {profile} {bin_mtime} < fuente {src_mtime}` (RFC3339 o epoch secs; una forma, estable en tests).

## Fase L2 — Call sites (propagación)

Archivos: `capsules.rs` y cualquier match/`unwrap` de `Option` sobre resolve nativo/wasm.

1. `Stale` → error de invocación visible al caller (JSON/`Err`), no fallback a otro perfil.
2. `NotFound` → comportamiento previo de ausencia (python/script fallback si existía).
3. Actualizar tests del crate que asuman `Option`.

## Fase L3 — Traza factual (CA6)

Archivo: `route_domain_core.rs` (`emit_dlt_batch_fracture`, pre-sellado batch, tests ~L1780/1855).

1. Clasificar `causa` / error de invoke:
   - contiene `capsule-stale` → `F-CAPSULA-BINARIO-FOSIL`
   - contrato entrada / `Campo obligatorio…payload` → `F-CAPSULA-CONTRATO-ENTRADA` + path si disponible
   - `iota-relay-unreachable` / health fail → `F-DLT-RELAY-SIN-SUPERVISOR`
2. `error_trace` = prefijo del `friction_id` + hecho medido (sin hipótesis).
3. Ajustar unit tests de fractura.

## Fase L4 — Purga log absoluto (CA9)

En `route_domain_core.rs` L1136–1159: borrar región agent log + `OpenOptions` a path host. Dejar retorno `skipped-empty-message`.

## Fase tests unitarios (CA5) — antes de L5

En `capsule_paths.rs` `#[cfg(test)]` con tempdir + cumulo mínimo inyectado o roots override testables:

- (a) release fresco → Ok(release)
- (b) release fósil + debug fresco → Stale (no Ok(debug))
- (c) sin fuentes → Ok + no panic
- (d) sin `freshness`/`profiles` en JSON → defaults

Comando: `cd SddIA && cargo test -p execute-process capsule_paths`

## Fase L5 — Build arranque (CA8)

Archivo: `start-sddia.sh` (`_ensure_orchestrator` o paso previo).

1. Tras/junto a `cargo build -p execute-process`, construir cápsulas nativas indexadas (mínimo: workspace packages bajo tools/skills/daemons con `[[bin]]`, o lista explícita alineada a members).
2. Preferir perfil coherente con runtime (`release` si profiles[0]=release).
3. Fallo de build → abortar ignición (no arrancar con fósiles conocidos).

## Fase L6 — Remediación física (CA1/CA2/CA7)

Ops en worktree (documentar en `execution.md`):

1. Tabla 17 cápsulas: veredicto rebuild/delete.
2. `cargo build --release -p iota-immutable-publisher` (+ resto según tabla).
3. Repro PBI §1.1 sobre release → OK.
4. Drenar `.SddIA/dlt/reanchor-queue/` (drain nativo o cierre documentado); verificar vacía / sin `payload`.
5. Smoke pre-sellado batch bajo lab (`SDDIA_LAB_SIMULATE_IOTA=1` si aplica).

## Cierre documental (post-Argos, misma rama)

1. `implementation.md` + `execution.md` (Tekton).
2. Argos → `validacion.md` APTO, `pbi_archived: true` (CA10).
3. Mover PBI R1 a `docs/todos/done/`; anotar padre sin reabrir (CA11).
4. `delivery-close-cycle` con `source_process: bug-fix`.

## Delegación (blueprint operativo)

| Fase proceso | Agente / acción | Artefacto |
|------------|-----------------|-----------|
| Diseño | agent:dedalo | `spec.md`, `plan.md` (este) |
| Ejecución | agent:tekton | código + `implementation.md` + `execution.md` |
| Verificación | agent:argos | `validacion.md` |
| Cierre | action:execute-process → `delivery-close-cycle` | PR |

Git: solo `skill:git-manager` / `./sddia-run.sh --tool git-manager`. Prohibido bypass raw destructivo. KM `docs/todos/`: solo Cúmulo / evento Kaizen — Tekton no siembra TODOs.
