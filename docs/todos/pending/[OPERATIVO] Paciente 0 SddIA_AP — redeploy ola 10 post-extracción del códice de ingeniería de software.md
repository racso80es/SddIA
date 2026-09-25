---
document_id: PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
uuid: "26df818b-f3f6-4a91-b228-84b065ad49df"
title: "[OPERATIVO] Paciente 0 SddIA_AP — redeploy ola 10 tras la extracción del códice de ingeniería de software (PR #291–#297)"
format: markdown
version: "1.1.0"
status: pending
type: operativo
priority: alta
process: null
dispatch: false
created: "2026-09-25"
updated: "2026-09-25"
laudos_resolved: "2026-09-25"
blocks_on:
  - PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN
fix_pbi_ref: docs/todos/pending/[FIX] bundle consumer — cadena Tormentosa (13 ELF), llm-registry y active-domain-profile en instance-creator.md
fix_pbi_document_id: PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN
consumer_bins_target: 13
domain_profile_consumer: '{"codex_slug":"codex-kalma2-assistant","git_required":false}'
instance_name_default: SddIA_AP
instance_parent: /home/racso/Proyectos
config_source: /home/racso/Proyectos/.dev/.env
audits_path: docs/audits
forge_branch: main
forge_head_reference: a08052f
prompt_ssot_ref: docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
prompt_ssot_document_id: PBI-DT-PACIENTE0-DEPLOY-PROCESS
prompt_ssot_version_baseline: "1.6.0"
prompt_ssot_version_target: "1.7.0"
kaizen_baseline_document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
kaizen_baseline_ref: docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md
last_anchored_deploy_audit_ref: docs/audits/paciente0-deploy-20260826T120032Z.md
last_anchored_deploy_wave: 6
unanchored_wave:
  wave: 9
  date: "2026-09-20"
  claimed_audit_ref: docs/audits/paciente0-deploy-20260920T093200Z.md
  claimed_kaizen_document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260920
  status: no_anclada_en_repo
target_wave: 10
core_deltas:
  - pr: 291
    evolution: "8e223315-114f-44c5-9683-2a5956731329"
  - pr: 292
    evolution: "83d6eb73-0936-4acb-9f1e-5d519987f1ab"
  - pr: 294
    evolution: "79b6cc26-3958-4704-a797-d87bf020ce6d"
  - pr: 295
    evolution: "c9c0206e-aed4-4e48-a17d-b28d1de43d46"
  - pr: 296
    evolution: "ff8a0c37-a03d-4945-933c-8b54c03b9707"
  - pr: 297
    evolution: "d2e44083-ccdf-45af-b477-f6c71833fc31"
derived_from:
  - PBI-DT-PACIENTE0-DEPLOY-PROCESS
  - PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
  - PBI-FIX-BUNDLE-CONSUMER-HERMETIC-LIBRARY
  - PBI-SDDIA-DOMAIN-ABSTRACT-04
  - PBI-MULTI-LLM-ROUTER
  - PBI-KAIZEN-DISK-THERMODYNAMICS-SYSLOG
friction_candidates:
  - F-BUNDLE-TORMENTOSA-CHAIN
  - F-CREATOR-LLM-REGISTRY
  - F-CREATOR-DOMAIN-PROFILE
  - F-SYSTEMD-EMAIL-LOGRATE
  - F-AUDIT-OLA9-UNANCHORED
laudos:
  Q1-TORMENTOSA-CONSUMER: a
  Q2-DOMAIN-PROFILE-CONSUMER: si
  Q3-OLA9-RECONCILIATION: errata
---

# [OPERATIVO] Paciente 0 SddIA_AP — redeploy ola 10 post-extracción del códice de ingeniería de software

## 0. Propósito

