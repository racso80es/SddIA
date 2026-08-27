---
document_id: PBI-KAIZEN-TEKTON-EVOLUTION-GATE-NO-POLL
title: "[KAIZEN] Tekton — aduana local evolution y veto de polling CI"
format: markdown
version: "1.1.0"
created: "2026-08-27"
updated: "2026-08-27T16:45:00Z"
status: done
priority: alta
process: feature
type: kaizen
dispatch: false
uuid: 07dc027a-fdb5-487a-9fea-1a5dd67d38ca
suggested_branch: feat/kaizen-tekton-evolution-gate-no-poll
persist_ref_suggested: docs/features/kaizen-tekton-evolution-gate-no-poll
source_pr_url: https://github.com/racso80es/SddIA/pull/206
source_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
incident_ref: "TEKTON_CI_POLL_EVOL_HASH — cierre PPR #203: hash_integrity placeholder + 3 commits ciegos de recálculo + polling GitHub Actions (EVOL_HASH_MISMATCH en wasi-runtime-smoke)"
related:
  - SddIA/norms/external-ai-constraints.md
  - .cursor/rules/tekton-fire-and-forget.mdc
  - SddIA/evolution/evolution_contract.md
  - SddIA/tools/sddia-qa/src/gate_evolution.rs
  - SddIA/tools/sddia-qa/src/main.rs
  - SddIA/skills/sddia-evolution-register/src/lib.rs
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.sh
  - SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  - .github/workflows/sddia-index-qa.yml
---

# [KAIZEN] Tekton — aduana local evolution y veto de polling CI

## 1. Falla estructural

Al cerrar PPR #203 (PR #206), Tekton consumió **3 commits + 3 runs de Actions** para arreglar un único finding (`EVOL_HASH_MISMATCH` en `b7e4a91c-…`), usando CI como depurador. Commits testigo: `4aced07`, `5ea8639`, `8963ec4` (los tres tocan **solo** la línea `hash_integrity` del mismo registro).

Causa raíz verificada en código (no hipótesis):

1. **Placeholder aceptado por omisión.** El registro se commiteó con `hash_integrity: "sha256:pending"`. La cápsula (`validate_record`, `lib.rs:214-229`) solo rechaza el campo **vacío** o el mismatch de recompute; no hay validación de forma `^sha256:[0-9a-f]{64}$`, así que un placeholder viaja como valor legítimo hasta que el recompute lo tumba en CI.

2. **Placeholders fósiles invisibles.** `correlators` (`lib.rs:249-258`) filtra `in_diff == true`: los registros no tocados por el diff nunca se validan. De los 95 registros versionados, **4** llevan placeholder en `main`:
   - `67110f2f-2be8-4fd3-b0a7-8dc400fe803f.md` → `sha256:pending`
   - `c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c.md` → `sha256:pending`
   - `c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14.md` → `sha256:pending-merge`
   - `a1c9e7f3-2b4d-5e6f-8a9b-0c1d2e3f4a5b.md` → `sha256:pending-anchor-on-merge`
   Cualquier futuro toque a esos ficheros reproduce el incidente. El patrón `pending-merge` / `pending-anchor-on-merge` sugiere una práctica recurrente de anclaje diferido nunca consumada.

3. **Asimetría de captura local/CI en `build_registry`** (`gate_evolution.rs:126-164`) — el diagnóstico del PBI v1.0.0 estaba invertido:
   - `frontmatter` se lee **siempre del working tree** (`load_frontmatter_yaml(&p)`, sobre `PathBuf`, sin pasar por git).
   - `raw` (lo que se hashea) sale de `read_blob(repo, rel, staged)` con `staged = !range`: en modo pre-commit lee `git show :{path}` (índice), en modo `--range` cae al **working tree** (`read_blob` solo consulta git si `staged == true`).
   - Consecuencia: `gate-evolution --range` local **ignora el blob del rango** y juzga el WT; en CI el checkout hace WT == HEAD, luego CI juzga el commit. Verde local ≠ verde CI mientras WT ≠ HEAD. Y en modo pre-commit el veredicto mezcla `raw` del índice con `frontmatter` del WT.

