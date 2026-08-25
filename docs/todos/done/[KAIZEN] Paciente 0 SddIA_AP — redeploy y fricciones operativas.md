---
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
uuid: "56aff1d3-d5f6-4502-9b5b-e5a57dc718e3"
title: "[KAIZEN] Paciente 0 (SddIA_AP): Redeploy, bundle y triaje de correo"
format: markdown
version: "1.2.0"
status: done
type: kaizen
priority: alta
process: feature
persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
execution_id: "c95fa63f-be71-481b-a927-475e7c885fd0"
created: "2026-08-24"
updated: "2026-08-25"
pbi_archived: true
audit_ref: docs/audits/kaizen-paciente0-redeploy-20260825.md
evolution_id: "916bf0f9-05ea-4408-8b6e-294e7efcc5f9"
redeploy_executed_at: "2026-08-24T17:56:25Z"
redeploy_t6_at: "2026-08-25T11:17:33Z"
instance_creator_correlation_id: "5d8c081d-49ed-4764-996c-a0937218ba2e"
instance_creator_t6_correlation_id: "9528fb5f-9d72-4990-adce-44bca7dc734d"
g5_event_id: "413e6edf-e19c-4ec4-8b5c-e9ccd6a4d13f"
forge_branch: feat/kaizen-paciente0-redeploy-fricciones
wui_port: 8766
systemd_unit: "sddia-email-watcher@home-racso-Proyectos-SddIA_AP.service"
derived_from: PBI-LAB-PACIENTE0-SDDIA-AP
supersedes_partial: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
instance_path: /home/racso/Proyectos/SddIA_AP
config_source: /home/racso/Proyectos/.dev
preprod_vault: /home/racso/Proyectos/SddIA_AP.preprod-vault
deploy_vault_staging: /home/racso/Proyectos/SddIA_AP.deploy-vault
friction_ids:
  - F-DEP-01
  - F-DEP-02
  - F-DEP-03
  - F-DEP-04
  - F-DEP-05
  - F-DEP-06
  - F-TRIAGE-01
  - F-TRIAGE-02
  - F-TRIAGE-03
tech_debt_ids:
  - DT-INSTANCE-CREATOR-CORE-ROOT
  - DT-START-SDDIA-BUNDLE-IGNITION
  - DT-BUNDLE-FRESH-BINS
  - DT-TRIAGE-SUBJECT-FALLBACK
  - DT-TRIAGE-LLM-QUALITY
  - DT-CONFIG-UX-ONBOARDING
blocks_on: []
---

# [KAIZEN] Paciente 0 (SddIA_AP) — Redeploy y fricciones operativas

## 0. Contexto

Redeploy operativo de **Paciente 0** (`SddIA_AP`) el **2026-08-24**, tras wipe post-ensayo (2026-08-20). Canal canónico: `build-release-bundle.sh` (perfil `consumer`) + `instance-creator` + ignición híbrida (script + systemd `email-watcher@%f`).

**Configuración inyectada:** bóveda personalizada en `/home/racso/Proyectos/.dev/.env` (no solo preprod-vault). Constitución/códice complementados desde `SddIA_AP.preprod-vault`.

**Resultado del redeploy:** instancia operativa (WUI `:8766` HTTP 200, EDA enrutando tras parche de binarios). **Valor E2E de correo accionable no demostrado** en este ciclo: triaje clasificó reunión de prueba como `passive` (§4).

Referencias: `docs/todos/done/[LABORATORIO] MVP Paciente 0 SddIA_AP.md`, `docs/todos/done/[KAIZEN] perfil ignición consumidor Filtro C.md`, `SddIA/norms/sddia-distribution-protocol.md` v1.1.0.

---

## 0bis. Procedimiento ejecutado (bitácora operativa)

Secuencia real del redeploy **2026-08-24** (forja `main` → instancia `/home/racso/Proyectos/SddIA_AP`). Tiempos aproximados en UTC salvo indicación.

### Fase A — Materialización del bundle

