---
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
uuid: "1c70e777-9b7f-4ad3-ada5-225ab6d141c6"
title: "[KAIZEN] Ignición Consumidor: Forja de Instancias, Empaquetado y Diagnóstico Local"
format: markdown
version: "0.3.0"
status: done
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

# [KAIZEN] Evolución de Despliegue: Forja de Instancias y Perfil Consumidor

## 0. Validación del refinamiento v0.3.0 (anti-alucinación)

| Afirmación del borrador | Dictamen | Corrección / ancla |
|-------------------------|----------|-------------------|
| `sddia-daemons@<instancia>.service` como SSOT | **Propuesta** | Hoy: `sddia-email-watcher@%f` (hermético) + `sddia-daemon@*` con WD **lab fijo** (F-08). Nombre final en forja. |
| `instance-creator` ya existe | **No** | 0 hits en repo; proceso a forjar vía entity-manager. |
| CLI `sddia execute-process instance-creator` | **Inexacto como string** | Canal real: `execute-process` / `./sddia-run.sh` (JSON stdin). Criterio: invocación canónica del proceso, no inventar binario `sddia`. |
| Crear `SddIA/norms/sddia-distribution-protocol.md` | **Ya existe** | Norma Vía C v1.0.0 presente. Acción = **evolucionar** (release bundle, instance-creator, ONBOARDING), no duplicar. Frontmatter `uuid` actual no es UUID v4 → rehabilitar en forja. |
| Empaquetar «Nodos de Control» (Cerbero/Cúmulo/Mayeuta) | **Impreciso** | Runtime = binarios + cápsulas/skills del códice + contratos. Roles ontológicos ≠ set de binarios. |
| Extirpar todo `.rs`/`.py` | **Faseado** | Meta: cero fuentes de ingeniería; lanzadores mínimos hasta paridad Rust. |
| Derrame = «Tékton» en Paciente 0 | **Parcial** | Observado: `github-bridge` en script, «Forjar Proceso», suscriptores Fracture (`enrich`/`materialize`). Tekton como agente no fue el emisor principal en el ensayo. |
| F-06 / F-07 / F-08 / F-09 / preprod | **Conservar** | Ya anclados en v0.2.1; el borrador 0.3.0 los omitía → reincorporados. |
| IMAP últimos 50 | **Conservar** | F-07; no estaba en borrador 0.3.0. |
| Estrategia tripartita (norma + proceso + ONBOARDING) | **Nueva / coherente** | Encaja con Vía C + DT-CONFIG (onboarding generado ≠ wizard UX). |
| Smoke solo vía instance-creator | **Ajuste** | Reusar `eda-local-topology-test` + `Local_QA_Requested`; instance-creator los **orquesta** post-ignición. |

**UUID / document_id:** inmutables (`1c70e777-…`).

---

## 1. Origen y destilación de fricción

Frontera Core ↔ Cliente (ensayo Paciente 0 + wipe 2026-08-20):

| ID | Fricción | Evidencia |
|----|----------|-----------|
| **F-04** | Derrame ontológico / Filtro C | Fracture → enrich/materialize; WUI forja; script intenta github-bridge |
| **R-07** | Colisión sensorial | `start-sddia.sh` + systemd `email-watcher@…AP` (PIDs duplicados) |
| **F-06** | Resolución opaca de cápsulas | `send-telegram-notification` ausente → DLQ; build local lo mitigó |
| **F-07** | Catch-up IMAP ≠ últimos 50 | `last_uid=0` + lote lookback |
| **F-08** | Systemd no hermético multi-cliente | `sddia-daemon@*` WD lab vs `email-watcher@%f` |
| **F-09** | Constitución fósil | L2 Windows+pwsh en Linux |
| — | Entropía física | Trasplante ~1,7 GiB (fuentes + `target` + docs) |
| — | Obsolescencia documental | Despliegue por manual estático / checklist; sin paridad binario↔docs |

