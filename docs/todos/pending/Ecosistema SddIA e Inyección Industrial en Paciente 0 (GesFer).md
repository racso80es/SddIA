---
document_id: PBI-KALMA2-MVP-01
uuid: "d7d00838-9ee6-472f-a164-95dcba2ceb80"
title: "[OPERATIVO] Paciente 0: Instanciación de Kalma2 MVP (Centinela de Correo + Sincronización de Activos)"
format: markdown
version: "2.1.0"
status: refinado-desglosado
role: umbrella
priority: alta
process: feature
feature_slug: kalma2-mvp-paciente-0
persist_ref: docs/features/kalma2-mvp-paciente-0
delivery_pbis:
  - id: PBI-KALMA2-MVP-01A
    uuid: "c209c150-8ab4-4f0d-bcf7-8fa7a6101de0"
    scope: "Circuito sensorial (T0-T5)"
  - id: PBI-KALMA2-MVP-01B
    uuid: "ed2f20b8-6e3d-4dbf-931c-d62e53ddf7c4"
    scope: "Sincronización de activos (T6-T8)"
ratifications:
  - id: R-01
    subject: "codex-contract v1.2.0 (bloque dlt opcional)"
    state: ratificado
  - id: R-02
    subject: "process_domain_roots += codex-kalma2-assistant/process"
    state: ratificado
depends_on:
  - id: PBI-SDDIA-DOMAIN-ABSTRACT-01
    state: resuelto
    evidence: "docs/todos/done/[ARQUITECTURA] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución.md"
supersedes_scope: "Paciente 0 = Kalma2 (GesFer relevado; ver Anexo Z)"
refined_by:
  - mayeuta
  - dedalo
  - cumulo
refined_at: "2026-08-17"
---

# [OPERATIVO] Paciente 0 — Kalma2 MVP: Autonomía Sensorial Periférica y Sincronización de Activos

## 0. Estado del refinamiento

| Hecho | Verificación |
|-------|--------------|
| Dependencia `PBI-SDDIA-DOMAIN-ABSTRACT-01` | **Resuelta** — PBI archivado en `docs/todos/done/` |
| Paciente 0 | **Kalma2** (GesFer relevado; Anexo Z conservado como legado histórico, fuera de alcance) |
| Puente cliente | **`SddIA/interfaces/kalma2-bridge/`** (Rust, `tiny_http`). `sddia-client-bridge.py` **no existe** (podado en feature `kalma2-bridge-rust`) |
| Genoma de triaje de correo | **Inexistente hoy**: sin clase ECST de email, sin Centinela de correo, sin tool de fetch HTTP genérico, sin códice Kalma2 |
| Refinamiento completo | `docs/features/kalma2-mvp-paciente-0/{spec,clarify,plan}.md` |
| Ratificaciones R-01 y R-02 | **Concedidas** por el Vértice Biológico (2026-08-17) |
| Rol de este documento | **Paraguas**: la ejecución se entrega vía `PBI-KALMA2-MVP-01A` y `01B` |

## 0.1 Desglose de entrega (ratificado 2026-08-17)

| PBI operativo | Alcance | Feature |
|---------------|---------|---------|
| `PBI-KALMA2-MVP-01A` | H1–H3 · circuito sensorial de correo (fases T0–T5) | `kalma2-mvp-sensorial-email` |
| `PBI-KALMA2-MVP-01B` | H4 · sincronización de activos (fases T6–T8) | `kalma2-mvp-sync-activos` |

Secuencia estricta: 01B no arranca hasta que 01A esté mergeado, porque su carga a sincronizar es el códice forjado en T2. Este paraguas se archiva cuando ambos cierran.

## 1. Propósito

Materializar la primera instanciación productiva de SddIA como Asistente Personal: el sistema asimila el flujo de correo del Vértice Biológico mediante un **Centinela Periférico lógicamente ciego**, aplica un **triaje de tres vías** cuya ley reside en un **activo descargable de la Librería** (`codex-kalma2-assistant`), y expone al cliente la capacidad de **reclamar la versión vigente de sus activos** desde el repositorio maestro (simulación DLT).

