---
document_id: AUDIT-INSTALLER-DEPLOY-APLICACIONES-20260926T113533Z
uuid: "58ea6333-c9b0-4998-9612-facb7e6b8114"
title: "Auditoría — despliegue installer determinista en /home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA"
created: "2026-09-26"
auditor: tekton
instance_path: /home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA
bundle_manifest: "20260925T122059Z"
bundle_profile: full-node
binaries: 34
pbi_refs:
  - PBI-ARQUITECTURA-DEPLOY-DETERMINISTA (c154bea0-c4c1-457e-b070-f7dfd3bc5f1b, PR #300)
  - PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA (d239cb31-a937-448c-941e-3808a3a28874)
  - PBI-DT-PACIENTE0-DEPLOY-PROCESS (7bf2bf4c-361e-4967-a58d-89dee74ea60d, v1.7.0)
feature_refs:
  - docs/features/sddia-deterministic-installer
  - docs/features/destilacion-sddia-installer
expected_by_user: "Despliegue total del asistente SddIA en la ruta de Aplicaciones"
verdict_installer_contract: APTO
verdict_assistant_operational: NO-APTO
verdict_story_claim: ERRATA-CORREGIDA
story_ref: docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md
---

# Auditoría — despliegue en Aplicaciones (installer determinista)

## 0. Errata que motiva la auditoría

La historia `HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN` v1.0.0 afirmó: *"El cliente en Aplicaciones son solo scripts de deploy; la instancia real es SddIA_AP (codex-kalma2-assistant → DOMAIN_AUTHORITY_DENIED)"*. **Falso por observación incompleta**: se inspeccionó `/home/racso/Aplicaciones/SddIA/` (atajos) y no `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA/` (instancia). Corregido en la historia v1.0.1 (§2.3 y G3).

## 1. Hechos observados (2026-09-26 13:35 UTC+2)

### 1.1 Instancia materializada

| Ítem | Valor |
|------|-------|
| ROOT | `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA` (last-resort del installer, I-DEP-PATH) |
| `MANIFEST.json` | `created_at 20260925T122059Z`, `profile: full-node`, `filtro_c: false`, `codex: null`, 34 binarios ELF |
| Genoma | `SddIA/{actions,agents,conscience,core,daemons,events,library,norms,process,scripts,skills,templates,tools}` + `interfaces/kalma2/` + `start-sddia.sh` + `sddia-run.sh` + `ONBOARDING.md` |
| Códices | `codex-software-engineering`, `codex-kalma2-assistant`, `codex-frontend-product-splus`, `codex-frontend-admin-splus`, `codex-backend-admin-splus` presentes |
| `.SddIA/` | `agenda constitution daemons inbox library llm-registry.json local.paths.json proofs systemd .dev/.env` |
| `.SddIA/active-domain-profile.json` | **ausente** → perfil `default` (`codex_slug: None`, `git_required: true`) → `has_software_authority = true` (regla legado D4) |
| `.SddIA/projects/` | ausente (sin proyectos registrados) |
| `.git` | ausente (no es repositorio) |
| `llm-registry.json` | `oracle-agy` (`skill:antigravity-cli-executor`) → fallback `oracle-gemini` (`tool:gemini-http-infer`); `model: ""` en ambos |

### 1.2 Bóveda de instancia

`ROOT/.SddIA/.dev/.env` (1898 B) es **copia byte a byte de la bóveda global de la forja** `/home/racso/Proyectos/SddIA/.dev/.env` (1898 B): mismas claves, mismo orden. Consecuencias:

| Clave | Estado en instancia | Efecto |
|-------|--------------------|--------|
| `SDDIA_AGENT_RUNTIME_COMMAND` = `SddIA/scripts/tools/kalma2-agent-runtime-cursor.sh` | presente; el script existe en el bundle | Runtime de agentes **activo** (Tekton ejecutable vía `cursor-agent`) |
| `SDDIA_AGENT_RUNTIME_CLI` = `/home/racso/.local/bin/cursor-agent --print --trust` | presente | Dependencia del IDE-CLI Cursor del host |
| `SDDIA_LLM_INFER_COMMAND` = `cursor-agent --print --mode ask` | presente, `SDDIA_LLM_REQUIRE_INFER=1` | Inferencia obliga a Cursor CLI |
| `SDDIA_EMAIL_IMAP_*` (host, user, **secret**) | presentes | Credenciales IMAP de la forja duplicadas en la instancia |
| `GEMINI_API_KEY` | presente | Secreto duplicado |
| `SDDIA_CLIENT_PORT` | **ausente** | `kalma2-bridge` cae al default `8765` |
| `TELEGRAM_BOT_TOKEN` / `TELEGRAM_ALLOWED_CHAT_ID` | **ausentes** | `telegram-watcher` aborta |
| `SDDIA_IOTA_RELAY_DIR` | ausente; `ROOT/.SddIA/services/iota-publish-relay` no existe | `iota-publish-relay` aborta |

Causa: `stage_vault()` en `SddIA/scripts/sddia-installer.sh` recorre `{FORGE}/.dev/.env` → `{FORGE}/.SddIA/.dev/.env` y hace `return 0` en el **primer** fichero encontrado. La bóveda de instancia de la forja (`.SddIA/.dev/.env`, 2120 B, `0600 root`) nunca se stagea. Esto cumple literalmente PBI §2 *"heredará la bóveda local estándar (`.dev/.env`)"*, pero produce una instancia sin claves de canal y con secretos ajenos.

### 1.3 Centinelas systemd (`ESC = home-racso-Aplicaciones-Asistencia_Tormentosa_SddIA`)

| Unidad | `is-active` | `is-enabled` | NRestarts | Causa raíz |
|--------|-------------|--------------|-----------|------------|
| `sddia-event-watcher@` | active | enabled | — | — |
| `sddia-event-sweeper@` | active | enabled | — | — |
| `sddia-email-watcher@` | active | enabled | — | Opera con IMAP de la forja (duplicado de SddIA_AP) |
| `sddia-github-bridge-watcher@` | active | enabled | — | — |
| `sddia-kalma2-bridge@` | **activating / auto-restart** | enabled | **15 924** | `bind 8765: Address already in use` — puerto ocupado por `kalma2-bridge` pid 97434 con `cwd=/home/racso/Proyectos/SddIA` (bridge de laboratorio de la forja) |
| `sddia-telegram-watcher@` | **activating / auto-restart** | enabled | **15 924** | `TELEGRAM_BOT_TOKEN / TELEGRAM_ALLOWED_CHAT_ID no configurados` (exit 2) |
| `sddia-iota-publish-relay@` | **activating / auto-restart** | enabled | **15 924** | `hijo Node ausente` (`SDDIA_IOTA_RELAY_DIR` no definido) |

Tres unidades en bucle de reinicio continuo desde `2026-09-25 12:21Z` (~25 h, ~1 reinicio/5,6 s cada una). Puertos: `8765` forja-lab, `8766` SddIA_AP, `8787` relay IOTA de la forja. **La instancia de Aplicaciones no tiene WUI escuchando en ningún puerto.**

### 1.4 Coexistencia de instancias

| Instancia | Perfil | WUI | Autoridad software | Runtime agentes |
|-----------|--------|-----|--------------------|-----------------|
| Forja `/home/racso/Proyectos/SddIA` | Core | `8765` (lab, pid 97434) | sí | sí |
| Paciente 0 `/home/racso/Proyectos/SddIA_AP` | consumer (13 ELF, `codex-kalma2-assistant`, `git_required:false`) | `8766` | **no** (`DOMAIN_AUTHORITY_DENIED`) | prohibido (Filtro C) |
| Aplicaciones `…/Asistencia_Tormentosa_SddIA` | full-node (34 ELF, perfil default legado) | **ninguna** (crash loop) | sí (legado D4) | sí (heredado de forja) |

## 2. Contraste con los PBIs

### 2.1 `PBI-ARQUITECTURA-DEPLOY-DETERMINISTA` (PR #300)

| CA | Declarado | Observación |
|----|-----------|-------------|
| Despliegue Atómico | APTO (dry-run + CI smoke). *"Deploy live ruta PBI = fuera de gate"* | El deploy live **sí se ejecutó** (`MANIFEST 20260925T122059Z`, 1 h 20 min después de la ola 10 de SddIA_AP), pero **ninguna feature ni auditoría lo registra**. No hay `docs/audits/*` del despliegue en Aplicaciones hasta este documento. |
| Payload Íntegro | APTO | Confirmado: 34 ELF ⊇ 13 consumer. |
| Teardown Limpio | APTO (dry-run) | No auditado en vivo. |
| Resiliencia de Sobreescritura | APTO | No aplica. |
| Zero-Touch §2 *"garantizando un despliegue operativo"* | implícito | **No cumplido**: la instancia arranca, pero el canal de interacción del asistente (WUI, Telegram) no opera. El PBI definía "operativo" solo como *cadena sin prompts*; no definió post-condición de disponibilidad de canal. |

### 2.2 `PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA` v1.2.0

| Punto | Observación |
|-------|-------------|
| §0 «`INSTANCE_ROOT` única → cuatro coordenadas» | Correcto y confirmado. La errata de la historia repite el mismo error ya diagnosticado en ese PBI (confundir atajos con instancia). |
| §2 canal *Installer default* → *"Gates G* / verificación HTTP: No"* (I-DEP-CEGUERA) | Coherente con la Ceguera de Ejecución del motor, pero deja **sin dueño** la verificación de que el asistente quede operativo. Para Paciente 0 la verificación es la ola (prompt manual); para el installer default **no existe ningún proceso que la ejecute**. |
| I-DEP-CREATOR `runtime_profile: engineering` | Cumplido; explica autoridad software en la instancia (perfil default, sin `active-domain-profile.json`). |
| CA-PERFIL *"full-node/engineering (installer) vs consumer (SddIA_AP)"* | Cumplido. Nota: full-node incluye `email-watcher` y `telegram-watcher` habilitados **sin** sus claves, lo que es incoherente con un perfil que pretende ser autosuficiente. |
| §3.3 *"Filtro C solo consumer"* | Cumplido: `filtro_c: false`. |

### 2.3 `PBI-DT-PACIENTE0-DEPLOY-PROCESS` v1.7.0 (SSOT Paciente 0)

No gobierna la instancia de Aplicaciones (canal distinto, §2 del PBI destilación). Su audit `paciente0-deploy-20260925T114629Z.md` (OLA-MEJORA, `wui_port: 8766`) es de SddIA_AP y **no** cubre Aplicaciones.

## 3. Veredictos

| Dimensión | Veredicto | Fundamento |
|-----------|-----------|------------|
| Contrato installer (I-DEP-*) | **APTO** | Bundle, creator, systemd `@%f` materializados según spec. |
| Expectativa del usuario *"despliegue total del asistente"* | **NO-APTO** | Sin WUI (puerto colisionado), sin Telegram (sin token), sin relay IOTA. Solo bus de eventos + email + github-bridge operan. |
| Higiene de secretos | **NO-APTO** | Bóveda global de la forja copiada íntegra (IMAP secret, `GEMINI_API_KEY`) a una instancia con root distinto; `email-watcher` consume el **mismo** buzón que SddIA_AP (doble consumo). |
| Trazabilidad | **NO-APTO** | Deploy live sin registro en `docs/audits` ni `docs/features` ni `SddIA/evolution`. |
| Afirmación de la historia (errata) | **CORREGIDA** | Ver §0. |

## 4. Brechas derivadas (candidatas a PBI; no forjadas en esta auditoría)

| ID | Brecha | Propuesta mínima |
|----|--------|------------------|
| DT-INST-VAULT-STAGE | `stage_vault` toma solo el primer `.env`; hereda bóveda **global** de forja y descarta la de instancia. | Componer ambas (global → instancia, precedencia instancia) o exigir `--vault` cuando no se pase perfil de canal; **nunca** copiar secretos de canal (`IMAP`, `TELEGRAM`) sin flag explícito. |
| DT-INST-PORT-COLLISION | `SDDIA_CLIENT_PORT` no derivado por instancia; default `8765` colisiona con forja. | Installer escribe `SDDIA_CLIENT_PORT` libre (sondeo `ss` **fuera** del motor ciego, o derivación determinista por `ESC`) en la bóveda de instancia; `kalma2-bridge` falla con exit distinto y `Restart=on-failure` con `StartLimitBurst` para evitar 15 k reinicios. |
| DT-INST-UNITS-CONDITIONAL | Unidades `telegram-watcher`/`iota-publish-relay` habilitadas sin sus prerequisitos. | `ConditionEnvironment=` / `ExecCondition=` en plantillas o installer habilita solo unidades cuyo launcher **y** claves existen. |
| DT-INST-POSTDEPLOY-VERIFY | Sin dueño de la verificación operativa del canal installer default. | Proceso Core `instance-health-verify` (o reutilizar `system-vitality-probe`) invocado tras `deploy` fuera del motor ciego; emite acta en `docs/audits`. |
| DT-INST-DOMAIN-PROFILE | Instancia full-node sin `active-domain-profile.json` → autoridad software por regla legado, no por declaración. | `instance-creator` escribe perfil explícito `codex-software-engineering` para `runtime_profile: engineering`. |
| DT-INST-MAILBOX-DUP | Dos instancias (`SddIA_AP`, Aplicaciones) sondean el mismo buzón IMAP. | Derivado de DT-INST-VAULT-STAGE; hasta corregir, deshabilitar `email-watcher@` en Aplicaciones o en SddIA_AP (laudo humano). |

## 5. Implicación para la historia Workspace Server 1×N

La instancia de Aplicaciones **es** el candidato natural a *instancia forjadora* (F5 de la historia): perfil full-node, autoridad software, runtime de agentes configurado, códices de dominio presentes, sin `.git` propio (opera sobre proyectos externos vía `project_slug`, exactamente el modelo ABSTRACT-04). Prerrequisitos antes de F5: resolver DT-INST-PORT-COLLISION y DT-INST-VAULT-STAGE, y declarar `active-domain-profile.json` explícito. G3 de la historia se reformula: no es *"falta autoridad"*, es *"la instancia con autoridad no está operativa ni saneada"*.

## 6. Acciones no ejecutadas (requieren laudo)

- Parar/deshabilitar las tres unidades en crash loop (`systemctl --user stop/disable sddia-{kalma2-bridge,telegram-watcher,iota-publish-relay}@${ESC}`): cambio de estado del sistema; no ejecutado.
- Redeploy con `--vault` saneado o `teardown --force`: destructivo; no ejecutado.
- Purga de secretos duplicados en `ROOT/.SddIA/.dev/.env`: mutación de bóveda; no ejecutada.
