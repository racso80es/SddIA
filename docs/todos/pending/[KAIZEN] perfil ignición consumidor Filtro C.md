---
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
uuid: "1c70e777-9b7f-4ad3-ada5-225ab6d141c6"
title: "[KAIZEN] Ignición Consumidor: Poda Filtro C, Empaquetado (Release Bundle) y Diagnóstico Local"
format: markdown
version: "0.2.1"
status: pending
type: kaizen
priority: alta
created: "2026-08-20"
updated: "2026-08-20"
derived_from: PBI-LAB-PACIENTE0-SDDIA-AP
preprod_vault_backup: "/home/racso/Proyectos/SddIA_AP.preprod-vault"
preprod_instance_path: "/home/racso/Proyectos/SddIA_AP"
tech_debt_ids:
  - DT-START-SDDIA-CONSUMER-PROFILE
  - DT-SYSTEMD-FULL-COVERAGE
friction_ids:
  - F-04
  - R-07
  - F-06
  - F-07
  - F-08
  - F-09
---

# [KAIZEN] Evolución de Despliegue: Perfil Consumidor, Empaquetado y Diagnóstico

## 0. Validación del refinamiento (anti-alucinación)

| Afirmación propuesta | Dictamen | Corrección / ancla |
|----------------------|----------|-------------------|
| Existe `sddia-daemons@<instancia>.service` | **Parcial** | En host hay `sddia-daemon@.service` + `sddia-daemons.target`, pero **no** parametrizan por raíz de instancia (WD fijo al lab). `sddia-email-watcher@` sí usa `%f`. Objetivo: unificar patrón hermético multi-cliente (§2.5, F-08). |
| `WorkingDirectory` → raíz `.SddIA/` | **Incorrecto** | Plantilla email: `WorkingDirectory=%f` = **raíz de instancia**. Binding obligatorio = raíz de instancia, no `.SddIA/`. |
| Empaquetar solo «Cerbero, Cúmulo, Mayeuta» | **Impreciso** | Binarios runtime + cápsulas/skills del códice + contratos/normas. |
| Extirpar *todo* `.rs`/`.py` | **Aspiracional / faseado** | Meta: cero fuentes de ingeniería; lanzadores mínimos hasta paridad Rust. |
| `build-release-bundle.sh` ya existe | **No** | Entidad a forjar. |
| Smoke `eda-local-topology-test` / `Local_QA_Requested` | **Parcialmente existente** | Reutilizar/adaptar; no reinventar. |
| `agenda-manager` = binario eferente | **Inexacto** | Skill `agenda:persist`. |
| Tres `email-watcher` + watermark estancado | **Observado** | R-07 + F-07. |
| Falta `send-telegram-notification` | **Observado (mitigado en ensayo)** | F-06; tras build local el poke E2E funcionó. |
| Constitución L2 Windows/pwsh en Linux | **Fósil** | F-09. |
| Centinelas globales compartidos | **F-08 en host** | `sddia-daemon@*` WD=`…/SddIA` (lab). |

**UUID / document_id:** inmutables.

---

## 1. Origen y destilación de fricción

| ID | Fricción | Evidencia |
|----|----------|-----------|
| **F-04** | Derrame ontológico / Filtro C | Fracture → enrich/materialize fracture en consumidor |
| **R-07** | Colisión receptores | `start-sddia.sh` + `sddia-email-watcher@…AP` |
| **F-06** | Binario eferente ausente | Telegram DLQ hasta `cargo build -p send-telegram-notification` |
| **F-07** | Catch-up IMAP ≠ últimos 50 | `last_uid=0` + lote lookback |
| **F-08** | Systemd no hermético multi-cliente | `sddia-daemon@*` WD lab fijo vs `email-watcher@%f` |
| **F-09** | Constitución fósil | `L2_ENV` Windows+pwsh en instancia Linux |
| — | Entropía de código | Trasplante ~**1,7 GiB** (mayoría `SddIA/target` + fuentes) |
| — | Ceguera post-despliegue | Sin smoke determinista obligatorio |

También: `github-bridge-watcher` en ignición script; WUI «Forjar Proceso» (`DT-START-SDDIA-CONSUMER-PROFILE`).

---

## 1bis. Entorno de preproducción (rescate Paciente 0)

Bóveda y metadatos del ensayo se tratan como **perfil preprod**. Secretos **fuera de git**:

| Ítem | Valor |
|------|-------|
| Backup | `/home/racso/Proyectos/SddIA_AP.preprod-vault/` |
| Contenido | `instance.SddIA.dev.env`, `root.dev.env`, `constitution/`, `codexes/`, `env-keys.inventory.txt`, `README.md` |
| product / workspace_id | `SddIA_AP` / `sddia-ap-paciente-0` |
| WUI preprod | `SDDIA_CLIENT_PORT=8766` |
| Códice | `codex-kalma2-assistant` (`sync-client-assets` / PEC `sddia-ap-lab-sync-001` success) |
| Systemd AP | `sddia-email-watcher@home-racso-Proyectos-SddIA_AP.service` |
| Filtro C Git | `core.hooksPath=/dev/null`; `.husky.disabled-filtro-c` |

### Claves bóveda (checklist rehidratación, sin valores)

