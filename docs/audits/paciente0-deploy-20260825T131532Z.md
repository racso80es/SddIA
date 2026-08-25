---
document_id: AUDIT-PACIENTE0-DEPLOY-20260825T131532Z
uuid: "3a4e3bb6-a15e-4105-8366-f4529c39e32b"
title: "Auditoría ola Paciente 0 — despliegue 20260825T131532Z"
created: "2026-08-25"
instance_path: /home/racso/Proyectos/SddIA_AP
bundle_manifest: "20260825T131532Z"
instance_creator_correlation_id: "bd5c7328-fc6b-4abc-a010-79cea1a1decf"
ola_verdict: OLA-ESTABLE
kaizen_baseline_document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
wave_matrix_ref: docs/audits/paciente0-deploy-20260825T131532Z.md
pbi_prompt_ref: docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
pbi_prompt_document_id: PBI-DT-PACIENTE0-DEPLOY-PROCESS
forge_branch: main
wui_port: 8766
g5_executed: false
kaizen_pbi_opened: false
---

# Auditoría — Paciente 0 deploy ola 4 (20260825T131532Z)

Cicatriz de **esta** ola. No reescribe `docs/audits/kaizen-paciente0-redeploy-20260825.md` ni el residual T6.

## 1. Canal

```text
forja main
  → vault staging SddIA_AP.deploy-vault (CONFIG_SOURCE + Filtro C; fill preprod si hueco)
  → build-release-bundle --skip-build KO L-BUNDLE-STALE (F-DEP-03 fail-closed)
  → rebuild SIN --skip-build --out /home/racso/Proyectos/SddIA_AP  (wipe: instancia ausente)
  → instance-creator skip_ignition + pin SDDIA_EXECUTE_PROCESS_BIN=forja/release
  → start-sddia.sh env -u SDDIA_EXECUTE_PROCESS_BIN ; perfil consumer ; jurisdicción systemd
```

Instancia preexistente: **no** (directorio ausente al inicio). Wipe canónico `--out INSTANCE_ROOT`. Vault: 6 ficheros copiados. Cero secretos en este archivo.

Mitigaciones del prompt aplicadas: pin release (F-DEP-07); `local.paths` ausente (no `{}` → no unlink); ignición `env -u` (F-DEP-09); `start-sddia` sync+`enable --now` (F-SYS-01). Lab `email-watcher@…SddIA`: inactive (R-07). Linger: yes.

## 2. Gates §5

| ID | Ola actual | Nota |
|----|------------|------|
| G0-config | APTO | `vault_env_present`; claves IMAP/LLM/Telegram en instancia (nombres); `AGENT_RUNTIME_*` = 0 |
| G-bundle | APTO | 0 `.rs`; `PY_LEAK=no`; `MANIFEST.json` `20260825T131532Z`; 7 ELF; `filtro_c=true`; **sin** ELF `github-bridge-watcher`. Scripts/md del daemon siguen en genoma inyectado (no es lanzador activo) |
| G1 | APTO | `.SddIA/`; `.events/{domain,orchestration,telemetry,pending}/`; `local.paths.json` no `{}` (`directories`/`files`/`paths`) |
| G2 | APTO | `constitution.json` `meta.product=SddIA_AP`; códice `codex-kalma2-assistant.md` |
| G3 | APTO | HTTP 200 `:8766`; 0 `cargo build`; journal watcher: `route-domain` enruta/purga; heartbeats OK |
| G3b | APTO | `sddia-{event-watcher,event-sweeper,kalma2-bridge,email-watcher}@%f` active; WD=`INSTANCE_ROOT`; ExecStart bajo instancia. Telegram `@%f` también active (token presente). github-bridge AP: inactive |
| G4 | APTO | sin ELF github-bridge; instancia sin `AGENT_RUNTIME_*`; códices locales sin `codex-software-engineering` |
| G-orch | APTO | creator pin release forja; ignición ELF bundle `…/SddIA_AP/SddIA/target/release/execute-process` |
| G5 | no auditado | opt-out redeploy rutinario (DA-5) |
| F-SMOKE-01 | no emitido | smoke `local_qa_emitted: false` |

Aviso no bloqueante: `build-release-bundle` imprimió `Input '$PWD' is not an absolute file system path` (escape systemd). `--out` absoluto; instancia correcta.

## 3. Matriz vs Kaizen baseline (`PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825`)

Baseline en `docs/todos/done/` (pending homónimo no existe). `friction_ids` baseline: F-DEP-07/08/09, F-DEP-05, F-SMOKE-01, F-SYS-01. Bundle baseline: `20260825T120132Z`. Creator release baseline: `c623d53e-…`.

| ID | Baseline | Ola actual | Delta |
|----|----------|------------|-------|
| F-DEP-01 CORE_ROOT | APTO con pin release | APTO ExecStart instancia | igual |
| F-DEP-02 cargo | APTO | APTO 0 cargo | igual |
| F-DEP-03 skip-build / PY | APTO (rechazo stale) | APTO rechazo + `PY_LEAK=no` | igual |
| F-DEP-04 overlay | APTO tras pin | APTO primer creator (path ausente) | mejoró (un solo creator; wipe) |
| F-DEP-07 debug-first | residual; pin obligatorio | pin aplicado; no se re-probó sin pin | igual (mitigación prompt) |
| F-DEP-08 stub `{}` | unlink ad-hoc | N/A (fichero ausente) | igual residual Core |
| F-DEP-09 env ignición | fuga pin forja | APTO `orquestador (bundle)` instancia | mejoró vs 12:01; alineado residual T6 |
| F-DEP-05 vault | cerrado en `.dev` | fill innecesario; claves en staging | igual |
| F-SMOKE-01 | DLQ `payload.branch` | no emisión `Local_QA_Requested` | igual/absorbido residual |
| F-SYS-01 | enable vía start-sddia / operador | `enable --now` en ignición | igual |
| F-TRIAGE-01…03 | no este ciclo | G5 no ejecutado | igual |
| G0–G4 / G3b / G-orch / G-bundle | APTO post-pin | APTO | igual |

## 4. Veredicto §6.3

**OLA-ESTABLE.** Gates G0–G4 / G3b / G-orch / G-bundle APTO con las mitigaciones del prompt. Ninguna regresión F-DEP-01…04 con ELF release. Fricciones residuales del Kaizen 20260825 (pin orquestador, stub `{}`, F-SYS-01 no en creator) no se duplican en PBI nuevo. Instancia viva: `http://127.0.0.1:8766/`.

## 5. Kaizen §8

No se abre PBI pending. SSOT residual: `docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md`.

## 6. Qué no se hizo

- Proceso Core `paciente0-deploy` (DA-2; semilla solo).
- G5 reunión sintético/IMAP.
- Overlay rsync (no había periféricos que preservar).
- Forja de genoma.
