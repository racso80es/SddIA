---
document_id: PBI-DT-PACIENTE0-DEPLOY-PROCESS
uuid: "7bf2bf4c-361e-4967-a58d-89dee74ea60d"
title: "[DEUDA] Paciente 0 — prompt de despliegue y proceso futuro"
format: markdown
version: "1.2.0"
status: deuda_tecnica
type: deuda
priority: alta
process: null
dispatch: false
process_candidate: paciente0-deploy
process_candidate_class: process
created: "2026-08-25"
updated: "2026-08-28"
instance_name_default: SddIA_AP
config_source: /home/racso/Proyectos/.dev/.env
instance_parent: /home/racso/Proyectos
audits_path: docs/audits
last_deploy_audit_ref: docs/audits/paciente0-deploy-20260826T120032Z.md
last_deploy_ola_verdict: OLA-MEJORA
post_ola_friction_open: null
fix_pbi_ref: docs/todos/done/[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher.md
fix_pbi_document_id: PBI-FIX-BUNDLE-TELEGRAM-GATEWAY
kaizen_pending_ref: null
kaizen_pending_document_id: null
last_kaizen_done_ref: docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md
last_kaizen_done_document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
antecesor_persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
antecesor_audit_ref: docs/audits/kaizen-paciente0-redeploy-20260825.md
tech_debt_ids:
  - DT-PACIENTE0-DEPLOY-PROCESS
  - DT-ORCHESTRATOR-DEBUG-FIRST
  - DT-LOCAL-PATHS-EMPTY-STUB
  - DT-IGNITION-ENV-ISOLATION
  - DT-SMOKE-ECST-LOCAL-QA
  - DT-SYSTEMD-USER-ENABLE
  - DT-CONFIG-UX-ONBOARDING
blocks_on:
  - PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
derived_from:
  - PBI-LAB-PACIENTE0-SDDIA-AP
  - PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
  - PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
---

# [DEUDA] Paciente 0 — prompt de despliegue y candidato a proceso

## 0. Propósito

Documento **operativo + semilla de proceso**. Sirve como **prompt** para que Tekton (o el operador) redepliegue Paciente 0 de forma reproducible, y como **contrato mínimo** de un proceso Core futuro (`paciente0-deploy`) que aún **no** está forjado (DA-2: no crear `{name}.md` bajo `directories.process` desde este PBI).

**Done de un ciclo de este prompt:** instancia viva + validaciones G* + **contraste valorativo** contra el último PBI Kaizen de despliegue Paciente 0 + auditoría bajo `paths.auditsPath` + PBI Kaizen **solo si** hay fricción nueva o regresión.

---

## 1. Prompt (copiar al Vértice Productivo)

```text
Despliega Paciente 0 siguiendo este PBI (PBI-DT-PACIENTE0-DEPLOY-PROCESS).

Constantes:
- Nombre de instancia por defecto: SddIA_AP
- Raíz instancia: /home/racso/Proyectos/SddIA_AP  (salvo override explícito)
- Configuración base: /home/racso/Proyectos/.dev/.env  (no inventar secretos; no loguearlos)
- Forja: repo SddIA actual (preferir rama main salvo indicación)
- Canal: Vía C — build-release-bundle (profile consumer, codex-kalma2-assistant)
          + instance-creator (skip_ignition) + ignición híbrida start-sddia.sh + systemd email-watcher@%f
- DA-3: ./sddia-run.sh (ELF nativo). DA-5: tras acuse JSON de execute-process, no poll de .events/
- Prohibido parchear {instancia}/start-sddia.sh ni SddIA/ inyectado (sddia-distribution-protocol)

Mitigaciones vigentes (Kaizen 20260825, aún no absorbidas en Core):
- Pin SDDIA_EXECUTE_PROCESS_BIN al ELF release de la FORJA para instance-creator (F-DEP-07).
  No usar el debug stale que sddia-run.sh prefiere por defecto.
- Si .SddIA/local.paths.json existe y es {}, unlink antes de reinyectar creator (F-DEP-08).
- Ignición: env -u SDDIA_EXECUTE_PROCESS_BIN (o pin al ELF del BUNDLE instancia) (F-DEP-09).
- systemctl --user: copiar unidad renderizada → ~/.config/systemd/user/ y enable --now (F-SYS-01).
- Lab email-watcher@…SddIA: no dejar active (R-07).
- Lab telegram-watcher@…SddIA: puede coexistir con AP **solo** si `TELEGRAM_BOT_TOKEN` lab ≠ instancia (mismo `TELEGRAM_ALLOWED_CHAT_ID` permitido). Si tokens iguales → stop/disable lab (R-07 análogo).
- Legado `sddia-daemon@telegram-watcher`: stop/disable si active (higiene pre-ignición soberana).

Cierre del estímulo:
1) Validaciones habituales (este documento §5).
2) Contrastar el ÚLTIMO PBI Kaizen de despliegue Paciente 0 (§6) y emitir veredicto de ola (§6.3).
3) Auditoría final en docs/audits/ (§7). No secretos.
4) Si hay fricción nueva o regresión de F-DEP absorbidas: generar PBI Kaizen en docs/todos/pending/
   con la estructura del último (bitácora, fricciones, deuda, criterios). Si no hay fricción: no abrir Kaizen.

No forjar el proceso paciente0-deploy en este estímulo; este PBI es la semilla.
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
| `WUI_PORT` | `8766` (`SDDIA_CLIENT_PORT` en bóveda) |
| `PROFILE` | `consumer` |
| `CODEX` | `codex-kalma2-assistant` |
| `SENSORIAL` | `systemd` (`SDDIA_SENSORIAL_JURISDICTION`) |
| `UNIT` | `sddia-email-watcher@$(systemd-escape -p "$INSTANCE_ROOT").service` |

**Bóveda instancia (Filtro C):** copiar claves de `CONFIG_SOURCE`; **omitir** `SDDIA_AGENT_RUNTIME_*` en `{instancia}/.SddIA/.dev/.env`. Forzar `SDDIA_RUNTIME_PROFILE=consumer` y `SDDIA_SENSORIAL_JURISDICTION=systemd` si no están en `.dev`. Completar huecos desde `PREPROD_VAULT` **solo** si aún faltan (Telegram, extras IMAP). No echo de valores.

**Layout vault staging (input de `instance-creator`):**

```text
{VAULT_STAGING}/
  root.dev.env              ← copia CONFIG_SOURCE
  instance.SddIA.dev.env    ← consumidor (sin AGENT_RUNTIME)
  constitution/             ← desde PREPROD_VAULT si existe
  codexes/                  ← desde PREPROD_VAULT si existe
```

---

## 3. Procedimiento de despliegue (pasos)

Ejecutar en orden. Canal canónico: `SddIA/norms/sddia-distribution-protocol.md` v1.2.0 + `SddIA/process/instance-creator.md`.

### 0 — Pin orquestador (F-DEP-07)

```bash
export SDDIA_EXECUTE_PROCESS_BIN="${FORGE_ROOT}/SddIA/target/release/execute-process"
# Si el ELF release no existe o cicatriz diverge: cargo build --release -p execute-process (forja), no skip-build ciego.
```

### 1 — Baseline Kaizen (antes de mutar instancia)

Localizar el **último** PBI Kaizen de despliegue Paciente 0:

1. `docs/todos/pending/` con `document_id` `PBI-KAIZEN-PACIENTE0-REDEPLOY-*` (más reciente gana).
2. Si no hay pending: `docs/todos/done/` mismo prefijo.
3. Cargar `friction_ids`, gates, `bundle_manifest`, `instance_creator_*_correlation_id`.

Ese documento es la **línea base** del contraste §6. Snapshot 2026-08-26: done `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825` (pending homónimo no existe). Última cicatriz de ola: `docs/audits/paciente0-deploy-20260826T120032Z.md`.

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
```

Gate inmediato: 0 `*.rs` en bundle; `strings` centinelas sin `execute-process.py`; 7 ELF consumidor; `MANIFEST.json` + `ONBOARDING.md`.

Si la instancia ya existía y debe preservarse `.SddIA` / `.events`: bundle a `dist/…` y rsync overlay **sin** pisar periféricos (ola T6). Wipe = `--out` directo a `INSTANCE_ROOT`.

### 4 — `instance-creator` (`skip_ignition`)

```bash
export SDDIA_EXECUTE_PROCESS_BIN="${FORGE_ROOT}/SddIA/target/release/execute-process"
# Si INSTANCE_ROOT/.SddIA/local.paths.json es {}: unlink (F-DEP-08)
./sddia-run.sh --process instance-creator --inputs "{
  \"instance_root\": \"${INSTANCE_ROOT}\",
  \"runtime_profile\": \"consumer\",
  \"vault_source\": \"${VAULT_STAGING}\",
  \"skip_ignition\": true
}"
```

Acuse JSON = éxito de inyección (DA-5). Verificar en el acuse: `success:true`, `vault_files_copied`, smoke topology. Luego, **sin poll EDA**:

- `local.paths.json` **no** `{}`.
- Unidad renderizada: `ExecStart` bajo `${INSTANCE_ROOT}/SddIA/` (no forja).

Si falla: no parchear instancia; reinyectar con pin release + unlink `{}`.

### 5 — systemd núcleo + sensorial (`@%f`)

`instance-creator` deja unidades en `${INSTANCE_ROOT}/.SddIA/systemd/` (`sddia-event-watcher@.service`, `sddia-event-sweeper@.service`, `sddia-kalma2-bridge@.service`, `sddia-email-watcher@.service`, …). `start-sddia.sh` (jurisdicción systemd) las re-materializa, copia a `~/.config/systemd/user/` y hace `enable --now` con `systemd-escape -p "$INSTANCE_ROOT"`.

```bash
# Linger (reboot sin login gráfico)
loginctl enable-linger "$(id -un)"
# Lab: stop/disable las mismas plantillas @escape(FORGE_ROOT) si están active
```

Copia manual opcional (si se ignora `start-sddia`):

```bash
mkdir -p "${HOME}/.config/systemd/user"
cp -f "${INSTANCE_ROOT}/.SddIA/systemd/"sddia-*.service \
  "${HOME}/.config/systemd/user/"
systemctl --user daemon-reload
ESC="$(systemd-escape -p "$INSTANCE_ROOT")"
for stem in sddia-event-watcher sddia-event-sweeper sddia-kalma2-bridge sddia-email-watcher; do
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

Esperado: jurisdicción `systemd`; **exit 0** (el script **no** queda en `wait`); log `unidades enable --now`; WUI `http://127.0.0.1:${WUI_PORT}/`; email/telegram **no** spawneados con `&`. Override lab atado a TTY: `SDDIA_DAEMON_JURISDICTION=script`.

### 7 — Cierre documental del ciclo de despliegue

§5 validaciones → §6 contraste de ola → §7 auditoría → §8 Kaizen condicional.

---

## 4. Olas de despliegue (contexto valorativo)

| Ola | Fecha | Artefacto | Canal | Veredicto breve |
|-----|-------|-----------|-------|-----------------|
| 0 Ensayo | 2026-08-20 | `PBI-LAB-PACIENTE0-SDDIA-AP` | clone + build debug, no bundle hermético | APTO laboratorio; G5 lote sin `actionable` (F-03) |
| 1 Redeploy | 2026-08-24 | `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824` | bundle+creator; parches instancia | WUI OK; F-DEP-01…06; G5 reunión → `passive` |
| 2 Absorción T6 | 2026-08-25 a.m. | audit `kaizen-paciente0-redeploy-20260825` + feature merge | bundle fresco + creator release | F-DEP-01…04 y F-TRIAGE-01 absorbidos; G5 sintético `actionable` |
| 3 Redeploy post-merge | 2026-08-25 12:01Z | `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825` (done) | mismo canal; `main` | Instancia OK **tras** pin release; F-DEP-07/08/09/SMOKE-01/SYS-01 |
| 4 Redeploy wipe | 2026-08-25 13:15Z | `AUDIT-PACIENTE0-DEPLOY-20260825T131532Z` | wipe; instancia ausente | **OLA-ESTABLE** |
| 5 Redeploy post-aislamiento | 2026-08-26 11:02Z | `AUDIT-PACIENTE0-DEPLOY-20260826T110203Z` | wipe; `main` + merge aislamiento | Gates **OLA-MEJORA**; post-ola Telegram conversacional **F-BUNDLE-06** (FIX lab) |
| 6 Redeploy post-PR #194 | 2026-08-26 12:00Z | `AUDIT-PACIENTE0-DEPLOY-20260826T120032Z` | wipe; `main` + bundle 8 cápsulas | **OLA-MEJORA**; G-telegram APTO; F-BUNDLE-06 cerrado en runtime |

Cada nueva ejecución de **este prompt** es una **ola N+1**. El contraste §6 la sitúa respecto a la ola Kaizen más reciente.

---

## 5. Validaciones habituales (gates)

No secretos. Fallo = NO APTO de ola (sigue §8).

| ID | Check | Criterio APTO |
|----|--------|----------------|
| G0-config | `CONFIG_SOURCE` existe; staging copió vault | `vault_env_present`; claves IMAP/LLM en instancia (nombres, no valores) |
| G-bundle | integridad artefacto | 0 `.rs`; `PY_LEAK=no`; `MANIFEST.json`; Filtro C sin `github-bridge-watcher`; **si** `telegram-watcher` ∈ manifest → ELF + `.md` `telegram-gateway` bajo `SddIA/target` (F-BUNDLE-06) |
| G1 | topología | `.SddIA/`, `.events/{domain,orchestration,telemetry,pending}/`; `local.paths.json` no `{}` |
| G2 | ley local | `constitution.json` `product=SddIA_AP`; códice `codex-kalma2-assistant` en `.SddIA/library/codexes/` |
| G3 | WUI + EDA | HTTP 200 en `:${WUI_PORT}`; log ignición `route-domain` / `route-telemetry` enruta; 0 `cargo build` |
| G3b | systemd | unidades AP `sddia-{event-watcher,event-sweeper,kalma2-bridge,email-watcher}@%f` `active`; `WorkingDirectory=INSTANCE_ROOT`; `ExecStart` bajo instancia |
| G4 | Filtro C | sin `github-bridge`; instancia sin `AGENT_RUNTIME_*`; sin `codex-software-engineering` en códices locales |
| G-orch | resolución ELF | creator usó **release** (o cicatriz vigente); ignición **sin** ELF forja si se aisló env (F-DEP-09) |
| G5 | First Blood | **Opcional** en redeploy rutinario (DA-5). Si se ejecuta: reunión estructural → `actionable` (antecesor T6). No esperar IMAP en bucle. |
| G-telegram | conversacional bot instancia | **Opcional** post-ola (DA-5). `./sddia-run.sh --process telegram-gateway --inputs '{"text":"…"}'` → `success:true` `emitted:true`; o mensaje al bot → journal sin `gateway rc=1` y respuesta Tormentosa/Aiúa. **No** sustituye gate correo→`send-telegram-notification` (canal distinto). |

Smoke creator `Local_QA_Requested` → DLQ `payload.branch` = **F-SMOKE-01** conocido: no tumba G3 si EDA real enruta; sí se anota en contraste.

**Telegram multi-instancia:** tokens distintos lab vs instancia → watchers `@forja` y `@AP` pueden coexistir. Mismo `TELEGRAM_ALLOWED_CHAT_ID` es válido (un operador, dos bots). Correo→Telegram usa `send-telegram-notification` (suele APTO); chat entrante usa `telegram-watcher` → `telegram-gateway` (validar G-telegram / F-BUNDLE-06).

---

## 6. Contraste contra el último PBI Kaizen de despliegue

### 6.1 Cómo elegir el SSOT de contraste

Único documento Kaizen **de despliegue Paciente 0** más reciente (`document_id` `PBI-KAIZEN-PACIENTE0-REDEPLOY-*`). Pending prevalece sobre done si `updated`/`redeploy_executed_at` es mayor.

Cargar: `friction_ids`, tabla de gates, `bundle_manifest`, mitigaciones ad-hoc.

### 6.2 Matriz de evolución (rellenar en la auditoría)

Para cada ID del Kaizen baseline y cada gate G*:

| Campo | Valores |
|-------|---------|
| Estado baseline | APTO / NO APTO / no auditado |
| Estado ola actual | APTO / NO APTO / no auditado |
| Delta | **mejoró** / **igual** / **regresionó** / **nuevo** |
| Nota | una línea; sin secretos |

IDs mínimos a contrastar (unión del último Kaizen + gates §5):

`F-DEP-01` … `F-DEP-09`, `F-DEP-05`, `F-SMOKE-01`, `F-SYS-01`, `F-BUNDLE-06` (si G-telegram auditado), `F-TRIAGE-01`…`03` (si G5), G0–G4, G3b, G-orch, G-bundle, G-telegram (opcional).

### 6.3 Veredicto valorativo de ola

Un párrafo + etiqueta:

| Etiqueta | Criterio |
|----------|----------|
| **OLA-MEJORA** | Ninguna regresión de F-DEP ya absorbidas; 0 fricción nueva bloqueante |
| **OLA-ESTABLE** | Mismas fricciones residuales (p. ej. F-DEP-07 aún no en Core); gates G1–G4 APTO con las mitigaciones del prompt |
| **OLA-REGRESIÓN** | Reaparece F-DEP-01/02/03/04 **con** ELF release pinneado, o G3/G3b KO |
| **OLA-NUEVA-FRICCIÓN** | IDs nuevos no listados en el Kaizen baseline |

El veredicto **obliga** §8 si es `OLA-REGRESIÓN` o `OLA-NUEVA-FRICCIÓN`. `OLA-ESTABLE` con residuales ya en pending **no** duplica PBI (actualizar bitácora del Kaizen existente o solo audit).

---

## 7. Auditoría final

Crear `{paths.auditsPath}/paciente0-deploy-{STAMP}.md` (`docs/audits/`, Cúmulo `auditsPath`).

Frontmatter mínimo: `document_id`, `uuid` v4, `created`, `instance_path`, `bundle_manifest`, `instance_creator_correlation_id`, `ola_verdict`, `kaizen_baseline_document_id`, `wave_matrix_ref` (este archivo o tabla embebida).

Cuerpo: canal usado; mitigaciones aplicadas (pin release, unlink `{}`, `env -u`); gates §5; **matriz §6.2 + veredicto §6.3**; fricciones nuevas si las hay; **§ post-ola sensorial** (Telegram/correo, tokens distintos, F-BUNDLE-06); qué **no** se hizo (p. ej. G5). Cero secretos. No reescribir audits T6 previos; este archivo es la cicatriz de **esta** ola.

---

## 8. PBI Kaizen pendiente (condicional)

**Generar** `docs/todos/pending/[KAIZEN] Paciente 0 ${INSTANCE_NAME} — redeploy {STAMP} y fricciones.md` **si y solo si** §6.3 ∈ {`OLA-REGRESIÓN`, `OLA-NUEVA-FRICCIÓN`} **o** el Kaizen pending vigente quedó obsoleto (nueva bitácora que no cabe en un apéndice).

**Excepción post-ola (no Kaizen redeploy):** fricción **F-BUNDLE-06** (cápsula `telegram-gateway` ausente en bundle consumidor) → **FIX lab** `PBI-FIX-BUNDLE-TELEGRAM-GATEWAY` vía `bug-fix` en forja; no duplicar Kaizen Paciente 0 ni parchear `SddIA/` en instancia.

Estructura (antecesor `docs/features/kaizen-paciente0-redeploy-fricciones` + PBI 20260824/20260825):

- Frontmatter: `document_id` `PBI-KAIZEN-PACIENTE0-REDEPLOY-{YYYYMMDD}`, `uuid`, `persist_ref` reservado, `derived_from` el Kaizen contrastado, `friction_ids`, `instance_path`, `config_source`, `bundle_manifest`, correlation ids.
- Cuerpo: §0 contexto; §0bis bitácora A–F; §0ter métricas; §1 fricciones (síntoma / causa / ad-hoc / DT); §2 qué sigue absorbido; §3 objetivos; §4 criterios; §5 orden de forja; §6 refs; §7 audit al cierre del **ciclo feature** (distinto del audit de ola §7).

**No generar** si `OLA-MEJORA` o `OLA-ESTABLE` y ya existe `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825` cubriendo F-DEP-07… — en ese caso solo audit de ola + enlace al pending.

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
| Vault | Staging desde `config_source` + Filtro C | cápsula/script; secretos opacos |
| Bundle | `build-release-bundle` consumer | script existente |
| Materialize | `instance-creator` `skip_ignition` + pin release | proceso existente |
| Sensorial | instalar unidad user `%f` + R-07 | OS; hoy operador (F-SYS-01) |
| Ignition | `start-sddia` con env aislado | script; detach |
| Validate | gates §5 | checks; sin poll post-acuse |
| Contrast | matriz vs Kaizen baseline | documental |
| Audit | escribir `docs/audits/…` | IDE / cápsula doc |
| KaizenGate | emitir PBI pending si §6.3 lo exige | documental |

### 9.3 Fuera del proceso

Wizard UX (`DT-CONFIG-UX-ONBOARDING`). Mutación de genoma (eso es el ciclo `feature` del Kaizen, no este proceso). G5 IMAP como default (opt-in `skip_g5=false`).

---

## 10. Referencias

| Ref | Uso |
|-----|-----|
| `SddIA/norms/sddia-distribution-protocol.md` | Vía C, bundle, creator |
| `SddIA/process/instance-creator.md` | Fases Topologia–Smoke |
| `SddIA/scripts/common/sddia_shell_lib.sh` | `_sddia_resolve_orchestrator` (F-DEP-07) |
| `SddIA/core/cumulo.paths.json` | `auditsPath`, `featurePath` |
| `docs/audits/paciente0-deploy-20260826T120032Z.md` | Ola 6 + G-telegram APTO |
| `docs/audits/paciente0-deploy-20260826T110203Z.md` | Ola 5 + post-ola Telegram |
| `docs/todos/done/[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher.md` | F-BUNDLE-06 cerrado (PR #194 + ola 6) |
| `docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md` | Kaizen baseline deploy |
| `docs/todos/done/[LABORATORIO] MVP Paciente 0 SddIA_AP.md` | Ola 0 |
| `docs/audits/kaizen-paciente0-redeploy-20260825.md` | Ola 2 T6 |
| `docs/features/kaizen-paciente0-redeploy-fricciones/` | Estructura documental Kaizen |

---

## 11. Criterios de cierre de **esta** deuda

- [x] Prompt copiable (§1) + procedimiento (§3) + gates + contraste de olas + audit + Kaizen condicional.
- [ ] Proceso `paciente0-deploy` forjado vía `entity-manager` (ciclo feature distinto).
- [ ] Absorción F-DEP-07/08/09 en Core (`PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825`) — **bloquea** la eliminación de mitigaciones del prompt.
- [x] Absorción F-BUNDLE-06 (`PBI-FIX-BUNDLE-TELEGRAM-GATEWAY`) — validado ola 6 (G-telegram APTO, bundle 8 cápsulas, sin copia manual ELF).
