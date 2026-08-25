---
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
uuid: "d4f13e9a-5d91-4ab8-a2f5-be2e6b8c4815"
title: "[KAIZEN] Paciente 0 (SddIA_AP): Redeploy 2026-08-25 post-absorción y fricciones residuales"
format: markdown
version: "1.0.0"
status: done
type: kaizen
priority: alta
process: feature
persist_ref: docs/features/kaizen-paciente0-redeploy-20260825
created: "2026-08-25"
updated: "2026-08-25"
pbi_archived: true
derived_from: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
antecesor_persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
antecesor_audit_ref: docs/audits/kaizen-paciente0-redeploy-20260825.md
redeploy_executed_at: "2026-08-25T12:01:32Z"
instance_creator_stale_correlation_id: "aacbcbb9-43b9-4793-91fb-cac8b14cc214"
instance_creator_release_correlation_id: "c623d53e-99cd-485e-912a-f166537c4f25"
bundle_manifest: "20260825T120132Z"
forge_branch: main
wui_port: 8766
systemd_unit: "sddia-email-watcher@home-racso-Proyectos-SddIA_AP.service"
instance_path: /home/racso/Proyectos/SddIA_AP
config_source: /home/racso/Proyectos/.dev
preprod_vault: /home/racso/Proyectos/SddIA_AP.preprod-vault
deploy_vault_staging: /home/racso/Proyectos/SddIA_AP.deploy-vault
friction_ids:
  - F-DEP-07
  - F-DEP-08
  - F-DEP-09
  - F-DEP-05
  - F-SMOKE-01
  - F-SYS-01
tech_debt_ids:
  - DT-ORCHESTRATOR-DEBUG-FIRST
  - DT-LOCAL-PATHS-EMPTY-STUB
  - DT-IGNITION-ENV-ISOLATION
  - DT-SMOKE-ECST-LOCAL-QA
  - DT-SYSTEMD-USER-ENABLE
  - DT-CONFIG-UX-ONBOARDING
blocks_on: []
---

# [KAIZEN] Paciente 0 (SddIA_AP) — Redeploy 2026-08-25 y fricciones residuales

## 0. Contexto

Redeploy operativo de **Paciente 0** (`SddIA_AP`) el **2026-08-25** desde forja `main` (Kaizen `kaizen-paciente0-redeploy-fricciones` ya mergeado). Canal canónico: `build-release-bundle.sh` (perfil `consumer`) + `instance-creator` + ignición híbrida (`start-sddia.sh` + systemd `email-watcher@%f`).

**Configuración inyectada:** bóveda `/home/racso/Proyectos/.dev/.env` + merge staging en `SddIA_AP.deploy-vault` (Telegram y extras IMAP desde `SddIA_AP.preprod-vault` cuando faltaban en `.dev`). Constitución/códice desde preprod.

**Resultado:** instancia operativa (WUI `:8766` HTTP 200, EDA enrutando, systemd `ExecStart` bajo instancia). **No** se re-ejecutó Gate G5 de correo reunión (fuera de este estímulo; DA-5).

Estructura y umbral de cierre: antecesor `docs/features/kaizen-paciente0-redeploy-fricciones` + PBI `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824`. Este ciclo **no** reabre F-TRIAGE-*; documenta fricciones **nuevas** del redeploy post-absorción.

Referencias: `docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy y fricciones operativas.md`, `docs/audits/kaizen-paciente0-redeploy-20260825.md`, `SddIA/norms/sddia-distribution-protocol.md` v1.2.0.

---

## 0bis. Procedimiento ejecutado (bitácora operativa)

Secuencia real **2026-08-25** (forja `main` → instancia `/home/racso/Proyectos/SddIA_AP`). Tiempos UTC.

### Fase A — Materialización del bundle

