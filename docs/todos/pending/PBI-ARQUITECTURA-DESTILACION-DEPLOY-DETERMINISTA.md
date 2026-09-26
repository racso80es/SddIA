---
document_id: PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA
uuid: "d239cb31-a937-448c-941e-3808a3a28874"
title: "[ARQUITECTURA] Destilación normativa del installer determinista (deploy + teardown)"
format: markdown
version: "1.2.0"
status: pending
priority: alta
type: arquitectura
process: feature
feature_name: destilacion-sddia-installer
branch_name: feat/destilacion-sddia-installer
persist_ref: docs/features/destilacion-sddia-installer
pbi_antecesor: PBI-ARQUITECTURA-DEPLOY-DETERMINISTA
pbi_antecesor_uuid: "c154bea0-c4c1-457e-b070-f7dfd3bc5f1b"
antecesor_pr: "https://github.com/racso80es/SddIA/pull/300"
antecesor_merge: "c26e2a2862fcfb5e0a689efd58f586aa6d36d941"
antecesor_persist_ref: docs/features/sddia-deterministic-installer
companion_teardown_feature: sddia-deterministic-teardown
companion_teardown_persist_ref: docs/features/sddia-deterministic-teardown
companion_teardown_branch: feat/sddia-deterministic-teardown
related:
  - docs/todos/done/PBI-ARQUITECTURA-DEPLOY-DETERMINISTA.md
  - docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
  - docs/todos/pending/[DEUDA] Paciente 0 — prompt de teardown.md
  - docs/features/sddia-deterministic-installer
  - docs/features/sddia-deterministic-teardown
  - SddIA/norms/sddia-distribution-protocol.md
  - SddIA/process/instance-creator.md
  - SddIA/scripts/sddia-installer.sh
---

# [ARQUITECTURA] Destilación normativa del installer determinista (deploy + teardown)

Naturaleza: contrato de infraestructura **bilateral** (montaje y desmontaje). **No** reimplementa el orquestador: `deploy` y `teardown` comparten motor (`SddIA/scripts/sddia-installer.sh`), misma resolución de ROOT y Ceguera de Ejecución. Destila invariantes físicos a norma Core; no sustituye los prompts Paciente 0 (deploy v1.7.0 / undeploy v1.0.0) como ley del installer default.

## 0. Filtro A — errata de la semilla v1.0.0

| Afirmación v1.0.0 | Veredicto | Hecho |
|-------------------|-----------|--------|
| Relacionado `…despliegue_2.md` | **Alucinación de path.** | SSOT deploy: `docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md` (`PBI-DT-PACIENTE0-DEPLOY-PROCESS` v1.7.0). SSOT teardown: `docs/todos/pending/[DEUDA] Paciente 0 — prompt de teardown.md` (`PBI-DT-PACIENTE0-UNDEPLOY-PROCESS` v1.0.0, uuid `dddee1ff-aeac-400b-85df-70374d37c45d`). |
| `BINS_CONSUMIDOR` = 13 ELF como constante del installer | **Incoherencia de perfil.** | Installer deploy usa `--profile full-node` + `instance-creator` `runtime_profile:engineering`. 13 ELF + Filtro C = canal **Paciente 0 consumer** (`SddIA_AP`), no el default del artefacto. Teardown **no** re-audita bundle. |
| `INSTANCE_ROOT` única | **Incoherencia espacial.** | Cuatro coordenadas: last-resort installer `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`; Paciente 0 `/home/racso/Proyectos/SddIA_AP`; atajos `/home/racso/Aplicaciones/SddIA/SddIA_Deploy.sh` y `SddIA_Teardown.sh` → forja `…/Proyectos/SddIA/sddia-installer.sh`. Precedencia: `--root` > env/`vault` `SDDIA_INSTALL_ROOT` > last-resort. |
| `SENSORIAL` como parámetro systemd | **Inexacto.** | Clave `SDDIA_SENSORIAL_JURISDICTION` (típ. `systemd`). |
| Gates G-bundle…G3b «que el script debe auditar» | **Colisión con Ceguera (deploy).** | Installer no interpreta HTTP/olas/LLM. G* = auditoría de **ola** Paciente 0, no post-condición de `deploy`. |
| Verificar `:8766` / `curl` en teardown del installer | **Colisión con Ceguera (teardown).** | Deuda undeploy §5 exige comprobación de puerto WUI; el motor determinista **no** hace poll HTTP. Predicado normativo destilable: `! -e ROOT` + unidades `@${ESC}` inactive/unknown + cero `/proc` cwd/exe ∈ ROOT. |
| `G-authority` del prompt deploy | **Alucinación de catálogo.** | No está en v1.7.0; es gate de audit ola 10. |
| IOTA `skipped-config-missing` como ley del installer | **Exceso / perfil cruzado.** | G-dlt **consumer**; teardown no deshabilita por ausencia de IOTA. |
| Purga: mover prompts de deuda a `done/` | **Exceso.** | Ola 11 deploy + `paciente0-*` no forjados. Destilar ≠ archivar. |
| «Matriz tridimensional» | **Alucinación ontológica.** | No existe en Cúmulo. |
| Forjar `SddIA/norms/…` a mano | **DA-2.** | Cadena `entity-manager` / laudo Core. |
| Segundo binario `sddia-undeploy` | **Prohibido en ciclo installer.** | `teardown` = subcomando del mismo motor. `paciente0-undeploy` = candidato proceso Core futuro (deuda), no duplicar CLI. |
| Default teardown = `SddIA_AP` | **Incoherencia con deploy PR #300.** | Motor default = last-resort Aplicaciones. Wipe `SddIA_AP` solo con `--root` explícito. |