| Paso | Acción | Resultado |
|------|--------|-----------|
| A1 | `build-release-bundle.sh --out SddIA_AP --codex codex-kalma2-assistant --profile consumer --skip-build` | OK ~1,4 s; `MANIFEST.json` `20260824T175625Z` |
| A2 | Gate integridad bundle | 0 archivos `.rs`; 7 binarios + 7 cápsulas resueltas |
| A3 | Artefactos raíz instancia | `start-sddia.sh`, `sddia-run.sh`, `ONBOARDING.md`, `interfaces/kalma2/` |

**Binarios empaquetados:** `execute-process`, `kalma2-bridge`, `event-watcher`, `event-sweeper`, `email-watcher`, `telegram-watcher`, `send-telegram-notification`. **Excluido** (Filtro C): `github-bridge-watcher`.

### Fase B — Vault staging e `instance-creator`

| Paso | Acción | Resultado |
|------|--------|-----------|
| B1 | Crear `SddIA_AP.deploy-vault/` | `root.dev.env` ← `/home/racso/Proyectos/.dev/.env` |
| B2 | `instance.SddIA.dev.env` | Merge Python: base `.dev` + `TELEGRAM_*` desde preprod si ausentes + perfil consumidor |
| B3 | Copiar `constitution/` y `codexes/` | Desde `SddIA_AP.preprod-vault` |
| B4 | `./sddia-run.sh --process instance-creator` | `success:true`, `correlation_id=5d8c081d-…`, `vault_files_copied=6`, `duration_ms=1` |
| B5 | Fases creator | Topologia ✓ · Vault ✓ · Systemd ✓ · Ignicion **skipped** · Smoke ✓ (`native-topology+local-qa`) |

**Inputs JSON:**

```json
{
  "instance_root": "/home/racso/Proyectos/SddIA_AP",
  "runtime_profile": "consumer",
  "vault_source": "/home/racso/Proyectos/SddIA_AP.deploy-vault",
  "skip_ignition": true
}
```

**Variables inyectadas en instancia (16 claves, sin valores):**  
`SDDIA_RUNTIME_PROFILE`, `SDDIA_SENSORIAL_JURISDICTION`, `SDDIA_CLIENT_PORT`, `SDDIA_EMAIL_*`, `SDDIA_LLM_*`, `TELEGRAM_*`, `SDDIA_EXECUTE_PROCESS_BIN` (añadida post-creator), `SDDIA_ENV`.

**Capa raíz `.dev/.env` (16 claves):** incluye aún `SDDIA_AGENT_RUNTIME_*` (Filtro C no podado en raíz; instancia sí).

### Fase C — Overlay y systemd (operador)

| Paso | Acción | Resultado |
|------|--------|-----------|
| C1 | Sustituir `local.paths.json` `{}` | Copia desde `starter-kit` (F-DEP-04) |
| C2 | Instalar plantilla systemd | `~/.config/systemd/user/sddia-email-watcher@.service` |
| C3 | `enable --now sddia-email-watcher@…SddIA_AP` | Primera ignición **KO** (ExecStart apuntaba a lab — F-DEP-01) |
| C4 | Re-render `%f` con `CORE_ROOT=$INST` | Unidad corregida; PID bajo `SddIA_AP/SddIA/target/` |
| C5 | Deshabilitar lab IMAP colisionado | `sddia-email-watcher@…SddIA` → `inactive` (R-07) |

### Fase D — Ignición núcleo (script)

| Paso | Acción | Resultado |
|------|--------|-----------|
| D1 | `./start-sddia.sh` (1.er intento) | **KO** — `cargo build` sin `Cargo.toml` (F-DEP-02) |
| D2 | Parche instancia `_ensure_orchestrator` | Resolver ELF antes de compilar |
| D3 | `./start-sddia.sh` (2.º intento) | WUI HTTP 200 `:8766`; EDA **KO** — rutas vía `execute-process.py` (F-DEP-03) |

**Entorno ignición:**

```bash
SDDIA_RUNTIME_PROFILE=consumer
SDDIA_SENSORIAL_JURISDICTION=systemd
SDDIA_EXECUTE_PROCESS_BIN=$INST/SddIA/target/release/execute-process
```

**Centinelas script (R-07):** `event-watcher`, `event-sweeper`, `kalma2-bridge`; **omitidos** `email-watcher` / `telegram-watcher` (jurisdicción systemd).

