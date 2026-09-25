---
document_id: PBI-DT-PACIENTE0-DEPLOY-PROCESS
uuid: "7bf2bf4c-361e-4967-a58d-89dee74ea60d"
title: "[DEUDA] Paciente 0 — prompt de despliegue y proceso futuro"
format: markdown
version: "1.7.0"
status: deuda_tecnica
type: deuda
priority: alta
process: null
dispatch: false
process_candidate: paciente0-deploy
process_candidate_class: process
created: "2026-08-25"
updated: "2026-09-25"
instance_name_default: SddIA_AP
config_source: /home/racso/Proyectos/.dev/.env
instance_parent: /home/racso/Proyectos
audits_path: docs/audits
last_deploy_audit_ref: docs/audits/paciente0-deploy-20260925T114629Z.md
last_deploy_ola_verdict: OLA-MEJORA
last_empirical_deploy_wave: 10
post_ola_friction_open: null
fix_pbi_ref: docs/todos/done/[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher.md
fix_pbi_document_id: PBI-FIX-BUNDLE-TELEGRAM-GATEWAY
kaizen_pending_ref: null
kaizen_pending_document_id: null
last_kaizen_done_ref: docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md
last_kaizen_done_document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
dlt_telemetry_done_ref: docs/todos/done/PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP.md
dlt_telemetry_document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
dlt_core_pr: "https://github.com/racso80es/SddIA/pull/264"
thermal_fix_ref: docs/todos/done/[FIX] Resiliencia Térmica en Heartbeat Audit y Poda Ontológica en Mayeuta.md
thermal_fix_document_id: PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA
thermal_fix_pr: "https://github.com/racso80es/SddIA/pull/267"
antecesor_persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
antecesor_audit_ref: docs/audits/kaizen-paciente0-redeploy-20260825.md
tech_debt_ids:
  - DT-PACIENTE0-DEPLOY-PROCESS
  - DT-SYSTEMD-USER-ENABLE
  - DT-CONFIG-UX-ONBOARDING
  - DT-LANCE-PROTOC-DEPENDENCY
absorbed_debt_ids:
  - DT-ORCHESTRATOR-DEBUG-FIRST
  - DT-LOCAL-PATHS-EMPTY-STUB
  - DT-IGNITION-ENV-ISOLATION
  - DT-SMOKE-ECST-LOCAL-QA
  - DT-BUNDLE-TELEGRAM-GATEWAY
  - DT-DLT-TELEMETRY-PACIENTE0
blocks_on: []
derived_from:
  - PBI-LAB-PACIENTE0-SDDIA-AP
  - PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
  - PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
  - PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
  - PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA
  - PBI-ARQ-CONSCIENCIA-UNIVERSAL
  - PBI-PREF-STORE-LANCEDB-MIGRATION
  - PBI-EMAIL-TRIAGE-HEURISTIC
  - PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
  - PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF
  - PBI-FIX-FRACTURE-64f37c7f7b34
  - PBI-TELEGRAM-INLINE-KEYBOARD
  - PBI-EMAIL-NOISE-HEURISTIC-DIGEST
  - PBI-EMAIL-DIGEST-PREFERENCE-REPLY
  - PBI-NUCLEO-INTERCEPCION-HABITOS-KALMA2
---

# [DEUDA] Paciente 0 — prompt de despliegue y candidato a proceso

## 0. Propósito

Documento **operativo + semilla de proceso**. Sirve como **prompt** para que Tekton (o el operador) redepliegue Paciente 0 de forma reproducible, y como **contrato mínimo** de un proceso Core futuro (`paciente0-deploy`) que aún **no** está forjado (DA-2: no crear `{name}.md` bajo `directories.process` desde este PBI).

**Done de un ciclo de este prompt:** instancia viva + validaciones G* + **contraste valorativo** contra el último PBI Kaizen de despliegue Paciente 0 + auditoría bajo `paths.auditsPath` + PBI Kaizen **solo si** hay fricción nueva o regresión.

**Hecho 2026-09-25:** última ola empírica anclada = **10** (`docs/audits/paciente0-deploy-20260925T114629Z.md`, OLA-MEJORA). Ola 6 permanece como antecesor anclado. Ola 9 (2026-09-20) **no anclada** (errata Q3). La próxima ejecución de este prompt es la **ola 11**.

Censo post-ola 6 (no inventar recuentos): ~80 PRs mergeados en `main` desde `2026-08-26T12:00Z` hasta PR #290. Este PBI **no** cataloga rehab DCC/CI/`github-bridge`. Cataloga solo deltas que cambian bundle, bóveda, WUI, centinelas, gates o el procedimiento Vía C. Ventana v1.5.0 (PRs #268 a #290 = 23 números): #271 y #274 son documentales; no van a §4.2.

---

## 0bis. Errata y clarificaciones operativas (no reintroducir)

| Afirmación previa / Asunción | Veredicto | Evidencia y Hecho Técnico |
|------------------------------|-----------|---------------------------|
| Ola 12 = PR #261 / commit `c53c3b1` | **Alucinación de referencia.** | Resiliencia térmica = PR **#267**, `PBI-FIX-THERMAL-RESILIENCE-HEARTBEAT-MAYEUTA`, evolution `c142dc19-b3d9-4810-bad2-734c1910606e`. PR #261 = `dcc-shell-executor-wasm-fallback`. |
| Proofs DLT en `.events/proofs/dlt-telemetry/` | **Inexacto.** | Cúmulo `eda_instance.proofs` = `.SddIA/proofs`. Código: `dlt_telemetry_anchor.rs` `PROOF_NAMESPACE=dlt-telemetry` → `{eda_instance.proofs}/dlt-telemetry/{event_id}.json`. |
| Olas 9–12 como olas de despliegue Paciente 0 | **Incoherencia.** | Cero audit `paciente0-deploy-*` posterior a `20260826T120032Z`. Son deltas de forja. Próxima ola empírica = **Ola 9**. |
| `--seal-capsules` como flag de `build-release-bundle.sh` | **Inexacto.** | Flag de `execute-process` (`capsule_seal.rs`). Bundle no lo expone. En instancia con `MANIFEST.json`, `_ensure_orchestrator` **no** recompila ni sella. |
| Relay IOTA si `IOTA_PUBLISH_RELAY_URL` está en bóveda consumer | **Inexacto.** | `start-sddia.sh` `_iota_dlt_required`: perfil `consumer` → return 1. L-REQUIRED (PR #208): enable relay solo si perfil ≠ consumer **y** `SDDIA_LAB_SIMULATE_IOTA=0` **y** URL loopback. |
| `IOTA_ANCHOR_PACKAGE_ID` como input de la cápsula Rust | **Inexacto.** | Lo consume el relay Node. `network=testnet` lo inyecta `route_domain_core`; no existe `IOTA_NETWORK` en bóveda. |
| G-heartbeat = cero locks huérfanos | **Inexacto post-#267.** | Lock con `started_at` < `btime` → `host_reboot_stale_lock` **sin** fractura. PID muerto **después** del boot sigue siendo fractura L3. |
| `DT-DLT-TELEMETRY-PACIENTE0` abierta | **Parcial.** | Plano A Core cerrado (PR #264). Plano B (anclaje físico en instancia) no ejecutado: no hay ola de redeploy. |
| G0 exige claves IOTA en instancia | **Exceso.** | IOTA opcional. Sin bóveda → `skipped-config-missing` (APTO de G-dlt en consumer). |
| `blocks_on: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825` | **Obsoleto.** | Ese Kaizen está en `docs/todos/done/`. Es baseline de contraste, no bloqueo. |
| `mayeuta-llm` en bundle consumidor | **Inexacto/Falso.** | No está en `CONSUMER_BINS` (9 ELF). `kalma2-bridge.sh` emite preflight `[WARN]` informativo (PR #281). Ignición **no** falla. `POST /api/chat` **sí** colapsa sin ese ELF. Tormentosa (`/api/aiua/interact`) no lo usa. |
| Asumir que `kalma2-bridge` release preexistente siempre sirve `/api/aiua/interact` | **Riesgo de ELF fósil.** | `AUDIT-KALMA2-WUI-TORMENTOSA-CHAT-20260909` (HTTP 404 por binario release viejo; host forja `:8765`, no instancia `:8766`). El bundle exige recompilación release fresca de `kalma2-bridge` con la ruta activa. |
| Exigir base de datos LanceDB física obligatoria para preferencias | **Exceso.** | JSON es SSOT de escritura bajo `paths.userPreferencesStore`. `lancedb-preferences-repo` (PR #269) es réplica dual-write **si** existe tabla en `{vectorStore}/lancedb/`. |
| Store de preferencias = `.SddIA/user_preferences/` | **Alucinación de ruta (v1.5.0).** | Cúmulo `paths.userPreferencesStore` = `.SddIA/vector_store/user_preferences/`. `user-preference-core` `STORE_REL_DEFAULT` idéntico. |
| `SDDIA_AIUA_THINKING_LEVEL` | **Alucinación de env (v1.5.0).** | No existe. Starter-kit y `gemini-http-infer`: `SDDIA_GEMINI_THINKING_LEVEL` **comentada, vacía**. Vacío = no enviar `thinkingConfig` (default modelo = medium). No es default HIGH. |
| `SDDIA_LLM_REQUIRE_INFER=1` ⇒ fallo explícito / exit ≠ 0 | **Inexacto (v1.5.0).** | Marca `classification-degraded: true` si peaje 0 y sin elevación estructural. No aborta el proceso de triaje. Default 0. |
| Códice consumidor con solo 1 proceso de correo | **Obsoleto post-#289.** | `codex-kalma2-assistant` v1.0.2 `process_membership`: `email-triage-gateway`, `email-noise-digest`, `email-digest-preference-reply`. `user-preference-ingest` es proceso Core, **no** membership del códice. |
| `telegram-watcher` solo procesa texto plano | **Incompleto post-#288.** | Watcher invoca `telegram-gateway` con XOR: `text` → `TelegramMessage_Received`; `callback_data` → `TelegramCallback_Received`. |
| UUID `49ce2db7-152d-4000-8000-000000000001` como evolution de PR #272 | **Alucinación (v1.5.0).** | Token de fractura `49ce2db7152d`. Evolution real: `0b3ef02b-debe-4532-91da-c265ed59942b`. Sin delta de instancia consumer. |
| Citar `uuid` de PBI / `execution_id` como evolution | **Incoherencia (v1.5.0).** | Contrato de este prompt: `validacion.md` / `SddIA/evolution/{uuid}.md`. §4.2 v1.6.0 usa `id_cambio` de `Evolution_log`. |
| «23 merges de forja posteriores» a la ola 6 | **Inexacto (v1.5.0).** | 23 = cardinalidad de números PR #268–#290. Post-ola 6 hay ~80 PRs. |
| `blocks_on` / gate por `PBI-FIX-FRACTURE-0142599491fb` | **Exceso.** | Fractura forja `sse_chat_stream` / `mayeuta-llm` exit 3. Consumer no empaqueta `mayeuta-llm`. No bloquea ola 9. |
| «Próxima ejecución = ola 9» (v1.6.0) | **Obsoleto.** | Ola 9 física 2026-09-20 no cristalizó (Q3=errata). Ola 10 anclada 2026-09-25. Próxima = **ola 11**. |
| G3: ruta `/api/aiua/interact` = APTO funcional | **Insuficiente.** | Combustión = `success:true` + `thought_id` (G3 v2). 13 ELF + `SddIA/conscience/aiua_core.md`. |
| G-bundle techo 9 ELF | **Obsoleto.** | `BINS_CONSUMIDOR` = **13** post-#298. + `user-preference-store` extra no tumba. |
| `instance-creator` v1.3.0 / `sddia-distribution-protocol` v1.2.3 como únicas normas | **Incompleto.** | Creator **v1.4.0** (registry + perfil). ABSTRACT-04 + Filtro C #292. |

---

## 1. Prompt (copiar al Vértice Productivo)

```text
Despliega Paciente 0 siguiendo este PBI (PBI-DT-PACIENTE0-DEPLOY-PROCESS v1.7.0).

Constantes:
- Nombre de instancia por defecto: SddIA_AP
- Raíz instancia: /home/racso/Proyectos/SddIA_AP  (salvo override explícito)
- Configuración base: /home/racso/Proyectos/.dev/.env  (no inventar secretos; no loguearlos)
- Forja: repo SddIA actual (preferir rama main salvo indicación)
- Canal: Vía C — build-release-bundle (profile consumer, codex-kalma2-assistant v1.0.2, 13 ELF + conscience)
          + instance-creator v1.4.0 (skip_ignition) + ignición híbrida start-sddia.sh + systemd email-watcher@%f
- DA-3: ./sddia-run.sh (ELF nativo). DA-5: tras acuse JSON de execute-process, no poll de .events/
- Prohibido parchear {instancia}/start-sddia.sh ni SddIA/ inyectado (sddia-distribution-protocol)

Última ola empírica de redeploy: 10 (2026-09-25T11:46Z, OLA-MEJORA). Ola 9 no anclada (errata). Esta ejecución = ola 11.
No inventar PRs/SHAs. Citar validacion.md / evolution UUID (Evolution_log), no execution_id ni uuid de PBI.

Estado de mitigaciones en Core (forja, no redeploy):
- F-DEP-07: ABSORBIDA. _sddia_resolve_orchestrator: debug solo si mtime > release; empate/stale → release.
- F-DEP-08: ABSORBIDA. instance-creator v1.3.0 sustituye local.paths.json vacío/{} por starter-kit.
- F-DEP-09: ABSORBIDA. start-sddia.sh descarta SDDIA_EXECUTE_PROCESS_BIN fuera de la raíz de instancia.
- F-SMOKE-01: ABSORBIDA. Smoke nativo local_qa_emitted:false.
- F-BUNDLE-06: ABSORBIDA (PR #194). telegram-gateway ∈ CONSUMER_BINS.
- F-SYS-02 / F-DEP-10: ABSORBIDA. ExecStart=%f/SddIA/…; _sddia_resolve_instance_root.
- iota-immutable-publisher ∈ CONSUMER_BINS (9 ELF). --seal-capsules es CLI de execute-process en forja, no del bundle.
- DLT telemetría y resiliencia (PR #264, #273, #277): fan-out Domain_Entity_Telemetry_Captured → iota-immutable-publisher.
  Consumer: relay NO se enable. Sin bóveda IOTA o sin relay → skipped-config-missing (OK_STATUSES). No es fallo de ola.
  Proof durable si hay digest: .SddIA/proofs/dlt-telemetry/{event_id}.json (eda_instance.proofs). Nunca .events/proofs/.
  Serialización de publicaciones IOTA previene colisión gas-version; ENETUNREACH es fail-soft sin fractura.
- Resiliencia térmica (PR #267) + inmunológico (PR #228): btime inhibe fractura de lock pre-boot; skew suspend/crash.
  No exigir cero orphan_lock post-reboot.
- Preferencias: JSON SSOT en .SddIA/vector_store/user_preferences/ (Cúmulo userPreferencesStore, PR #204).
  Adaptador lancedb-preferences-repo (PR #269): dual-write solo si existe tabla en .SddIA/vector_store/lancedb/.
  Forja sin --skip-build: _sddia_require_protoc. Instancia con MANIFEST.json no recompila.
- Triaje de correo (PR #266, #268, #287, #289): códice v1.0.2 con email-triage-gateway, email-noise-digest,
  email-digest-preference-reply. Triaje-P cold-start consulta preferencias (P-EXEMPT-C / P-MUTE-SENDER).
  SDDIA_LLM_REQUIRE_INFER opt-in (default 0): peaje 0 → classification-degraded; no aborta el proceso.
- Telegram interactivo (PR #288): send-telegram-notification con inline_keyboard; watcher → telegram-gateway XOR
  text|callback_data. Ambos ELF ∈ CONSUMER_BINS.
- Kalma2 WUI (PR #276, #280, #281, #282, #285, #290): POST /api/aiua/interact activo. Compilar release fresco
  (evitar 404 ELF fósil). Preflight [WARN] mayeuta-llm ausente = APTO de ignición en consumer; Chat /api/chat
  degradado esperado (ELF fuera de CONSUMER_BINS). 503/red agy sanitizados en epidermis, sin retry.
- Tormentosa: gemini-http-infer live (PR #254). Env real: SDDIA_GEMINI_THINKING_LEVEL (vacío = default modelo).
  SDDIA_AGY_EFFORT / SDDIA_AGY_TIMEOUT_SECS opcionales. No existe SDDIA_AIUA_THINKING_LEVEL.

Mitigaciones operativas vigentes:
- Forja sin --skip-build: _sddia_require_protoc (lance-encoding). Instancia bundle: no aplica.
- Compilación de bundle: cargo build --release de los 9 ELF (evitar binarios fósiles; strings kalma2-bridge).
- systemctl --user: start-sddia (jurisdicción systemd) copia unidades y enable --now. Creator con skip_ignition no hace enable (F-SYS-01 residual).
- Linger: loginctl enable-linger $(id -un).
- Lab email-watcher@…SddIA: no dejar active si comparte IMAP (R-07).
- Lab telegram-watcher@…SddIA: coexistir solo si TELEGRAM_BOT_TOKEN lab ≠ instancia.
- Legado sddia-daemon@telegram-watcher: stop/disable si active.

Cierre del estímulo:
1) Validaciones §5.
2) Contrastar el ÚLTIMO PBI Kaizen de despliegue Paciente 0 (§6) y veredicto §6.3.
3) Auditoría docs/audits/ (§7). No secretos.
4) Kaizen pending solo si OLA-REGRESIÓN u OLA-NUEVA-FRICCIÓN.
No forjar el proceso paciente0-deploy en este estímulo.
```

---

## 2. Constantes y rutas

| Clave | Valor por defecto |
|-------|-------------------|
| `INSTANCE_NAME` | `SddIA_AP` |
| `INSTANCE_ROOT` | `/home/racso/Proyectos/${INSTANCE_NAME}` |
| `CONFIG_SOURCE` | `/home/racso/Proyectos/.dev/.env` |
| `VAULT_STAGING` | `/home/racso/Proyectos/${INSTANCE_NAME}.deploy-vault` |
| `PREPROD_VAULT` | `/home/racso/Proyectos/${INSTANCE_NAME}.preprod-vault` (constitución/códice si existen) |
| `FORGE_ROOT` | repo SddIA (cwd del operador) |
| `WUI_PORT` | `8766` (`SDDIA_CLIENT_PORT` en bóveda; default Core/starter-kit `8765`) |
| `PROFILE` | `consumer` |
| `CODEX` | `codex-kalma2-assistant` v1.0.2 (`email-triage-gateway`, `email-noise-digest`, `email-digest-preference-reply`) |
| `SENSORIAL` | `systemd` (`SDDIA_SENSORIAL_JURISDICTION`) |
| `UNIT` | `sddia-email-watcher@$(systemd-escape -p "$INSTANCE_ROOT").service` |
| `BINS_CONSUMIDOR` | 13 ELF: 9 previos + `llm-router`, `gemini-http-infer`, `antigravity-cli-executor`, `thought-graph-access`. Creator v1.4.0 materializa `llm-registry.json` y `active-domain-profile.json`. |
| `PROOFS_DLT` | `{eda_instance.proofs}/dlt-telemetry/` → `.SddIA/proofs/dlt-telemetry/` |
| `HEARTBEAT_AUDIT` | `{daemons_instance.state}/heartbeat-audit.json` → `.SddIA/daemons/state/heartbeat-audit.json` |
| `PREFERENCES_STORE` | `{instancia}/.SddIA/vector_store/user_preferences/` (`paths.userPreferencesStore`; JSON SSOT) |
| `VECTOR_STORE` | `{instancia}/.SddIA/vector_store/` (`paths.vectorStore`; réplica LanceDB opcional en `lancedb/`) |

**Bóveda instancia (Filtro C):** copiar claves de `CONFIG_SOURCE`; **omitir** `SDDIA_AGENT_RUNTIME_*` en `{instancia}/.SddIA/.dev/.env`. Forzar `SDDIA_RUNTIME_PROFILE=consumer` y `SDDIA_SENSORIAL_JURISDICTION=systemd` si no están en `.dev`. Completar huecos desde `PREPROD_VAULT` **solo** si aún faltan (Telegram, extras IMAP). IOTA: ver bloque siguiente; no es requisito de G0.

**Telemetría DLT — dos planos (no mezclar):**

| Plano | Estado 2026-09-12 | Contrato |
|-------|-------------------|----------|
| A — Core | Cerrado PR #264, afinado PR #273, #277 | Fan-out `Domain_Entity_Telemetry_Captured` → `iota-immutable-publisher` + `memory-evolution-ingest`. Skip `skipped-config-missing` ∈ `OK_STATUSES` si `config-missing: IOTA_WALLET_SECRET` o `iota-publish-unavailable`. Resiliencia a `ENETUNREACH` y colisiones `gas-version`. |
| B — Instancia lab | **No ejecutado** (no hay ola de redeploy post-#264) | Anclaje Testnet real. Consumer **no** arranca `sddia-iota-publish-relay@%f`. |

Variables **opcionales** (solo si se persigue Plano B, simulación, o afinado de IA / sensorial). No echo.

- `IOTA_WALLET_SECRET` o `{instancia}/.SddIA/.dev/wallet.key`
- `IOTA_PUBLISH_RELAY_URL` (p. ej. `http://127.0.0.1:8787/v1/publish`) — no implica enable del relay en consumer
- `IOTA_ANCHOR_PACKAGE_ID` — relay Node, no la cápsula Rust
- `SDDIA_LAB_SIMULATE_IOTA=1` → proofs `mode=lab-simulated` **sin** relay (no usa el skip config-missing)
- `SDDIA_LAB_MOCK_IOTA_URL` no vacío **precede** al relay y anula anclaje físico
- `SDDIA_LLM_REQUIRE_INFER=1` → opt-in: peaje LLM 0 sin elevación → `classification-degraded` (default 0: fallback determinista)
- `SDDIA_GEMINI_THINKING_LEVEL` (`high`\|`medium`\|`low`) — starter-kit **comentado/vacío**; vacío = default del modelo
- `SDDIA_AGY_EFFORT` (`low`\|`medium`\|`high`) / `SDDIA_AGY_TIMEOUT_SECS` — vector agy Tormentosa; starter-kit comentado

Default Paciente 0 (consumer, sin forzar IOTA): G-dlt APTO con `skipped-config-missing`, cero proof, cero DLQ de este evento.

**Layout vault staging (input de `instance-creator`):**

```text
{VAULT_STAGING}/
  root.dev.env              ← copia CONFIG_SOURCE
  instance.SddIA.dev.env    ← consumidor (sin AGENT_RUNTIME; IOTA opcional)
  wallet.key                ← opcional
  constitution/             ← desde PREPROD_VAULT si existe
  codexes/                  ← desde PREPROD_VAULT si existe (codex-kalma2-assistant v1.0.2)
```

---

## 3. Procedimiento de despliegue (pasos)

Ejecutar en orden. Canal canónico: `SddIA/norms/sddia-distribution-protocol.md` v1.2.3 + `SddIA/process/instance-creator.md` v1.4.0.

### 0 — Verificación de dependencias y orquestador (forja)

```bash
# Solo si se va a compilar en forja (sin --skip-build). Instancia con MANIFEST.json no entra aquí.
source "${FORGE_ROOT}/SddIA/scripts/common/sddia_shell_lib.sh"
_sddia_require_protoc || {
  echo "[AVISO] protoc ausente. Instalar: sudo apt-get install -y protobuf-compiler o binario en ~/.local/bin"
}

export SDDIA_EXECUTE_PROCESS_BIN="${FORGE_ROOT}/SddIA/target/release/execute-process"
```

### 1 — Baseline Kaizen (antes de mutar instancia)

Localizar el **último** PBI Kaizen de despliegue Paciente 0:

1. `docs/todos/pending/` con `document_id` `PBI-KAIZEN-PACIENTE0-REDEPLOY-*` (más reciente gana).
2. Si no hay pending: `docs/todos/done/` mismo prefijo.
   - Snapshot 2026-09-12: done `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825`.
   - Complementario Core (no es Kaizen de redeploy): `PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP` (done, Plano A).
3. Cargar `friction_ids`, gates, `bundle_manifest`, `instance_creator_*_correlation_id`.

Línea base del contraste §6. Cicatrices de ola empírica:

- `docs/audits/paciente0-deploy-20260826T120032Z.md` (Ola 6 — último redeploy wipe)
- `docs/audits/paciente0-centinelas-email-sordo-20260826.md` (Ola 7: diagnóstico runtime, no wipe)
- `docs/audits/kaizen-aislamiento-multi-instancia-20260826.md` (Ola 8: Core `%f`)

### 2 — Vault staging

Materializar `VAULT_STAGING` desde `CONFIG_SOURCE` (§2). chmod 600 en `*.env`. Cero secretos en git / logs / PBI.

### 3 — Bundle hermético

```bash
cd "$FORGE_ROOT"
./SddIA/scripts/build-release-bundle.sh \
  --out "$INSTANCE_ROOT" \
  --codex codex-kalma2-assistant \
  --profile consumer
# --skip-build solo si L-BUNDLE-STALE OK (testigos SHA-256). Rechazo = rebuild sin --skip-build (F-DEP-03, esperado).
# Verificación de frescura kalma2-bridge (evitar 404 por ELF fósil, AUDIT-KALMA2-WUI-TORMENTOSA-CHAT-20260909):
strings "$INSTANCE_ROOT/SddIA/target/release/kalma2-bridge" | grep -F "api/aiua/interact" >/dev/null || {
  echo "[ERROR] kalma2-bridge no expone /api/aiua/interact (ELF fósil). Compilar release en forja antes de empaquetar." >&2
  exit 1
}
```

Gate inmediato: 0 `*.rs` en bundle; 0 `Cargo.toml`; `strings` centinelas sin `execute-process.py`; **9 ELF consumidor** (lista §2); `MANIFEST.json` + `ONBOARDING.md`. Filtro C: sin `github-bridge-watcher`. Sin `mayeuta-llm` (esperado).

Si la instancia ya existía y debe preservarse `.SddIA` / `.events`: bundle a `dist/…` y rsync overlay **sin** pisar periféricos (ola T6). Wipe = `--out` directo a `INSTANCE_ROOT`.

### 4 — `instance-creator` (`skip_ignition`)

```bash
export SDDIA_EXECUTE_PROCESS_BIN="${FORGE_ROOT}/SddIA/target/release/execute-process"
./sddia-run.sh --process instance-creator --inputs "{
  \"instance_root\": \"${INSTANCE_ROOT}\",
  \"runtime_profile\": \"consumer\",
  \"vault_source\": \"${VAULT_STAGING}\",
  \"skip_ignition\": true
}"
```

Acuse JSON = éxito de inyección (DA-5). En `instance-creator` v1.3.0:

- **F-DEP-08:** `materialize_local_paths` sustituye `local.paths.json` ausente/vacío/`{}` por starter-kit.
- **F-SMOKE-01:** smoke nativo `local_qa_emitted: false`.
- **F-SYS-02:** plantillas `ExecStart=%f/SddIA/...`. Creator **no** hace `enable --now` (F-SYS-01).
- Puede renderizar `sddia-iota-publish-relay@.service`; en consumer **no** se enable.

Verificar en el acuse: `success:true`, `vault_files_copied`, smoke topology. Luego, **sin poll EDA**:

- `local.paths.json` **no** `{}`.
- Unidades renderizadas: `ExecStart` bajo `%f/SddIA/...` (no path de forja horneado).

### 5 — systemd núcleo + sensorial (`@%f`)

`instance-creator` deja unidades en `${INSTANCE_ROOT}/.SddIA/systemd/`. Núcleo/sensorial consumer: `sddia-event-watcher@.service`, `sddia-event-sweeper@.service`, `sddia-kalma2-bridge@.service`, `sddia-email-watcher@.service`, `sddia-telegram-watcher@.service`. `WorkingDirectory=%f`. Email: `ExecStart=%f/SddIA/daemons/email-watcher.sh`. Resto fábrica: `ExecStart=%f/SddIA/scripts/daemons/<daemon>.sh`.

`start-sddia.sh` (jurisdicción systemd) las re-materializa, copia a `~/.config/systemd/user/` y `enable --now` con `systemd-escape -p "$INSTANCE_ROOT"`. No enable `github-bridge-watcher` ni `iota-publish-relay` en consumer.

```bash
loginctl enable-linger "$(id -un)"
# Lab: stop/disable las mismas plantillas @escape(FORGE_ROOT) si están active
```

Copia manual **solo** si se omite `start-sddia` (F-SYS-01):

```bash
mkdir -p "${HOME}/.config/systemd/user"
cp -f "${INSTANCE_ROOT}/.SddIA/systemd/"sddia-*.service \
  "${HOME}/.config/systemd/user/"
systemctl --user daemon-reload
ESC="$(systemd-escape -p "$INSTANCE_ROOT")"
for stem in sddia-event-watcher sddia-event-sweeper sddia-kalma2-bridge sddia-email-watcher sddia-telegram-watcher; do
  systemctl --user enable --now "${stem}@${ESC}.service"
done
```

### 6 — Ignición núcleo

```bash
cd "$INSTANCE_ROOT"
mkdir -p .SddIA/daemons/logs
env -u SDDIA_EXECUTE_PROCESS_BIN \
  SDDIA_RUNTIME_PROFILE=consumer \
  SDDIA_SENSORIAL_JURISDICTION=systemd \
  ./start-sddia.sh >> .SddIA/daemons/logs/start-sddia.log 2>&1
```

Esperado:

- `MANIFEST.json` presente → `_ensure_orchestrator` **no** llama `cargo` ni `--seal-capsules`; resuelve ELF del bundle.
- F-DEP-09: `_sddia_discard_foreign_orchestrator_pin "$REPO_ROOT"`.
- Jurisdicción `systemd`; el script espera health WUI + heartbeats obligatorios (hasta 45 s **dentro** del propio script) y **exit 0**; no queda como daemon. Tekton no añade poll post-exit.
- Log: unidades `enable --now`; WUI `http://127.0.0.1:${WUI_PORT}/`; email/telegram **no** spawneados con `&`.
  Preflight en log de `kalma2-bridge`: `[WARN] mayeuta-llm no encontrado...` es APTO de ignición en consumer.
- Heartbeats: `.SddIA/daemons/state/heartbeat-audit.json`. Post-reboot: `host_reboot_stale_lock` sin `System_Fracture_Detected` es APTO.
- DLT: consumer no enable relay. Fan-out fail-soft. No exigir `/health` de relay.
- Override lab atado a TTY: `SDDIA_DAEMON_JURISDICTION=script`.

### 7 — Cierre documental del ciclo de despliegue

§5 validaciones → §6 contraste de ola → §7 auditoría → §8 Kaizen condicional.

---

## 4. Olas de despliegue (empíricas) vs deltas de forja

### 4.1 Olas empíricas Paciente 0

| Ola | Fecha | Artefacto | Canal | Veredicto breve |
|-----|-------|-----------|-------|-----------------|
| 0 Ensayo | 2026-08-20 | `PBI-LAB-PACIENTE0-SDDIA-AP` | clone + build debug, no bundle hermético | APTO laboratorio; G5 lote sin `actionable` (F-03) |
| 1 Redeploy | 2026-08-24 | `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824` | bundle+creator; parches instancia | WUI OK; F-DEP-01…06; G5 reunión → `passive` |
| 2 Absorción T6 | 2026-08-25 a.m. | audit `kaizen-paciente0-redeploy-20260825` + feature merge | bundle fresco + creator release | F-DEP-01…04 y F-TRIAGE-01 absorbidos; G5 sintético `actionable` |
| 3 Redeploy post-merge | 2026-08-25 12:01Z | `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825` (done) | mismo canal; `main` | Instancia OK **tras** pin release; F-DEP-07/08/09/SMOKE-01/SYS-01 |
| 4 Redeploy wipe | 2026-08-25 13:15Z | `AUDIT-PACIENTE0-DEPLOY-20260825T131532Z` | wipe; instancia ausente | **OLA-ESTABLE** |
| 5 Redeploy post-aislamiento | 2026-08-26 11:02Z | `AUDIT-PACIENTE0-DEPLOY-20260826T110203Z` | wipe; `main` + merge aislamiento | Gates **OLA-MEJORA**; post-ola Telegram conversacional **F-BUNDLE-06** (FIX lab) |
| 6 Redeploy post-PR #194 | 2026-08-26 12:00Z | `AUDIT-PACIENTE0-DEPLOY-20260826T120032Z` | wipe; `main` + bundle 8 ELF (aún sin publisher) | **OLA-MEJORA**; G-telegram APTO; F-BUNDLE-06 cerrado en runtime |
| 7 Centinelas sordos | 2026-08-26 | `AUDIT-PACIENTE0-CENTINELAS-EMAIL-SORDO-20260826` | diagnóstico runtime AP | Colisión plantillas home / wrappers cwd forja (`F-SYS-02`, `F-DEP-10`) |
| 8 Aislamiento multi-instancia | 2026-08-26 | `kaizen-aislamiento-multi-instancia-20260826` | Core PR #193 + `instance-creator` v1.3.0 | `%f` universal; `_sddia_resolve_instance_root`; 0 `pkill` cruzado |
| 9 (no anclada) | 2026-09-20 | — | física; audit/Kaizen no en `main` (Q3=errata) | No SSOT. Fricciones absorbidas en #292. |
| 10 Redeploy post-códice + FIX #298 | 2026-09-25 11:46Z | `AUDIT-PACIENTE0-DEPLOY-20260925T114629Z` | wipe; 13 ELF + conscience; creator v1.4.0 | **OLA-MEJORA**; G3 v2 `success:true`; `faa18af8` |

Cada nueva ejecución de **este prompt** es la **ola 11** (siguiente a §4.1). El contraste §6 la sitúa respecto al Kaizen de redeploy más reciente (`PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825`) **y** respecto a los deltas §4.2.

### 4.2 Deltas de forja posteriores a la ola 6 (no son olas de despliegue)

Citar `validacion.md` / `id_cambio` de `Evolution_log`. Prohibido SHA corto no anclado. Prohibido uuid de PBI / `execution_id` en esta tabla. Omitidos: rehab DCC, CI crónica, `github-bridge-watcher` (Filtro C), PRs documentales #271/#274.

| Fecha | PR / evolution | Qué cambia para el próximo redeploy |
|-------|----------------|-------------------------------------|
| 2026-08-26 | PR #197 · `16239778-a5bc-4a55-8996-9301e51a6176` | `email-watcher`: bootstrap IMAP si cambia cuenta o watermark obsoleto. |
| 2026-08-27 | PR #204/#205 · `7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b` | Store JSON `paths.userPreferencesStore` = `.SddIA/vector_store/user_preferences/`. |
| 2026-08-27 | PR #208 · `1243c58b-8e93-4897-ba3e-3efc26564673` | `sddia-iota-publish-relay@%f`. L-REQUIRED: **no** enable en consumer. |
| 2026-08-28 | PR #218 · `a91f2d40-6e3b-4c8a-b7f1-2d9e0c5a84f6` | `iota-immutable-publisher` ∈ `CONSUMER_BINS` (9 ELF). `--seal-capsules` en `execute-process` (forja). Aduana `SDDIA_CAPSULE_ANCHOR`. |
| 2026-08-28 | PR #219 · `eb6fb73a-9ded-49a1-a2a9-314624358b4b` | Este PBI vive en `docs/todos/pending/` (`type: deuda`, `dispatch: false`). No cambia el canal Vía C. |
| 2026-08-29 | PR #222 · `a1535038-8db5-4351-8a81-cfa5586b8c5b` | Telemetría cognitiva LLM + SSE Kalma2. Superficie WUI; no ELF extra. |
| 2026-08-29 | PR #224 · `97d96117-49cf-4db7-b860-acd65bee216a` | `GET /api/system-health` + panel Kalma2. No gate bloqueante. |
| 2026-08-29 | PR #228 · `7f3a9e2b-1c4d-4f8a-9b6e-0d5c8a1f3e72` | Heartbeat: discriminación suspend/crash + fagocitosis. Complementa G-heartbeat. |
| 2026-08-31 | PR #238 · `6b8504a4-b45f-468f-8a9f-7b188a8ca4a1` | `email-watcher`: keepalive heartbeat 10s. |
| 2026-08-31 | PR #241 · `4d384bb1-f89d-41ce-835a-9db6d6bed114` | LanceDB físico. Forja: `protoc`. `memory-evolution-ingest` ya no es JSON-fallback. |
| 2026-08-31 | PR #243 · `0b5a80fd-6932-457c-bad0-f9e8b46390d0` | Censo Kalma2, lock huérfano y sonda de vitalidad. |
| 2026-09-01 | PR #244 · `docs/fixes/email-watcher-elf-fosil-1933c0a0fe2c/validacion.md` | Reciclo ELF `email-watcher` fósil post-keepalive. Bundle fresco = ELF nuevo. |
| 2026-09-04 | PR #254 · `4c775b1f-5eea-46c6-94ff-25e1861bf9b1` | `gemini-http-infer` live (Tormentosa). Sin bóveda Gemini → 503; epidermis lo sanea post-#282. |
| 2026-09-06 | PR #264 · `ad46c2d6-30fc-451e-8e74-5b19f4f2602e` | Fan-out DLT + proof `.SddIA/proofs/dlt-telemetry/`. Plano B instancia pendiente. |
| 2026-09-06 | PR #266 · `95441293-1049-4016-8112-a322919d34e8` | Triaje-P cold-start: `memory:pref-query`, `P-EXEMPT-C`, `P-MUTE-SENDER` en `email-triage-gateway`. |
| 2026-09-07 | PR #267 · `c142dc19-b3d9-4810-bad2-734c1910606e` | `daemon-heartbeat-audit` v1.2.0: puerta `btime`; `host_reboot_stale_lock` sin fractura. |
| 2026-09-07 | PR #268 · `29e6cbc5-6d13-45b9-bd62-f24e3fdb0c45` | Triaje correo: peaje LLM desde `telemetry_receipt`; `SDDIA_LLM_REQUIRE_INFER` → `classification-degraded`. |
| 2026-09-08 | PR #269 · `84233af2-b3c3-40e8-9b28-0aef17c87c4c` | LanceDB preferences: dual-write si tabla en `{vectorStore}/lancedb/`; JSON permanece SSOT. |
| 2026-09-08 | PR #270 · `4f009a69-e575-4d34-af85-e496d0dea370` | Latido Aiúa CLI: `thought-graph-access`, `Thought_Persisted`. Tormentosa en WUI depende de esto + #276. |
| 2026-09-08 | PR #273 · `57e1dcd8-c04c-4818-818c-fb04ecf046e2` | `ENETUNREACH` transitorio en DLT: fail-soft, sin fractura. |
| 2026-09-09 | PR #275 · `cef04e8e-36a5-40d1-b697-e00a0640db87` | `thinkingLevel` en `gemini-http-infer` vía `SDDIA_GEMINI_THINKING_LEVEL` (starter-kit vacío ≠ HIGH). |
| 2026-09-09 | PR #276 · `bdd512bd-42ea-4c45-ab97-2e0fa8ea37c8` | Puente perceptivo WUI→Aiúa (`POST /api/aiua/interact`), botón `#aiua-pulse`. |
| 2026-09-09 | PR #277 · `5f09da5c-a96c-4475-9ce6-15d55cc5840c` | Serialización IOTA y tolerancia a colisión gas-version sin fractura. |
| 2026-09-09 | PR #280 · `51a53297-7e8d-469e-8bb2-8b413b28a366` | Sello CA-6: ELF release fósil de `kalma2-bridge` → 404 en `/api/aiua/interact`. Mitigación: rebuild + `strings`. |
| 2026-09-09 | PR #281 · `0e4e3e14-c6e2-417f-8331-332310766679` | Preflight WARN en `kalma2-bridge.sh` ante `mayeuta-llm` ausente; orden release-first. |
| 2026-09-09 | PR #282 · `24061402-d129-49b1-8a35-ee2388ee4816` | Sanitización HTTP 503 Gemini en epidermis WUI sin retry (DA-5). |
| 2026-09-09 | PR #283 · `1dc4055c-b0c8-40ff-a30d-d257152fb8df` | Combustión Tormentosa vía `antigravity-cli` (`SDDIA_AGY_EFFORT`, `SDDIA_AGY_TIMEOUT_SECS`). |
| 2026-09-10 | PR #284 · `1bcd50a0-aa1c-468e-b2a0-87ce54923c9a` | Extraer texto agy antes de persistir pensamiento. |
| 2026-09-10 | PR #285 · `89661fec-bed6-4c3d-8169-3922ddbe7a2f` | Sanitización epidermis WUI ante red/caída del vector agy. |
| 2026-09-10 | PR #286 · `60b81a68-39ac-4ccd-b920-6ab759b474d8` | Anatomía motora Aiúa: tendones y despacho `Aiua_Process_Requested`. |
| 2026-09-11 | PR #287 · `cf1ddf69-3dc6-4576-8245-e47c9536b000` | Proceso `email-noise-digest` en códice v1.0.2. |
| 2026-09-11 | PR #288 · `4a402f77-9c77-45c8-8383-0a64dcfde71e` | `reply_markup` + `callback_query`; gateway XOR `text`/`callback_data`. |
| 2026-09-12 | PR #289 · `7e4c1a90-2b6d-4f18-9c3a-5d8e0b1a2476` | `email-digest-preference-reply` en códice v1.0.2 (botonera `dpref`). |
| 2026-09-12 | PR #290 · `89d3ce2b-4bde-49af-8945-8aeb9c4e84cf` | Tendón `delegar_habito` Kalma2 → `User_Preference_Change_Requested`. |
| 2026-09-13 | PR #291 · `8e223315-114f-44c5-9683-2a5956731329` | `LogRateLimit` fábrica daemon; watcher silencia skips. |
| 2026-09-20 | PR #292 · `83d6eb73-0936-4acb-9f1e-5d519987f1ab` | Bundle hermético `MANIFEST.json`; poda `library/codexes` + Filtro C fail-closed. |
| 2026-09-25 | PR #294 · `79b6cc26-3958-4704-a797-d87bf020ce6d` | `LogRateLimit` unidades instancia; `source_sha256` reanclado. |
| 2026-09-25 | PR #295 · `c9c0206e-aed4-4e48-a17d-b28d1de43d46` | Reserva IOTA / exit 3 prótesis no escalan a Kintsugi. |
| 2026-09-25 | PR #296 · `ff8a0c37-a03d-4945-933c-8b54c03b9707` | ABSTRACT-04: códice software fuera del consumer. |
| 2026-09-25 | PR #297 · `d2e44083-ccdf-45af-b477-f6c71833fc31` | `llm-router` + registro de oráculos. |
| 2026-09-25 | PR #298 · `806c9463-4c82-4216-a246-d5650a8553e9` | 13 ELF Tormentosa; creator v1.4.0 registry+perfil; LogRateLimit email. |
| 2026-09-25 | ola 10 · `faa18af8-60e1-4d3f-b824-990f89cee208` | Bundle empaqueta `SddIA/conscience`; `start-sddia` no exige git. |

Forja-only (sin gate de instancia; no reabrir como fricción de ola): PR #272 (`0b3ef02b-debe-4532-91da-c265ed59942b`, F4c `gh`/`api.github.com`), PR #278 (`67a74480-f2b3-440d-a467-7dfe32726705`, triaje asíncrono de fracturas), PR #279 (`0d6cd80e-a292-4919-b99c-37d0568ee76e`, `ephemeral-cache-purger`).

---

## 5. Validaciones habituales (gates)

No secretos. Fallo = NO APTO de ola (sigue §8).

| ID | Check | Criterio APTO |
|----|--------|----------------|
| G0-config | `CONFIG_SOURCE` existe; staging copió vault | `vault_env_present`; claves IMAP/LLM **nombres** (no valores). IOTA **no** obligatoria. |
| G-bundle | integridad artefacto | 0 `.rs`; 0 `Cargo.toml`; `PY_LEAK=no`; `MANIFEST.json`; **13 ELF** §2 + `SddIA/conscience/aiua_core.md`; Filtro C sin `github-bridge-watcher`; `handle_aiua_interact` en ELF (grep exacto de ruta puede fallar por literales adyacentes). `mayeuta-llm` no forma parte de `CONSUMER_BINS` |
| G1 | topología | `.SddIA/`, `.events/{domain,orchestration,telemetry,pending}/`; `local.paths.json` no `{}`; `.SddIA/vector_store/user_preferences/` operable (o creable en primer `put_revision`) |
| G2 | ley local | `constitution.json` `product=SddIA_AP`; códice `codex-kalma2-assistant` v1.0.2 en `.SddIA/library/codexes/` con los 3 procesos de `process_membership` |
| G3 | WUI + Aiúa | HTTP 200 en `:${WUI_PORT}`; **0** `cargo build` en instancia; preflight `[WARN] mayeuta-llm` = APTO. `POST /api/aiua/interact` → `success:true` + `thought_id` (G3 v2). `cápsula tool 'llm-router' no encontrada` = NO APTO. Chat `/api/chat` degradado **no** tumba G3. |
| G3b | systemd | `sddia-{event-watcher,event-sweeper,kalma2-bridge,email-watcher,telegram-watcher}@%f` `active`; `WorkingDirectory=%f`; `ExecStart=%f/SddIA/...`. **No** exigir `iota-publish-relay` ni `github-bridge-watcher` en consumer. |
| G4 | Filtro C | sin `github-bridge` activo; instancia sin `AGENT_RUNTIME_*`; sin `codex-software-engineering` en códices locales |
| G-orch | resolución ELF | creator usó **release** (o cicatriz vigente); ignición descarta pines foráneos (`_sddia_discard_foreign_orchestrator_pin`); bundle no recompila |
| G-heartbeat | resiliencia térmica | Existe `.SddIA/daemons/state/heartbeat-audit.json`. Obligatorios (`event-watcher`, `event-sweeper`, `kalma2-bridge`) `missed_cycles < 3` tras ignición. `host_reboot_stale_lock` **sin** `System_Fracture_Detected` = APTO. Fractura L3 solo si PID muerto con `started_at >= btime`. |
| G-dlt | fan-out telemetría | Consumer default: `skipped-config-missing` o ausencia de DLQ de `Domain_Entity_Telemetry_Captured`. Si `SDDIA_LAB_SIMULATE_IOTA=1`: proof en `.SddIA/proofs/dlt-telemetry/` con digest ≠ `batched-digest`. Anclaje físico Testnet = Plano B (opt-in; exige relay **fuera** de consumer o cambio de perfil — no es default de este prompt). |
| G-telegram | interactividad y bot instancia | **Opcional** post-ola (DA-5). `./sddia-run.sh --process telegram-gateway --inputs '{"text":"…"}'` → `success:true` `emitted:true` (`TelegramMessage_Received`); o `callback_data` XOR → `TelegramCallback_Received`. Journal watcher: `gateway rc=1` / `gateway callback rc=1` = NO APTO de este gate. |
| G-preferences | almacén de hábitos | `.SddIA/vector_store/user_preferences/` recibe revisiones JSON; réplica LanceDB solo si la tabla existe (dual-write sin error). Ausencia de LanceDB = APTO. |
| G5 | First Blood | **Opcional** en redeploy rutinario (DA-5). Si se ejecuta: reunión estructural → `actionable`. No esperar IMAP en bucle. |

**F-SMOKE-01:** `local_qa_emitted: false`; no DLQ de `Local_QA_Requested` por smoke.

**Telegram multi-instancia:** tokens distintos lab vs instancia → watchers `@forja` y `@AP` pueden coexistir. Mismo `TELEGRAM_ALLOWED_CHAT_ID` es válido.

---

## 6. Contraste contra el último PBI Kaizen de despliegue

### 6.1 Cómo elegir el SSOT de contraste

Único documento Kaizen **de despliegue Paciente 0** más reciente (`document_id` `PBI-KAIZEN-PACIENTE0-REDEPLOY-*`). Pending prevalece sobre done si `updated`/`redeploy_executed_at` es mayor.

Cargar: `friction_ids`, tabla de gates, `bundle_manifest`, mitigaciones ad-hoc. Los PBI lab/fix de forja (DLT, térmica, Aiúa, hábitos) **no** sustituyen este SSOT; se contrastan como deltas §4.2.

### 6.2 Matriz de evolución (rellenar en la auditoría)

Para cada ID del Kaizen baseline y cada gate G*:

| Campo | Valores |
|-------|---------|
| Estado baseline | APTO / NO APTO / no auditado |
| Estado ola actual | APTO / NO APTO / no auditado |
| Delta | **mejoró** / **igual** / **regresionó** / **nuevo** |
| Nota | una línea; sin secretos |

IDs mínimos: `F-DEP-01`…`F-DEP-09`, `F-DEP-05`, `F-SMOKE-01`, `F-SYS-01` (residual: creator no enable), `F-SYS-02`, `F-BUNDLE-06`, `F-TRIAGE-01`…`03` (si G5), G0–G4, G3b, G-orch, G-bundle, G-heartbeat, G-dlt, G-telegram, G-preferences.

### 6.3 Veredicto valorativo de ola

Un párrafo + etiqueta:

| Etiqueta | Criterio |
|----------|----------|
| **OLA-MEJORA** | Ninguna regresión de F-DEP ya absorbidas; 0 fricción nueva bloqueante |
| **OLA-ESTABLE** | Mismas fricciones residuales (F-SYS-01: enable lo hace `start-sddia`, no el creator); G1–G4 APTO |
| **OLA-REGRESIÓN** | Reaparece F-DEP-01/02/03/04 **con** ELF release pinneado, o G3/G3b KO |
| **OLA-NUEVA-FRICCIÓN** | IDs nuevos no listados en el Kaizen baseline |

El veredicto **obliga** §8 si es `OLA-REGRESIÓN` o `OLA-NUEVA-FRICCIÓN`. `OLA-ESTABLE` con residuales ya en pending **no** duplica PBI.

No tratar como fricción nueva: `skipped-config-missing` en consumer sin bóveda IOTA; `host_reboot_stale_lock` sin fractura; ausencia de unidad relay en consumer; `[WARN] mayeuta-llm` en ignición consumer; Chat `/api/chat` degradado sin ese ELF; ausencia de tabla LanceDB de preferencias.

---

## 7. Auditoría final

Crear `{paths.auditsPath}/paciente0-deploy-{STAMP}.md` (`docs/audits/`, Cúmulo `auditsPath`).

Frontmatter mínimo: `document_id`, `uuid` v4, `created`, `instance_path`, `bundle_manifest`, `instance_creator_correlation_id`, `ola_verdict`, `kaizen_baseline_document_id`, `wave_matrix_ref`, `core_deltas_ref` (tabla §4.2 o enlace).

Cuerpo: canal usado; mitigaciones (`%f`, pin release, descarte pin foráneo, bundle sin cargo); gates §5; **matriz §6.2 + veredicto §6.3**; fricciones nuevas si las hay; **§ post-ola sensorial** (Telegram/correo, tokens distintos, F-BUNDLE-06, XOR text/callback); **§ DLT** (skip vs proof en `.SddIA/proofs/dlt-telemetry/`; si Plano B no se intentó, declararlo); **§ heartbeat** (`classification` por daemon; no exigir cero orphan post-reboot); **§ preferencias y WUI** (ruta `/api/aiua/interact`, store `.SddIA/vector_store/user_preferences/`, LanceDB opt-in); qué **no** se hizo (G5, Plano B, Chat Mayeuta). Cero secretos. No reescribir audits previos.

---

## 8. PBI Kaizen pendiente (condicional)

**Generar** `docs/todos/pending/[KAIZEN] Paciente 0 ${INSTANCE_NAME} — redeploy {STAMP} y fricciones.md` **si y solo si** §6.3 ∈ {`OLA-REGRESIÓN`, `OLA-NUEVA-FRICCIÓN`} **o** el Kaizen pending vigente quedó obsoleto.

**Excepción post-ola (no Kaizen redeploy):** cápsulas ausentes en bundle consumidor → **FIX lab** vía `bug-fix` en forja; no duplicar Kaizen Paciente 0 ni parchear `SddIA/` en instancia.

Estructura (antecesor `docs/features/kaizen-paciente0-redeploy-fricciones` + PBI 20260824/20260825):

- Frontmatter: `document_id` `PBI-KAIZEN-PACIENTE0-REDEPLOY-{YYYYMMDD}`, `uuid`, `persist_ref` reservado, `derived_from` el Kaizen contrastado, `friction_ids`, `instance_path`, `config_source`, `bundle_manifest`, correlation ids.
- Cuerpo: §0 contexto; §0bis bitácora A–F; §0ter métricas; §1 fricciones (síntoma / causa / ad-hoc / DT); §2 qué sigue absorbido; §3 objetivos; §4 criterios; §5 orden de forja; §6 refs; §7 audit al cierre del **ciclo feature** (distinto del audit de ola §7).

**No generar** si `OLA-MEJORA` o `OLA-ESTABLE` y `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825` cubre las deudas residuales — solo audit de ola + enlace al done.

---

## 9. Candidato a proceso Core (`paciente0-deploy`)

**No forjar ahora.** Cuando se abra feature: `entity-manager` → `process-creator`. Norma: `sddia-distribution-protocol`. Orquesta; no sustituye `instance-creator` ni el bundle.

### 9.1 Identidad propuesta

| Campo | Valor |
|-------|--------|
| `id` | `paciente0-deploy` |
| `type` | `process` |
| `inputs` | `instance_name` (default `SddIA_AP`), `instance_parent`, `config_source`, `forge_root`, `skip_g5` (default true), `wipe` (default false) |
| `outputs` | `instance_root`, `bundle_manifest`, `ola_verdict`, `audit_ref`, `kaizen_pbi_ref` (nullable) |

### 9.2 Fases propuestas

| Fase | Intent | Motor |
|------|--------|-------|
| Baseline | Resolver último PBI Kaizen PACIENTE0-REDEPLOY | lectura `docs/todos/{pending,done}/` |
| Vault | Staging desde `config_source` + Filtro C (IOTA opcional) | cápsula/script; secretos opacos |
| Bundle | `build-release-bundle` consumer (9 ELF) | script existente |
| Materialize | `instance-creator` `skip_ignition` + pin release | proceso existente |
| Sensorial | `start-sddia` jurisdicción systemd hace enable `@%f`; R-07 | OS; F-SYS-01 = creator no enable |
| Ignition | `start-sddia` env aislado, descarte pin foráneo, bundle sin cargo | script; detach tras health |
| Validate | gates §5 | checks; sin poll post-acuse |
| Contrast | matriz vs Kaizen baseline + deltas §4.2 | documental |
| Audit | escribir `docs/audits/…` | IDE / cápsula doc |
| KaizenGate | emitir PBI pending si §6.3 lo exige | documental |

### 9.3 Fuera del proceso

Wizard UX (`DT-CONFIG-UX-ONBOARDING`). Mutación de genoma (ciclo `feature` del Kaizen). G5 IMAP como default (opt-in `skip_g5=false`). Anclaje IOTA físico (Plano B; perfil ≠ consumer). Forja del proceso `paciente0-deploy`. Empaquetar `mayeuta-llm` en `CONSUMER_BINS` (Chat Mayeuta).

---

## 10. Referencias

| Ref | Uso |
|-----|-----|
| `SddIA/norms/sddia-distribution-protocol.md` | Vía C, bundle hermético, creator (v1.2.3) |
| `SddIA/process/instance-creator.md` | Fases Topologia–Smoke, starter-kit, registry+perfil, `%f` (v1.4.0) |
| `SddIA/process/daemon-heartbeat-audit.md` | Argos, `btime`, `host_reboot_stale_lock` (v1.2.0) |
| `SddIA/process/telegram-gateway.md` | XOR `text`/`callback_data` (v1.0.2) |
| `SddIA/library/codexes/codex-kalma2-assistant.md` | Códice asistente personal (v1.0.2) |
| `SddIA/library/norms/todos-jurisdiction.md` | Jurisdicción `docs/todos/` (v1.1.0) |
| `SddIA/core/cumulo.paths.json` | `auditsPath`, `eda_instance.proofs`, `daemons_instance.state`, `userPreferencesStore`, `vectorStore` |
| `SddIA/scripts/common/sddia_shell_lib.sh` | `_sddia_resolve_orchestrator`, `_sddia_discard_foreign_orchestrator_pin`, `_sddia_require_protoc` |
| `SddIA/scripts/build-release-bundle.sh` | `CONSUMER_BINS` (13 ELF) + `SddIA/conscience`. Sin flag `--seal-capsules`. |
| `SddIA/evolution/Evolution_log.md` | SSOT de `id_cambio` citados en §4.2 |
| `docs/audits/paciente0-deploy-20260925T114629Z.md` | Última ola empírica de redeploy (10) |
| `docs/audits/paciente0-deploy-20260826T120032Z.md` | Ola 6 (antecesor anclado) |
| `docs/audits/paciente0-deploy-20260826T110203Z.md` | Ola 5 + post-ola Telegram |
| `docs/audits/paciente0-centinelas-email-sordo-20260826.md` | Ola 7: colisión plantillas / wrappers |
| `docs/audits/kaizen-aislamiento-multi-instancia-20260826.md` | Ola 8: `%f` universal |
| `docs/audits/kalma2-wui-tormentosa-chat-20260909.md` | Diagnóstico empírico **forja** `:8765`: ruta `/api/aiua/interact` y ELF fósil |
| `docs/todos/done/[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher.md` | F-BUNDLE-06 (PR #194) |
| `docs/todos/done/PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP.md` | Plano A DLT (PR #264) |
| `docs/todos/done/[FIX] Resiliencia Térmica en Heartbeat Audit y Poda Ontológica en Mayeuta.md` | PR #267 |
| `docs/todos/done/[KAIZEN] Triaje de correo — batería 20260907 inferencia nula.md` | Peaje LLM y `SDDIA_LLM_REQUIRE_INFER` (PR #268) |
| `docs/todos/done/[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start).md` | Triaje-P (PR #266) |
| `docs/features/memoria-preferencias-usuario/` | Store JSON `userPreferencesStore` (PR #204/#205) |
| `docs/features/pref-store-lancedb-migration/` | Réplica LanceDB; JSON SSOT (PR #269) |
| `docs/features/nucleo-aiua-tormentosa-motor/` | Arranque CLI Aiúa (PR #270) |
| `docs/fixes/route-domain-event-enetunreach-41717b4bb229/` | Fail-soft DLT `ENETUNREACH` (PR #273) |
| `docs/features/aiua-audit-findings/` | `SDDIA_GEMINI_THINKING_LEVEL` (PR #275) |
| `docs/features/kalma2-aiua-perceptive-bridge/` | Puente perceptivo WUI→Aiúa (PR #276) |
| `docs/fixes/route-domain-event-gas-version-60db1db67e49/` | Serialización IOTA / gas-version (PR #277) |
| `docs/fixes/kalma2-bridge-aiua-interact-stale-elf/` | Rebuild release fresco (PR #280) |
| `docs/fixes/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34/` | Preflight WARN no bloqueante (PR #281) |
| `docs/fixes/kalma2-wui-aiua-503-sanitize/` | 503 Gemini en epidermis (PR #282) |
| `docs/features/aiua-antigravity-cli-vector/` | Vector agy (PR #283) |
| `docs/fixes/aiua-agy-empty-response-text/` | Texto agy vacío (PR #284) |
| `docs/fixes/kalma2-wui-agy-network-sanitize/` | Red agy en WUI (PR #285) |
| `docs/features/aiua-motor-anatomy-eda/` | Anatomía motora Aiúa (PR #286) |
| `docs/features/email-noise-heuristic-digest/` | `email-noise-digest` (PR #287) |
| `docs/features/telegram-inline-keyboard/` | Botonera + XOR callback (PR #288) |
| `docs/features/email-digest-preference-reply/` | `email-digest-preference-reply` (PR #289) |
| `docs/features/intercepcion-habitos-kalma2/` | `delegar_habito` (PR #290) |
| `docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md` | Kaizen baseline deploy |
| `docs/todos/done/[LABORATORIO] MVP Paciente 0 SddIA_AP.md` | Ola 0 |
| `docs/audits/kaizen-paciente0-redeploy-20260825.md` | Ola 2 T6 |
| `docs/features/kaizen-paciente0-redeploy-fricciones/` | Estructura documental Kaizen |
| `docs/features/lab-paciente0-dlt-telemetry-mvp/` | Spec proofs / skip |
| `docs/fixes/thermal-resilience-heartbeat-mayeuta/` | Spec `btime` |
| `docs/features/kaizen-aduana-dlt-relay-supervisado/` | L-REQUIRED consumer |
| `docs/features/lancedb-real-vector-memory/` | `protoc` / ingest |

---

## 11. Criterios de cierre de **esta** deuda

- [x] Prompt copiable (§1) + procedimiento (§3) + gates + contraste de olas + audit + Kaizen condicional.
- [ ] Proceso `paciente0-deploy` forjado vía `entity-manager` (ciclo feature distinto).
- [x] Absorción F-DEP-07/08/09 y F-SMOKE-01 en Core.
- [x] Absorción F-BUNDLE-06 (`PBI-FIX-BUNDLE-TELEGRAM-GATEWAY`) — validado ola 6.
- [x] Absorción F-SYS-02 / F-DEP-10 — `%f` universales.
- [x] `iota-immutable-publisher` en bundle consumidor (9 ELF). Plano A DLT en Core (PR #264). Plano B instancia: pendiente de ola 9.
- [x] `protoc` identificado en `sddia_shell_lib.sh` (`_sddia_require_protoc`); aplica a **forja**, no a instancia `MANIFEST.json`.
- [x] Errata v1.3.0 absorbida en v1.4.0 (PRs, proofs, olas vs deltas, L-REQUIRED consumer, G-heartbeat `btime`).
- [x] Deltas consumer post-ola 6 catalogados (no solo ventana #268–#290).
- [x] Ampliación v1.5.0 (PRs #268–#290, códice v1.0.2, WUI/Telegram/preferencias) restaurada y refinada en v1.6.0.
- [x] Filtro A v1.6.0: rutas Cúmulo, env reales, UUIDs de `Evolution_log`, XOR Telegram, peaje LLM, Chat vs ignición.
- [x] Prevención de ELF fósil en `kalma2-bridge` con verificación activa de `/api/aiua/interact` en G-bundle.
- [x] Preflight de `mayeuta-llm` clarificado: APTO de ignición; Chat `/api/chat` degradado esperado en consumer.
- [x] Códice `codex-kalma2-assistant` v1.0.2: tres procesos de `process_membership` (sin confundir con `user-preference-ingest`).