| Paso | Acción | Resultado |
|------|--------|-----------|
| A1 | `build-release-bundle.sh --out /home/racso/Proyectos/SddIA_AP --codex codex-kalma2-assistant --profile consumer --skip-build` | **KO** L-BUNDLE-STALE: cicatriz `execute-process` divergente (source distinta, ELF igual). Gate F-DEP-03 **APTO** (fail-closed). |
| A2 | Mismo comando **sin** `--skip-build` | OK ~21 s; `MANIFEST.json` `20260825T120132Z` |
| A3 | Gate integridad | 0 `.rs`; `PY_LEAK=no`; 7 binarios + 7 cápsulas |

**Binarios:** `execute-process`, `kalma2-bridge`, `event-watcher`, `event-sweeper`, `email-watcher`, `telegram-watcher`, `send-telegram-notification`. **Excluido** (Filtro C): `github-bridge-watcher`.

### Fase B — Vault staging e `instance-creator`

| Paso | Acción | Resultado |
|------|--------|-----------|
| B1 | Staging `SddIA_AP.deploy-vault/` | `root.dev.env` ← `.dev/.env`; instancia = `.dev` − `AGENT_RUNTIME_*` + perfil consumidor |
| B2 | Fill preprod si ausente | `TELEGRAM_*`, `SDDIA_EMAIL_{SNIPPET,LOOKBACK,MAX_UIDS}` |
| B3 | `constitution/` + `codexes/` | Desde `SddIA_AP.preprod-vault` |
| B4 | `./sddia-run.sh --process instance-creator` (sin pin ELF) | `success:true`, `cid=aacbcbb9-…`, `vault_files_copied=6` — **ELF debug stale** (F-DEP-07) |
| B5 | Overlay / systemd tras B4 | `local.paths.json={}` (F-DEP-04 reaparecido); `ExecStart` → **lab** `/home/racso/Proyectos/SddIA/SddIA/daemons/email-watcher.sh` (F-DEP-01 reaparecido) |
| B6 | Unlink stub `{}` + reinyección con `SDDIA_EXECUTE_PROCESS_BIN=…/release/execute-process` | `cid=c623d53e-…`; overlay starter-kit; `ExecStart` bajo `SddIA_AP` |

**Inputs JSON (B4/B6):**

```json
{
  "instance_root": "/home/racso/Proyectos/SddIA_AP",
  "runtime_profile": "consumer",
  "vault_source": "/home/racso/Proyectos/SddIA_AP.deploy-vault",
  "skip_ignition": true
}
```

**Causa B4:** `_sddia_resolve_orchestrator` prueba `target/debug/execute-process` **antes** que `release`. Debug del **2026-08-24 19:41** (pre-Kaizen). Release del **2026-08-25 14:01** (bundle fresco).

### Fase C — Overlay y systemd (operador)

| Paso | Acción | Resultado |
|------|--------|-----------|
| C1 | Copiar unidad renderizada → `~/.config/systemd/user/` | Obligatorio: creator no hace `enable` (F-SYS-01) |
| C2 | `daemon-reload` + `enable --now` `%f` AP | Tras B6: `active`; WD=`SddIA_AP`; ExecStart instancia |
| C3 | Lab `email-watcher@…SddIA` | Ya `inactive` (R-07) |

### Fase D — Ignición núcleo (script)

| Paso | Acción | Resultado |
|------|--------|-----------|
| D1 | `nohup ./start-sddia.sh` (perfil `consumer`, jurisdicción `systemd`) | Log: **0** `cargo build`; orquestador bundle-safe |
| D2 | WUI + heartbeats | HTTP 200 `:8766`; `event-watcher` / `event-sweeper` / `kalma2-bridge` ACTIVO |
| D3 | R-07 | `email-watcher` y `telegram-watcher` omitidos del script (systemd) |
| D4 | Orquestador resuelto | Forge `…/SddIA/target/release/execute-process` por **fuga** de `SDDIA_EXECUTE_PROCESS_BIN` del shell operador (F-DEP-09). Centinelas sí bajo `{instancia}/SddIA/target/` |

### Fase E — Absorción F-DEP-05 en `.dev` (post-ignición, este PBI § operador)

