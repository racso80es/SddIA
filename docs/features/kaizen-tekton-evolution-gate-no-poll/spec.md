---
feature_name: kaizen-tekton-evolution-gate-no-poll
created: "2026-08-27"
process: feature
base: main
scope: evolution-gate-fidelity-rehash-da6
version_spec: "1.0.0"
document_id: PBI-KAIZEN-TEKTON-EVOLUTION-GATE-NO-POLL
uuid: "07dc027a-fdb5-487a-9fea-1a5dd67d38ca"
persist_ref: docs/features/kaizen-tekton-evolution-gate-no-poll
branch_name: feat/kaizen-tekton-evolution-gate-no-poll
execution_id: "96471044-003a-457a-bf59-041e94053b12"
dedalo_verdict: ok
laudos:
  - L-ONE-SOURCE
  - L-REV-NOT-BOOL
  - L-DELTA-VALIDATE
  - L-REHASH-OP
  - L-REHASH-SURGICAL
  - L-FORMAT-FIRST
  - L-ALL-UNIVERSE
  - L-HOOK-DELEGATE
  - L-DA6-TOUCHPOINT
---

# Spec — kaizen-tekton-evolution-gate-no-poll

## 1. Decisiones Dedalo

| ID | Decisión | Rationale |
|----|----------|-----------|
| **L-ONE-SOURCE** | `frontmatter` se parsea del `raw` capturado (`parse_frontmatter_from_str`), nunca de `PathBuf` en disco. | K-FIDEL; mezcla WT/índice |
| **L-REV-NOT-BOOL** | `read_blob(repo, rel, rev)` con `rev ∈ {":", "HEAD"}`. Pre-commit: `":"`. `--range` y `--all`: `"HEAD"`. Prohibido `staged: bool`. | Mayeuta D2 |
| **L-DELTA-VALIDATE** | `verdict`: L-SELF exime **solo** correlación material. Registros `in_diff` siempre pasan `validate_canonical` (formato + hash). | Commits ciegos solo tocaban el registro |
| **L-REHASH-OP** | Cápsula `operation: rehash`. CLI `sddia-qa evolution-rehash --id <uuid> [--json] [--dry-run]` invoca la cápsula; persiste `result.detail` en `{evolution}/{id}.md`. Índice intocado. | SSOT `canonical_hash`; WASI = mismo dígito que el gate |
| **L-REHASH-SURGICAL** | `rehash` sustituye **solo** la línea `hash_integrity:`; no regenera el cuerpo vía `render_detail`. | Conservar `relacionado` extra, fechas, YAML no canónico |
| **L-FORMAT-FIRST** | En `validate_canonical` (nombre real; PBI decía `validate_record`): si `hash_integrity` no cumple `^sha256:[0-9a-f]{64}$` → `EVOL_HASH_MISMATCH` **antes** del recompute. Mensaje: `{fname}: placeholder/formato inválido; sddia-qa evolution-rehash --id {uuid}`. Hex minúsculas. | K-FORMAT |
| **L-ALL-UNIVERSE** | `--all` incompatible con `--range`. Audita todos los `{uuid}.md` bajo `directories.evolution` (Cúmulo). Blobs `HEAD`. `request.audit: "universe"`. Cápsula: `validate_canonical` de todos; **cero** correlación material. Excluir `Evolution_log.md` y `evolution_contract.md`. | K-FOSIL |
| **L-HOOK-DELEGATE** | Hook **no** lista rutas. `sddia-qa gate-evolution --json --range --if-touched`: si el rango no toca `directories.evolution` → `success: true`, `skipped: if-touched`, exit 0. En `pre_push_gate.sh` **antes** de `route-domain-event`. | Contrato §7; R4 |
| **L-DA6-TOUCHPOINT** | Norma `external-ai-constraints` v1.6.0 vía `entity-manager` (`norm`). Rule: solo extender `tekton-fire-and-forget.mdc`. | DA-2; Mayeuta D6 |

## 2. Circuito

```
pre-commit          → gate-evolution --json              (rev ":")
pre-push --if-touched → gate-evolution --json --range     (rev HEAD)
CI delta            → gate-evolution --json --range      (rev HEAD; checkout = HEAD)
CI universo         → gate-evolution --json --all        (tras saneamiento, mismo PR)

rotura de hash      → sddia-qa evolution-rehash --id UUID
                    → cápsula operation=rehash + canonical_hash
                    → reescritura in situ → gate local exit 0 → un commit → un push
```