## 2. Alcance

### En alcance

| Hito | Entrega |
|------|---------|
| H1 | Centinela `email-watcher` (IMAP read-only, ceguera lógica) + clase ECST `Email_Received` |
| H2 | Códice `codex-kalma2-assistant` + norma táctica `email-triage-matrix` + proceso de triaje empacado en el códice |
| H3 | Triaje de 3 vías con prefiltro determinista antes de inferencia LLM + clase ECST `Email_Triaged` |
| H4 | Tubería de sincronización de activos: proceso `sync-client-assets` + acción `download-remote-asset` + cápsula `github-raw-fetcher` bajo DI (`asset:fetch`) + endpoint en `kalma2-bridge` + botón en la WUI |

### Fuera de alcance (explícito)

- Minteo real en IOTA Rebased (H4 simula el ledger vía `github-raw-fetcher`; el pivote a `iota-ipfs-fetcher` es una permuta de binding, no de proceso).
- `SddIA Forge` / `SddIA Portal` / paquete `@sddia/core` distribuible (Anexo Z, Fases 1 y 3).
- Cualquier mutación del buzón del Vértice Biológico (borrado, movimiento, marcado). MVP **read-only**.
- Wallet firmada / permisos criptográficos de descarga.

## 3. Hitos refinados

### H1 — Centinela Periférico (Tacto Inerte)

- Centinela `email-watcher` conforme a `daemons-contract v1.0.0`: **cero juicio**, cero invocación de `execute-process`, cero lectura de genoma.
- Única salida al sistema: instancia ECST `Email_Received` escrita en `./.events/domain/` del **workspace de instancia** (resuelto por `WorkingDirectory`), más `Daemon_Heartbeat` en `./.events/telemetry/`.
- Idempotencia por watermark de UID en `.SddIA/daemons/state/email-watcher.json`: morir y reiniciar no reprocesa correo antiguo.
- Payload ligero: el cuerpo íntegro **no** viaja por el bus; se persiste fuera de Git y el evento porta una referencia.

### H2 — Enrutamiento semántico y Códice

- `codex-kalma2-assistant.md` en `SddIA/library/codexes/` con Cicatriz Digital completa (`uuid` v4, SemVer, `nature: domain-codex`, `composition[]`, `hash_signature`) y bloque `dlt` preparado para minteo.
- La **ley de triaje** no reside en el Core: vive en la norma táctica `email-triage-matrix.md` compuesta por el códice, y el proceso de triaje se empaca bajo el directorio del códice (jurisdicción `process_domain_roots`).
- El Core solo enruta: `event-watcher` → `route-domain-event` → suscriptor declarado en `event-domain-subscriptions.json`.

### H3 — Triaje Entrópico de tres vías

| Vía | Veredicto | Efecto MVP |
|-----|-----------|------------|
| 1 | `noise` (Filtro C) | Descarte lógico. Registro en `Email_Triaged`. **Sin mutación del buzón** |
| 2 | `passive` | Notificación visible en la WUI de Kalma2 |
| 3 | `actionable` | Extracción estructurada + asiento en agenda local |

**Peaje termodinámico:** prefiltro determinista (heurística sobre remitente/asunto, sin LLM) resuelve la vía 1 antes de gastar inferencia. Solo el correo ambiguo escala a LLM.

### H4 — Sincronización de Activos (Simulador de Minteo)

- Cadena: WUI → `POST` en `kalma2-bridge` → `execute-process --process sync-client-assets` → acción `download-remote-asset` (abstracción de negocio, ignora el origen) → cápsula resuelta por **DI** vía capacidad `asset:fetch` → `filesystem-manager` sobrescribe el activo en `{instancia}/.SddIA/library/codexes/`.
- Validación de hash contra el `hash_signature` del activo remoto antes de sobrescribir. Hash discordante = abortar sin escribir.
- Pivote DLT futuro: sustituir el `provider` de `asset:fetch` en `capability-bindings.md`. Proceso y acción intactos.