### Fase E — Regeneración binarios y estabilización

| Paso | Acción | Resultado |
|------|--------|-----------|
| E1 | `cargo build --release` (7 paquetes) en forja | ~71 s |
| E2 | Copia ELF → `SddIA_AP/.../target/{release,debug}/` | Gate `strings event-watcher`: sin `.py` |
| E3 | Reinicio `email-watcher@AP` + `start-sddia.sh` | EDA enruta; logs `enrutado (route-domain) (purgado)` |
| E4 | Catch-up IMAP post-estabilización | +100 `.eml`, +100 proofs (lote histórico buzón) |

### Fase F — Ensayo correo reunión (valor E2E)

| Paso | Hora (CEST) | Evento |
|------|-------------|--------|
| F1 | ~20:05:56 | Correo recibido IMAP (UID `104579`, asunto reunión 25/08/2026 10:00) |
| F2 | ~20:06:55 | Triaje emitido — `passive` / `llm` / peaje 0 (F-TRIAGE-*) |
| F3 | — | WUI inbox vacío; Telegram silenciado (solo `actionable`) |

**Latencia sensorial → triaje (correo reunión):** ~59 s (IMAP poll 60 s + enrutado).

### Identidad instancia materializada

| Campo | Valor |
|-------|--------|
| `product` / `workspace_id` | `SddIA_AP` / `sddia-ap-paciente-0` |
| Constitución | `.SddIA/constitution/CONSTITUTION.md` (preprod) |
| Códice | `.SddIA/library/codexes/codex-kalma2-assistant.md` |
| WUI | `http://127.0.0.1:8766/` |
| Bus runtime | `./.events/{domain,orchestration,telemetry,pending}/` |

---

## 0ter. Métricas e indicadores (snapshot)

**Timestamp snapshot:** `2026-08-24T18:15:32Z` (post-estabilización EDA).

### Despliegue / artefacto

| Métrica | Valor |
|---------|--------|
| Tamaño instancia en disco | **54 MiB** |
| Tamaño `SddIA/target/` | **39 MiB** |
| Tiempo bundle (`--skip-build`) | ~1,4 s |
| Tiempo `cargo build --release` (7 crates) | ~71 s |
| `instance-creator` `duration_ms` | 1 |
| Smoke creator | `success:true`, modo `native-topology+local-qa` |
| Archivos vault copiados | 6 |
| Unidades systemd materializadas | 2 (`email-watcher@`, `daemon@`) |
| Gate `.rs` en bundle | 0 |

### Circuito sensorial / triaje (post catch-up)

| Métrica | Valor |
|---------|--------|
| Proofs `email-triaged` | **100** |
| Veredicto `noise` | **83** (83 % — `decision_path: deterministic`, regla C-LIST mayoritario) |
| Veredicto `passive` | **17** (17 % — `decision_path: llm`) |
| Veredicto `actionable` | **0** |
| Proofs LLM con `tokens_in+out > 0` | **0** |
| Ficheros `.eml` inbox | **101** |
| Asientos agenda | **0** |
| Eventos pendientes `domain/` | **0** (drenados tras fix) |
| Dead-letter | **3** (ventana F-DEP-03) |

### Gates redeploy vs ensayo original (PBI-LAB)

| Gate | Ensayo 2026-08-20 | Redeploy 2026-08-24 |
|------|-------------------|---------------------|
| G1 Topología | APTO | APTO (con parches manuales) |
| G2 Códice | APTO | APTO (preprod vault) |
| G3 WUI + EDA | APTO | APTO **tras** F-DEP-03 |
| G3b systemd SIGKILL | APTO | No re-auditado en este ciclo |
| G5 First Blood actionable | APTO (50 UIDs; 0 actionable en lote) | **NO APTO** reunión → `passive` |
| Inbox WUI items | 0 (diseño F-03) | 0 |
| Perfil consumidor Filtro C | APTO condicionado | APTO operativo; `AGENT_RUNTIME_*` residual en `.dev/` raíz |

### Peaje termodinámico (lote 100 proofs)