Claves que el staging rellenó desde preprod se incorporan a `/home/racso/Proyectos/.dev/.env` para que el próximo redeploy no dependa del merge preprod: `TELEGRAM_*`, `SDDIA_EMAIL_{SNIPPET,LOOKBACK,MAX_UIDS}`, `SDDIA_RUNTIME_PROFILE=consumer`, `SDDIA_SENSORIAL_JURISDICTION=systemd`. `AGENT_RUNTIME_*` permanece en raíz (D11 / Filtro C: se poda solo en bóveda **instancia**).

### Identidad instancia materializada

| Campo | Valor |
|-------|--------|
| `product` / `workspace_id` | `SddIA_AP` / `sddia-ap-paciente-0` |
| Constitución | `.SddIA/constitution/` (preprod) |
| Códice | `.SddIA/library/codexes/codex-kalma2-assistant.md` |
| WUI | `http://127.0.0.1:8766/` |
| Bus runtime | `./.events/{domain,orchestration,telemetry,pending}/` |

---

## 0ter. Métricas e indicadores (snapshot)

**Timestamp snapshot:** `2026-08-25T12:04:00Z` (post-ignición).

| Métrica | Valor |
|---------|-------|
| Bundle `created_at` | `20260825T120132Z` |
| `.rs` en bundle | 0 |
| Binarios / cápsulas | 7 / 7 |
| `--skip-build` pre-rebuild | exit 1 (cicatriz) |
| Rebuild sin `--skip-build` | ~21 s |
| `instance-creator` (stale) | `aacbcbb9-…` `vault_files_copied=6` |
| `instance-creator` (release) | `c623d53e-…` `vault_files_copied=6` |
| WUI | HTTP 200 `:8766` |
| Systemd email-watcher@AP | `active` |
| `cargo build` en log ignición | 0 |
| `github-bridge-watcher` | ausente |
| `AGENT_RUNTIME_*` en instancia `.env` | 0 |

### Gates vs Kaizen absorbido (2026-08-24 / T6)

| Gate | T6 (release ELF) | Redeploy 12:01 (1.ª pasada debug) | Tras pin release |
|------|------------------|-----------------------------------|------------------|
| G1 Topología | APTO | **NO APTO** `local.paths.json={}` | APTO |
| G2 Códice | APTO | APTO (vault) | APTO |
| G3 WUI + EDA | APTO | No se ignitó sobre stub | APTO |
| systemd CORE_ROOT | APTO | **NO APTO** ExecStart lab | APTO |
| F-DEP-02 cargo | APTO | — | APTO (0 cargo) |
| F-DEP-03 skip-build | APTO | APTO (rechazo correcto) | — |
| G5 reunión | APTO (sintético) | No re-auditado | No re-auditado |

---

## 1. Fricciones de este redeploy

Las F-DEP-01…04 **siguen absorbidas en el handler release**. Reaparecieron porque el **CLI resolvió un ELF anterior al Kaizen**. No consolidar parches de instancia como SSOT.