## 4. Genoma a forjar

Toda entidad vía `execute-process --process entity-manager`. UUIDs finales los emite la forja.

| Familia | Entidad | Destino |
|---------|---------|---------|
| event (domain) | `email-received` | `SddIA/events/domain/` |
| event (domain) | `email-triaged` | `SddIA/events/domain/` |
| daemon | `email-watcher` | `SddIA/daemons/` + cápsula Rust |
| library norm | `email-triage-matrix` | `SddIA/library/norms/` |
| library codex | `codex-kalma2-assistant` | `SddIA/library/codexes/` |
| process (empacado) | `email-triage-gateway` | `SddIA/library/codexes/codex-kalma2-assistant/process/` |
| process | `sync-client-assets` | `SddIA/process/` |
| action | `download-remote-asset` | `SddIA/actions/` |
| tool | `github-raw-fetcher` | `SddIA/tools/` |
| skill | `agenda-manager` | `SddIA/skills/` |
| template | `sddia-email-watcher@.service.template` | `SddIA/templates/systemd/` |

Mutaciones de SSOT: `codex-contract` → v1.2.0 (bloque `dlt` opcional), `capability-bindings` (`asset:fetch`, `mail:triage`), `cumulo.paths.json` (`process_domain_roots`), `event-domain-subscriptions.json`, índices de familia.

## 5. Criterios de aceptación

- [ ] **Trazabilidad sin fugas:** correo entrante → `Email_Received` → triaje → `Email_Triaged` → visible en la WUI, sin intervención en terminal.
- [ ] **Ceguera espacial:** ninguna ruta absoluta del cliente aparece en `SddIA/`. El acoplamiento ocurre solo en runtime vía `WorkingDirectory` de systemd.
- [ ] **Ceguera lógica:** `email-watcher` no invoca `execute-process`, no lee genoma, no decide veredicto. Auditable por inspección de su cápsula.
- [ ] **Resiliencia:** el servicio sobrevive al bloqueo de sesión y resucita en <5 s tras `SIGKILL` (`Restart=always`, `RestartSec=5`).
- [ ] **Idempotencia:** reinicio del Centinela no genera eventos duplicados (watermark verificado).
- [ ] **Peaje termodinámico medible:** `Email_Triaged` porta veredicto, ruta de decisión (`deterministic` \| `llm`) y coste. Verbosidad comercial no secuestra la clasificación.
- [ ] **No destructividad:** cero operaciones de escritura IMAP en el MVP.
- [ ] **Sincronización íntegra:** el botón de la WUI actualiza el códice del cliente; hash discordante aborta sin escribir.
- [ ] **Pivote DLT sin fractura:** cambiar el `provider` de `asset:fetch` no requiere editar `sync-client-assets` ni `download-remote-asset`.
- [ ] **Soberanía de interacción:** la caja de prompt de Kalma2 coexiste con el flujo de correo en background sin degradar el servidor local.
- [ ] **Cicatriz Digital:** toda entidad nueva con `uuid` v4, SemVer, `contract`, `hash_signature` y fila en su `index.md`.

## 6. Reglas de Acero

1. **Ceguera Espacial** — el Core no conoce la ruta del cliente; opera sobre el Códice inyectado en `{instancia}/.SddIA/library/codexes/`.
2. **Identidad de Activo (NFT lógico)** — el Códice nace con estructura preparada para minteo (hash canónico + bloque `dlt` en estado `pre-mint`).
3. **Ausencia de alucinación** — nada de proveedores, SDKs ni servicios no pedidos. IMAP genérico configurado por entorno de instancia.
4. **Tubería hermética** — E/S de cápsulas estrictamente `capsule-json-io` schema 2.0 por stdin/stdout.
5. **Fire-and-Forget (DA-5)** — tras el acuse JSON del CLI, prohibido `sleep`/polling/`AwaitShell`.

