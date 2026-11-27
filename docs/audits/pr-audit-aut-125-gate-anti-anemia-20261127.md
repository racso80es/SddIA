---
audit_event_reference: "3899742f-04f5-480b-a1f6-866d93d1d5a3"
target_entity_id: "refactor/gate-anti-anemia-entidades-dominio"
pr_url: "https://github.com/racso80es/SddIA/pull/480"
tracker_ref: "AUT-125"
persist_ref: "docs/refactors/gate-anti-anemia-entidades-dominio"
contract_ref: "SddIA/events/domain/pull-request-audited.md"
event_type: "PullRequest_Audited"
event_id: "e999fc7a-4c2c-4b0d-aebe-7c4c2ab62677"
event_uri: "eda://domain/e999fc7a-4c2c-4b0d-aebe-7c4c2ab62677"
emitter_action: "emit-pr-audited-event"
date: "2026-11-27"
audit_mode: "static-diff-audit"
base_branch: "main"
base_commit_sha: "fd7ef31b51e795d1aaea2aba53b7cae693a019db"
head_commit_sha: "9d7b71725ff93ed40600f7623f48489b09b88e46"
verdict: "NO_APTO"
resolution: "REJECT"
violated_rules:
  - "actions-contract.md#1,3: pérdida de inputs/outputs, mutación de context y borrado de especificaciones/seguridad en 7 acciones (enrich-fracture-pbi-kaizen, sync-entity-index, purge-sandbox-cache, emit-tracker-sync-failed, emit-validation-started-event, emit-work-initiated-event, purge-repo-retention)"
  - "index-symmetry: corrupción de SddIA/actions/index.md (capabilities vacías y 237 líneas duplicadas), conteo domain=0 en SddIA/events/index.md y 25 UUIDs rotados sin sincronizar agents/process/skills/suites/events/domain index.md ni purgar SddIA/core/eda-coverage.json"
  - "semver-integrity: downgrade de versiones en codex-kalma2-assistant.md, codex-software-engineering.md y features-documentation-pattern.md"
  - "AUT-125/Q1,KA-DOC-1,KA-DOC-2: remediación boilerplate en 71 entidades, audit-inventory.json sin inventario previo y bypass de reglas contractuales en entity_no_anemia.rs"
  - "forges-integrity: pérdida de metadatos en patch_action_content_update, borrado de cola en sync_action_index_row, ausencia de rollback en run_action_forge/run_agent_forge y omisión silenciosa en run_suite_forge"
  - "pr-hygiene: árbol de trabajo sucio con validacion.md contradictorio respecto a HEAD y ausencia de evidencia CI verde"
---

# Auditoría Técnica de Pull Request — AUT-125 (`refactor/gate-anti-anemia-entidades-dominio`)

- **Veredicto:** `NO_APTO`
- **Resolución ECST (`PullRequest_Audited`):** `REJECT`
- **Referencia de auditoría (`audit_event_reference`):** `3899742f-04f5-480b-a1f6-866d93d1d5a3`
- **PR:** https://github.com/racso80es/SddIA/pull/480
- **Rango auditado:** `main` (`fd7ef31b51e795d1aaea2aba53b7cae693a019db`) … `HEAD` (`9d7b71725ff93ed40600f7623f48489b09b88e46`) + cambios sin commitear observados en el worktree
- **Tipo de pasada:** auditoría estática estricta sobre diff real y archivos afectados (sin ejecución de tests/build)

---

## Hallazgos críticos / bloqueantes

1. **Destrucción de especificaciones reales de dominio en `SddIA/actions/` (pérdida de datos, contratos I/O y controles de seguridad)**
   - `SddIA/actions/enrich-fracture-pbi-kaizen.md:1-27` y `SddIA/actions/sync-entity-index.md:1-26`: se eliminaron **80 y 83 líneas** respectivamente de especificación operativa vigente (pasos de orquestación, tablas de ruteo, límites, 7 `inputs` y 3–5 `outputs`), sustituyéndolos por `inputs: []`, `outputs: []` y texto plantilla vacío (`Entidad ... del Core SddIA` / `Ejecución conforme al contrato de familia...`). Además, `enrich-fracture-pbi-kaizen.md:6` sobrescribió `context: "knowledge-management"` por `"ecosystem-evolution"`.
   - `SddIA/actions/purge-sandbox-cache.md:1-29`: degradó `contract` de `v1.3.0` a `v1.2.0` (`L5`), cambió `context` de `"filesystem-ops"` a `"ecosystem-evolution"` (`L6`), vació `inputs`/`outputs` a `[]` (`L10-L11`) y **eliminó la regla de seguridad del gate regex** (`^/tmp/cursor-sandbox-cache(...)`) y el flujo en dos tiempos.
   - `SddIA/actions/emit-tracker-sync-failed.md:1-31`, `SddIA/actions/emit-validation-started-event.md:1-31`, `SddIA/actions/emit-work-initiated-event.md:1-31` y `SddIA/actions/purge-repo-retention.md:1-28`: perdieron sus esquemas `inputs`/`outputs` (`[]`) y, en `emit-tracker-sync-failed.md:6` y `purge-repo-retention.md:6`, se alteró su `context` de seguridad (`tracker-operations` / `filesystem-ops` → `ecosystem-evolution`), violando `SddIA/actions/actions-contract.md:22-42`.

