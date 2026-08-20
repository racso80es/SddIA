---
feature_name: kaizen-consumer-ignition-filtro-c
created: "2026-08-20"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C (perfil consumidor + ignición hermética)
branch_name: feat/kaizen-consumer-ignition-filtro-c
persist_ref: docs/features/kaizen-consumer-ignition-filtro-c
pbi_ref: docs/todos/pending/[KAIZEN] perfil ignición consumidor Filtro C.md
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
uuid: "1c70e777-9b7f-4ad3-ada5-225ab6d141c6"
execution_id: "9594b963-49a2-4ca0-8173-35ed0a986b63"
mayeuta_verdict: ok
laudo: perfil-consumidor-tripartita-via-c
---

# Clarificación — kaizen-consumer-ignition-filtro-c

Transcript Mayeuta (2026-08-20). Semilla PBI v0.3.0 (anti-alucinación §0) → requisito termodinámico. Filtro A contra genoma vigente; no implementar a ciegas.

Fuentes: PBI adjunto; `start-sddia.sh` L20–21; `interfaces/kalma2` (`Forjar Proceso`); `event-domain-subscriptions.json` (Fracture→enrich/materialize); `email-watcher` `uid_search_criterion` (`last==0` → `SINCE` lookback); plantilla `sddia-email-watcher@.service` (`WorkingDirectory=%f`); norma `sddia-distribution-protocol` v1.0.0 (uuid no-v4); 0 hits `instance-creator` / `build-release-bundle`; smoke `Local_QA_Requested` + tool `eda-local-topology-test`; preprod vault fuera de git.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 (Kaizen no tiene proceso hermano) |
| `feature_name` | `kaizen-consumer-ignition-filtro-c` |
| Rama | `feat/kaizen-consumer-ignition-filtro-c` |
| `persist_ref` | `docs/features/kaizen-consumer-ignition-filtro-c` |
| `document_id` / uuid | inmutables del PBI |
| `execution_id` | `9594b963-49a2-4ca0-8173-35ed0a986b63` |
| Fase | Estabilización Mayeuta (esta sesión) |
| Origen | Ensayo Paciente 0 + wipe 2026-08-20; deudas F-04/R-07/F-06–F-09 |

---

## D1 — Filtro C runtime (F-04)

| Semilla | Hecho SSOT | Laudo |
|---------|------------|-------|
| Sin `github-bridge-watcher` en consumidor | `OPTIONAL_DAEMONS=(telegram-watcher github-bridge-watcher)` en `start-sddia.sh` | Perfil **consumidor** excluye bridge; lab/core puede conservarlo bajo perfil ingeniería |
| Sin «Forjar Proceso» usable | Botón + `forjarProceso()` en `kalma2` | Ocultar/deshabilitar en perfil consumidor; no borrar el código del lab |
| Sin Fracture → forja | Suscriptores `enrich-fracture-pbi-kaizen` / `materialize-fracture-pbi` / `materialize-kaizen-alert-doc` | Gate por perfil/códice: consumidor **no** enruta a procesos de ingeniería; **prohibido** borrar suscriptores del Core lab |

**Toll:** Filtro C = proyección de perfil, no castración del genoma de ingeniería.

---

## D2 — Anti-colisión sensorial (R-07)

| Semilla | Hecho SSOT | Laudo |
|---------|------------|-------|
| Systemd ⇒ script no spawnea watchers | Plantilla email `@%f` hermética; script spawnea `email-watcher` + opcionales | Si jurisdicción sensorial activa (`sddia-email-watcher@…` / sucesor multi-daemon hermético) → `start-sddia.sh` **no** lanza `email-watcher` ni `telegram-watcher`. Un watermark writer por instancia |

Detección: unidad user activa cuyo `WorkingDirectory` = raíz de instancia, o flag explícito de bóveda (`SDDIA_SENSORIAL_JURISDICTION=systemd`). Preferir señal determinista documentada en Dedalo.

---

## D3 — IMAP catch-up (F-07)