| ID | Síntoma | Causa raíz | Corrección ad-hoc (2026-08-25) | Acción Kaizen |
|----|---------|------------|--------------------------------|---------------|
| **F-DEP-07** | Primera pasada creator: `ExecStart` lab + `local.paths.json={}` pese a Core absorbido | `_sddia_resolve_orchestrator` prefiere `target/debug/` si el ELF existe, aunque sea más viejo que `release` | `SDDIA_EXECUTE_PROCESS_BIN` → release; reinyectar creator | **DT-ORCHESTRATOR-DEBUG-FIRST:** preferir release, o el ELF cuya cicatriz `.sha256` coincida, o fallar si debug es más viejo que release |
| **F-DEP-08** | Reinyección release **no** reescribía overlay | `materialize_local_paths`: si el fichero existe, `return Ok` — no distingue stub `{}` | `unlink` del `{}` antes del 2.º creator | **DT-LOCAL-PATHS-EMPTY-STUB:** sustituir `{}` / overlay vacío; no tratar existencia como validez |
| **F-DEP-09** | Log ignición: orquestador = ELF **forja** release, no bundle instancia | Shell operador exportó `SDDIA_EXECUTE_PROCESS_BIN` (sesión creator); `start-sddia` / `_ensure_orchestrator` lo honran | Centinelas sí en instancia; orquestador de rutas = forja (mismo build) | **DT-IGNITION-ENV-ISOLATION:** `env -u SDDIA_EXECUTE_PROCESS_BIN` al ignitar instancia; o pin en bóveda instancia al ELF del bundle |
| **F-DEP-05** | Staging debió mergear Telegram/extras desde preprod | `.dev` sin inventario mínimo consumidor completo | Claves copiadas a `.dev` en este PBI (Fase E) | Inventario en `.dev` cerrado a nivel operador; wizard sigue `DT-CONFIG-UX-ONBOARDING` |
| **F-SMOKE-01** | `Local_QA_Requested` → dead-letter `missing required payload.branch` | Smoke nativo emite evento incompleto vs contrato ECST | Padres quedan en `pending/` (skip routed-ok) | **DT-SMOKE-ECST-LOCAL-QA:** payload ECST válido o no emitir clase de orquestación |
| **F-SYS-01** | `enable --now` y copia a `~/.config/systemd/user/` siguen siendo operador | Creator renderiza en `{instancia}/.SddIA/systemd/` y documenta enable diferido | Copia + restart manual | **DT-SYSTEMD-USER-ENABLE:** opcional `install_user_unit=true`; no bloquear skip_ignition |

### 1.1 Comportamiento esperado vs observado

| Circuito | Esperado (post-Kaizen T6) | Observado 1.ª pasada | Observado tras pin release |
|----------|---------------------------|----------------------|----------------------------|
| `instance-creator` CORE_ROOT | instancia | **lab** | instancia |
| `local.paths.json` | starter-kit | `{}` | starter-kit |
| `--skip-build` stale | exit 1 | exit 1 | — |
| `./start-sddia.sh` cargo | 0 | — | 0 |
| WUI `:8766` | HTTP 200 | — | 200 |
| systemd `%f` | WD + ExecStart instancia | ExecStart lab (unidad copiada) | instancia |
| Orquestador ignición | ELF bundle instancia | — | ELF forja (F-DEP-09) |

### 1.2 Correcciones manuales (no canónicas — deuda)

```text
1. build-release-bundle sin --skip-build (gate stale OK)
2. Vault staging .dev + preprod fill
3. instance-creator (debug stale) → unlink {} + creator con ELF release
4. cp unidad user + enable --now
5. start-sddia nohup (hereda SDDIA_EXECUTE_PROCESS_BIN de sesión)
6. Incorporar huecos vault → Proyectos/.dev
```

Prohibido consolidar 3–5 como SSOT. Absorber F-DEP-07/08/09 en Core (`sddia_shell_lib.sh`, `instance_creator.rs`, contrato de ignición).

---

## 2. Fricciones absorbidas que **no** regresionan (si ELF = genoma actual)

Verificado en 2.ª pasada (release 2026-08-25):

| ID previo | Estado |
|-----------|--------|
| F-DEP-01 CORE_ROOT | Handler release correcto; regresión solo con debug stale |
| F-DEP-02 cargo en bundle | Log ignición: 0 `cargo build` |
| F-DEP-03 ELF `.py` / skip-build | Gate rechazó skip-build; `PY_LEAK=no` |
| F-DEP-04 starter-kit | Materializa si el path **no** existe; F-DEP-08 cubre el hueco `{}` |
| F-TRIAGE-01/02 | Fuera de este redeploy (no G5) |

---

## 3. Objetivos de arquitectura (alcance Kaizen)