2. **Corrupción grave de catálogos e índices canónicos (`actions/index.md`, `events/index.md`, `codexes/index.md`, `norms/index.md`)**
   - `SddIA/actions/index.md:19-285`:
     - En `L19-L47`, todas las filas del catálogo presentan la columna `Capabilities` corrompida con backticks vacíos (`` ``, `` ``) por un bug de regex en `docs/refactors/gate-anti-anemia-entidades-dominio/rebuild_actions_index.py:46` (`line.split("-", 1)[-1]` conserva el espacio inicial antes de las comillas y captura únicamente `" "`).
     - En `L49-L285`, quedaron **237 líneas de filas duplicadas y tablas multilínea rotas** (incluidos restos de prueba `Test. / ## 2. Orquestación / Paso.` en `L58-L65`) porque `rebuild_actions_index.py:89` busca el cierre con `text.find("\n## ", start + 1)` y se detiene en el primer `## 2. Orquestación` incrustado dentro de la tabla corrupta en lugar de `## Archivos en carpeta no catalogados como acción`.
   - `SddIA/events/index.md:22,26`: corrompió el conteo de eventos `domain` de `40` a **`0`** y el total ECST de `48` a **`8`**.
   - `SddIA/library/codexes/codex-kalma2-assistant.md:4`, `SddIA/library/codexes/codex-software-engineering.md:4` y `SddIA/library/norms/features-documentation-pattern.md:4` (y sus índices `SddIA/library/codexes/index.md:19-20` y `SddIA/library/norms/index.md:24`): **downgrades injustificados de versión SemVer** (`1.0.2 → 1.0.0`, `1.1.0 → 1.0.0`, `1.3.3 → 1.0.0`) al inyectar una sección artificial `## Propósito (complemento)`.

3. **Rotación de 25 UUIDs inmutables sin sincronizar sus `index.md` ni `eda-coverage.json`**
   - Se mutó el campo `uuid` en **25 archivos `.md`** (violando la inmutabilidad declarada en `SddIA/actions/actions-contract.md:17` y contratos homólogos) sin actualizar sus índices, rompiendo la simetría exigida por `AUT-125`:
     - `SddIA/agents/cerbero.md:2` vs `SddIA/agents/index.md:19`
     - `SddIA/process/action-creator.md:2` vs `SddIA/process/index.md:31`
     - `SddIA/skills/cryptography-manager.md:2` y `SddIA/skills/filesystem-manager.md:2` vs `SddIA/skills/index.md:20-21`
     - `SddIA/suites/fail-fast-lab.md:2` vs `SddIA/suites/index.md:8`
     - `SddIA/events/domain/domain-entity-deprecated.md:2` vs `SddIA/events/domain/index.md:56`
   - `SddIA/core/eda-coverage.json` acumula entradas huérfanas con los UUIDs antiguos (p. ej. `L963`, `L1029`, `L1419`, `L1581`, `L1713`, `L1953`) coexistiendo con los nuevos.

4. **Remediación masiva cosmética (*boilerplate gaming*) que contradice `AUT-125` (KA-DOC-1 / KA-DOC-2)**
   - `docs/refactors/gate-anti-anemia-entidades-dominio/remediate_bulk.py:57-75` inyectó la misma plantilla sintética (`Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.`) en **71 entidades `.md`** (p. ej. `SddIA/actions/emit-pr-audited-event.md:19-27`, `SddIA/daemons/email-watcher.md:29-35`, `SddIA/tools/llm-router.md:17-23`), duplicando además el primer párrafo existente o añadiendo `## Propósito (complemento)`.
   - `docs/refactors/gate-anti-anemia-entidades-dominio/audit-inventory.json:1` solo guarda el resultado post-remediación vacío (`"violations":[]`), incumpliendo `KA-DOC-1` (informe trazable de no conformidades detectadas antes de remediar).