**Instancia (prevalece):** `TELEGRAM_BOT_TOKEN`, `TELEGRAM_ALLOWED_CHAT_ID`, `SDDIA_LLM_*`, `SDDIA_EMAIL_IMAP_*`, `SDDIA_EMAIL_MAILBOX`, `SDDIA_EMAIL_POLL_SECONDS`, `SDDIA_EMAIL_SNIPPET_CHARS`, `SDDIA_EMAIL_INITIAL_LOOKBACK_DAYS`, `SDDIA_EMAIL_MAX_UIDS_PER_POLL`, `SDDIA_AGENT_RUNTIME_*`, `SDDIA_CLIENT_PORT`.

**Raíz:** subset LLM + IMAP + AGENT_RUNTIME + `SDDIA_ENV` + `SDDIA_CLIENT_PORT` (Telegram solo en instancia).

### Métricas al cierre del ensayo

| Métrica | Valor |
|---------|-------|
| inbox `.eml` | 55 |
| proofs | 55 (`noise` 50, `passive` 4, `actionable` 1) |
| agenda | 3 |
| Binarios al cierre | daemons EDA + `kalma2-bridge` + `mayeuta-llm` + `send-telegram-notification` + `github-raw-fetcher` |
| E2E actionable | WUI OK; Telegram OK post-F-06 |

### Re-despliegue (post-kaizen)

1. Cerrar al menos R-07 + F-06 (checklist bundle) + F-07 antes de materializar de nuevo.
2. Restaurar bóveda desde `SddIA_AP.preprod-vault` (README del backup).
3. Re-render/enable unidad systemd solo con instancia recreada.
4. Puerto **8766**; no compartir token Telegram ni app-password IMAP con lab forja.
5. Corregir constitución L2 (F-09) en plantilla consumidor.

---

## 2. Objetivos de arquitectura

### 2.1 Poda de perfil consumidor (Filtro C)

1. Sin `github-bridge-watcher` en perfil consumidor.
2. WUI sin «Forjar Proceso» usable.
3. Sin suscriptores de forja ante Fracture si no hay códice de ingeniería.
4. **Anti-colisión R-07:** con systemd sensorial, `start-sddia.sh` no spawnea `email-watcher`/`telegram-watcher`. Un solo escritor de watermark por instancia.

### 2.2 Ignición IMAP — últimos 50

Primer catch-up: solo **50 UIDs más recientes**; watermark = max del lote (F-07).

### 2.3 Encapsulamiento físico (Release Bundle)

- Forjar `build-release-bundle`.
- Runtime + códice + cápsulas del grafo de capacidades (F-06); sin fuentes de ingeniería ni deps de build.
- Verificación: `send-telegram-notification` presente tras bundle con `codex-kalma2-assistant`.

### 2.4 Smoke local

Reutilizar `eda-local-topology-test` + `Local_QA_Requested`; gate `success: true`.

### 2.5 Instanciación hermética multi-cliente

1. Prohibidos centinelas globales compartidos (F-08).
2. Unidades parametrizadas por raíz de instancia (`%f`) para **todos** los daemons (evolucionar `sddia-daemon@` lab-fijo → patrón tipo `email-watcher@`).
3. `WorkingDirectory` = raíz instancia; `EnvironmentFile` = `%f/.SddIA/.dev/.env`.
4. `start-sddia.sh` solo sobre la carpeta ejecutora.

---

## 3. Fuera de alcance

- Wizard UX (`DT-CONFIG-UX-ONBOARDING`).
- Sustituir Kalma2 WUI completa.
- Dominios de negocio ajenos al perfil consumidor.

`DT-SYSTEMD-FULL-COVERAGE` **absorbida** por §2.5.

---

## 4. Criterios de cierre

### Filtro C / R-07 / F-09

- [ ] Sin github-bridge / Forjar Proceso / suscriptores forja en consumidor.
- [ ] Gate fracture sintética → cero procesos de ingeniería.
- [ ] Systemd sensorial ⇒ `start-sddia` no duplica sensores (R-07).
- [ ] Constitución consumidor sin L2 Windows fósil (F-09).
- [ ] Docs starter-kit / constitución local.

### IMAP últimos 50

- [ ] Primer poll: máx. 50 UIDs más altos; watermark = max.

### Release bundle + F-06

- [ ] Bundle sin fuentes de ingeniería.
- [ ] Cápsulas del códice incluidas (`send-telegram-notification` verificable).
- [ ] E2E actionable → Telegram + WUI sin DLQ por binario ausente.

### Smoke + multi-cliente (F-08)

- [ ] Smoke `success: true` en paciente fresco.
- [ ] Dos instancias: WD distintos; sin locks/credenciales cruzados.
- [ ] `sddia-daemon@*` (o sucesor) no queda atado a una sola ruta lab.

---

## 5. Notas de forja

Vía proceso `feature`/`kaizen`. UUID `1c70e777-9b7f-4ad3-ada5-225ab6d141c6` en evolution al cerrar.

**Orden:** (1) R-07 + F-07 + Filtro C, (2) F-06 bundle, (3) smoke, (4) F-08/F-09 systemd + constitución.

---

## 6. Referencias

| Ref | Uso |
|-----|-----|
| `docs/todos/done/[LABORATORIO] MVP Paciente 0 SddIA_AP.md` §11 | Ensayo |
| `/home/racso/Proyectos/SddIA_AP.preprod-vault/` | Bóveda preprod (no git) |
| `SddIA/templates/systemd/sddia-email-watcher@.service.template` | SSOT `%f` |
| `~/.config/systemd/user/sddia-daemon@.service` | Patrón lab actual (F-08) |
| `start-sddia.sh` | Ignición híbrida |
| `local-qa-requested` / `eda-local-topology-test` | Smoke |
| `codex-kalma2-assistant` / `send-telegram-notification` / `agenda-manager` | Bundle |