## 3. Fidelidad (`gate_evolution.rs`)

### 3.1 Captura

| Modo | Diff | Blob registros `in_diff` | Blob resto (`--all`) | Índice |
|------|------|--------------------------|----------------------|--------|
| pre-commit (sin flags) | `git diff --cached` | `git show :{path}` | n/a | `:{log}` si existe, else WT |
| `--range` | `{origin/main\|main}...HEAD` | `git show HEAD:{path}` | n/a | `HEAD:{log}` |
| `--all` | no usado para correlators | todos UUID `.md` vía `HEAD:{path}` | idem | `HEAD:{log}` |

Si `git show HEAD:{path}` falla (nunca commiteado): no silenciar con WT. Finding `EVOL_CUMULO` o omitir registro no versionado. `--all` en CI asume archivos en HEAD.

### 3.2 Parseo

Extraer `parse_frontmatter_from_str(&str)` junto a `parse_frontmatter` en `execute-process` `parser.rs` (misma regla `split("---")`). `build_registry` usa el str del blob.

### 3.3 Invariante

Árbol limpio (WT = HEAD = índice): `--range` ≡ job `evolution gate (delta)`.

## 4. Cápsula `sddia-evolution-register`

Contrato skill: `inputs.operation` += `rehash`. `audit` opcional: `delta` (default) \| `universe`.

### 4.1 `validate_canonical`

Orden: campos canónicos → **formato hash** → recompute `canonical_hash(raw)` vs declarado.

`canonical_hash` **no se toca**:

1. Filtrar líneas cuyo trim-start empieza por `hash_integrity:`.
2. `replace("\r\n", "\n")` sobre el original **antes** o el join ya es LF; vigente: strip por `lines()` (LF/CRLF) + `join("\n")`.
3. `lines()` no emite línea vacía extra por newline final → **payload sin newline terminal**.
4. SHA-256 hex minúsculas, prefijo `sha256:`.

### 4.2 `verdict`

- `audit != universe`:
  1. Validar correlators (`in_diff`) siempre.
  2. Si `material` vacío tras (1) sin findings → `EVOL_OK` L-SELF (correlación).
  3. Si `material` no vacío → cobertura `relacionado` como hoy.
- `audit == universe`: solo (1) sobre **todos** los records inyectados; skip material.

### 4.3 `rehash`

Entrada: `id_cambio` UUID v4 + `registry` con el registro (CLI carga WT del fichero: el operador re-ancla el árbol de trabajo).

Salida: `detail` = raw con línea `hash_integrity: "sha256:{64hex}"` sustituida o insertada tras `descripcion_breve` si faltaba; `hash_integrity` en result; `index` ausente o eco del actual **sin patch**.

CLI `--dry-run`: no escribe. Persistencia: mismo patrón atómico que `persist` pero **solo** el detalle.

## 5. CLI `sddia-qa`

Usage: añadir `evolution-rehash --id UUID [--json] [--dry-run]` y `gate-evolution [--json] [--range|--all] [--if-touched]`.

`--range` y `--all` mutuamente excluyentes. `--if-touched` solo tiene sentido con `--range`.

`evolution-rehash` resuelve path `{directories.evolution}/{id}.md` vía Cúmulo. UUID inválido o fichero ausente → exit ≠ 0, `EVOL_CUMULO` / `EVOL_RECORD_INVALID`.

## 6. Contrato `evolution_contract.md` 1.1.2

Documento bajo `directories.evolution` (no genoma DA-2). Bump `version` / `contrato_version`. Sin cambio de algoritmo.

Añadir en §2 `hash_integrity`:

- Forma: `sha256:` + exactamente 64 hex `[0-9a-f]`. Placeholders (`pending`, `pending-merge`, `pending-anchor-on-merge`, …) **no conformes**.
- Payload canónico: frontmatter+cuerpo UTF-8 LF; se **eliminan** las líneas `hash_integrity:`; `str::lines().join("\n")` **descarta el newline final** del fichero; CRLF se normaliza al partir líneas.

§8: `EVOL_HASH_MISMATCH` cubre mismatch **y** formato. `gate-evolution --all` certifica universo oficial (no solo delta). Comando de re-anclaje: `sddia-qa evolution-rehash`.

Norma: prohibido recálculo ad hoc (Python, `sha256sum`, `openssl`) que no replique el payload canónico.

## 7. CI

`.github/workflows/sddia-index-qa.yml` job actual `evolution gate (delta)` se mantiene.

