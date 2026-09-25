---
document_id: AUDIT-PACIENTE0-DEPLOY-20260925T114629Z
uuid: "c942682f-6ff5-4b3d-a3c3-5459ff06fb18"
title: "Auditoría ola Paciente 0 — despliegue 20260925T114629Z"
created: "2026-09-25"
instance_path: /home/racso/Proyectos/SddIA_AP
bundle_manifest: "20260925T114055Z"
instance_creator_correlation_id: "b4ac5748-dc6c-4a14-9277-bfc8a497b228"
ola_verdict: OLA-MEJORA
deploy_gates_verdict: APTO
kaizen_baseline_document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
wave: 10
wave_matrix_ref: docs/audits/paciente0-deploy-20260925T114629Z.md
pbi_prompt_ref: docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
pbi_prompt_document_id: PBI-DT-PACIENTE0-DEPLOY-PROCESS
prompt_ssot_version: "1.7.0"
fix_pbi_ref: docs/todos/done/[FIX] bundle consumer — cadena Tormentosa (13 ELF), llm-registry y active-domain-profile en instance-creator.md
fix_pbi_document_id: PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN
friction_ids_post_ola: []
forge_branch: feat/paciente0-redeploy-ola10-codex-extraction
forge_head_baseline: 67e6a2b
core_deltas_ref: "PBI §2 PR #291–#297 + FIX #298 + faa18af8"
laudos_applied:
  Q1: a
  Q2: si
  Q3: errata
unanchored_wave_note: "Ola 9 (2026-09-20) no anclada; Q3=errata. No SSOT."
consumer_bins: 13
wui_port: 8766
g5_executed: false
g_telegram_executed: true
kaizen_pbi_opened: false
preprod_vault_present: false
---

# Auditoría — Paciente 0 deploy ola 10 (20260925T114629Z)

Cicatriz de **esta** ola. No reescribe audits previos. Cero secretos.

## 1. Canal

```text
forja feat/paciente0-redeploy-ola10-codex-extraction (baseline main 67e6a2b; FIX #298 mergeado)
  → vault staging SddIA_AP.deploy-vault (CONFIG_SOURCE + Filtro C + SDDIA_GEMINI_MODEL)
  → build-release-bundle --out INSTANCE_ROOT (wipe; 13 ELF + user-preference-store)
  → instance-creator skip_ignition cid=b4ac5748… (registry+perfil materializados)
  → start-sddia.sh KO: _sddia_ensure_hooks_path exige git (instancia no-repo)
  → fallback F-SYS-01: enable --now 5 unidades @%f
  → overlay post-absorción F-BUNDLE-CONSCIENCE-MISSING + F-START-SDDIA-NON-GIT-HOOKS (faa18af8)
```

Wipe: instancia ausente. Vault: 12 ficheros; 5 `AGENT_RUNTIME_*` omitidas. `PREPROD_VAULT` ausente; constitución `meta.product=SddIA_AP` y códice local desde staging. Linger=yes. Lab email IMAP distinto (R-07: lab se deja). Telegram tokens distintos.

## 2. Gates

| ID | Ola 10 | Nota |
|----|--------|------|
| G0-config | APTO | `vault_env_present`; IMAP/LLM/Telegram/Gemini (nombres). `SDDIA_GEMINI_MODEL` inyectado desde forja. IOTA no. |
| G-bundle v2 | APTO | 0 `.rs`; `PY_LEAK=no`; `filtro_c=true`; 13 ELF núcleo + `user-preference-store`; `source_digests` 13; `library/codexes/` = index/contract/kalma2. `strings \| grep -F api/aiua/interact` no casa (literales adyacentes); `handle_aiua_interact` presente. Overlay: `conscience/aiua_core.md`. |
| G1 | APTO | `.SddIA/`; `.events/{domain,orchestration,telemetry,pending}/`; `local.paths.json` ≠ `{}`; store preferencias creable. |
| G2 | APTO | `constitution.json` `meta.product=SddIA_AP`; códice v1.0.2 local + 3 `process_membership`. |
| G3 v2 | APTO | HTTP 200 `:8766`. Primer POST (pre-overlay) `success:false` `genoma ausente …/aiua_core.md`. Tras overlay: HTTP 200 `success:true` `thought_id` 64 hex, `response=vivo`, `duration_ms=12324`, `telemetry.model` set, `tokens` objeto. `llm-router` encontrado. |
| G3b v2 | APTO | 5 unidades `@%f` active; `LogRateLimitIntervalSec=30s` / `Burst=500` (email incluida). github-bridge/iota-relay AP inactive. |
| G4 v2 | APTO | Filtro C build OK; 0 archivos `codex-software-engineering*`; perfil `{"codex_slug":"codex-kalma2-assistant","git_required":false}`. Residual: `index.md` cataloga fila CSE (no archivo). |
| G-orch | APTO | Creator pin forja release; ignición ELF bundle (mismo SHA `execute-process`). 0 `cargo build` en instancia. |
| G-heartbeat | APTO | 5 daemons `healthy` `missed_cycles=0`. |
| G-dlt | APTO | Consumer: `Thought_Persisted` → iota `config-missing: IOTA_WALLET_SECRET`. Sin Plano B. |
| G-telegram | APTO | `telegram-gateway` `success:true` `emitted:true` + `TelegramMessage_Received`. |
| G-preferences | APTO | Store JSON creable; LanceDB ausente. |
| G5 | no auditado | opt-out DA-5. |
| G-eda-roots | APTO | Ignición sin `cannot read`/panic. `PBI_Forged` sintético → dead-letter `ecst-gate` (`event_type not cataloged`); no `System_Fracture` por roots. Distinto de `no_subscriber` (clase ausente en Core). |
| G-authority | APTO | `./sddia-run.sh --process feature --inputs '{}'` → `Proceso no encontrado: feature`. Cero workspace-init. |
| G-llm-registry | APTO | Creator `llm_registry_materialized:true`; `oracle-agy` `affinity:aiua` `fallback:oracle-gemini`; `oracle-gemini` active; `model` vacío. |
| G-agy-host | APTO | `agy` en host; `PATH` instancia incluye `~/.local/bin`; G3 v2 en 12s. |
| F-SMOKE-01 | no emitido | `local_qa_emitted: false`. |

