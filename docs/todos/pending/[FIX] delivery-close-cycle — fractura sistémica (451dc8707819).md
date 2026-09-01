---
document_id: PBI-FIX-FRACTURE-451dc8707819
uuid: "bc16d090-2f7c-4845-8134-032989b094dc"
title: "[FIX] delivery-close-cycle — pr_title con metacaracter en argv de shell-executor"
format: markdown
version: "1.1.0"
created: "2026-09-01"
updated: "2026-09-01"
status: "abierto"
refinement_status: "refinado"
priority: alta
process: bug-fix
fracture_hash: 451dc8707819
fracture_process: delivery-close-cycle
friction_id: F-DCC-PR-TITLE-METACHAR
friction_ids:
  - F-DCC-PR-TITLE-METACHAR
  - F-DCC-PR-BODY-METACHAR-MISNOMER
  - F-MAYEUTA-PR-METACHAR-BLIND
incident_ref: "System_Fracture_Detected — 451dc8707819"
specimen_cycle: "feat/kaizen-ci-step-runtime-gt-1min (PR #246; 1.ª DCC; residual fuera de ese PR)"
suggested_branch: fix/dcc-pr-title-metachar-451dc8707819
persist_ref_suggested: docs/fixes/dcc-pr-title-metachar-451dc8707819
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/skills/shell-executor/src/main.rs
  - SddIA/norms/skill-io-shell-executor-frozen.md
  - docs/todos/done/[Kaizen] delivery-close — snapshot vacío y pr_body newlines en shell-executor.md
  - docs/fixes/kaizen-delivery-close-snapshot-pr-body/spec.md
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (c51acf014c0f).md
  - docs/todos/done/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
---

# [FIX] delivery-close-cycle — `pr_title` con metacaracter en argv

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` |
| Emisor | `execute-process` |
| Acción intentada | `Apertura en forja` |
| Fase física | `capsule_delivery_gh_pr` (`phase_capsules.rs`) |
| Cápsula | `skill:shell-executor` → `executable: gh` |
| `error_code` de fase | `PR_BODY_METACHAR` (misnomer; ver F2) |
| `friction_id` emitido | `F-DCC-APERTURA-EN-FORJA` (derivado del nombre de fase; `delivery_phase_failed` no propaga `error_code`) |
| Clasificación | `F-DCC-PR-TITLE-METACHAR` (causa) + `F-DCC-PR-BODY-METACHAR-MISNOMER` + `F-MAYEUTA-PR-METACHAR-BLIND` |

## Traza de error

```
[PR_BODY_METACHAR] arguments[3] contains forbidden shell metacharacters
```

`fracture_hash` = SHA-256 de esa traza, 12 hex: **`451dc8707819`** (verificado).

Índice 0-based de `shell-executor` (`main.rs` `arguments.iter().enumerate()`). En `capsule_delivery_gh_pr` el vector es:

| Índice | Token |
|--------|--------|
| `[0]` | `pr` |
| `[1]` | `create` |
| `[2]` | `--title` |
| **`[3]`** | **`pr_title`** |
| `[4]` | `--head` |
| `[5]` | `branch_name` |
| `[6]` | `--base` |
| `[7]` | `target_branch` |
| `[8]` | `--body-file` (si hay `pr_body`) |
| `[9]` | path absoluto de `pr-body.md` |

**No** es `arguments[9]` (el body). El Kaizen K2 (`--body-file`, PR #129) ya desvió el markdown multilínea al fichero; el preflight `is_shell_token_safe` aplica al **path**, no al contenido.

## Contexto de reproducción (especimen)

Primera inyección DCC del ciclo `feat/kaizen-ci-step-runtime-gt-1min` (2026-09-01), inputs `.tmp/dcc-kaizen-ci-step.json` (el fichero fue **sobrescrito** por reintentos; título reconstruido del transcript operador, no del JSON vigente).

```text
pr_title: feat: kaizen CI — steps >1 min (cache integrity + ingest itest)
```

`assert_safe_token` / `is_shell_token_safe` rechazan `\n\r;|><\`` , `&&`, `$(`, `&`. El título contiene **`>`**. El em-dash `—` y los paréntesis **no** están en esa lista.