1. Resolución de orquestador: no preferir debug stale frente a release fresco (F-DEP-07).
2. Topología: stub `local.paths.json={}` no es overlay válido (F-DEP-08).
3. Ignición instancia: no heredar `SDDIA_EXECUTE_PROCESS_BIN` de la forja (F-DEP-09).
4. Smoke `Local_QA_Requested` ECST-completo o no emitir (F-SMOKE-01).
5. (Opcional) `instance-creator` instala unidad user systemd (F-SYS-01).
6. Inventario `.dev` consumidor: cerrado en Fase E a nivel operador; Core no auto-mergea bóvedas (laudo D2 del antecesor).

## 4. Criterios de cierre

### Despliegue motor

- [ ] `_sddia_resolve_orchestrator` no selecciona debug más viejo que release / cicatriz vigente.
- [ ] `instance-creator` reemplaza `local.paths.json` vacío `{}`.
- [ ] Redeploy Paciente 0 con **un** `instance-creator` (sin unlink ni pin ELF) deja ExecStart bajo `{instancia}/` y overlay no vacío.
- [ ] `start-sddia.sh` en instancia resuelve ELF del bundle aunque el entorno operador tenga `SDDIA_EXECUTE_PROCESS_BIN` de la forja.
- [ ] Smoke creator: no dead-letter por `payload.branch` ausente en `Local_QA_Requested` (o no emitir esa clase).

### Operador / bóveda

- [x] Huecos Telegram + extras IMAP + perfil consumer/systemd en `/home/racso/Proyectos/.dev/.env`.

### Fuera

- G5 correo reunión (antecesor T6).
- F-TRIAGE-03 inbox `passive`.
- Wizard `DT-CONFIG-UX-ONBOARDING`.

---

## 5. Orden sugerido de forja

```text
(1) F-DEP-07 resolver orquestador (debug vs release vs cicatriz)
(2) F-DEP-08 overlay {} no es no-op
(3) F-DEP-09 aislamiento env ignición instancia
(4) F-SMOKE-01 ECST Local_QA
(5) F-SYS-01 opcional install user unit
(6) Redeploy smoke Paciente 0 — un solo instance-creator
```

Mutación de genoma: proceso `feature` → `entity-manager` donde aplique. Handler `instance_creator.rs` y `sddia_shell_lib.sh` viven fuera de DA-2 / en scripts. UUID PBI: `d4f13e9a-5d91-4ab8-a2f5-be2e6b8c4815`.

Init lab (cuando se abra el ciclo): `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` + `SDDIA_EXECUTE_PROCESS_BIN` pin **release**. `feature_name`: `kaizen-paciente0-redeploy-20260825`. Rama: `feat/kaizen-paciente0-redeploy-20260825`.

---

## 6. Referencias

| Ref | Uso |
|-----|-----|
| `docs/features/kaizen-paciente0-redeploy-fricciones/` | Estructura / laudo antecesor (T6) |
| `docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy y fricciones operativas.md` | Bitácora 2026-08-24 |
| `docs/audits/kaizen-paciente0-redeploy-20260825.md` | Auditoría T6 |
| `SddIA/norms/sddia-distribution-protocol.md` | Vía C v1.2.0 |
| `SddIA/process/instance-creator.md` | Fases Topologia–Smoke |
| `SddIA/scripts/common/sddia_shell_lib.sh` | `_sddia_resolve_orchestrator` (F-DEP-07) |
| `/home/racso/Proyectos/SddIA_AP/` | Instancia redeploy 2026-08-25 |
| `/home/racso/Proyectos/.dev/.env` | Bóveda personalizada (Fase E) |
| `/home/racso/Proyectos/SddIA_AP.deploy-vault` | Vault staging |
| `/home/racso/Proyectos/SddIA_AP/MANIFEST.json` | `20260825T120132Z` |
| `/home/racso/Proyectos/SddIA_AP/.SddIA/daemons/logs/start-sddia.log` | Ignición |

## 7. Auditoría al cierre

Al finalizar el ciclo feature: documento bajo `paths.auditsPath` (`docs/audits`) con bitácora empírica de este redeploy, fricciones F-DEP-07… y qué quedó absorbido en Core. No duplicar el audit T6 `kaizen-paciente0-redeploy-20260825.md`.