| `decision_path` | n | `tokens_*` típico | `duration_ms` típico |
|-----------------|---|-------------------|----------------------|
| `deterministic` | 83 | 0 | 0 |
| `llm` | 17 | 0 | 0 |

Interpretación: Clasificacion LLM **no consumió inferencia medible** en ningún proof del lote; refuerza F-TRIAGE-02.

### Comandos de verificación reproducibles

```bash
INST=/home/racso/Proyectos/SddIA_AP
curl -s -o /dev/null -w 'WUI %{http_code}\n' http://127.0.0.1:8766/
systemctl --user is-active "sddia-email-watcher@$(systemd-escape -p "$INST").service"
find "$INST/.SddIA/proofs/email-triaged" -name '*.json' | wc -l
rg -c '"verdict":"actionable"' "$INST/.SddIA/proofs/email-triaged" || echo "actionable=0"
cat "$INST/.SddIA/proofs/email-triaged/6e552199-416d-4043-91c9-fb1bae9e2057.json"
```

---

## 1. Fricciones de despliegue (comportamiento no esperado)

| ID | Síntoma | Causa raíz | Corrección ad-hoc (2026-08-24) | Acción Kaizen |
|----|---------|------------|--------------------------------|---------------|
| **F-DEP-01** | Unidad `sddia-email-watcher@…SddIA_AP` arrancaba binario bajo `/home/racso/Proyectos/SddIA/` (lab forja) | `instance-creator` sustituye `@@SDDIA_CORE_ROOT@@` con el **repo forjador** (`execute-process` invocado desde `SddIA/`), no con la raíz de instancia | Re-render manual de plantilla con `$INST` como core root; copia a `~/.config/systemd/user/` | **DT-INSTANCE-CREATOR-CORE-ROOT:** fase Systemd debe usar `instance_root` como `CORE_ROOT`, no `repo` del CLI |
| **F-DEP-02** | `./start-sddia.sh` aborta: `cargo build -p execute-process` — `Cargo.toml` ausente en bundle | Bundle hermético excluye fuentes; `_ensure_orchestrator()` siempre intenta compilar antes de resolver ELF | Parche local en `{instancia}/start-sddia.sh`: resolver binario existente **antes** de `cargo build` | **DT-START-SDDIA-BUNDLE-IGNITION:** detectar bundle (`MANIFEST.json` o ausencia de `Cargo.toml`) y omitir build; gate en Core, no parche por instancia |
| **F-DEP-03** | `event-watcher` loguea `python3: can't open file …/execute-process.py`; EDA no enruta | `build-release-bundle --skip-build` empaquetó ELF **obsoletos** (`strings event-watcher` contenía `execute-process.py`) | Recompilar desde `main` en forja y copiar ELF a `SddIA_AP/SddIA/target/{release,debug}/` | **DT-BUNDLE-FRESH-BINS:** gate post-build (sin cadena `.py` en centinelas); `--skip-build` solo si hash/build-id ≥ genoma actual |
| **F-DEP-04** | `local.paths.json` vacío `{}` | `instance-creator` escribe stub mínimo si ausente | Copia manual desde `SddIA/scripts/starter-kit/.SddIA/local.paths.json` | Materializar starter-kit completo en fase Topología (no `{}`) |
| **F-DEP-05** | Bóveda `/home/racso/Proyectos/.dev` sin bloque Telegram | Usuario pidió config desde `.dev` raíz; Telegram solo en preprod-vault | Merge en vault staging: claves `TELEGRAM_*` desde preprod si faltan en `.dev` | Documentar inventario mínimo consumidor; wizard futuro (`DT-CONFIG-UX-ONBOARDING`) |
| **F-DEP-06** | Cola residual en `.events/domain/` + logs `Skip … max attempts (3)` | Eventos emitidos durante ventana F-DEP-03 (rutas fallidas) | Drenaje tras fix de binarios; algunos eventos ya en DLQ / skip | Smoke post-ignición debe fallar si `route-domain*` no alcanza `success:true` en N eventos de laboratorio |

### 1.1 Comportamiento esperado vs observado (matriz §2.1 lab)