5. **Defectos funcionales y de validación en `SddIA/engine/execute-process/src/engine/entity_no_anemia.rs`**
   - **Reglas contractuales anuladas (`L360-L385`):** `let has_second = ... || sections.len() >= 2;` (`L368`) anula `rule.second_kw` para cualquier archivo con ≥2 encabezados `##` (y `<2` ya falla por `SECTIONS_INSUFFICIENT`). De igual modo, `has_purpose` (`L361-L365`) omite `rule.purpose_kw` si la primera sección tiene ≥40 caracteres no-espacio.
   - **Heurística errónea en `truncated_tail` (`L202-L226`):** pese al nombre `vowel_cut`, no comprueba vocales; marca `BODY_TRUNCATED` ante cualquier última línea no vacía (≥12 bytes, sin `#`) que termine en alfanumérico sin `.`, `)`, `]` ni `` ` ``, provocando falsos positivos en listas o prosa válida sin punto final.
   - **Defectos de clasificación, prefijos y rango git en `scan_changed` (`L512-L548`):**
     - `L514` usa `git diff --name-only base_ref HEAD --` (dos puntos en vez de `base_ref...HEAD`), evaluando archivos cambiados en `origin/main` ajenos al PR y fallando si `origin/main` no existe localmente.
     - `L533` y `L543` comparan `p.starts_with(d)` sin delimitador de ruta (`/`), vulnerable a colisiones de prefijo.
     - `L541-L545` elige la familia con `.find(...)` según el orden de `genome_entity_dirs` (donde `codex` precede a `process_domain_roots` y `events_domain_roots`), sin aplicar el `rank` usado en `scan_all` (`L471-L494`). Procesos y eventos bajo `SddIA/library/codexes/...` se clasifican erróneamente como `codex` en PPR/`--if-touched`.
   - **Parseo frágil y errores silenciados (`L46-L54`, `L265`):** `split_frontmatter` no exige inicio con `---` y `L265` silencia errores de parseo YAML mediante `.unwrap_or(YamlValue::Null)`.

6. **Bugs de integridad y pérdida de datos en `forges/common.rs` y `forges/factory.rs`**
   - `SddIA/engine/execute-process/src/forges/common.rs:664-678`: `patch_action_content_update` sobrescribe `inputs`/`outputs` con `[]` y `contract` con `"1.2.0"` cuando no vienen en el payload de `update`, sin preservar el frontmatter existente.
   - `SddIA/engine/execute-process/src/forges/common.rs:844-866`: `sync_action_index_row` deja `skip_garbage = true` tras actualizar la última fila de la tabla, borrando todas las secciones posteriores de `SddIA/actions/index.md`.
   - `SddIA/engine/execute-process/src/forges/factory.rs:344-385`: en `run_action_forge` (`update`), `validate_entity_file` corre después de escribir a disco y devuelve `Err` sin rollback; ignora `UUID_NOT_V4` (`L354`) y pasa defaults de `inputs` (`ecosystem-evolution`, `[]`) a `sync_action_index_row` (`L366-L385`).
   - `SddIA/engine/execute-process/src/forges/factory.rs:1430-1439`: `run_suite_forge` retorna temprano tras `artifact_yaml_replacements`, descartando silenciosamente `markdown_body_replacements` si llegan ambos.
   - `SddIA/engine/execute-process/src/forges/factory.rs:1084-1108`: `run_agent_forge` con solo `artifact_yaml_replacements` muta el archivo en disco y luego retorna `Err` sin rollback ni refresco de `hash_signature`.

7. **Estado contradictorio e inconsistente entre `HEAD` y worktree**
   - En `HEAD`, `docs/refactors/gate-anti-anemia-entidades-dominio/validacion.md` figuraba con `global: NO_APTO` y CI `PENDIENTE-CI`, mientras que el worktree mantenía cambios sin commitear con `global: APTO` / `verdict: aprobado` contradictorios con sus propios checks en `NO_APTO` y con `.SddIA/observability/map-snapshot.json` modificado sin trazabilidad.

---

## Hallazgos menores / mejoras (Kaizen)

- `docs/refactors/gate-anti-anemia-entidades-dominio/rebuild_actions_index.py` y `docs/refactors/gate-anti-anemia-entidades-dominio/remediate_bulk.py`: scripts temporales versionados bajo `docs/`; `remediate_bulk.py:14` cablea `SddIA/target/debug/sddia-qa` y duplica `sanitize_action_index_description` (`common.rs:753-793`).
- `SddIA/engine/execute-process/src/engine/entity_no_anemia.rs:29-44`: compilación de `Regex` en cada invocación (usar `OnceLock`/`LazyLock`) e indentación inconsistente en `L42`; condición redundante en `L191`.
- `SddIA/engine/execute-process/src/engine/entity_no_anemia.rs:391,424-461`: `genome_entity_dirs` omite `capability_contracts` e `infrastructure_adapters` (`SddIA/core/cumulo.paths.json:69,74`); `walk_md_files:391` silencia errores de `fs::read_dir`.
- `SddIA/tools/sddia-qa/src/gate_entity_no_anemia.rs:14-37`: `--base` sin valor o seguido de otra bandera consume la bandera como ref git; errores de `scan_all`/`scan_changed` no respetan `--json`.
- `SddIA/engine/execute-process/src/engine/pull_request_review.rs:175,246-254`: `"origin/main"` cableado y `anemia_detail` omitido en el payload `Ok(...)`.
- `SddIA/engine/execute-process/src/engine/phase_capsules.rs:475-515`: invoca `sddia-qa` externo priorizando `target/debug` sobre `target/release` en lugar de usar `entity_no_anemia::scan_changed` in-process.
- `SddIA/evolution/a8f3c2e1-9d4b-4f7a-b6e5-1c2d3e4f5a6b.md:8-56`: faltan `SddIA/core/eda-coverage.json` y `docs/audits/AUT-125.md` en `relacionado`.
- `SddIA/engine/execute-process/src/engine/entity_no_anemia.rs:565,599,609-624`: directorios temporales basados solo en `std::process::id()` (colisión en tests paralelos) y exclusiones de `UUID_NOT_V4` / `STUB_MARKER` en las aserciones.

---

## Huecos de test y verificación recomendada

**Cobertura actual verificada en el diff:** únicamente 4 tests unitarios en `SddIA/engine/execute-process/src/engine/entity_no_anemia.rs:558-638`.

**Huecos críticos:**
- 0 tests para `scan_changed` (`base...HEAD`, `rank` en `library/codexes/*/process|events`, archivos borrados, prefijos de directorio, ref ausente).
- 0 tests para `gate_entity_no_anemia::run` (`--json`, `--if-touched`, `--base`, errores en JSON).
- 0 tests para `pull_request_review::tech_triage` con `gate-entity-no-anemia` ni para `phase_capsules::capsule_index_integrity_audit_gate`.
- 0 tests para `patch_action_content_update`, `sync_action_index_row` (última fila de tabla), `sanitize_action_index_description`, `patch_artifact_yaml_replacements` y rollback en `run_action_forge` / `run_agent_forge` / `run_suite_forge`.
- 0 tests de regresión para `split_frontmatter`, `truncated_tail` y cumplimiento efectivo de `purpose_kw` / `second_kw`.

**Comandos de verificación a ejecutar tras corregir los bloqueantes:**

```bash
cd SddIA && cargo test -p execute-process entity_no_anemia
cd SddIA && cargo test -p execute-process pull_request_review
cd SddIA && cargo test -p execute-process forges
cd SddIA && cargo test -p sddia-qa

cd SddIA && cargo build -p sddia-qa -p execute-process
SddIA/target/debug/sddia-qa verify-process-integrity
SddIA/target/debug/sddia-qa verify-tools-index
SddIA/target/debug/sddia-qa gate-entity-no-anemia --json
SddIA/target/debug/sddia-qa gate-entity-no-anemia --if-touched --base origin/main --json
SddIA/target/debug/sddia-qa gate-evolution --json --range --require-synced-base
SddIA/target/debug/sddia-qa gate-evolution --json --all
```

---

## Resumen para registro de auditoría (`PullRequest_Audited`)

Payload ECST conforme a `SddIA/events/domain/pull-request-audited.md` para emisión mediante `emit-pr-audited-event` (`emitter_agent: "argos"`):

```json
{
  "audit_event_reference": "3899742f-04f5-480b-a1f6-866d93d1d5a3",
  "target_entity_id": "refactor/gate-anti-anemia-entidades-dominio",
  "resolution": "REJECT",
  "tracker_ref": "AUT-125",
  "violated_rules": [
    "actions-contract.md#1,3: pérdida de inputs/outputs, mutación de context y borrado de especificaciones/seguridad en 7 acciones (enrich-fracture-pbi-kaizen, sync-entity-index, purge-sandbox-cache, emit-tracker-sync-failed, emit-validation-started-event, emit-work-initiated-event, purge-repo-retention)",
    "index-symmetry: corrupción de SddIA/actions/index.md (capabilities vacías y 237 líneas duplicadas), conteo domain=0 en SddIA/events/index.md y 25 UUIDs rotados sin sincronizar agents/process/skills/suites/events/domain index.md ni purgar SddIA/core/eda-coverage.json",
    "semver-integrity: downgrade de versiones en codex-kalma2-assistant.md, codex-software-engineering.md y features-documentation-pattern.md",
    "AUT-125/Q1,KA-DOC-1,KA-DOC-2: remediación boilerplate en 71 entidades, audit-inventory.json sin inventario previo y bypass de reglas contractuales en entity_no_anemia.rs",
    "forges-integrity: pérdida de metadatos en patch_action_content_update, borrado de cola en sync_action_index_row, ausencia de rollback en run_action_forge/run_agent_forge y omisión silenciosa en run_suite_forge",
    "pr-hygiene: árbol de trabajo sucio con validacion.md contradictorio respecto a HEAD y ausencia de evidencia CI verde"
  ]
}
```