---

## 1bis. Entorno de preproducción (rescate Paciente 0)

Perfil preprod. Secretos **fuera de git**:

| Ítem | Valor |
|------|-------|
| Backup | `/home/racso/Proyectos/SddIA_AP.preprod-vault/` |
| Contenido | `instance.SddIA.dev.env`, `root.dev.env`, `constitution/`, `codexes/`, `env-keys.inventory.txt`, `README.md` |
| product / workspace_id | `SddIA_AP` / `sddia-ap-paciente-0` |
| WUI | `SDDIA_CLIENT_PORT=8766` |
| Códice | `codex-kalma2-assistant` (PEC sync success) |
| Instancia wipe | `/home/racso/Proyectos/SddIA_AP` eliminada; unidad AP `disabled` |

### Claves bóveda (rehidratación)

**Instancia:** Telegram, `SDDIA_LLM_*`, `SDDIA_EMAIL_*`, `SDDIA_AGENT_RUNTIME_*`, `SDDIA_CLIENT_PORT`.  
**Raíz:** subset LLM/IMAP/RUNTIME/`SDDIA_ENV`/PORT (Telegram solo instancia).

### Métricas ensayo

55 `.eml` / 55 proofs (`noise` 50, `passive` 4, `actionable` 1) / agenda 3 / E2E actionable WUI+Telegram post-F-06.

### Re-despliegue

Preferente vía **`instance-creator`** + bundle hermético (este PBI). Fallback manual: restaurar bóveda preprod + puerto 8766 + F-09 constitución.

---

## 2. Objetivos de arquitectura

### 2.1 Poda de perfil consumidor (Filtro C)

1. Orquestador / `start-sddia.sh`: sin `github-bridge-watcher` en perfil consumidor.
2. WUI: sin «Forjar Proceso» usable.
3. Sin enrutamiento Fracture → herramientas de forja si no hay códice de ingeniería.
4. Constitución consumidor sin L2 Windows (F-09).

### 2.2 Aislamiento sensorial y resolución dinámica

1. **Anti-colisión R-07:** si la jurisdicción sensorial es systemd (unidades `@%f` / sucesor multi-daemon hermético), `start-sddia.sh` **no** spawnea `email-watcher`/`telegram-watcher`. Un watermark writer por instancia.
2. **IMAP últimos 50 (F-07):** primer catch-up = 50 UIDs más recientes.
3. **Resolución de cápsulas (F-06):** el build/bundle lee el códice inyectado y compila/empaqueta dependencias eferentes (tools + skills del grafo).

### 2.3 Empaquetado hermético (Release Bundle)

- Forjar `build-release-bundle` (script y/o cápsula).
- Contenido: binarios runtime, bus inerte, contratos/códice necesarios, cápsulas del grafo — **sin** fuentes de ingeniería ni deps de build.
- Generar automáticamente **`ONBOARDING.md`** del cliente con paridad respecto al artefacto entregado (pasos, puertos, variables, systemd).
- Extirpación total `.rs`/`.py`: meta faseada.

### 2.4 Persistencia del despliegue (estrategia tripartita)

| Capa | Artefacto | Acción |
|------|-----------|--------|
| Norma canónica | `SddIA/norms/sddia-distribution-protocol.md` | **Evolucionar** Vía C: bundle hermético, instance-creator, ONBOARDING, multi-cliente `%f` |
| Motor ejecutable | proceso `instance-creator` (Rust vía `execute-process`) | Instanciar topología `.SddIA/`, inyectar secretos (desde vault/plantilla), registrar systemd hermético, ignitar daemons |
| Proyección consumidor | `ONBOARDING.md` en el bundle | Paridad absoluta con binarios/versiones entregados |

### 2.5 Verificación de vida (smoke)