| Circuito | Esperado (Completo consumidor) | Observado en redeploy |
|----------|------------------------------|------------------------|
| WUI `:8766` | HTTP 200 | OK |
| `email-watcher` systemd | `active`, WD=`SddIA_AP` | OK tras F-DEP-01 |
| EDA enrutamiento | `route-domain` / `route-domain-event` OK | **KO** hasta F-DEP-03; **OK** después |
| Inbox WUI correo | Visible si `actionable` | **Vacío** — ver §4 (F-TRIAGE-*) |
| Telegram eferente | Poke si `actionable` | **Silenciado** — ver §4 |

### 1.2 Correcciones manuales aplicadas (no canónicas — deuda)

```text
1. build-release-bundle → /home/racso/Proyectos/SddIA_AP
2. Vault staging (.deploy-vault) desde Proyectos/.dev + preprod (constitution, codex, TELEGRAM)
3. instance-creator (skip_ignition)
4. Parche start-sddia.sh instancia (_ensure_orchestrator)
5. systemd %f con CORE_ROOT=instancia
6. cargo build forja + copia ELF frescos
7. Reinicio start-sddia + email-watcher@AP
```

Prohibido consolidar estos pasos como SSOT; deben absorberse en `instance-creator`, `build-release-bundle` y `start-sddia.sh` del Core.

---

## 2. Triaje de correo — lógica de extracción (punto 2)

**ID:** `F-TRIAGE-01` · **Deuda:** `DT-TRIAGE-SUBJECT-FALLBACK`

### 2.1 Incidente reproducible

Correo de prueba (2026-08-24 ~20:06 CEST):

- **Asunto:** `Reunión con Racso el 25/08/2026 a las 10:00`
- **UID IMAP:** `104579`
- **Proof:** `.SddIA/proofs/email-triaged/6e552199-416d-4043-91c9-fb1bae9e2057.json`
- **Veredicto emitido:** `passive` · `decision_path: llm`
- **Efecto:** WUI inbox vacío; Telegram sin poke (solo `actionable`)

El asunto **cumple** el contrato de `email-triage-matrix.md` (reunión + fecha extraíble). Tests existentes en `email_triage.rs` (`extract_actionable_from_encoded_meeting_subject`) demuestran que la extracción estructural funciona sobre ese patrón.

### 2.2 Causa en motor

En `classify_llm` (`SddIA/engine/execute-process/src/engine/handlers/email_triage.rs`), el fallback `extract_actionable_from_subject` **solo** se ejecuta si:

```text
verdict.is_empty()  OR  (verdict == "actionable" && datetime.is_none())
```

Si el LLM devuelve explícitamente `passive`, **no se reevalúa** el asunto aunque contenga reunión + `dd/mm/yyyy hh:mm`. Contradice el espíritu de la matriz: Triaje-C no concluyó → Clasificacion no debería **degradar** señales estructurales inequívocas del asunto.

### 2.3 Objetivo Kaizen

| Ítem | Acción |
|------|--------|
| **Regla** | Tras Clasificacion LLM, si `extract_actionable_from_subject(subject_plain)` devuelve `(title, datetime)` completo → elevar a `actionable` salvo regla C-* previa (`noise`) |
| **Prioridad** | Triaje-C `noise` > extracción estructural asunto > veredicto LLM ambiguo |
| **Handler** | `email_triage.rs` — fase Clasificacion / post-LLM |
| **Tests** | Caso UID 104579; LLM mock `{"verdict":"passive"}` + asunto reunión → `actionable` |
| **Gate** | Correo reunión → proof `actionable` → WUI inbox + Telegram poke |

Mutación vía proceso `bug-fix` o `feature` + `entity-manager` (no edición manual del genoma fuera de forja).

---

## 3. Triaje de correo — calidad LLM / inferencia (punto 3)

**ID:** `F-TRIAGE-02` · **Deuda:** `DT-TRIAGE-LLM-QUALITY`

### 3.1 Observación

En el proof `6e552199-…`:

```json
"thermodynamic_cost": { "duration_ms": 0, "tokens_in": 0, "tokens_out": 0 },
"verdict": "passive",
"decision_path": "llm"
```