---

# Anexo A — Sistema Nervioso Periférico (Centinela instanciado)

## A.1 Filosofía

Prohibidos scripts interactivos o emuladores de terminal atados a la sesión gráfica. El Tacto Inerte se instancia como **servicio parametrizado de systemd (usuario)**: un único binario vigila múltiples instancias de consumo, recibiendo la coordenada física como parámetro.

## A.2 Plantilla (correcciones frente a v1)

Tres defectos del borrador v1, corregidos:

| Defecto v1 | Corrección |
|-----------|-----------|
| `ExecStart=event-watcher` (ruta relativa) | systemd exige **ruta absoluta**; y `event-watcher` es un Centinela **ya existente** del bus EDA, no el de correo |
| Un solo `sddia-watcher@.service` genérico | Una plantilla por Centinela: `sddia-email-watcher@.service` |
| Ruta del Core cableada en genoma | El genoma publica un **template con marcador**; el operador lo renderiza en la capa OS del host |

Genoma: `SddIA/templates/systemd/sddia-email-watcher@.service.template`

```ini
[Unit]
Description=SddIA Email Watcher (Centinela Periférico) — Instancia: %f
Documentation=https://github.com/racso80es/SddIA
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
# %f decodifica la ruta escapada de la instancia de consumo.
# El Core nunca conoce este valor: lo inyecta systemd en el encendido.
WorkingDirectory=%f
EnvironmentFile=-%f/.SddIA/.dev/.env
Environment="SDDIA_ENV=production"
# Marcador renderizado por el operador en el host (ruta absoluta obligatoria).
ExecStart=@@SDDIA_CORE_ROOT@@/SddIA/daemons/email-watcher.sh
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=default.target
```

## A.3 Protocolo de ignición

```bash
# 1. Renderizado del template en la capa OS del host (fuera del genoma)
sed "s|@@SDDIA_CORE_ROOT@@|$(pwd)|g" \
  SddIA/templates/systemd/sddia-email-watcher@.service.template \
  > ~/.config/systemd/user/sddia-email-watcher@.service

# 2. Recarga de la matriz de demonios
systemctl --user daemon-reload

# 3. Ignición apuntando a la instancia de consumo
systemctl --user enable --now \
  "sddia-email-watcher@$(systemd-escape -p /ruta/absoluta/instancia).service"
```

## A.4 Credenciales

Exclusivamente en `{instancia}/.SddIA/.dev/.env` (jerarquía `env_hierarchy.instance`, fuera de Git):
`SDDIA_EMAIL_IMAP_HOST`, `SDDIA_EMAIL_IMAP_PORT`, `SDDIA_EMAIL_IMAP_USER`, `SDDIA_EMAIL_IMAP_SECRET`, `SDDIA_EMAIL_MAILBOX`, `SDDIA_EMAIL_POLL_SECONDS`.

Prohibido cualquier secreto en `SddIA/`.

---

# Anexo Z — Legado histórico (GesFer, relevado)

Conservado como registro arqueológico. **Ninguna de sus fases pertenece al alcance de este PBI**; las que sigan vivas se re-materializarán como PBIs propios.

| Fase | Contenido | Estado |
|------|-----------|--------|
| 1 | Extracción de `@sddia/core`, tubería `capsule-json-io`, cáscaras Forge/Portal | Parcialmente absorbido por `PBI-SDDIA-DOMAIN-ABSTRACT-01/02/03`; resto sin PBI |
| 2 | Simetría fractal `.SddIA/` + Centinelas en los 4 microservicios de GesFer | Relevado: el patrón fractal se valida ahora sobre Kalma2 |
| 3 | Forja de códices de dominio C# + Triaje Argos + minteo IOTA Rebased | Diferido a fase DLT |
| 4 | Inyección de códices en runtime, Caja de Ceguera Espacial, peaje termodinámico | Absorbido y reducido a MVP en H2/H4 |