- `instance-creator` (o fase final del proceso) ejecuta **`eda-local-topology-test`** / estímulo **`Local_QA_Requested`** post-ignición.
- Certifica bus (p. ej. latido / reacción `event-sweeper`), presencia de cápsulas del códice y carga de bóveda **sin** filtrar secretos a logs.
- Gate: `success: true`.

### 2.6 Instanciación hermética multi-cliente (F-08)

1. Prohibidos centinelas globales compartidos.
2. Toda unidad con `WorkingDirectory` = raíz de instancia (`%f`); `EnvironmentFile` = `%f/.SddIA/.dev/.env`.
3. Evolucionar `sddia-daemon@` (WD lab) al patrón parametrizado por instancia.
4. `start-sddia.sh` / `instance-creator` solo operan sobre la carpeta objetivo.

---

## 3. Fuera de alcance

- Wizard UX interactivo (`DT-CONFIG-UX-ONBOARDING`); el `ONBOARDING.md` generado **no** lo sustituye como producto, pero reduce dependencia de manuales estáticos.
- Sustituir Kalma2 WUI completa.
- Dominios de negocio ajenos al perfil consumidor.

`DT-SYSTEMD-FULL-COVERAGE` absorbida por §2.2 / §2.6.

---

## 4. Criterios de cierre

### Filtro C / R-07 / F-09

- [ ] Consumidor sin github-bridge / Forjar Proceso / suscriptores forja.
- [ ] Gate fracture → cero procesos de ingeniería.
- [ ] Systemd sensorial ⇒ sin doble spawn desde script (R-07).
- [ ] Constitución sin L2 Windows (F-09).

### IMAP / cápsulas

- [ ] Catch-up = últimos 50 UIDs (F-07).
- [ ] Bundle incluye cápsulas del códice (`send-telegram-notification` verificable) (F-06).

### Bundle + tripartita

- [ ] `build-release-bundle` → paquete hermético + `ONBOARDING.md` autogenerado alineado al artefacto.
- [ ] Norma `sddia-distribution-protocol` evolucionada (Vía C + bundle + instance-creator); UUID frontmatter válido.
- [ ] Proceso `instance-creator` despliega Paciente 0 operativo (rehidratación preprod opcional) vía `execute-process` / `sddia-run`.
- [ ] Post-ignición: smoke `success: true`.

### Multi-cliente (F-08)

- [ ] ≥2 instancias en el mismo host: cero PIDs/locks/credenciales cruzados; WD distintos.

---

## 5. Notas de forja

Mutación vía proceso `feature`/`kaizen` (entity-manager). UUID `1c70e777-9b7f-4ad3-ada5-225ab6d141c6` en evolution al cerrar.

**Orden sugerido:**  
(1) R-07 + F-07 + Filtro C runtime → (2) F-06 resolución cápsulas en bundle → (3) evolucionar `sddia-distribution-protocol` + `ONBOARDING.md` → (4) forjar `instance-creator` + smoke → (5) F-08/F-09 systemd + constitución → (6) re-despliegue Paciente 0 desde preprod vault.

---

## 6. Referencias

| Ref | Uso |
|-----|-----|
| `docs/todos/done/[LABORATORIO] MVP Paciente 0 SddIA_AP.md` §11 | Ensayo |
| `/home/racso/Proyectos/SddIA_AP.preprod-vault/` | Preprod (no git) |
| `SddIA/norms/sddia-distribution-protocol.md` | Vía C — evolucionar |
| `SddIA/templates/systemd/sddia-email-watcher@.service.template` | Patrón `%f` |
| `~/.config/systemd/user/sddia-daemon@.service` | F-08 lab-fijo |
| `start-sddia.sh` / `sddia-run.sh` | Ignición / execute-process |
| `local-qa-requested` / `eda-local-topology-test` | Smoke |
| `codex-kalma2-assistant` / `send-telegram-notification` / `agenda-manager` | Bundle |
| `sync-client-assets` | Inyección códice (precede/complementa instance-creator) |