## 1. Intención

SSOT normativo **declarativo** para **ambos** sentidos del ciclo físico:

| Sentido | Comando | Semilla de deuda | Motor (empiria) |
|---------|---------|------------------|-----------------|
| Montaje | `deploy` | `PBI-DT-PACIENTE0-DEPLOY-PROCESS` v1.7.0 | PR #300 + `sddia-deterministic-installer` |
| Desmontaje | `teardown` | `PBI-DT-PACIENTE0-UNDEPLOY-PROCESS` v1.0.0 | `feat/sddia-deterministic-teardown` (endurecimiento motor + smoke; atajo host fuera de git) |

El contrato normativo futuro (`sddia-installer-contract`) debe tener **dos capítulos simétricos**: `deploy` (I-DEP-*) y `teardown` (I-TEAR-*), sin mezclar gates G3 HTTP con `teardown`.

## 2. Cuatro jurisdicciones (no mezclar)

| Canal | Comando | Perfil bundle (solo deploy) | ROOT típico | Gates G* / verificación HTTP |
|-------|---------|----------------------------|-------------|------------------------------|
| **Installer default** | `deploy` / `teardown` | `full-node` / n/a | last-resort Aplicaciones o `--root` | No |
| **Paciente 0** | prompt manual / futuro `paciente0-*` | `consumer` (deploy) | `/home/racso/Proyectos/SddIA_AP` | Sí (ola); teardown §5 manual |
| **Atajo Deploy** | `SddIA_Deploy.sh` | delega `deploy` | igual installer | No |
| **Atajo Teardown** | `SddIA_Teardown.sh` | delega `teardown --force` | igual installer | No |

Atajos bajo `/home/racso/Aplicaciones/SddIA/`; no versionados en forja.

## 3. Superficie a destilar

### 3.1 Deploy — invariantes I-DEP-* (PR #300)

| ID | Hecho medible |
|----|----------------|
| I-DEP-CLI | `deploy`. Flags: `--root`, `--vault`, `--force`, `--skip-build`, `--dry-run`. Cero prompts. |
| I-DEP-PATH | Abort ROOT forja o bajo forja (exit 1). |
| I-DEP-LIVE | Vivo + deploy sin `--force` → exit **2**. |
| I-DEP-BUNDLE | `build-release-bundle --out ROOT --profile full-node`. |
| I-DEP-CREATOR | `instance-creator` `skip_ignition:true` `runtime_profile:engineering`. |
| I-DEP-SYS | `enable --now sddia-<daemon>@${ESC}` si launcher existe. |
| I-DEP-HOST | `systemctl`, `systemd-escape`; sin `--skip-build`: `cargo`. |

### 3.2 Teardown — invariantes I-TEAR-* (deuda + motor endurecido)

| ID | Hecho medible | Origen deuda §3 |
|----|----------------|-----------------|
| I-TEAR-CLI | `teardown`. Flags: `--root`, `--force`, `--dry-run`. Sin `--vault`/`--skip-build`. | — |
| I-TEAR-PATH | Abort ROOT vacío, `/`, `$HOME`, forja o bajo forja (exit 1). Misma precedencia ROOT que deploy. | §4 `rm` guard |
| I-TEAR-FORCE | Sin `--force` → exit **3** (consentimiento explícito). Atajo host inyecta `--force`. `--dry-run` no exige `--force`. | Invariante no interactivo |
| I-TEAR-SIGNAL | SIGTERM luego KILL solo PIDs con `cmdline` `{ROOT}/start-sddia.sh` o cwd/exe bajo `ROOT` (`/proc`). No `pkill -f` global. | §1, §3 |
| I-TEAR-SYSTEMD | Por instancia `@${ESC}`: `stop`, `disable`, `reset-failed`. Stems mínimos: email, event-watcher/sweeper, kalma2-bridge, telegram, github-bridge, iota-publish-relay. **No** `rm` plantillas `sddia-*@.service` en `~/.config/systemd/user/`. **No** tocar unidades cuyo `%f` = forja (`UNIT_LAB`). | §2 |
| I-TEAR-LOCK | `_sddia_stop_lock_pid` sobre `.SddIA/daemons/status/*.lock`. | locks |
| I-TEAR-WIPE | `rm -rf ROOT` si directorio existe; si ROOT ya ausente, systemd huérfano igual (§3 deuda «APTO»). | §4 |
| I-TEAR-VAULT | Prohibido borrar forja, `/home/racso/Proyectos/.dev`, `*.deploy-vault`, `*.preprod-vault` (default). | §2 fuera de alcance |
| I-TEAR-CEGUERA | Cero `curl`/`ss` en motor. Verificación destilada: `test ! -e ROOT`; `systemctl is-active` inactive/unknown para `@${ESC}`; sin procesos bajo ROOT. | §5 (traducido) |