Peaje cero sugiere **sin inferencia real** (`mayeuta-llm` / `SDDIA_LLM_*`): respuesta instantánea o mock que devuelve `passive` por defecto, sin consumir tokens.

Bóveda instancia incluye `SDDIA_LLM_CLI_COMMAND` y `SDDIA_LLM_INFER_COMMAND` (desde `/home/racso/Proyectos/.dev`), pero la clasificación no reflejó el acto implícito en el asunto.

### 3.2 Objetivos Kaizen

| Ítem | Acción |
|------|--------|
| **Auditoría runtime** | Verificar invocación efectiva de `mayeuta-llm` en `email-triage-gateway` (logs sin secretos; `tokens_*` > 0 en reuniones ambiguas) |
| **Contrato bóveda** | Documentar en ONBOARDING/bundle: LLM **recomendado** para Clasificacion; sin LLM → solo Triaje-C + extracción asunto (§2) |
| **Hard-fail opcional** | `SDDIA_LLM_REQUIRE_INFER=1` en consumidor: si inferencia falla, no emitir `passive` silencioso — degradar a extracción asunto o marcar `classification-degraded` en proof |
| **Prompt / matriz** | Alinear prompt Clasificacion con `email-triage-matrix.md`: reunión/cita con fecha en asunto → candidato `actionable` pendiente extracción |
| **Smoke consumidor** | Tras redeploy: 1 correo reunión → `actionable` + agenda + Telegram (Gate G5 redeploy) |

### 3.3 Relación con UX (F-TRIAGE-03)

**ID:** `F-TRIAGE-03` — Inbox WUI (`GET /api/email-inbox`) filtra **solo** `verdict=actionable` (`kalma2-bridge.rs`). Correo clasificado `passive` **no aparece** aunque el triaje fue correcto según negocio.

- **Comportamiento actual:** coherente con ensayo Paciente 0 (F-03 archivado).
- **Fricción usuario:** reunión visible solo en proofs, no en WUI ni Telegram.
- **Derivación opcional:** PBI UX «inbox pasivo / historial triaje» (distinto de `DT-CONFIG-UX-ONBOARDING`).

---

## 4. Evidencia del ensayo de correo (First Blood redeploy)

| Campo | Valor |
|-------|--------|
| `message_uid` | `104579` |
| `event_id` (proof) | `6e552199-416d-4043-91c9-fb1bae9e2057` |
| `subject` (decodificado) | Reunión con Racso el 25/08/2026 a las 10:00 |
| `verdict` | `passive` |
| `decision_path` | `llm` |
| `.eml` | `.SddIA/inbox/104579.eml` |
| Agenda | **Ausente** (solo `actionable` persiste) |
| WUI | «Sin fricción accionable.» |
| Telegram | Sin poke (`route_domain_core`: `verdict != actionable` → skip) |

Instancia al snapshot §0ter: **100** proofs · **83** `noise` · **17** `passive` · **0** `actionable` · **101** `.eml` · **0** agenda · **3** dead-letter.

**Cadena causal verificada (correo reunión):**

```text
email-watcher (systemd, WD=SddIA_AP)
  → Email_Received (UID 104579)
  → event-watcher → route-domain-event / route-domain
  → email-triage-gateway (Triaje-C skip → Clasificacion llm)
  → Email_Triaged verdict=passive
  → proof 6e552199-… · WUI/Telegram omitidos (≠ actionable)
```

---

## 5. Objetivos de arquitectura (alcance Kaizen)

### 5.1 Despliegue (F-DEP-*)

1. `instance-creator`: `CORE_ROOT=%instance_root` en plantillas systemd; starter-kit completo en Topología.
2. `build-release-bundle`: gate binarios frescos; rechazar ELF con referencias legacy `.py`.
3. `start-sddia.sh`: rama bundle-safe (sin `cargo build` si ELF resuelto).
4. Smoke post-ignición: enrutar ≥1 evento domain de laboratorio con `success:true` (no solo topología de dirs).

### 5.2 Triaje (F-TRIAGE-*)

1. Fallback extracción asunto post-LLM (§2).
2. Calidad / trazabilidad inferencia LLM (§3).
3. (Opcional) Superficie WUI para `passive` informativo.

