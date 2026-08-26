---
document_id: AUDIT-PACIENTE0-DEPLOY-20260826T120032Z
uuid: "7bb7b1dd-c37b-4d9e-9707-4bece1e21fe5"
title: "Auditoría ola Paciente 0 — despliegue 20260826T120032Z"
created: "2026-08-26"
instance_path: /home/racso/Proyectos/SddIA_AP
bundle_manifest: "20260826T115731Z"
instance_creator_correlation_id: "79157722-b3d6-47ea-950d-90682517ae6f"
ola_verdict: OLA-MEJORA
deploy_gates_verdict: APTO
post_ola_operational_verdict: F-BUNDLE-06_CERRADO
kaizen_baseline_document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
wave_matrix_ref: docs/audits/paciente0-deploy-20260826T120032Z.md
pbi_prompt_ref: docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
pbi_prompt_document_id: PBI-DT-PACIENTE0-DEPLOY-PROCESS
fix_pbi_ref: docs/todos/done/[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher.md
fix_pbi_document_id: PBI-FIX-BUNDLE-TELEGRAM-GATEWAY
friction_ids_post_ola: []
forge_branch: main
forge_head: 117e461
isolation_merge: fb12e07
telegram_gateway_merge: 0247e7e
wui_port: 8766
g5_executed: false
g_telegram_executed: true
kaizen_pbi_opened: false
fix_pbi_opened: false
preprod_vault_present: false
telegram_tokens_distinct: not_rehashed
telegram_chat_id_shared: not_rehashed
---

# Auditoría — Paciente 0 deploy ola 6 (20260826T120032Z)

Cicatriz de **esta** ola. No reescribe `docs/audits/paciente0-deploy-20260826T110203Z.md` (ola 5) ni audits T6.

## 1. Canal

```text
forja main (HEAD 117e461; merge telegram-gateway 0247e7e / PR #194)
  → vault staging SddIA_AP.deploy-vault (CONFIG_SOURCE + Filtro C)
  → build-release-bundle --skip-build OK L-BUNDLE-STALE (cicatriz SHA-256 8 ELF)
  → --out /home/racso/Proyectos/SddIA_AP  (wipe: instancia ausente)
  → instance-creator skip_ignition + pin SDDIA_EXECUTE_PROCESS_BIN=forja/release
  → start-sddia.sh env -u SDDIA_EXECUTE_PROCESS_BIN ; perfil consumer ; jurisdicción systemd
```

Instancia preexistente: **no**. Wipe canónico `--out INSTANCE_ROOT`. Vault: 9 ficheros copiados. Cero secretos en este archivo.

`PREPROD_VAULT` ausente. Constitución: staging previo + `meta.product=SddIA_AP`. Códice local: `codex-kalma2-assistant`.

Mitigaciones del prompt: pin release (F-DEP-07); `local.paths` ausente pre-creator (no `{}` → no unlink); ignición `env -u` (F-DEP-09); `start-sddia` sync+`enable --now` (F-SYS-01). Lab `email-watcher@…SddIA`: ya inactive (R-07). Lab `telegram-watcher@…SddIA`: **se deja active** (tokens distintos ola 5; no re-huella). Legado `sddia-daemon@telegram-watcher`: inactive. Linger: yes.

`--skip-build` **OK** (delta vs ola 5: entonces L-BUNDLE-STALE KO → rebuild). Aviso no bloqueante: `Input '$PWD' is not an absolute file system path`.

## 2. Gates §5

| ID | Ola actual | Nota |
|----|------------|------|
| G0-config | APTO | `vault_env_present`; claves IMAP/LLM/Telegram (nombres); `AGENT_RUNTIME_*` instancia = 0 (raíz `.dev` = 5; Filtro C) |
| G-bundle | APTO | 0 `.rs`; `PY_LEAK=no`; `MANIFEST.json` `20260826T115731Z`; **8 ELF / 8 cápsulas**; `filtro_c=true`; **ELF+`.md` `telegram-gateway`**; sin ELF `github-bridge-watcher` (scripts `.sh`/`.md` genoma residuales, no binario) |
| G1 | APTO | `.SddIA/`; `.events/{domain,orchestration,telemetry,pending}/`; `local.paths.json` no `{}` (`directories`/`files`/`paths`) |
| G2 | APTO | `constitution.json` `meta.product=SddIA_AP`; códice `codex-kalma2-assistant.md` |
| G3 | APTO | HTTP 200 `:8766`; 0 `cargo build`; journal watcher: `route-domain` enruta/purga; heartbeats OK |
| G3b | APTO | `sddia-{event-watcher,event-sweeper,kalma2-bridge,email-watcher,telegram-watcher}@%f` active, NRestarts=0; `WorkingDirectory=%f`; `ExecStart=%f/SddIA/…`. github-bridge AP: inactive/disabled |
| G4 | APTO | sin ELF github-bridge; instancia sin `AGENT_RUNTIME_*`; códices locales sin `codex-software-engineering` |
| G-orch | APTO | creator pin release forja (`cid=79157722-…`); ignición ELF bundle `…/SddIA_AP/SddIA/target/release/execute-process`. cwd PIDs = instancia |
| G5 | no auditado | opt-out redeploy rutinario (DA-5) |
| G-telegram | APTO | `telegram-gateway` instancia: `success:true` `emitted:true` `event_type=Manual_Task_Requested` + sello `TelegramMessage_Received`. Journal watcher **desde 13:57**: 0 `gateway rc=1`, 0 `409` |
| F-SMOKE-01 | no emitido | smoke `local_qa_emitted: false` |

Lanzador de `event-watcher` resolvió `target/debug/event-watcher` bajo instancia; mismo size/mtime que `target/release` (copia dual del bundle). No es ELF forja ni stale. Residual de path F-DEP-07.

## 3. Matriz vs Kaizen baseline (`PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825`)

Baseline en `docs/todos/done/`. Ola 5 previa: `AUDIT-PACIENTE0-DEPLOY-20260826T110203Z` OLA-MEJORA gates / F-BUNDLE-06 abierto post-ola.

| ID | Baseline / ola 5 | Ola actual | Delta |
|----|------------------|------------|-------|
| F-DEP-01 CORE_ROOT | APTO `ExecStart=%f` | APTO `ExecStart=%f` | igual |
| F-DEP-02 cargo | APTO | APTO 0 cargo | igual |
| F-DEP-03 skip-build / PY | APTO (ola 5: rechazo stale + rebuild) | APTO `--skip-build` cicatriz OK; `PY_LEAK=no` | mejoró |
| F-DEP-04 overlay | APTO path ausente | APTO wipe + primer creator | igual |
| F-DEP-07 debug-first | residual; pin obligatorio | pin aplicado; daemon path `debug/` = mismo ELF release | igual (mitigación prompt) |
| F-DEP-08 stub `{}` | unlink ad-hoc / N/A | N/A (fichero ausente) | igual residual Core |
| F-DEP-09 env ignición | APTO `orquestador (bundle)` | APTO `env -u` + bundle instancia | igual |
| F-DEP-05 vault | CONFIG_SOURCE completo | igual; preprod ausente | igual |
| F-SMOKE-01 | no emisión | no emisión | igual |
| F-SYS-01 | enable vía start-sddia | `enable --now` en ignición | igual |
| F-SYS-02 / F-DEP-10 / F-CEN-PKILL | APTO ola 5 | APTO NRestarts=0; cwd/exe instancia; WUI `:8766` | igual |
| **F-BUNDLE-06** | **NO APTO** post-ola 5; cerrado en forja PR #194 | **APTO** bundle 8 cápsulas + G-telegram CLI | **mejoró** |
| F-TRIAGE-01…03 | no este ciclo | G5 no ejecutado | igual |
| G0–G4 / G3b / G-orch / G-bundle | APTO | APTO (G-bundle ahora 8 ELF) | mejoró |
| G-telegram | NO APTO ola 5 | APTO | mejoró |

## 4. Veredicto §6.3

**OLA-MEJORA.** Gates G0–G4 / G3b / G-orch / G-bundle / G-telegram APTO. Ninguna regresión F-DEP-01…04. F-BUNDLE-06 **cerrado en runtime** Paciente 0 (bundle post-PR #194, sin copia manual ELF). Fricciones residuales del Kaizen 20260825 (pin orquestador aún en prompt, stub `{}`) no se duplican en PBI nuevo.

Instancia viva: `http://127.0.0.1:8766/`.

## 5. Seguimiento §8 (PBI)

No Kaizen redeploy (`OLA-MEJORA`; residuales F-DEP-07/08 ya en `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825`). No FIX nuevo.

SSOT residual deploy: `docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md`. F-BUNDLE-06: `docs/todos/done/[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher.md`.

## 6. Post-ola — G-telegram (esta ola, no diferido)

| Canal | Resultado |
|-------|-----------|
| CLI `telegram-gateway` `--inputs '{"text":"ola6-gate ping"}'` (ELF instancia) | **APTO** — `emitted:true`; `telegram_message_received_id=23a5f956-…` |
| Watcher AP journal desde ignición | **APTO** — bucle keepalive; 0 `gateway rc=1` |
| Mensaje humano al bot → respuesta Tormentosa | **no auditado** (DA-5; CLI cubre cápsula; estímulo operador opcional) |
| Correo → `send-telegram-notification` | **no re-auditado** (APTO ola 5; cápsula sigue en bundle) |

Tokens: no re-huella (clasificador). Claves `TELEGRAM_*` presentes por nombre en bóveda instancia. Lab telegram-watcher forja permanece active.

## 7. Qué no se hizo

- Proceso Core `paciente0-deploy` (DA-2; semilla solo).
- G5 reunión sintético/IMAP.
- Overlay rsync (no había periféricos; wipe).
- Forja de genoma en este estímulo.
- Restaurar `PREPROD_VAULT` (ausente); constitución desde staging ola 5.
- Mensaje Telegram humano post-ignición (G-telegram vía CLI).
- Re-huella SHA-12 de tokens.