Segunda DCC del mismo ciclo, mismo proceso, título saneado (`feat: kaizen CI step runtime gt 1 min`) → PR [#246](https://github.com/racso80es/SddIA/pull/246). El producto CI **no** es la deuda de este PBI.

`docs/features/kaizen-ci-step-runtime-gt-1min/.tmp/pr-body.md` existe: K2 escribió el body **antes** de `invoke_shell_executor`. El contenido actual del fichero pertenece a un DCC **posterior** (`dcc-restore-pbi.json`, rama `fix/restore-pbi-kaizen-ci-step-archive`); no usarlo como specimen del hash `451dc8707819`.

## Errata Mayeuta (síntesis auto-generada v1.0.0)

No es el falso positivo de recursión hook (`SDDIA_HOOK_DELIVERY_CLOSE`) documentado en `c51acf014c0f` / `c339de406e29` / `d0cfd5b66ff1` / `0c5268362b9a`. Aquí el clasificador **no inventó causa**: `analyze_fracture_kaizen` no tiene cubo para `forbidden shell metacharacters` / `PR_BODY_METACHAR` / `arguments[N]`, y la traza no dispara `failed`/`block`/`gh pr`/`recurs`. Cae al catch-all:

> Causa raíz no clasificada automáticamente para `delivery-close-cycle`; requiere laudo humano.

El veredicto `process_fix` es el **default** del catch-all, no un análisis. La propuesta «Auditar proceso / acción / emisor» es vacía. Hueco: `F-MAYEUTA-PR-METACHAR-BLIND`. **Prohibido** reimplementar la guarda hook.

## Cadena de fricciones

El síntoma Kintsugi es el último eslabón. El detonante es un título GitHub-legal que la aduana argv de `shell-executor` rechaza.

| # | Friction | Naturaleza | Emite evento |
|---|----------|-----------|:---:|
| F1 | **`pr_title` con `>` (u homólogo) va por argv** | Causa raíz de proceso | No (aduana cápsula) |
| F2 | **`error_code` `PR_BODY_METACHAR` para cualquier metacaracter** | Misnomer / telemetría opaca | No |
| F3 | **Mayeuta sin cubo** | Ceguera clasificadora | Contamina el PBI |
| F4 | **`System_Fracture_Detected` + `F-DCC-APERTURA-EN-FORJA`** | Síntoma / colapso | Sí (`451dc8707819`) |

### F1 — `pr_title` en argv sin preflight (`F-DCC-PR-TITLE-METACHAR`)

`capsule_delivery_gh_pr` construye `--title {title}` y lo pasa a `invoke_shell_executor`. Solo el path de `--body-file` pasa `is_shell_token_safe` **antes** de invocar. Título, head y base no.

`gh pr create` **no** expone `--title-file` (simétrico a `--body-file`). Relajar `assert_safe_token` para admitir `>` en argv quedó **fuera de alcance** del Kaizen #129 (spec: «incorrecto; preferir `--body-file`»). Ese invariante se mantiene: no abrir la allowlist.

GitHub acepta `>` en títulos. El operador puede inyectar el título del PBI (`steps >1 min`) o uno derivado. DCC no puede representar ese título vía la cápsula actual.

**Dentro:** preflight de **todos** los tokens argv (paridad `is_shell_token_safe`) **antes** de `invoke_shell_executor`; fallo tipado `PR_TITLE_METACHAR` (no reciclar `PR_BODY_METACHAR`); envelope con el token ofensivo (índice + campo `pr_title`/`branch_name`/`target_branch`) y un título saneado opcional **o** `blocked` accionable. `gh` no tiene `--title-file`: saneo determinista (p. ej. `>` → `gt` / texto ASCII) o rechazo explícito al operador. Test: `pr_title` con `>` → no `arguments[3]`; o aborta con código distinto de body.

### F2 — Misnomer `PR_BODY_METACHAR` (`F-DCC-PR-BODY-METACHAR-MISNOMER`)

`classify_delivery_error` mapea cualquier `forbidden shell metacharacters` (y el preflight de path) a `PR_BODY_METACHAR`. El test vigente congela `arguments[9]`. Este specimen es `arguments[3]`. `emit_dcc_phase_fractures` ignora `error_code` y sella `F-DCC-APERTURA-EN-FORJA` (misma etiqueta síntoma que `c51acf014c0f`, causa distinta: allí era `pr_url` irresoluble).

**Dentro:** códigos distintos (`PR_TITLE_METACHAR` / `PR_BODY_METACHAR` / genérico `SHELL_METACHAR` si el índice no es title ni body-file). Propagar `error_code` al payload de fractura (`friction_id` estable, no solo el slug de fase).

### F3 — Mayeuta ciego (`F-MAYEUTA-PR-METACHAR-BLIND`)

Cubo nuevo sobre `error_trace` (no sobre `process_name`): `PR_BODY_METACHAR` / `PR_TITLE_METACHAR` / `forbidden shell metacharacters` + `arguments[`. Veredicto `process_fix`. Texto: argv title/head/base, no recursión hook, no reabrir K2 body-file. Test de no-regresión: traza de este hash ≠ «recursión» ≠ «Auditar proceso».

### F4 — Escalado Kintsugi

Fallo **determinista de proceso** (título representable en GitHub, no en argv). No es gate F4b (evolution/EDA), ni DNS, ni PAT `workflow`. Emitir `System_Fracture_Detected` en este specimen fue **correcto** (deuda real). Tras F1, un `blocked` con `friction_id: F-DCC-PR-TITLE-METACHAR` puede dejar de materializar PBI (paridad F4b/F4c); no adelantar esa supresión sin el preflight.

`emit_dcc_phase_fractures` no rellena `persist_ref` / `branch_name` (opcionales del evento) → Cúmulo no los puso en `related[]`. Hueco de telemetría; no es causa raíz.

## Fuera de alcance

- Relajar la allowlist de `shell-executor` (`\n`, `>`, `&`, backticks en argv).
- Reabrir K2 `--body-file` / snapshot dirty (PR #129). Este hash **no** es regresión de newlines en `pr_body`.
- Producto Kaizen CI (PR #246, umbrales CA1/CA5).
- Snapshot que omite untracked `docs/todos/` (`dcc-restore-pbi.json`). Otra deuda.
- Reimplementar `SDDIA_HOOK_DELIVERY_CLOSE` / `SDDIA_SKIP_HOOKS=1` global.

## Mandato

Corregir F1–F3 en `bug-fix`. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado. **Prohibido** tratar el saneo manual del título operador como Done de este PBI.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis v1.0.0 sustituida — catch-all Mayeuta; no era diagnóstico)*

### Diagnóstico de causa raíz

- `capsule_delivery_gh_pr` pasa `pr_title` como `arguments[3]`; `>` (y homólogos de `assert_safe_token`) abortan Apertura en forja (`F-DCC-PR-TITLE-METACHAR`).
- El `error_code` `PR_BODY_METACHAR` y el `friction_id` `F-DCC-APERTURA-EN-FORJA` ocultan el campo ofensivo (`F-DCC-PR-BODY-METACHAR-MISNOMER`).
- Mayeuta no clasifica esta traza (`F-MAYEUTA-PR-METACHAR-BLIND`).

### Veredicto evolutivo

**Corrección de proceso oficial** (`process_fix`) — handler `delivery-gh-pr` + cubo Mayeuta. No es norma nueva ni prompt.

### Propuestas

- **Corrección de proceso oficial:** preflight argv completo; `PR_TITLE_METACHAR` distinto de body; envelope con índice/campo; cubo Mayeuta anclado a la traza; test `pr_title` con `>`.
- **No** relajar `assert_safe_token`. **No** reimplementar guarda hook.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

## Criterio de cierre

- [ ] F1: título con `>` no colapsa como `arguments[3]` (saneo o `blocked` tipado)
- [ ] F2: `error_code` / `friction_id` distinguen title vs body
- [ ] F3: Mayeuta clasifica esta traza; no catch-all ni recursión hook
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/` en el mismo PR