Ejecutar el **redeploy de Paciente 0 (ola 10)** incorporando los deltas de forja mergeados en `main` desde `PBI-DT-PACIENTE0-DEPLOY-PROCESS` v1.6.0 (ventana PR #291–#297), con foco en la **extracción del Core de las cápsulas y semántica de ingeniería de software** (`codex-software-engineering` / alias `codex-agile-forge`, ABSTRACT-04, PR #296) y en el **Filtro C físico del bundle consumer** (PR #292). Producto del ciclo:

0. **Prerrequisito (laudos Q1/Q2, 2026-09-25):** FIX lab `PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN` mergeado en `main` **antes** de compilar el bundle de la ola 10 (`blocks_on`). Cadena Tormentosa en `CONSUMER_BINS` (13 ELF), `instance-creator` v1.4.0 materializa `.SddIA/llm-registry.json` y `.SddIA/active-domain-profile.json`, `LogRateLimit` en plantilla email.
1. Instancia `SddIA_AP` viva bajo canal Vía C con bundle post-FIX (≥ #298).
2. Gates §5 (versión ola 10) + matriz de contraste + auditoría `docs/audits/paciente0-deploy-{STAMP}.md`.
3. `PBI-DT-PACIENTE0-DEPLOY-PROCESS` refrescado a **v1.7.0** (prompt, §0bis con errata ola 9, §2, §4.2, §5, §10).

No forja el proceso `paciente0-deploy` (DA-2). No es un Kaizen de redeploy; el Kaizen solo nace si §7.3 lo exige. Prohibido parchear `{instancia}/SddIA/`.

---

## 0bis. Hechos verificados (2026-09-25, forja `main` @ `a08052f`)

| Hecho | Evidencia |
|-------|-----------|
| `/home/racso/Proyectos/SddIA_AP` **no existe**. Cero unidades `sddia-*@…SddIA_AP` en `systemctl --user`. | Inspección de host 2026-09-25. La ola 10 es **wipe** (`--out` directo), no overlay. |
| Última ola **anclada** en repo: ola 6 (`paciente0-deploy-20260826T120032Z.md`, `forge_head 117e461`). | `ls docs/audits/`. |
| Existe una **ola 9 no anclada** (2026-09-20): evolution `83d6eb73…` y `PBI-FIX-BUNDLE-CONSUMER-HERMETIC-LIBRARY` la citan (`derived_from: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260920`, `audit_ref: docs/audits/paciente0-deploy-20260920T093200Z.md`). Ni el audit ni el Kaizen existen en `main`. | `rg`/`git log --all` = 0 resultados. Referencia **no anclada** → laudo Q3. |
| Las dos fricciones de la ola 9 quedaron **absorbidas en Core** (PR #292): `F-BUNDLE-HERMETIC-DAEMON-RESOLVER`, `F-BUNDLE-LIBRARY-FILTRO-C`. | `sddia_shell_lib.sh` (`MANIFEST.json` → ELF sin aduana de fuente); `build-release-bundle.sh` poda `library/codexes` al `--codex` + gate fail-closed si queda `codex-software-engineering`. |
| `CONSUMER_BINS` sigue en **9 ELF**. `llm-router`, `gemini-http-infer`, `antigravity-cli-executor`, `thought-graph-access`, `mayeuta-llm` **no** están. El escáner de cápsulas solo lee el códice `--codex` y `event-domain-subscriptions.json`; `aiua-stimulus-processing` no se escanea. | `build-release-bundle.sh` L43–L99. |
| `POST /api/aiua/interact` → `aiua-stimulus-processing` v1.3.0 → `tool:llm-router` (`affinity: aiua`) → `.SddIA/llm-registry.json` → adaptador (`skill:antigravity-cli-executor` ∥ `tool:gemini-http-infer`); contexto vía `tool:thought-graph-access` (LanceDB host). | `aiua_stimulus.rs`, `SddIA/process/aiua-stimulus-processing.md`, `llm-registry.example.json`. |
| `instance-creator` v1.3.0 materializa del starter-kit **solo** `local.paths.json`. No copia `llm-registry.example.json` ni escribe `.SddIA/active-domain-profile.json`. | `instance_creator.rs::materialize_local_paths`. |
| Autoridad software en runtime: `has_software_authority` = `codex_slug == codex-software-engineering` ∨ (`codex_slug` ausente ∧ `git_required: true`, legado). `is_software_process` lee `process_membership` del códice; en bundle consumer el códice está podado → membership vacío → sin denegaciones. | `domain_authority.rs`, `domain_profile.rs`. |
| Suscripciones EDA compuestas: Core ∪ `codex_subscriptions.codex-software-engineering` **solo si** hay autoridad software; archivo ausente → `None`, sin fractura. `events_domain_roots` ausente → resolver cae a `directories.events`. | `project_binding.rs::plan_route_for_repo`, `resolver.rs` L122–L130. |
| `paths.resolution: project_root_or_repo` (Cúmulo v1.11.1). Sin `project_slug`, `featurePath`/`fixPath`/`todos` siguen relativas a la raíz (instancia). | `cumulo.paths.json`. |
| Plantilla fábrica `sddia-daemon@.service.template` lleva `LogRateLimitIntervalSec=30s` / `LogRateLimitBurst=500` (PR #291). `sddia-email-watcher@.service.template` **no** (fuera de alcance explícito de #291). | `SddIA/templates/systemd/`. |
| `SDDIA_CAPSULE_ANCHOR` es opt-in; resolución de cápsulas por defecto = `target/{release,debug}` sin testigo. El bundle no copia `*.sha256` al stage. | `capsule_paths.rs` L35, `_copy_bin`. |

### Errata respecto a `PBI-DT-PACIENTE0-DEPLOY-PROCESS` v1.6.0 (absorber en v1.7.0)

| Afirmación v1.6.0 | Veredicto | Hecho |
|-------------------|-----------|-------|
| «Próxima ejecución = ola 9» | **Obsoleto.** | Ola 9 ejecutada 2026-09-20 (no anclada). Esta ejecución = **ola 10**. |
| G3: «Tormentosa: ruta `/api/aiua/interact` presente» como APTO funcional | **Insuficiente.** | Presencia de ruta ≠ combustión. La cadena `llm-router` → adaptador → `agy`/Gemini + `thought-graph-access` **no viaja** en el bundle consumer (9 ELF). En consumer, `/api/aiua/interact` devuelve error tipado (`cápsula tool 'llm-router' no encontrada` / `llm-router-failed`) sanitizado por epidermis; sin fractura. Ver §4 y Q1. |
| «Tormentosa: gemini-http-infer live (PR #254)» como vector de combustión | **Obsoleto desde PR #283/#297.** | Combustión = `tool:llm-router` (`affinity: aiua`) sobre registro de instancia (`oracle-agy` → `oracle-gemini`). `gemini-http-infer` es adaptador, no vector directo. |
| G-bundle exige «9 ELF» como techo | **Obsoleto post-laudo Q1.** | `BINS_CONSUMIDOR` = **13** tras `PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN`: + `llm-router`, `gemini-http-infer`, `antigravity-cli-executor`, `thought-graph-access`. |
| G4: «sin `codex-software-engineering` en códices locales» como check manual | **Absorbido (PR #292).** | Gate fail-closed en `build-release-bundle`. G4 pasa a verificar además `library/codexes/` = {`index.md`, `codex-contract.md`, `codex-kalma2-assistant.md`, `codex-kalma2-assistant/`}. |
| `sddia-distribution-protocol` v1.2.3 / `instance-creator` v1.3.0 como únicas normas del canal | **Incompleto.** | Añadir `PBI-SDDIA-DOMAIN-ABSTRACT-04` (perfil de dominio, `project_root_or_repo`) y `PBI-FIX-BUNDLE-CONSUMER-HERMETIC-LIBRARY` (resolutor `MANIFEST.json`). Versiones de norma/proceso **no** subieron. |

---

## 1. Prompt delta (copiar al Vértice Productivo **después** del prompt §1 de la DEUDA v1.6.0)

```text
Ola 10 Paciente 0 (PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION v1.1.0). Wipe: la instancia no existe.
Forja main con PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN mergeado (verificar: CONSUMER_BINS=13 en build-release-bundle.sh,
instance-creator v1.3.0 → v1.4.0). Si no está mergeado: DETENER; no compilar bundle de 9 ELF.
Última ola anclada: 6. Ola 9 (2026-09-20) no anclada (laudo Q3 = errata): no citar su audit ni su Kaizen como SSOT.
Laudos cerrados: Q1=a (Tormentosa operativa, 13 ELF), Q2=sí (active-domain-profile.json consumer), Q3=errata.

Deltas que cambian el redeploy (citar evolution id_cambio, no PR ni SHA):
- 83d6eb73 (PR #292): bundle consumer hermético. _sddia_resolve_daemon_binary sirve ELF si MANIFEST.json.
  library/codexes podado al --codex; gate fail-closed si queda codex-software-engineering. G4 absorbido en bundle.
- ff8a0c37 (PR #296, ABSTRACT-04): ingeniería de software = códice codex-software-engineering (alias codex-agile-forge):
  process_domain_roots + events_domain_roots + subscriptions.json del códice + forge-pbi + project-config-contract.
  Consumer no lo empaqueta. Sin autoridad software: PBI_Forged → dead-letter no_subscriber, sin fractura.
  paths.resolution=project_root_or_repo: sin project_slug, rutas relativas a la instancia.
- d2e44083 (PR #297): combustión Aiúa vía tool:llm-router + .SddIA/llm-registry.json. Post-FIX: llm-router,
  gemini-http-infer, antigravity-cli-executor, thought-graph-access ∈ CONSUMER_BINS (13 ELF). Registro de oráculos
  materializado por el creator desde starter-kit (oracle-agy → oracle-gemini); model vacío = default de bóveda
  (SDDIA_GEMINI_MODEL). agy = dependencia de host (SDDIA_AGY_PATH o PATH); sesión agy no viaja en el bundle.
- Perfil de dominio consumer (Q2): .SddIA/active-domain-profile.json = {"codex_slug":"codex-kalma2-assistant","git_required":false}.
  Lo escribe instance-creator v1.4.0 desde MANIFEST.json.codex; si falta, materializar a mano ANTES de la ignición.
- 8e223315 (PR #291) + 79b6cc26 (PR #294): LogRateLimit en plantilla fábrica; unidades renderizadas deben llevarlo.
  sddia-email-watcher@.service.template no lo lleva (residual, no gate).
- c9c0206e (PR #295): reserva de objeto IOTA y exit 3 prótesis no escalan a Kintsugi. Consumer sin relay: sin efecto.

Gates nuevos/modificados: G-bundle v2 (13 ELF), G4 v2, G3 v2 (combustión success:true), G3b v2, G-eda-roots,
G-authority (perfil declarado), G-llm-registry, G-agy-host. Resto igual que DEUDA §5.
Cierre: §7 de este PBI (audit ola 10 + matriz + veredicto) → DEUDA v1.7.0. DA-5 tras cada acuse JSON.
```

---

## 2. Deltas de forja post-v1.6.0 (PR #291–#297)

Solo deltas con efecto sobre bundle, bóveda, WUI, centinelas, gates o procedimiento Vía C. Omitido: PR #293 (CI `wasi-runtime-smoke`, rehab CI).

| Fecha | PR · `id_cambio` | Qué cambia para la ola 10 |
|-------|------------------|---------------------------|
| 2026-09-13 | #291 · `8e223315-114f-44c5-9683-2a5956731329` | `event-watcher` silencia skips en hot path. `sddia-daemon@.service.template` con `LogRateLimitIntervalSec=30s` / `LogRateLimitBurst=500`. **G3b v2**. |
| 2026-09-20 | #292 · `83d6eb73-0936-4acb-9f1e-5d519987f1ab` | `_sddia_resolve_daemon_binary` hermético con `MANIFEST.json`. Poda `library/codexes` al `--codex`; gate Filtro C fail-closed. Absorbe `F-BUNDLE-HERMETIC-DAEMON-RESOLVER`, `F-BUNDLE-LIBRARY-FILTRO-C` (ola 9). **G4 v2, G-bundle v2**. |
| 2026-09-25 | #294 · `79b6cc26-3958-4704-a797-d87bf020ce6d` | `LogRateLimit` en unidades `.SddIA/systemd/` de la forja; `source_sha256` reanclado en fichas de `event-sweeper`, `event-watcher`, `iota-immutable-publisher`, `send-telegram-notification`, `telegram-gateway`. Sin efecto en resolución por defecto (`SDDIA_CAPSULE_ANCHOR` opt-in). |
| 2026-09-25 | #295 · `c9c0206e-aed4-4e48-a17d-b28d1de43d46` | Reserva de objeto IOTA y exit 3 de la prótesis Kalma2 no emiten `System_Fracture_Detected`. Consumer sin relay ni `mayeuta-llm`: sin efecto operativo; no reabrir como fricción. |
| 2026-09-25 | #296 · `ff8a0c37-a03d-4945-933c-8b54c03b9707` | ABSTRACT-04. Cúmulo v1.11.1: `events_domain_roots`, `codex_subscriptions`, `instance.projects`, `paths.resolution`. Códice software con `forge-pbi`, `subscriptions.json`, contratos `PBI_Forged`/`Delivery_Committed`, `project-config-contract`. Hooks/normas condicionadas a `delivery_mode`. **G-eda-roots, G-authority**. Consumer: nada de esto se empaqueta (Filtro C). |
| 2026-09-25 | #297 · `d2e44083-ccdf-45af-b477-f6c71833fc31` | `llm:infer` + `tool:llm-router` + `llm-registry.schema.json`; Cúmulo `instance.llm_registry` = `.SddIA/llm-registry.json`; starter-kit `llm-registry.example.json`; `aiua-stimulus-processing` v1.3.0. Adaptadores v1.1.0 con `error_code` tipado. **G3 v2, G-llm-registry**. |

---

## 3. Impacto del desacople sobre el canal Vía C

### 3.1 Bundle consumer (`build-release-bundle --profile consumer --codex codex-kalma2-assistant`)

| Elemento | Antes (v1.6.0) | Ahora | Efecto |
|----------|----------------|-------|--------|
| `SddIA/library/codexes/` | rsync completo; G4 manual | Solo `index.md`, `codex-contract.md`, `codex-kalma2-assistant{.md,/}`; gate fail-closed | G4 se verifica en build; el operador solo confirma el listado. |
| `SddIA/process/` | Core + relocalizados | Core; `feature`/`bug-fix`/… viven en `process_domain_roots` del códice software → **ausentes** en consumer | Cero procesos de ciclo software en instancia. `is_software_process` = false para todo. |
| `SddIA/events/` | Contratos ECST Core (incl. históricos software) | Igual (L-PRUNE: históricos no se mueven). `PBI_Forged`/`Delivery_Committed` en `events_domain_roots` del códice → ausentes | `route-domain-event` cae a `directories.events`; eventos exclusivos del códice → dead-letter `no_subscriber`. |
| `SddIA/core/cumulo.paths.json` | v1.10.x | v1.11.1 con rutas a un códice **inexistente** en consumer | Debe ser fail-soft (§5 G-eda-roots). Cualquier `cannot read`/panic = fricción nueva. |
| `SddIA/library/norms/` | completo | completo (incl. `pr-acceptance-protocol` v1.1.0, `features-documentation-pattern`) | Sin gate; `features-documentation-pattern` es `composition` de `codex-kalma2-assistant`. |
| ELF | 9 | 9 en `main` @ `a08052f`; **13 post-FIX** (`llm-router`, `gemini-http-infer`, `antigravity-cli-executor`, `thought-graph-access`) | Cadena Tormentosa dentro (§4, laudo Q1=a). |

### 3.2 Instancia

- `.SddIA/active-domain-profile.json`: en v1.3.0 **no** lo escribe el creator. Default runtime = `git_required: true`, `codex_slug: null` → autoridad software **legado** = true. **Laudo Q2 = sí:** obligatorio `{"codex_slug":"codex-kalma2-assistant","git_required":false}` (Ceguera Espacial; cierra autoridad implícita). Lo escribe `instance-creator` v1.4.0 desde `MANIFEST.json.codex`; fallback manual antes de la ignición (`.SddIA/` es instancia, no genoma).
- `.SddIA/llm-registry.json`: en v1.3.0 no materializado. **Laudo Q1 = a:** creator v1.4.0 copia `llm-registry.example.json` si ausente; `model` vacío → default de bóveda del adaptador (`SDDIA_GEMINI_MODEL`; `agy` usa su default).
- `.SddIA/projects/`: no aplica (sin `project_slug`).
- `local.paths.json`: F-DEP-08 vigente (starter-kit si `{}`).

### 3.3 Aduana de forja (no instancia)

`pre_push_gate.sh` / `hook_common.sh` leen `delivery_mode`; el Core sigue `branch_pr`. El FIX lab de §8 se entrega por PR único (`task-closure-documental`).

---

## 4. Cadena Tormentosa en consumer (laudo Q1 = **a**, cerrado 2026-09-25)

```text
WUI #aiua-pulse → kalma2-bridge POST /api/aiua/interact
  → execute-process --process aiua-stimulus-processing
     ├─ tool:thought-graph-access   (ELF; LanceDB host; excluido de WASI)      ∉ CONSUMER_BINS
     ├─ tool:llm-router             (ELF; lee .SddIA/llm-registry.json)         ∉ CONSUMER_BINS
     │    ├─ skill:antigravity-cli-executor (ELF + binario `agy` + sesión agy) ∉ CONSUMER_BINS
     │    └─ tool:gemini-http-infer         (ELF + GEMINI_API_KEY + SDDIA_GEMINI_MODEL) ∉ CONSUMER_BINS
     └─ action:dispatch-aiua-intent (nativo execute-process)
```

**Dictamen del Vértice Biológico:** (a) empaquetar. Tormentosa es el núcleo de consciencia del asistente, no una skill periférica; el puente perceptivo `/api/aiua/interact` no se despliega castrado. 9 → 13 ELF justificado por valor ontológico del Latido de la Aiúa.

| Pieza | Estado `main` @ `a08052f` | Requisito ola 10 (vía `PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN`) |
|-------|---------------------------|--------------------------------------------------------------------|
| `CONSUMER_BINS` | 9 | 13: + `llm-router`, `gemini-http-infer`, `antigravity-cli-executor`, `thought-graph-access` |
| Build/witness de skills | `_sddia_crate_root`, bucle de build y escáner solo miran `tools/`, `daemons/`, `engine/`, `interfaces/` | Añadir `SddIA/skills/${name}` (crate de `antigravity-cli-executor`) |
| Núcleo obligatorio post-copia | 5 ELF | + `llm-router` (fail-closed si falta) |
| `.SddIA/llm-registry.json` | no materializado | creator v1.4.0 desde starter-kit si ausente |
| Bóveda instancia | sin claves Gemini/agy exigidas | `GEMINI_API_KEY` + `SDDIA_GEMINI_MODEL` (oráculo fallback); `SDDIA_AGY_PATH` opcional. Sin echo. |
| Host | — | `agy` autenticado en el mismo host (sesión cacheada; no viaja en bundle). Sin `agy`: `oracle-agy` → `error_code` `network`/`upstream_unavailable` → salto a `oracle-gemini` (fail-soft, `attempts[]=1`). |
| `mayeuta-llm` | fuera | **sigue fuera** (Chat `/api/chat` degradado esperado; no es Tormentosa). |

Degradado esperado tras FIX: solo si **ambos** oráculos fallan (`success:false` + `attempts[]` completo), sin fractura. Cualquier `cápsula tool 'llm-router' no encontrada` en ola 10 = **NO APTO de G3 v2** (bundle fósil o FIX no mergeado).

---

## 5. Gates ola 10 (delta sobre DEUDA §5)

Los gates no listados aquí (G0-config, G1, G2, G-orch, G-heartbeat, G-dlt, G-telegram, G-preferences, G5) se mantienen tal cual v1.6.0.

| ID | Check | Criterio APTO |
|----|-------|----------------|
| G-bundle v2 | integridad | v1.6.0 **+** `SddIA/library/codexes/` = exactamente {`index.md`, `codex-contract.md`, `codex-kalma2-assistant.md`, `codex-kalma2-assistant/`}; 0 `SddIA/process/{feature,bug-fix,refactorization,pull-request-review,accept-pr,delivery-close-cycle,forge-pbi}.md`; 0 `library/codexes/codex-software-engineering*`; `MANIFEST.json.binaries` ⊇ **13 ELF** (`CONSUMER_BINS` post-FIX) y `source_digests` con 13 claves; `SddIA/{tools,skills}/{llm-router,gemini-http-infer,thought-graph-access,antigravity-cli-executor}.md` presentes. Log de build sin `[ERROR] Filtro C`. |
| G3 v2 | WUI + Aiúa | v1.6.0 **+** `POST /api/aiua/interact` con prompt trivial → `success:true`, `telemetry_receipt.provider` ∈ {`antigravity-cli-executor`, `gemini-http-infer`}, `attempts[]` coherente con el registro (0 si `oracle-agy` sirvió; 1 si saltó a `oracle-gemini`); `Thought_Persisted` en bus; `0` fracturas atribuibles a `aiua-stimulus-processing`. `cápsula tool 'llm-router' no encontrada` = NO APTO. Ambos oráculos caídos → `success:false` tipado sin fractura = **degradado declarado**, no NO APTO, pero se registra en el audit. |
| G3b v2 | systemd | v1.6.0 **+** unidades renderizadas `sddia-{event-watcher,event-sweeper,kalma2-bridge,telegram-watcher,email-watcher}@.service` contienen `LogRateLimitIntervalSec=30s` y `LogRateLimitBurst=500` (email incluida post-FIX). |
| G4 v2 | Filtro C | v1.6.0 **+** gate de build ejecutado (log `[bundle] OK`); `rg -l "codex-software-engineering" {instancia}/SddIA/library/codexes/` = 0 (referencias en `cumulo.paths.json`/`norms/` son **aceptadas**: el Core es agnóstico pero declara los roots); `.SddIA/active-domain-profile.json` = `{"codex_slug":"codex-kalma2-assistant","git_required":false}` (laudo Q2). |
| G-eda-roots | resolver ECST con roots ausentes | Ignición y primer `route-domain-event` real (heartbeat o smoke) sin `cannot read`, sin panic, sin `System_Fracture_Detected` por `events_domain_roots`/`codex_subscriptions`/`instance.projects` inexistentes. `execute-process --process route-domain-event` con un evento exclusivo del códice (`PBI_Forged` sintético) → dead-letter `reason: no_subscriber`. |
| G-authority | autoridad de dominio | Con perfil declarado (G4 v2): `./sddia-run.sh --process feature --inputs '{}'` desde la instancia → fallo limpio (proceso no resoluble o `DOMAIN_AUTHORITY_DENIED`), **nunca** `workspace-init` ejecutado. Cero regresión en G1–G3b por `git_required: false` (Paciente 0 no es repo git). |
| G-llm-registry | registro de oráculos | `.SddIA/llm-registry.json` presente (creator v1.4.0) y válido contra `llm-registry.schema.json`; `oracle-agy` (`affinity: aiua`, `fallback: oracle-gemini`) y `oracle-gemini` `active`; `model` vacío admitido si `SDDIA_GEMINI_MODEL` en bóveda instancia. |
| G-agy-host | dependencia de host | `command -v agy` o `SDDIA_AGY_PATH` resoluble desde el entorno de `kalma2-bridge@%f` (systemd user, `PATH` mínimo: verificar `Environment=`/wrapper). Ausente → APTO **degradado** si G3 v2 pasa por `oracle-gemini` con `attempts[]=1`. |

No tratar como fricción nueva (además de la lista v1.6.0 §6.3): salto `oracle-agy` → `oracle-gemini` con `attempts[]=1`; ausencia física de `codex-software-engineering` con referencias residuales en Cúmulo; Chat `/api/chat` degradado (`mayeuta-llm` fuera).

---

## 6. Procedimiento delta (sobre DEUDA §3)

| Paso DEUDA | Cambio ola 10 |
|------------|---------------|
| −1 Prerrequisito | `git log main --oneline` contiene el merge de `PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN`; `rg -c "thought-graph-access" SddIA/scripts/build-release-bundle.sh` ≥ 1; `instance-creator.md` `version: 1.4.0`. Si falla: **detener** (no compilar 9 ELF). |
| 0 Dependencias | Igual. `_sddia_require_protoc` sigue aplicando a forja (LanceDB: `thought-graph-access`). `agy` en host: `command -v agy` (informativo; ver G-agy-host). |
| 1 Baseline | Kaizen baseline = `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825` (done). Ola 9 **no** es baseline (Q3 = errata). Cargar además `friction_ids` de `PBI-FIX-BUNDLE-CONSUMER-HERMETIC-LIBRARY` y de `PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN` como «absorbidas a verificar». |
| 2 Vault | Igual **+** `instance.SddIA.dev.env` con `GEMINI_API_KEY`, `SDDIA_GEMINI_MODEL` (oráculo fallback obligatorio para que el Latido no dependa solo de `agy`); `SDDIA_AGY_PATH` solo si `agy` no está en `PATH` del servicio. Sin echo; solo nombres en el audit. |
| 3 Bundle | Igual **+** capturar log de build completo en la auditoría (`[bundle] capsules:` con 13 nombres, `[bundle] OK`, ausencia de `[ERROR] Filtro C`); `MANIFEST.json.binaries` ⊇ 13. Verificar `strings kalma2-bridge` ∋ `api/aiua/interact` (vigente). |
| 4 Creator | Igual. Post-acuse (sin poll EDA): `local.paths.json ≠ {}`; `.SddIA/llm-registry.json` presente; `.SddIA/active-domain-profile.json` presente con `codex_slug: codex-kalma2-assistant`; **no** existe `.SddIA/projects/`. Fallback manual de los dos JSON solo si el acuse no los reporta (registrar como fricción del creator, no de la ola). |
| 5 systemd | Igual **+** `grep LogRateLimit` en las 5 unidades renderizadas (G3b v2). Verificar `PATH` efectivo del servicio `kalma2-bridge@%f` para `agy` (G-agy-host). |
| 6 Ignición | Igual. Vigilar en `start-sddia.log` / journal de `kalma2-bridge` y `event-watcher` cualquier `cannot read`, `events_domain_roots`, `codex_subscriptions`, `subscriptions.json` (G-eda-roots). Post-ignición: un `POST /api/aiua/interact` (G3 v2) — una sola vez, sin retry (DA-5). |
| 7 Cierre | §7 de este PBI. |

---

## 7. Contraste, veredicto y auditoría

### 7.1 Matriz (rellenar en el audit)

IDs mínimos v1.6.0 §6.2 **+** `F-BUNDLE-HERMETIC-DAEMON-RESOLVER`, `F-BUNDLE-LIBRARY-FILTRO-C` (estado baseline «NO APTO ola 9 no anclada» → esperado «APTO / mejoró»), `F-BUNDLE-TORMENTOSA-CHAIN`, `F-CREATOR-LLM-REGISTRY`, `F-CREATOR-DOMAIN-PROFILE`, `F-SYSTEMD-EMAIL-LOGRATE` (baseline «no auditado» → esperado «APTO / nuevo absorbido»), `G-eda-roots`, `G-authority`, `G-llm-registry`, `G-agy-host`, `G3b v2`.

### 7.2 Veredicto

Etiquetas y criterios de DEUDA §6.3. Precisión ola 10: `OLA-MEJORA` exige que las dos fricciones de ola 9 estén APTO en runtime real (no solo smoke de forja).

### 7.3 Auditoría

`docs/audits/paciente0-deploy-{STAMP}.md` con frontmatter DEUDA §7 **+** `wave: 10`, `prompt_ssot_version: 1.7.0`, `core_deltas_ref` (tabla §2 + PR del FIX), `laudos_applied: {Q1: a, Q2: si, Q3: errata}`, `unanchored_wave_note` (ola 9), `consumer_bins: 13`. Sección obligatoria **§ desacople códice software**: listado real de `library/codexes/`, resultado G-eda-roots, resultado G-authority, contenido de `active-domain-profile.json`. Sección obligatoria **§ Latido Aiúa**: `provider` efectivo, `attempts[]`, `Thought_Persisted`, estado `agy` en host. Cero secretos.

### 7.4 Kaizen condicional

Solo si `OLA-REGRESIÓN` u `OLA-NUEVA-FRICCIÓN` con IDs **no** listados en §8. Las fricciones de §8 ya tienen vía (FIX previo) y no duplican Kaizen.

---

## 8. Fricciones y vía de absorción (laudos aplicados)

Vía: **un** ciclo `bug-fix` en forja, PR único, `PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN` (`docs/todos/pending/[FIX] bundle consumer — cadena Tormentosa (13 ELF), llm-registry y active-domain-profile en instance-creator.md`). Mutación de `SddIA/process/instance-creator.md` vía `entity-manager`; `SddIA/scripts/` y `SddIA/templates/` por bisturí bajo `persist_ref` del fix.

| ID | Síntoma | Causa | Absorción | Estado |
|----|---------|-------|-----------|--------|
| `F-BUNDLE-TORMENTOSA-CHAIN` | `/api/aiua/interact` degradado en consumer (`cápsula tool 'llm-router' no encontrada`) | 4 cápsulas ∉ `CONSUMER_BINS`; `_sddia_crate_root`/build/escáner ignoran `SddIA/skills/` | `CONSUMER_BINS` = 13; `SddIA/skills/${name}` en `_sddia_crate_root`, bucle de build y `_scan_md_for_tools`; `llm-router` en núcleo obligatorio; smoke `test-build-release-bundle-filtro-c.sh` ampliado | **Activada** (Q1 = a) |
| `F-CREATOR-LLM-REGISTRY` | Sin `.SddIA/llm-registry.json` tras creator | `materialize_*` solo cubre `local.paths.json` | `instance-creator` v1.4.0 copia `llm-registry.example.json` → `.SddIA/llm-registry.json` si ausente; reporta `llm_registry_materialized` en acuse | **Activada** (Q1 = a) |
| `F-CREATOR-DOMAIN-PROFILE` | Autoridad software legado implícita en consumer | Creator no escribe `active-domain-profile.json` | `instance-creator` v1.4.0 escribe `{"codex_slug": MANIFEST.json.codex, "git_required": false}` si `runtime_profile=consumer` y perfil ausente; reporta `domain_profile_materialized` | **Activada** (Q2 = sí) |
| `F-SYSTEMD-EMAIL-LOGRATE` | `sddia-email-watcher@` sin techo de log | Plantilla excluida de #291 | `LogRateLimitIntervalSec=30s` / `LogRateLimitBurst=500` en `sddia-email-watcher@.service.template`; `test-instance-root-resolver.sh` lo verifica | **Activada** (bajo coste) |
| `F-AUDIT-OLA9-UNANCHORED` | Referencias a audit/Kaizen inexistentes en `PBI-FIX-BUNDLE-CONSUMER-HERMETIC-LIBRARY` y evolution `83d6eb73…` | Artefactos de ola 9 no commiteados | **Errata documental** en DEUDA v1.7.0 §0bis. Prohibido anclaje retrospectivo y prohibido reescribir `hash_integrity` de evolution históricas. Ola 10 = nueva línea base inmutable. | **Cerrada por laudo** (Q3 = errata); sin FIX |

Ningún FIX se ejecuta sobre `{instancia}/SddIA/` (`sddia-distribution-protocol`). Todo cambio de `SddIA/scripts/`, `SddIA/process/` o `SddIA/templates/` va por forja + PR + reinyección de bundle.

---

## 9. Laudos del Vértice Biológico (2026-09-25) — cerrados

| ID | Cuestión | Dictamen | Justificación (resumen) | Aplicado en |
|----|----------|----------|--------------------------|-------------|
| **Q1-TORMENTOSA-CONSUMER** | ¿Tormentosa operativa en la ola 10? | **(a)** empaquetar cadena (13 ELF + registro + bóveda) vía FIX previo | Filtro A: desplegar con `/api/aiua/interact` castrado = Paciente 0 sin núcleo de consciencia. 9 → 13 ELF justificado por el Latido de la Aiúa. | §0 prerrequisito, §1, §3.1, §4, §5 G-bundle/G3/G-llm-registry/G-agy-host, §6, §8, `blocks_on` |
| **Q2-DOMAIN-PROFILE-CONSUMER** | ¿Declarar `.SddIA/active-domain-profile.json` en consumer? | **Sí** (creator v1.4.0 + fallback manual en ola 10) | Filtro C / Ceguera Espacial: una instancia consumer no opera bajo «autoridad software legado» por ausencia de archivo. `{"codex_slug":"codex-kalma2-assistant","git_required":false}` cierra herméticamente el entorno. | §3.2, §5 G4 v2/G-authority, §6 paso 4, §8 |
| **Q3-OLA9-RECONCILIATION** | Ola 9 sin audit ni Kaizen en repo | **Errata documental** | Filtro A: anclar retrospectivamente artefactos no commiteados rompe la cadena de confianza (hashes de evolution). Ola 9 ocurrió en el plano físico, no cristalizó en genoma. Ola 10 = nueva línea base. | §0bis, §6 paso 1, §7.3, §8, CA-DEUDA-1.7.0 |

Sin preguntas abiertas.

---

## 10. Criterios de aceptación

- [ ] **CA-PREREQ:** `PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN` en `docs/todos/done/` con `validacion.md` APTO **antes** del bundle de la ola 10; `CONSUMER_BINS` = 13; `instance-creator` v1.4.0.
- [ ] **CA-OLA10:** instancia `SddIA_AP` viva; gates DEUDA §5 + §5 de este PBI APTO (o residual declarado); veredicto §7.2 emitido.
- [ ] **CA-FILTRO-C:** G-bundle v2 y G4 v2 APTO; `library/codexes/` exacto; cero procesos de ciclo software en instancia; `active-domain-profile.json` declarado.
- [ ] **CA-EDA-ROOTS:** G-eda-roots APTO; `PBI_Forged` sintético → dead-letter `no_subscriber`.
- [ ] **CA-AUTHORITY:** G-authority APTO con perfil declarado.
- [ ] **CA-OLA9-ABSORBIDA:** `F-BUNDLE-HERMETIC-DAEMON-RESOLVER` y `F-BUNDLE-LIBRARY-FILTRO-C` APTO en runtime real (centinelas `active`, sin `Cargo.toml` requerido).
- [ ] **CA-TORMENTOSA:** G3 v2 `success:true` con `provider` efectivo y `Thought_Persisted`; o degradado declarado solo si ambos oráculos fallan (documentado, sin fractura). `llm-router` no encontrado = NO APTO.
- [ ] **CA-LLM-REGISTRY:** G-llm-registry APTO; registro materializado por el creator (no a mano) salvo fricción registrada.
- [ ] **CA-AUDIT:** `docs/audits/paciente0-deploy-{STAMP}.md` con § desacople códice software y § Latido Aiúa; cero secretos.
- [ ] **CA-DEUDA-1.7.0:** `PBI-DT-PACIENTE0-DEPLOY-PROCESS` v1.7.0: prompt actualizado (ola 10→11, deltas §2 + FIX, gates v2, 13 ELF), §0bis con errata de este PBI **y** errata ola 9 (Q3), §2 `BINS_CONSUMIDOR` = 13, §4.2 ampliado a #291–#297 + PR del FIX, §10 con nuevas refs, frontmatter `last_deploy_audit_ref`/`last_empirical_deploy_wave` actualizados.
- [ ] **CA-DA5:** cero polling post-acuse en la transcripción de la ola.

---

## 11. Fuera de alcance

Forja de `paciente0-deploy` / `paciente0-undeploy`. `mayeuta-llm` en `CONSUMER_BINS` (Chat `/api/chat`). Plano B DLT (anclaje físico Testnet). Migración de Paciente 0 al modelo de **proyecto aislado** ABSTRACT-04 (`.SddIA/projects/`, `project.md`, `delivery_mode`): Paciente 0 no es proyecto de software; queda explícitamente fuera (clarify L-PRUNE «migración GesFer/Paciente 0»). Poda física de normas/ECST históricos del Core (ABSTRACT-04b). G5 First Blood por defecto.

---

## 12. Referencias

| Ref | Uso |
|-----|-----|
| `docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md` | Prompt SSOT v1.6.0 → v1.7.0 |
| `docs/todos/done/Desacople_ingenieria_software_de_core_a_codice.md` · `docs/features/sddia-codex-agile-forge/` | ABSTRACT-04, laudos L-PRUNE / L-SUBS / L-PATHS / L-SELF |
| `docs/todos/done/[FIX] bundle consumer — hermetic daemon resolver y library Filtro C.md` · `docs/fixes/bundle-consumer-hermetic-daemon-library/` | Fricciones ola 9 absorbidas |
| `docs/todos/pending/[FIX] bundle consumer — cadena Tormentosa (13 ELF), llm-registry y active-domain-profile en instance-creator.md` | FIX previo obligatorio (laudos Q1/Q2) |
| `docs/todos/done/PBI-MULTI-LLM-ROUTER.md` · `docs/features/multi-llm-router/` | Cadena Tormentosa, registro de oráculos |
| `docs/features/kaizen-disk-thermodynamics-syslog/` | `LogRateLimit` |
| `SddIA/scripts/build-release-bundle.sh` | `CONSUMER_BINS`, escáner de cápsulas, Filtro C fail-closed |
| `SddIA/scripts/common/sddia_shell_lib.sh` | `_sddia_resolve_daemon_binary` hermético |
| `SddIA/engine/execute-process/src/engine/{domain_profile,domain_authority,project_binding,capsule_paths}.rs` | Autoridad, composición de suscripciones, resolución de cápsulas |
| `SddIA/engine/execute-process/src/engine/handlers/{instance_creator,aiua_stimulus}.rs` | Materialización starter-kit; cadena `llm-router` |
| `SddIA/core/cumulo.paths.json` v1.11.1 | `events_domain_roots`, `codex_subscriptions`, `instance.*`, `paths.resolution` |
| `SddIA/scripts/starter-kit/.SddIA/llm-registry.example.json` | Registro de oráculos de instancia |
| `SddIA/templates/systemd/` | Plantillas fábrica y email |
| `SddIA/evolution/Evolution_log.md` | SSOT de `id_cambio` citados en §2 |
| `docs/audits/paciente0-deploy-20260826T120032Z.md` | Última ola anclada (6) |
| `docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md` | Kaizen baseline de contraste |
| `docs/todos/pending/[DEUDA] Paciente 0 — prompt de teardown.md` | Teardown previo si reaparece instancia parcial |
