---
document_id: AUDIT-PACIENTE0-DEPLOY-20260826T110203Z
uuid: "eb367243-8607-47f0-8877-692d63555c89"
title: "Auditoría ola Paciente 0 — despliegue 20260826T110203Z"
created: "2026-08-26"
instance_path: /home/racso/Proyectos/SddIA_AP
bundle_manifest: "20260826T110203Z"
instance_creator_correlation_id: "a2a506c6-86e3-4396-a0b8-91d72ec50cda"
ola_verdict: OLA-MEJORA
deploy_gates_verdict: APTO
post_ola_operational_verdict: F-BUNDLE-06_ABIERTO
kaizen_baseline_document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
wave_matrix_ref: docs/audits/paciente0-deploy-20260826T110203Z.md
pbi_prompt_ref: docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
pbi_prompt_document_id: PBI-DT-PACIENTE0-DEPLOY-PROCESS
fix_pbi_ref: docs/todos/pending/[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher.md
fix_pbi_document_id: PBI-FIX-BUNDLE-TELEGRAM-GATEWAY
friction_ids_post_ola:
  - F-BUNDLE-06
forge_branch: main
forge_head: bffdbeb
isolation_merge: fb12e07
wui_port: 8766
g5_executed: false
g_telegram_executed: true
kaizen_pbi_opened: false
fix_pbi_opened: true
preprod_vault_present: false
telegram_tokens_distinct: true
telegram_chat_id_shared: true
---

# Auditoría — Paciente 0 deploy ola 5 (20260826T110203Z)

Cicatriz de **esta** ola. No reescribe `docs/audits/kaizen-paciente0-redeploy-20260825.md`, el residual T6, ni `docs/audits/paciente0-centinelas-email-sordo-20260826.md`.

## 1. Canal

```text
forja main (HEAD bffdbeb; aislamiento multi-instancia merge fb12e07)
  → vault staging SddIA_AP.deploy-vault (CONFIG_SOURCE + Filtro C)
  → build-release-bundle --skip-build KO L-BUNDLE-STALE (F-DEP-03 fail-closed; source digest diverge, ELF hash igual)
  → rebuild SIN --skip-build --out /home/racso/Proyectos/SddIA_AP  (wipe: instancia ausente)
  → instance-creator skip_ignition + pin SDDIA_EXECUTE_PROCESS_BIN=forja/release
  → start-sddia.sh env -u SDDIA_EXECUTE_PROCESS_BIN ; perfil consumer ; jurisdicción systemd
```

Instancia preexistente: **no** (directorio ausente; árbol perdido durante el ciclo de aislamiento). Wipe canónico `--out INSTANCE_ROOT`. Vault: 9 ficheros copiados. Cero secretos en este archivo.

`PREPROD_VAULT` ausente. Constitución: starter-kit + `meta.product=SddIA_AP`. Códice local: `codex-kalma2-assistant` desde forja `library/codexes/`.

Mitigaciones del prompt aplicadas: pin release (F-DEP-07); `local.paths` ausente (no `{}` → no unlink); ignición `env -u` (F-DEP-09); `start-sddia` sync+`enable --now` (F-SYS-01). Lab `email-watcher@…SddIA`: **estaba active** → stop/disable (R-07). Legado `sddia-daemon@telegram-watcher`: stop/disable. Linger: yes.

## 2. Gates §5

| ID | Ola actual | Nota |
|----|------------|------|
| G0-config | APTO | `vault_env_present`; claves IMAP/LLM en instancia (nombres); `AGENT_RUNTIME_*` instancia = 0 (raíz `.dev` conserva 5; Filtro C) |
| G-bundle | APTO | 0 `.rs`; `PY_LEAK=no`; `MANIFEST.json` `20260826T110203Z`; 7 ELF; `filtro_c=true`; **sin** ELF `github-bridge-watcher` |
| G1 | APTO | `.SddIA/`; `.events/{domain,orchestration,telemetry,pending}/`; `local.paths.json` no `{}` (`directories`/`files`/`paths`) |
| G2 | APTO | `constitution.json` `meta.product=SddIA_AP`; códice `codex-kalma2-assistant.md` |
| G3 | APTO | HTTP 200 `:8766`; 0 `cargo build`; journal watcher: `route-domain` enruta/purga; heartbeats OK |
| G3b | APTO | `sddia-{event-watcher,event-sweeper,kalma2-bridge,email-watcher}@%f` active, NRestarts=0; `WorkingDirectory=%f`; `ExecStart=%f/SddIA/…`. Telegram `@%f` active. github-bridge AP: inactive/disabled |
| G4 | APTO | sin ELF github-bridge; instancia sin `AGENT_RUNTIME_*`; códices locales sin `codex-software-engineering` |
| G-orch | APTO | creator pin release forja (`cid=a2a506c6-…`); ignición ELF bundle `…/SddIA_AP/SddIA/target/release/execute-process`. cwd PIDs = instancia |
| G5 | no auditado | opt-out redeploy rutinario (DA-5) |
| F-SMOKE-01 | no emitido | smoke `local_qa_emitted: false` |

Aviso no bloqueante: `build-release-bundle` imprimió `Input '$PWD' is not an absolute file system path` (escape systemd). `--out` absoluto; instancia correcta.

Lanzador de `event-watcher` resolvió `target/debug/event-watcher` bajo instancia; mismo size/mtime que `target/release` (copia dual del bundle). No es ELF forja ni stale.

## 3. Matriz vs Kaizen baseline (`PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825`)

Baseline en `docs/todos/done/`. `friction_ids` baseline: F-DEP-07/08/09, F-DEP-05, F-SMOKE-01, F-SYS-01. Bundle baseline: `20260825T120132Z`. Ola 4 previa: `20260825T131532Z` OLA-ESTABLE.

Contraste adicional (no en baseline 20260825; audit sordo 20260826 + Kaizen aislamiento mergeado): F-SYS-02, F-DEP-10, F-CEN-PKILL.

| ID | Baseline | Ola actual | Delta |
|----|----------|------------|-------|
| F-DEP-01 CORE_ROOT | APTO con pin / ExecStart instancia | APTO `ExecStart=%f` (molde universal) | mejoró |
| F-DEP-02 cargo | APTO | APTO 0 cargo | igual |
| F-DEP-03 skip-build / PY | APTO (rechazo stale) | APTO rechazo + `PY_LEAK=no` | igual |
| F-DEP-04 overlay | APTO tras pin | APTO primer creator (path ausente) | igual |
| F-DEP-07 debug-first | residual; pin obligatorio | pin aplicado; Core ya elige release si debug no es más nuevo; daemon `event-watcher` path `debug/` = mismo ELF release | igual (mitigación prompt; residual de path) |
| F-DEP-08 stub `{}` | unlink ad-hoc | N/A (fichero ausente) | igual residual Core |
| F-DEP-09 env ignición | fuga pin forja | APTO `orquestador (bundle)` instancia; Core descarta pin ajeno | mejoró (absorbido + `env -u`) |
| F-DEP-05 vault | cerrado en `.dev` | preprod ausente; CONFIG_SOURCE completo (profile/juris/port ya en `.dev`) | igual |
| F-SMOKE-01 | DLQ `payload.branch` | no emisión `Local_QA_Requested` | igual |
| F-SYS-01 | enable vía start-sddia | `enable --now` en ignición | igual |
| F-SYS-02 plantilla user | no en baseline; NO APTO 20260826 a.m. | `~/.config/systemd/user/sddia-*.service` `ExecStart=%f`; `@AP` y `@forja` mismo molde, `%f` distinto | mejoró |
| F-DEP-10 SCRIPT_DIR | no en baseline; NO APTO 20260826 a.m. | cwd/exe bajo `SddIA_AP`; inbox `:8766` ≠ `:8765` | mejoró |
| F-CEN-PKILL | no en baseline; crash-loop a.m. | NRestarts=0 | mejoró |
| F-TRIAGE-01…03 | no este ciclo | G5 no ejecutado | igual |
| G0–G4 / G3b / G-orch / G-bundle | APTO post-pin | APTO | igual |

## 4. Veredicto §6.3

**OLA-MEJORA.** Gates G0–G4 / G3b / G-orch / G-bundle APTO. Ninguna regresión F-DEP-01…04 con ELF release. F-SYS-02 / F-DEP-10 / F-CEN-PKILL (sordos 20260826) quedan **APTO en runtime** de Paciente 0 tras merge `kaizen-aislamiento-multi-instancia` + este wipe. Inbox WUI de instancia distinto del de forja. Fricciones residuales del Kaizen 20260825 (pin orquestador aún en prompt, stub `{}`) no se duplican en PBI nuevo.

Instancia viva: `http://127.0.0.1:8766/`.

## 5. Seguimiento §8 (PBI)

No Kaizen redeploy Paciente 0 (`OLA-MEJORA` en gates; residuales F-DEP-07/08 ya en Kaizen 20260825).

**FIX lab cerrado (forja):** `PBI-FIX-BUNDLE-TELEGRAM-GATEWAY` — PR [#194](https://github.com/racso80es/SddIA/pull/194); `build-release-bundle.sh` empaqueta `telegram-gateway` con gate F-BUNDLE-06. **Pendiente:** redeploy Paciente 0 con bundle nuevo (sin copia manual ELF).

SSOT residual deploy: `docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md`. Aislamiento: `docs/todos/done/[KAIZEN] aislamiento multi-instancia centinelas.md`.

## 6. Post-ola — empiría sensorial Telegram (2026-08-26)

Validación operadora **después** de gates §5. Cero secretos (tokens: huella SHA-12, no valores).

### 6.1 Configuración bóveda

| Clave | Lab forja (`.SddIA/.dev/.env`) | Paciente 0 (`.SddIA/.dev/.env`) |
|-------|-------------------------------|--------------------------------|
| `TELEGRAM_BOT_TOKEN` | `sha12=0b6b4b…` (bot **lab**) | `sha12=6f534a…` (bot **Paciente 0**) — **distintos** |
| `TELEGRAM_ALLOWED_CHAT_ID` | `1077…0983` | `1077…0983` — **mismo chat operador** |

Coexistencia `sddia-telegram-watcher@…SddIA` + `@…SddIA_AP` **no** implica `409 getUpdates` entre bots (tokens distintos). El `409` del journal temprano (08:03) fue competencia sobre **un** bot o legado; no el caso nominal lab+AP con tokens separados.

### 6.2 Canales Telegram — resultado

| Canal | Bot | Cápsulas críticas | Resultado |
|-------|-----|-------------------|-----------|
| Correo → notificación (`Email_Triaged` → `send-telegram-notification`) | Paciente 0 | `email-watcher`, `send-telegram-notification` ✓ bundle | **APTO** — empírico operador |
| Mensaje escrito («sigues?») → Tormentosa | Paciente 0 | `telegram-watcher` ✓, **`telegram-gateway` ✗** | **NO APTO** — `gateway rc=1` |

Watcher AP recibió update (`update_id=551975545`); falló `execute-process --process telegram-gateway` → tool capsule ausente bajo `SddIA/target`.

### 6.3 Fricción post-ola

| ID | Estado | Nota |
|----|--------|------|
| **F-BUNDLE-06** | **CERRADO (forja)** | Fix PR #194: bundle 8 capsules incluye `telegram-gateway`; redeploy AP pendiente |
| F-SYS-02 / F-DEP-10 / F-CEN-PKILL | APTO | cwd/exe bajo `SddIA_AP`; inbox WUI `:8766` ≠ `:8765` |
| Stop `sddia-daemon@telegram-watcher` (legado lab) | higiene | Bot lab; **no** causa del silencio Paciente 0 |

### 6.4 Veredicto compuesto ola 5

| Ámbito | Veredicto |
|--------|-----------|
| Gates despliegue §5 (infra, EDA, WUI, systemd) | **OLA-MEJORA** |
| Sensorial Telegram conversacional post-ola | **Pendiente redeploy AP** (fix forja mergeado; validar G-telegram tras bundle nuevo) |

## 7. Qué no se hizo

- Proceso Core `paciente0-deploy` (DA-2; semilla solo).
- G5 reunión sintético/IMAP.
- Overlay rsync (no había periféricos; wipe).
- Forja de genoma en este estímulo (FIX documentado para ciclo `bug-fix`).
- Restaurar `PREPROD_VAULT` (ausente); constitución reconstruida desde starter-kit.
- Redeploy Paciente 0 con bundle post-PR #194 (G-telegram / CA4–CA5).