Preflight `[WARN] mayeuta-llm` = APTO ignición. Chat `/api/chat` degradado esperado.

## 3. § desacople códice software

`SddIA/library/codexes/`: `index.md`, `codex-contract.md`, `codex-kalma2-assistant.md`, `codex-kalma2-assistant/`. 0 procesos `feature`/`bug-fix`/… en instancia. Cúmulo v1.11.1 con roots hacia códice inexistente: fail-soft. `.SddIA/active-domain-profile.json` = `codex-kalma2-assistant` / `git_required: false`.

## 4. § Latido Aiúa

Post-overlay: `POST /api/aiua/interact` `success:true`. `thought_id` persistido; evento `Thought_Persisted` emitido (DLQ IOTA = skip config-missing). WUI flatten no expone `telemetry_receipt.provider` ni `attempts[]` (epidermis). `agy` en host. Fractura `9157ce41` = `route-domain-event` / `merkle-batch-preseal` (DLT), **no** `aiua-stimulus-processing`.

## 5. Matriz vs Kaizen baseline + ola 9 no anclada

| ID | Baseline | Ola 10 | Delta |
|----|----------|--------|-------|
| F-DEP-01…09 / F-SMOKE-01 / F-SYS-02 / F-BUNDLE-06 | APTO | APTO | igual |
| F-SYS-01 | residual (creator no enable) | residual + `start-sddia` no-git hasta overlay | igual/absorbido |
| F-BUNDLE-HERMETIC-DAEMON-RESOLVER | NO APTO ola 9 no anclada | APTO runtime (ELF vía MANIFEST, 0 Cargo.toml) | mejoró |
| F-BUNDLE-LIBRARY-FILTRO-C | NO APTO ola 9 no anclada | APTO (archivos CSE 0; index residual) | mejoró |
| F-BUNDLE-TORMENTOSA-CHAIN | no auditado | APTO (13 ELF + combustión) | nuevo absorbido |
| F-CREATOR-LLM-REGISTRY | no auditado | APTO | nuevo absorbido |
| F-CREATOR-DOMAIN-PROFILE | no auditado | APTO | nuevo absorbido |
| F-SYSTEMD-EMAIL-LOGRATE | no auditado | APTO | nuevo absorbido |
| F-BUNDLE-CONSCIENCE-MISSING | — | APTO post-`faa18af8` | descubierta+absorbida |
| F-START-SDDIA-NON-GIT-HOOKS | — | APTO post-`faa18af8` | descubierta+absorbida |
| G-eda-roots / G-authority / G-llm-registry / G-agy-host / G3b v2 | — | APTO | nuevo |

## 6. Veredicto

**OLA-MEJORA.** Cero regresión F-DEP. Fricciones ola 9 APTO en runtime. Fricciones §8 APTO. Dos IDs nuevos absorbidos en el mismo ciclo (`faa18af8`); no abren Kaizen. Residual documental: `index.md` CSE; G-eda-roots `ecst-gate` ≠ `no_subscriber`.

## 7. Post-ola / DLT / heartbeat / WUI

Sensorial: Telegram APTO; G5 no ejecutado. DLT Plano B no intentado. Heartbeat 5/5 healthy. WUI `:8766` + Latido OK. No se hizo: G5, Plano B, Chat Mayeuta, forja `paciente0-deploy`.