Nuevo step **en el mismo job** (tras tests cápsula + delta), o step hermano:

```text
evolution gate (universe): sddia-qa gate-evolution --json --all
```

Orden de merge: saneamiento de los 4 fósiles **en el mismo commit/PR** que activa `--all`. `main` no queda rojo.

## 8. Pre-push

`pre_push_gate.sh`, tras `skip_hooks` / `in_delivery_close_cycle`, **antes** del bucle `Local_QA_Requested`:

```text
resolve_sddia_qa
sddia-qa gate-evolution --json --range --if-touched
success == false → BLOCKED, exit 1
```

Cero `SddIA/evolution` literal en el hook. `SDDIA_SKIP_HOOKS` / guard DCC intactos.

## 9. DA-6

`SddIA/norms/external-ai-constraints.md` 1.5.0 → 1.6.0 (`entity-manager` update `norm`).

Texto DA-6:

- Tras el primer log de check GitHub fallido: prohibido `sleep` de espera, `gh pr checks` en bucle, `gh run rerun` del **mismo `headSha`**.
- Finding → parche local → si el diff toca `directories.evolution`, `gate-evolution --json --range` exit 0 → un push.
- Prohibido empujar docs de cierre con check rojo conocido.

`.cursor/rules/tekton-fire-and-forget.mdc`: añadir sección DA-6 (mismo eje latencia). No crear otra rule.

## 10. Saneamiento fósiles (mismo PR)

| UUID | Acción |
|------|--------|
| `67110f2f-2be8-4fd3-b0a7-8dc400fe803f` | `evolution-rehash --id` |
| `c2e8f4a1-7b3d-4e9c-a5f6-8d1e2f3a4b5c` | idem |
| `c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14` | idem |
| `a1c9e7f3-2b4d-5e6f-8a9b-0c1d2e3f4a5b` | idem |

No editar el cuerpo. Verificar `gate-evolution --all` exit 0.

## 11. Tests

| ID | Locus | Aserción |
|----|-------|----------|
| T-FMT | cápsula | `sha256:pending` → `EVOL_HASH_MISMATCH`, mensaje contiene `placeholder/formato` |
| T-NL | cápsula | raw con `\n` final vs sin él → mismo `canonical_hash`; CRLF ≡ LF |
| T-FIDEL | `sddia-qa` (repo git tmp) | HEAD canónico + WT corrupto → `--range` OK; HEAD corrupto + WT canónico → `--range` KO |
| T-DELTA | cápsula | solo path evolution `in_diff` + hash placeholder → KO (no L-SELF ciego) |
| T-ALL | cápsula | `audit: universe` valida registro `in_diff: false` |
| T-REHASH | CLI o cápsula | placeholder → detail con 64 hex; `--dry-run` no escribe |

## 12. Genoma vs docs

| Path | Vía |
|------|-----|
| `SddIA/skills/sddia-evolution-register.md` + crate | `entity-manager` update skill; crate en el mismo ciclo DA-4 |
| `SddIA/norms/external-ai-constraints.md` | `entity-manager` update norm |
| `SddIA/evolution/evolution_contract.md` | edición directa (fuera de tabla DA-2) |
| `SddIA/tools/sddia-qa/src/*` | ciclo DA-4 (sin `{name}.md` de tool) |
| `SddIA/engine/execute-process/src/core/parser.rs` | ciclo DA-4 |
| hooks, workflow, `.cursor/rules/` | ciclo DA-4 |
| `directories.evolution/{fósiles}` | `evolution-rehash` (no bisturí de hash) |

## 13. Criterios

| ID | Criterio |
|----|----------|
| K-FIDEL | `--range` juzga HEAD; fm y raw misma fuente; T-FIDEL verde |
| K-REHASH | `evolution-rehash --id` → registro pasa `--range`/`--all` sin edición manual del hash |
| K-FORMAT | T-FMT verde |
| K-FOSIL | 0 placeholders en `SddIA/evolution/*.md`; `--all` exit 0 |
| K-LOCAL | pre-push bloquea rama de prueba con registro evolution corrupto en el rango |
| K-NOPOLL | norma 1.6.0 + rule actualizada |
| K-DOC | PBI `done/` + `validacion.md` APTO en el mismo PR |

## 14. Fuera

Algoritmo `canonical_hash`. Rehab accept-pr. Dashboard CI. `hash_signature` de eventos. Reescritura amplia de `pre_commit_gate.sh`.