| Semilla | Hecho SSOT | Laudo |
|---------|------------|-------|
| Primer catch-up = últimos 50 UIDs | `last==0` → `SINCE` + `initial_lookback_days` (default 60); `max_uids_per_poll=50` recorta lote, **no** selecciona los 50 más recientes | Cambiar criterio `last==0`: resolver los **50 UIDs máximos** del mailbox (o equivalente IMAP determinista), no ventana calendárica. Conservar `max_uids_per_poll` como techo de lote |

---

## D4 — Empaquetado y cápsulas (F-06)

| Semilla | Hecho SSOT | Laudo |
|---------|------------|-------|
| Bundle hermético + grafo de códice | 0 hits `build-release-bundle` | Forjar artefacto (script y/o cápsula) que lea códice inyectado y empaquete binarios + cápsulas eferentes del grafo (`send-telegram-notification` verificable) |
| Extirpar todo `.rs`/`.py` | Meta PBI | **Faseada**: este ciclo exige cero fuentes de ingeniería **en el bundle de entrega**; no exige borrar fuentes del repo upstream |

---

## D5 — Estrategia tripartita (norma + proceso + ONBOARDING)

| Capa | Semilla | Laudo |
|------|---------|-------|
| Norma | «Crear» protocolo | **Rechazado.** Evolucionar `SddIA/norms/sddia-distribution-protocol.md` (Vía C + bundle + instance-creator + ONBOARDING + multi-cliente `%f`). Rehabilitar `uuid` a UUID v4 en forja vía `entity-manager`/`norm-creator` |
| Motor | `instance-creator` | **CREATE** vía `entity-manager` (0 hits hoy). Invocación: `./sddia-run.sh --process instance-creator`. Prohibido inventar binario `sddia` |
| Proyección | `ONBOARDING.md` | Generado **por** el bundle / instance-creator; paridad absoluta con artefactos entregados. **No** sustituye `DT-CONFIG-UX-ONBOARDING` (wizard) |

`sync-client-assets` precede/complementa; no lo absorbe este Kaizen como sustituto.

---

## D6 — Smoke post-ignición

| Semilla | Hecho SSOT | Laudo |
|---------|------------|-------|
| Smoke propio | Existen `eda-local-topology-test` + `Local_QA_Requested` | `instance-creator` **orquesta** esos activos post-ignición. Gate: `success: true`. Prohibido inventar segunda batería paralela |

---

## D7 — Multi-cliente systemd (F-08) + constitución (F-09)

| ID | Laudo |
|----|-------|
| **F-08** | Toda unidad sensorial/daemon de instancia: `WorkingDirectory=%f`, `EnvironmentFile=%f/.SddIA/.dev/.env`. Evolucionar `sddia-daemon@` lab-fijo al patrón parametrizado. Prohibidos centinelas globales compartidos entre clientes |
| **F-09** | Constitución de **instancia consumidor** sin L2 Windows+pwsh en hosts Linux. No reescribir `CONSTITUTION_CORE` del lab salvo proyección de plantilla consumidor |

---

## D8 — Fuera de alcance (invariantes)

- Wizard UX interactivo (`DT-CONFIG-UX-ONBOARDING`).
- Sustituir Kalma2 WUI completa.
- Dominios de negocio ajenos al perfil consumidor.
- Re-despliegue Paciente 0 desde preprod vault: **criterio de aceptación opcional** (demostración); no bloquea merge si smoke lab + dual-instancia F-08 pasan.

---

## D9 — Orden de forja (handoff Dedalo)

1. R-07 + F-07 + Filtro C runtime  
2. F-06 resolución cápsulas en bundle  
3. Evolucionar norma + generador `ONBOARDING.md`  
4. Forjar `instance-creator` + smoke  
5. F-08 / F-09 systemd + constitución consumidor  
6. (Opcional) re-despliegue Paciente 0 desde vault

**Mutación genoma:** solo `entity-manager` / creators. DA-4 topología activa. DA-5 fire-and-forget.

---

## Veredicto

`mayeuta_verdict: ok` · laudo `perfil-consumidor-tripartita-via-c`
