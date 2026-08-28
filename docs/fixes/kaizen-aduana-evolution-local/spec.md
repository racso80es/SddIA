---
feature_name: kaizen-aduana-evolution-local
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-aduana-evolution-local
persist_ref: docs/fixes/kaizen-aduana-evolution-local
pbi_ref: docs/todos/pending/[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub.md
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL
uuid: fedb9597-a2a3-4c5b-825c-e3c7f3186b1b
scope: aduana-evolution-local
base: main
execution_id: "6b617fa0-ced7-49d5-9e1f-48356f3f26d3"
---

# Spec — Aduana evolution local

## Problema

PR #209 llegó a CI con 18 `EVOL_MATERIAL_UNREGISTERED`. Ninguna capa local lo detectó. El contrato (`evolution_contract.md`) exige correlato en `SddIA/evolution/` para mutación material bajo `SddIA/`; la única capa que lo aplicó fue el job `wasi-runtime-smoke` (`gate-evolution --json --range` tras `git fetch origin main`).

## Defectos (mapa D → capa)

| ID | Hecho verificado | Capa |
|----|------------------|------|
| D1 | `.git/hooks/` solo `*.sample`; `core.hooksPath` indefinido | instalación |
| D2 | `should_skip_pre_push_present` vacía `branches[]` → `exit 0` **antes** de `gate-evolution` | `pre_push_gate.sh` L44–52 |
| D3 | `--if-touched` salta si el rango **no** toca `evolution/` (`range_touches_evolution`) | `gate_evolution.rs:365` |
| D4 | `range_diff_spec` elige ref por existencia (`origin/main` luego `main`); sin frescura, sin fetch | `gate_evolution.rs:28–37` |
| D5 | `git()` = `Command::output()` sin techo de latencia | `gate_evolution.rs:16–26` |
| D6 | `capsule_delivery_impact_assessment`: `impact: none`, `sddia_paths: []`, nota stub | `phase_capsules.rs:472–496` |
| D7 | Handler y contrato DCC: impacto solo si `source_process == "feature"` | stub + `delivery-close-cycle.md` |
| D8 | DCC no declara fase `gate-evolution`; aduana EDA sí | proceso v1.1.1 |
| D9 | pre-commit: `gate-evolution --json` sobre **staged**; CI: `--range` de rama | `pre_commit_gate.sh` vs workflow |
| D10 | Contrato DCC documenta `SDDIA_SKIP_HOOKS` en git-manager; código usa `SDDIA_HOOK_DELIVERY_CLOSE` | deriva `.md` ↔ hook |

D3 es tautología: `EVOL_MATERIAL_UNREGISTERED` solo ocurre cuando `evolution/` **no** está en el diff. El flag anula el gate en su caso central.

D4 no es binario red/no-red. Escalera: `synced` / `stale` (habitual, `origin/main` existe y viejo) / `local`. Sub-bloqueo posible: `accept_pr.rs` fusiona en `main` local y `L-FAILSOFT-SYNC` tolera push KO → `main` por delante de `origin/main`.

## Solución

### L1 — Hooks vivos sin instalador (CA1, CA2)

`core.hooksPath` = `SddIA/scripts/qa/git-hooks` (directorio versionado). Aplicación: bootstrap del ecosistema (`start-sddia` / init de instancia), **idempotente**. Los wrappers `pre-commit` / `pre-push` / `post-merge` ya resuelven `REPO_ROOT` y honran `SDDIA_SKIP_HOOKS`. `install-hooks.sh` queda compatibilidad, no requisito.

`sddia-qa verify-hooks --json`: finding si `hooksPath` no resuelve a ese árbol; remedio literal en `message`. Mutación de crate `sddia-qa` (bajo `directories.tools`): en ejecución, vía ciclo autorizado; este diseño no toca genoma.

### L2 — Predicado `--if-touched` (CA3, CA11 a–c)

Sustituir `range_touches_evolution` por `range_touches_material`: prefijos de material desde `cumulo.paths.json` (`directories` de genoma bajo `SddIA/`, no solo el literal `SddIA/evolution/`).

| Diff | Comportamiento |
|------|----------------|
| Toca material genómico | Ejecutar veredicto (aunque `evolution/` no esté) |
| Solo `evolution/` o sin material | Skip explícito `skipped: if-touched`, `EVOL_OK` |

Prefijos: no cablear lista paralela a `pre_commit_gate.sh::GENOME_PREFIXES`; SSOT Cúmulo.

### L3 — Base de rango con degradación declarada (CA5, CA5b, CA13, CA14)

`range_diff_spec` deja de devolver `String`; estructura `base_resolution`: `{ mode, ref, spec, age_seconds, fetch_outcome }`.

- Fetch **opt-in** (flag del disparador, p. ej. `--sync-base`). `--range` solo **no** toca red.
- Presupuesto 2–3 s: `spawn` + `try_wait` + `Instant` + `child.kill()`. Prohibido `timeout(1)` (Windows). `GIT_TERMINAL_PROMPT=0` + SSH `BatchMode=yes`.
- Modos: `synced` (fetch OK o `--require-synced-base` y ref fresco); `stale` (`origin/main` existe, `age_seconds` sobre umbral o fetch no intentado); `local` (sin remoto).
- Degradación **no** altera `exitCode` ni `reason_codes`. Advertencia humana: **stderr**. stdout = una línea JSON (`capsule-json-io`).
- CI: `--require-synced-base`; `mode != synced` → exit ≠ 0. El workflow **sigue** haciendo `fetch` propio; **no** pasa `--sync-base` al gate.

### L4 — Orden de guardas pre-push (CA4, transitorio)

Hasta L6: mover `gate-evolution` **antes** de `if ${#branches[@]} -eq 0`. PR OPEN/MERGED no exime el gate. Tras L6 el gate **desaparece** del hook (anti-duplicación). Test CA11d: simular skip de presentación + gate ejecutado.

### L5 — Impacto real (CA7, CA8)

`capsule_delivery_impact_assessment`: diff contra la misma `base_resolution` que el gate (reutilizar `diff_paths`, no reimplementar). Filtrar material vía Cúmulo. `sddia_paths` no vacío si hay mutaciones. Evaluar `feature` \| `bug-fix` \| `refactorization`. Quitar filtro `source_process == "feature"` del handler **y** del contrato (L6).

### L6 — Fase en DCC + genoma (CA9, CA10, D10)

Forja: `./sddia-run.sh --process entity-manager` sobre `delivery-close-cycle` (domain root `codex-software-engineering/process/`). Nueva fase de verificación evolution (junto a Aduana EDA), invocando `sddia-qa gate-evolution --json --range` (sin `--if-touched` invertido; con predicado L2). Recalc `hash_signature`. Alinear nota anti-recursión a `SDDIA_HOOK_DELIVERY_CLOSE`. Tras la fase: retirar invocación duplicada en `pre_push_gate.sh`.

### L7 — Granularidad (CA6)

**Laudo:** retirar `gate-evolution` de `pre_commit_gate.sh`. Conservar VPI + audit EDA. Exigencia evolution = **rango de rama**, no commit. Motivo: staged-vs-rango es la fricción que dejó los hooks sin instalar; endurecer pre-commit sin homologar reproduce `SDDIA_SKIP_HOOKS=1`.

## Fuera de alcance

- 18 altas evolution del PR #209 (ciclo ajeno).
- Contrato de códigos `EVOL_*` (se consume).
- Aduana EDA / `orphan_count`.
- `PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA`.
- Smoke CA12 y CI `--require-synced-base` en workflow: ejecución, no este commit de diseño.

## Criterios (mapeo)

| CA | Capa |
|----|------|
| AEL-CA1, CA2 | L1 |
| AEL-CA3, CA11 a–c | L2 |
| AEL-CA5, CA5b, CA13, CA14 | L3 |
| AEL-CA4, CA11d | L4 |
| AEL-CA7, CA8 | L5 |
| AEL-CA9, CA10 | L6 |
| AEL-CA6 | L7 |
| AEL-CA12 | Smoke post-implementación |
