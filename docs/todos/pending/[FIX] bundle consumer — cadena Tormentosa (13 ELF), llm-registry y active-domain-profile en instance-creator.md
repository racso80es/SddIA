---
document_id: PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN
uuid: "806c9463-4c82-4216-a246-d5650a8553e9"
title: "[FIX] bundle consumer — cadena Tormentosa (13 ELF), llm-registry y active-domain-profile en instance-creator, LogRateLimit email"
format: markdown
version: "1.0.0"
created: "2026-09-25"
updated: "2026-09-25"
status: pending
priority: alta
type: fix
process: bug-fix
dispatch: false
suggested_branch: fix/bundle-consumer-tormentosa-chain
persist_ref_suggested: docs/fixes/bundle-consumer-tormentosa-chain
spawned_by: PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
spawned_by_ref: docs/todos/pending/[OPERATIVO] Paciente 0 SddIA_AP — redeploy ola 10 post-extracción del códice de ingeniería de software.md
derived_from:
  - PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
  - PBI-FIX-BUNDLE-CONSUMER-HERMETIC-LIBRARY
  - PBI-MULTI-LLM-ROUTER
  - PBI-SDDIA-DOMAIN-ABSTRACT-04
  - PBI-KAIZEN-DISK-THERMODYNAMICS-SYSLOG
blocks:
  - PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
friction_ids:
  - F-BUNDLE-TORMENTOSA-CHAIN
  - F-CREATOR-LLM-REGISTRY
  - F-CREATOR-DOMAIN-PROFILE
  - F-SYSTEMD-EMAIL-LOGRATE
laudos_source:
  Q1-TORMENTOSA-CONSUMER: a
  Q2-DOMAIN-PROFILE-CONSUMER: si
genome_mutations:
  - entity: instance-creator
    class: process
    from: "1.3.0"
    to: "1.4.0"
    via: entity-manager
scripts_mutations:
  - SddIA/scripts/build-release-bundle.sh
  - SddIA/scripts/qa/test-build-release-bundle-filtro-c.sh
  - SddIA/scripts/qa/test-instance-root-resolver.sh
  - SddIA/templates/systemd/sddia-email-watcher@.service.template
engine_mutations:
  - SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs
consumer_bins_from: 9
consumer_bins_to: 13
---

# [FIX] bundle consumer — cadena Tormentosa, llm-registry, active-domain-profile, LogRateLimit email

**Ciclo:** `bug-fix` · `fix/bundle-consumer-tormentosa-chain` · `docs/fixes/bundle-consumer-tormentosa-chain/` · PR único (`task-closure-documental`).
**Bloquea:** ola 10 de Paciente 0 (`PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION` `blocks_on`). El bundle de la ola 10 se compila **después** del merge de este FIX.

## 1. Síntoma (verificado en `main` @ `a08052f`, 2026-09-25)