---

## 6. Criterios de cierre

Cierre T6 2026-08-25: `docs/audits/kaizen-paciente0-redeploy-20260825.md`. O11 (`delivery-close-cycle` / PR) pendiente.

### Despliegue

- [x] Redeploy Paciente 0 **solo** vía `instance-creator` + bundle, sin parches manuales en `{instancia}/start-sddia.sh`.
- [x] `systemd` `%f` con `ExecStart` bajo `{instancia}/SddIA/` sin intervención operador.
- [x] `build-release-bundle --skip-build` rechazado si ELF stale (gate F-DEP-03).
- [x] Smoke: `route-domain*` OK en instancia recién creada. (`Email_Triaged` `413e6edf-…` enrutado; probe clase no ECST no es centinela)

### Triaje

- [x] Correo reunión (patrón asunto §4) → `verdict=actionable`, asiento agenda, WUI inbox. Telegram poke: `message_id=9` (instancia).
- [x] Test unitario: LLM `passive` + asunto reunión → `actionable` vía extracción (§2).
- [x] Proof con `tokens_in/out` documentados cuando LLM activo, o flag `classification-degraded` si inferencia omitida. (G5: tokens 0 + `subject_elevation`; código `REQUIRE_INFER` presente)

---

## 7. Orden sugerido de forja

```text
(1) F-DEP-03 gate bundle + rebuild centinelas en CI
(2) F-DEP-01 + F-DEP-04 en instance-creator
(3) F-DEP-02 start-sddia bundle-safe
(4) F-TRIAGE-01 extracción asunto post-LLM
(5) F-TRIAGE-02 auditoría mayeuta-llm + REQUIRE_INFER consumidor
(6) Redeploy smoke Paciente 0 + Gate G5 reunión
(7) F-TRIAGE-03 UX (opcional, PBI separado)
```

Mutación de genoma: proceso `feature` / `bug-fix` → `entity-manager`. UUID PBI: `56aff1d3-d5f6-4502-9b5b-e5a57dc718e3`.

---

## 8. Referencias

| Ref | Uso |
|-----|-----|
| `docs/todos/done/[LABORATORIO] MVP Paciente 0 SddIA_AP.md` | Ensayo original, gates G0–G5, F-03 |
| `docs/todos/done/[KAIZEN] perfil ignición consumidor Filtro C.md` | instance-creator, bundle, F-06–F-09 |
| `SddIA/norms/sddia-distribution-protocol.md` | Vía C v1.2.0 |
| `SddIA/process/instance-creator.md` | Fases Topologia–Smoke |
| `SddIA/library/norms/email-triage-matrix.md` | Matriz tres vías |
| `SddIA/engine/execute-process/src/engine/handlers/email_triage.rs` | Triaje-C, Clasificacion, extracción |
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | Filtro inbox `actionable` |
| `SddIA/core/event-domain-subscriptions.json` | `Email_Triaged` → Telegram |
| `/home/racso/Proyectos/SddIA_AP/` | Instancia redeploy 2026-08-24 |
| `/home/racso/Proyectos/.dev/.env` | Bóveda personalizada inyectada |
| `/home/racso/Proyectos/SddIA_AP.deploy-vault` | Vault staging del redeploy |
| `/home/racso/Proyectos/SddIA_AP/MANIFEST.json` | Manifiesto bundle consumer |
| `/home/racso/Proyectos/SddIA_AP/ONBOARDING.md` | Proyección autogenerada bundle |
| `/home/racso/Proyectos/SddIA_AP/.SddIA/daemons/logs/start-sddia.log` | Log ignición (F-DEP-02/03) |


## 9. al finalizar PBI, generar documento auditoria indicando lo relevante de la prueba empirica de despliegue paciente 0, con contenido relevante de kaicen y aquella otra información que pueda ser relevante a consultar en el futuro.

**Hecho (2026-08-25):** `docs/audits/kaizen-paciente0-redeploy-20260825.md` (`document_id: AUDIT-KAIZEN-PACIENTE0-REDEPLOY-20260825`). Evolution: `SddIA/evolution/916bf0f9-05ea-4408-8b6e-294e7efcc5f9.md`.