**Simetría deploy ↔ teardown:** mismo `resolve_root` / `ESC`; `deploy --force` sobre vivo = teardown previo + deploy (reusa `do_teardown`).

### 3.3 Del prompt deploy v1.7.0 — universal (no consumer-only)

Bundle hermético, `%f`, IOTA fail-soft solo consumer, Filtro C solo consumer (ver v1.1.0).

### 3.4 Del prompt teardown v1.0.0 — qué **no** copiar al contrato

- Proceso `paciente0-undeploy` (fases SignalScript…Verify) = **roadmap** DA-2, no texto del contrato installer.
- Tabla «`:8766` sin kalma2» como paso del CLI (sustituir por I-TEAR-CEGUERA).
- Default `INSTANCE_ROOT` = `SddIA_AP` (solo canal Paciente 0).

### 3.5 Fuera de destilación (ambos sentidos)

Olas, PR tables, Kaizen, G5, G-telegram, secretos en logs, prompts en prosa.

## 4. Plan de ejecución

1. **Init** `feature` `destilacion-sddia-installer` (`persist_ref` de este PBI). Raw Kernel. Relé IDE.
2. **Reconocer motor bilateral** ya en forja: `deploy` (main) + `teardown` (rama `feat/sddia-deterministic-teardown` o merge previo a norma). Smoke: `test-sddia-installer.sh` cubre ambos sentidos (dry-run, exit 2/3, abort forja).
3. **Norma Core** `sddia-installer-contract.md` (DA-2): capítulos **Deploy** (I-DEP-*) y **Teardown** (I-TEAR-*); tabla códigos de salida (1 path, 2 live-gate deploy, 3 force teardown); matriz perfil consumer vs installer default.
4. **Punteros en deuda** (opcional): `installer_contract_ref` en frontmatter deploy **y** teardown. **No** archivar deudas.
5. **Código:** solo gaps normativos no cubiertos por smoke. Prohibido segundo motor o poll HTTP.
6. **Cierre:** un PR; `validacion.md` APTO + CI; `accept-pr` post-verde.

## 5. Criterios de aceptación

* [ ] **CA-NORM:** `SddIA/norms/sddia-installer-contract.md` forjado DA-2; incluye **I-DEP-*** e **I-TEAR-***; uuid/SemVer/`hash_signature`.
* [ ] **CA-BILATERAL:** contrato documenta simetría ROOT/ESC y `deploy --force` → teardown previo; distingue atajos Deploy/Teardown.
* [ ] **CA-NO-ENTROPÍA:** cero prompts, olas, matriz tridimensional, path `_2.md`.
* [ ] **CA-PERFIL:** full-node/engineering (installer) vs consumer (`SddIA_AP`); teardown default ≠ `SddIA_AP`.
* [ ] **CA-CEGUERA:** sin HTTP/LLM en `deploy` ni `teardown`; G3/G-heartbeat/G-dlt no son fases del CLI.
* [ ] **CA-TEAR-DESTIL:** predicados de deuda §5 traducidos a I-TEAR-CEGUERA (sin `curl` en motor).
* [ ] **CA-DEUDA:** `PBI-DT-PACIENTE0-DEPLOY-PROCESS` y `PBI-DT-PACIENTE0-UNDEPLOY-PROCESS` en `pending/` salvo laudo de supersesión.
* [ ] **CA-CI:** `run_id` verde antes de `global: APTO` y `accept-pr`.

## 6. Fuera de gate

Reimplementar motor. Deploy/teardown live sin mandato. Forjar `paciente0-deploy` / `paciente0-undeploy` en este ciclo. Mutar `instance-creator.rs`. Plano B IOTA. G5. Versionar atajos host en git (opcional documentar en contrato como convención operador, no artefacto Core).