4. **No existe vía canónica para re-anclar un hash.** La cápsula solo expone `verdict | alta | modificacion | baja` (`lib.rs:559-560`); `sddia-qa` expone `gate-evolution` y `evolution-register` (`main.rs:180-181`). Editado el cuerpo de un registro, **no hay comando** que reescriba su `hash_integrity` → recálculo ad hoc. Y la semántica real es no obvia: `canonical_hash` (`lib.rs:65-77`) hace `raw.lines().filter(!hash_integrity:).join("\n")` + `replace("\r\n","\n")`, es decir **descarta el newline final** — matiz que el contrato 1.1.1 (§ campos, línea 34: "payload canónico … UTF-8, LF") no explicita y que costó los commits `5ea8639` y `8963ec4`.

5. **Vigilancia remota.** Tras el primer log rojo: `sleep` + `gh pr checks` en bucle + `gh run rerun` del mismo `headSha`. DA-5 veta el polling **post-acuse del CLI**; no menciona GitHub Actions, y el patrón migró intacto a CI.

## 2. Objetivo medible

Que un `EVOL_HASH_MISMATCH` se resuelva **en local, en una invocación, con un commit y un push**, y que el verde local sea predictivo del verde CI.

| Métrica | Antes (PPR #203) | Después |
|---------|------------------|---------|
| Commits para un finding de hash | 3 | 1 |
| Runs de Actions consumidos | 3 | 1 |
| `sleep` / `gh run rerun` en el cierre | ≥1 | 0 |
| Placeholders `pending*` en `SddIA/evolution/` | 4 / 95 | 0 |
| Registros no-`in_diff` auditados | 0 | todos |

## 3. Alcance

### Dentro

**A. Fidelidad del gate (`gate_evolution.rs`)**
- `frontmatter` y `raw` deben proceder de **la misma fuente**: parsear el frontmatter del `raw` capturado, no del fichero en disco.
- `--range` debe leer el **blob de HEAD** (`git show HEAD:{path}`) para las rutas del rango, no el working tree; `read_blob` pasa a recibir la revisión (`":"`, `"HEAD"`) en vez de un booleano.
- Invariante: `gate-evolution --range` sobre un árbol limpio da el mismo veredicto que el job `evolution gate (delta)`.

**B. Re-anclaje canónico (SSOT de hash)**
- Subcomando `sddia-qa evolution-rehash --id <uuid> [--json] [--dry-run]` (nombre a fijar en spec) que delega en `canonical_hash` de la cápsula y reescribe `hash_integrity` in situ.
- Prohibido en la norma cualquier recálculo ad hoc (Python, `sha256sum`, `openssl`) que no replique strip-línea + LF + sin newline final.
- Documentar la semántica exacta de `canonical_hash` en `evolution_contract.md` (bump a 1.1.2, sin cambiar el algoritmo).

**C. Rechazo explícito de placeholder**
- `validate_record`: `hash_integrity` que no cumpla `^sha256:[0-9a-f]{64}$` → `EVOL_HASH_MISMATCH` con mensaje accionable (`placeholder/formato inválido`), antes del recompute.

**D. Saneamiento de fósiles + cobertura total**
- Re-anclar los cuatro registros `pending*` de `main` con el comando de (B).
- Auditar registros no-`in_diff`: modo `gate-evolution --all` y su cableado en el workflow, **habilitado bloqueante solo tras el saneamiento** (mismo PR, para no dejar `main` rojo ni la deuda perpetua).

**E. Norma: veto de vigilancia CI (DA-6)**
- `external-ai-constraints.md` → v1.6.0 con **DA-6 — Veto de Vigilancia Remota**: tras el primer log de check fallido, prohibido `sleep` de espera, `gh pr checks` en bucle y `gh run rerun` del **mismo `headSha`**. Un finding → un parche local verificado → un push.
- Gate local previo obligatorio: si el diff toca `directories.evolution`, `sddia-qa gate-evolution --json --range` con `exitCode: 0` **antes** del push.
- Prohibido empujar docs de cierre mientras haya un check rojo conocido (rearma CI sin aportar).
- Touchpoint `.cursor/rules/`: extender `tekton-fire-and-forget.mdc` (mismo eje de latencia) en vez de crear una rule nueva.

**F. Aduana física pre-push**
- `pre_push_gate.sh`: si el diff `origin/main...HEAD` toca `directories.evolution`, ejecutar `gate-evolution --range` y bloquear con `success == false`, respetando `in_delivery_close_cycle` y `SDDIA_SKIP_HOOKS` (operador humano).
- Sin listas de rutas ad hoc en el hook (contrato § 99): resolver por Cúmulo.

**G. Tests**
- Fixture `sha256:pending` → `EVOL_HASH_MISMATCH` por formato.
- Fixture con newline final / CRLF → hash estable (blinda la semántica documentada en B).
- Test de fidelidad: registro correcto en HEAD + WT corrupto (y viceversa) → `--range` juzga HEAD.

### Fuera

- Cambiar el algoritmo `canonical_hash` (invalidaría todos los registros conformes; solo se **documenta**).
- Rehabilitación `accept-pr` / umbrales Radamanto (PPR #203, ya cerrado).
- Dashboard de CI / Espejo de Consciencia (PBI distinto).
- Reescribir `pre_commit_gate.sh` más allá de heredar la fidelidad de (A).

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| K-FIDEL | `--range` juzga blobs de HEAD; `frontmatter` y `raw` comparten fuente | Test de fidelidad (G) verde; veredicto local == job `evolution gate (delta)` sobre árbol limpio |
| K-REHASH | Existe comando que re-ancla `hash_integrity` vía cápsula | `sddia-qa evolution-rehash --id <uuid>` → registro pasa `--range` sin edición manual |
| K-FORMAT | Placeholder o formato no `sha256:`+64hex → `EVOL_HASH_MISMATCH` con mensaje explícito | Fixture (G) verde |
| K-FOSIL | Cero `hash_integrity` no canónicos en `SddIA/evolution/` | `rg -n 'hash_integrity:' SddIA/evolution/*.md \| rg -v 'sha256:[0-9a-f]{64}"'` sin salida; `gate-evolution --all` exit 0 |
| K-LOCAL | Push con delta evolution exige gate local exit 0 | `pre_push_gate.sh` bloquea en rama de prueba con registro corrupto |
| K-NOPOLL | DA-6 publicada y difundida | `external-ai-constraints.md` v1.6.0 + `tekton-fire-and-forget.mdc` actualizada |
| K-DOC | `features-documentation-pattern` v1.2.0 | PBI en `docs/todos/done/` y `validacion.md` APTO en el **mismo** PR |

## 5. Riesgos y decisiones abiertas

| # | Asunto | Nota |
|---|--------|------|
| R1 | Placeholders con sufijo `-merge` / `-anchor-on-merge` | ¿Anclaje diferido deliberado por un proceso? El contrato 1.1.1 (línea 61) lo declara **no conforme** ("no rellenar"). Resolver en `clarify`: clarificación realizada : localizar quién los emite (grep en `process/`, `skills/`) antes de re-anclar. |
| R2 | `--all` bloqueante puede enrojecer `main` | Mitigación: saneamiento (D) en el mismo PR que la activación. |
| R3 | Cambiar la fuente de `raw` en modo pre-commit puede alterar veredictos hoy verdes | El algoritmo no cambia; solo se elimina la mezcla índice/WT. Cubrir con test antes de tocar el hook. |
| R4 | `pre_push_gate.sh` ya orquesta `delivery-close-cycle` | El gate evolution debe ir **antes** de `route-domain-event` y respetar el guard de re-entrada. |

## 6. Evidencia origen

| Campo | Valor |
|-------|-------|
| PR | https://github.com/racso80es/SddIA/pull/206 |
| Job | `wasi-runtime-smoke` → `evolution gate (delta)` (`.github/workflows/sddia-index-qa.yml:110-114`) |
| Finding | `b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e.md: hash mismatch` |
| Commits ciegos | `4aced07`, `5ea8639`, `8963ec4` |
| Fósiles detectados | 4 / 95: `67110f2f-…`, `c2e8f4a1-…` (`sha256:pending`); `c4a91e7b-…` (`pending-merge`); `a1c9e7f3-…` (`pending-anchor-on-merge`) |
| PBI origen | `PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY` |