| ID | Síntoma | Evidencia |
|----|---------|-----------|
| `F-BUNDLE-TORMENTOSA-CHAIN` | En instancia consumer, `POST /api/aiua/interact` → `aiua-stimulus-processing` v1.3.0 falla con `cápsula tool 'llm-router' no encontrada bajo SddIA/target` (o `thought-graph-access failed`). Puente perceptivo WUI castrado. | `build-release-bundle.sh` `CONSUMER_BINS` = 9; `llm-router`, `gemini-http-infer`, `antigravity-cli-executor`, `thought-graph-access` ausentes. `_scan_md_for_tools` solo lee el códice `--codex` y `event-domain-subscriptions.json`; `aiua-stimulus-processing.md` (Core) no se escanea. `_sddia_crate_root`, bucle `local_pkgs` y escáner ignoran `SddIA/skills/` (crate de `antigravity-cli-executor`). |
| `F-CREATOR-LLM-REGISTRY` | Tras `instance-creator`, no existe `.SddIA/llm-registry.json`; `llm-router` → `llm-registry-missing`. | `instance_creator.rs`: única materialización de starter-kit = `local.paths.json` (`materialize_local_paths`). Starter-kit ya trae `.SddIA/llm-registry.example.json` (PR #297). Cúmulo `instance.llm_registry` = `.SddIA/llm-registry.json`. |
| `F-CREATOR-DOMAIN-PROFILE` | Instancia consumer sin `.SddIA/active-domain-profile.json` → `ExecutionProfile::default()` = `git_required: true`, `codex_slug: null` → `has_software_authority` = **true** (legado). | `domain_profile.rs`, `domain_authority.rs` D4. Creator no escribe el perfil. Laudo Q2: viola Ceguera Espacial en consumer. |
| `F-SYSTEMD-EMAIL-LOGRATE` | `sddia-email-watcher@.service` renderizada sin techo de log. | `SddIA/templates/systemd/sddia-email-watcher@.service.template` sin `LogRateLimit*`; excluida explícitamente de PR #291 (`kaizen-disk-thermodynamics-syslog` spec § Fuera). |

## 2. Causa raíz

1. `CONSUMER_BINS` se definió antes de PR #270/#283/#297 (Latido Aiúa, combustión CLI, router). El escáner de cápsulas deriva de códice + suscripciones, no de los procesos que `kalma2-bridge` invoca por HTTP (`aiua-stimulus-processing`).
2. La topología de crates del bundle (`tools/`, `daemons/`, `engine/`, `interfaces/`) no contempla `skills/` con crate Rust.
3. `instance-creator` v1.3.0 nació antes de `instance.llm_registry` y del perfil de dominio por instancia (ABSTRACT-01/02/04).
4. Plantilla email fuera del alcance de #291.

## 3. Diseño

### 3.1 `SddIA/scripts/build-release-bundle.sh`

- `CONSUMER_BINS` += `llm-router`, `gemini-http-infer`, `antigravity-cli-executor`, `thought-graph-access` → **13**. Orden: tras `iota-immutable-publisher`. Comentario: «Cadena Tormentosa (laudo Q1 ola 10)».
- `_sddia_crate_root`: añadir `"$REPO_ROOT/SddIA/skills/${name}"`.
- Bucle `local_pkgs`: añadir condición `[[ -f "$REPO_ROOT/SddIA/skills/${name}/Cargo.toml" ]]`.
- `_scan_md_for_tools`: reconocer también `skill:` y `SddIA/skills/${hit}.md|/`. Escanear además `SddIA/process/aiua-stimulus-processing.md` (proceso Core invocado por `kalma2-bridge`; sin generalizar a todo `SddIA/process/`, Filtro C).
- Núcleo obligatorio post-copia (`for must in …`): añadir `llm-router` y `thought-graph-access` (fail-closed si faltan).
- Gate F-DEP-03 (`strings … execute-process.py`): sin cambio.
- `MANIFEST.json`: sin cambio de esquema; `binaries`/`source_digests` reflejan 13.
- `ONBOARDING.md` § Perfil consumidor: añadir línea «Tormentosa: `llm-router` + adaptadores empaquetados; registro en `.SddIA/llm-registry.json`; `agy` es dependencia de host».

### 3.2 `instance-creator` v1.4.0 (`entity-manager`) + `instance_creator.rs`

- Fase **Topologia**, tras `materialize_local_paths`:
  - `materialize_llm_registry(repo, &sddia)`: si `.SddIA/llm-registry.json` ausente → copiar `SddIA/scripts/starter-kit/.SddIA/llm-registry.example.json`. Nunca sobrescribir. Reportar `llm_registry_materialized: true|false`.
  - `materialize_domain_profile(instance_root, &sddia, profile, inputs)`: si `runtime_profile ∈ {consumer, consumidor}` y `.SddIA/active-domain-profile.json` ausente → escribir `{"codex_slug": <slug>, "git_required": false}`; `<slug>` = `inputs.codex_slug` ≻ `{instance_root}/MANIFEST.json`.`codex` ≻ omitir (sin `MANIFEST.json` ni input: no escribir; reportar `domain_profile_materialized: false`, `reason: codex-unknown`). Nunca sobrescribir. Perfil ≠ consumer: no escribir (forja auto-hospedada sigue legado).
- Frontmatter `instance-creator.md`: `version: 1.4.0`; fase Topologia `intent` ampliado («… materializar llm-registry.json y active-domain-profile.json (consumer) si ausentes»); input opcional documentado `codex_slug`.
- Smoke nativo (`run_smoke`): `checks.llm_registry_present`, `checks.domain_profile_present` (informativos; no tumban el smoke si `reason: codex-unknown`).

### 3.3 Plantilla email

`SddIA/templates/systemd/sddia-email-watcher@.service.template` `[Service]`: `LogRateLimitIntervalSec=30s`, `LogRateLimitBurst=500` (paridad con fábrica). Sin tocar `ExecStart=%f/SddIA/daemons/email-watcher.sh`.

### 3.4 Fuera

`mayeuta-llm` en `CONSUMER_BINS` (Chat `/api/chat`). `agy` dentro del bundle. Cambios en `llm-router`, adaptadores, `aiua-stimulus-processing`. `kalma2-bridge`. `start-sddia.sh`. Anclaje `SDDIA_CAPSULE_ANCHOR` por defecto. Poda de referencias residuales a `codex-software-engineering` en Cúmulo (ABSTRACT-04b).

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-BINS | `build-release-bundle.sh --profile consumer --codex codex-kalma2-assistant` (sin `--skip-build`) produce stage con los 13 ELF en `SddIA/target/release/`; `MANIFEST.json.binaries` ⊇ 13; `source_digests` con 13 claves; testigos `.sha256` escritos para los 4 nuevos (incl. skill). |
| CA-SKILL-CRATE | `SDDIA_BUNDLE_DIGEST_ONLY=antigravity-cli-executor ./SddIA/scripts/build-release-bundle.sh` devuelve `sha256:…` (crate de skill resuelto). |
| CA-FAILCLOSED | Stage sin `llm-router` → `[ERROR] binario obligatorio ausente en bundle: llm-router`, exit ≠ 0. |
| CA-SMOKE-FILTRO-C | `test-build-release-bundle-filtro-c.sh` ampliado: verifica los 4 ELF nuevos y `SddIA/{tools,skills}/*.md` correspondientes en stage; sigue verificando Filtro C. Verde. |
| CA-CREATOR-REGISTRY | `instance-creator` sobre instancia vacía → `.SddIA/llm-registry.json` igual a `llm-registry.example.json`; segunda ejecución no sobrescribe (test Rust con contenido distinto preexistente). |
| CA-CREATOR-PROFILE | `runtime_profile: consumer` + `MANIFEST.json` `codex: codex-kalma2-assistant` → `.SddIA/active-domain-profile.json` = `{"codex_slug":"codex-kalma2-assistant","git_required":false}`; sin `MANIFEST.json` ni `codex_slug` → no escribe + `reason: codex-unknown`; `runtime_profile: forge` → no escribe; preexistente → no sobrescribe. Tests Rust (4 casos). |
| CA-AUTHORITY | Con el perfil escrito, `resolve_execution_profile` → `has_software_authority == false` (test unitario sobre fixture de instancia). |
| CA-EMAIL-LOGRATE | `test-instance-root-resolver.sh` verifica `LogRateLimitIntervalSec=30s` y `LogRateLimitBurst=500` también en `sddia-email-watcher@.service.template`. Verde. |
| CA-RESOLVER | `test-daemon-binary-resolver.sh` verde (sin regresión hermética PR #292). |
| CA-BUILD | `cargo build -p execute-process --release` y `cargo test -p execute-process` OK. `sddia-qa` índices OK. |
| CA-GOV | `instance-creator.md` mutado vía `entity-manager` (DA-2); evolution con `uuid` de este PBI y de `instance-creator` (`dead5ca7-c0b9-42ef-aad6-171991fb524f`); `gate-evolution --range` `exitCode: 0`. |
| CA-DOC | Cascada `docs/fixes/bundle-consumer-tormentosa-chain/`; PBI en `done/` + `validacion.md` `pbi_archived: true` en el mismo PR. |

## 5. Riesgos

- **Tamaño del bundle / build**: `thought-graph-access` arrastra LanceDB; `_sddia_require_protoc` ya cubre la forja. Sin efecto en instancia (`MANIFEST.json` no recompila).
- **Testigo de skill**: `_sddia_source_digest` recorre `path` deps del `Cargo.toml`; verificar que `antigravity-cli-executor` no depende de crates fuera de `SddIA/` (CA-SKILL-CRATE).
- **Perfil en forja**: `materialize_domain_profile` solo actúa en `consumer`; el repo SddIA auto-hospedado no cambia (AC-CORE-SELF de ABSTRACT-04 intacto).
- **`agy` ausente en host**: fail-soft ya implementado en router (salto a `oracle-gemini`); no es alcance de este FIX.

## 6. Referencias

- `docs/todos/pending/[OPERATIVO] Paciente 0 SddIA_AP — redeploy ola 10 post-extracción del códice de ingeniería de software.md` §4, §8, §9
- `SddIA/scripts/build-release-bundle.sh` (`CONSUMER_BINS`, `_sddia_crate_root`, `_scan_md_for_tools`, núcleo obligatorio)
- `SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs` (`materialize_local_paths`, `run_smoke`, `run`)
- `SddIA/engine/execute-process/src/engine/{domain_profile,domain_authority}.rs`
- `SddIA/process/instance-creator.md` v1.3.0
- `SddIA/scripts/starter-kit/.SddIA/llm-registry.example.json`
- `SddIA/templates/systemd/{sddia-daemon,sddia-email-watcher}@.service.template`
- `docs/fixes/bundle-consumer-hermetic-daemon-library/` (PR #292) · `docs/features/multi-llm-router/` (PR #297) · `docs/features/kaizen-disk-thermodynamics-syslog/` (PR #291)
