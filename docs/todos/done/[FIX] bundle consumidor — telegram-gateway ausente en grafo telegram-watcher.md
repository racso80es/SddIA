---
document_id: PBI-FIX-BUNDLE-TELEGRAM-GATEWAY
uuid: "67110f2f-2be8-4fd3-b0a7-8dc400fe803f"
title: "[FIX] bundle consumidor — telegram-gateway ausente en grafo telegram-watcher"
format: markdown
version: "1.0.0"
status: done
type: bug-fix
priority: alta
process: bug-fix
persist_ref: docs/fixes/bundle-consumer-telegram-gateway
created: "2026-08-26"
updated: "2026-08-26"
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/194
closed: "2026-08-26"
derived_from:
  - PBI-DT-PACIENTE0-DEPLOY-PROCESS
  - AUDIT-PACIENTE0-DEPLOY-20260826T110203Z
incident_ref: "Paciente 0 ola 5 — gateway rc=1; mensaje Telegram sin respuesta Tormentosa"
audit_ref: docs/audits/paciente0-deploy-20260826T110203Z.md
instance_path: /home/racso/Proyectos/SddIA_AP
bundle_manifest: "20260826T110203Z"
friction_ids:
  - F-BUNDLE-06
tech_debt_ids:
  - DT-BUNDLE-TELEGRAM-GATEWAY
blocks_on: []
---

# [FIX] bundle consumidor — `telegram-gateway` ausente en grafo `telegram-watcher`

## 0. Contexto

Empiría **2026-08-26** tras ola 5 de despliegue Paciente 0 (`AUDIT-PACIENTE0-DEPLOY-20260826T110203Z`). Gates G0–G4 del prompt **APTO**; instancia viva (`:8766`). Post-ola:

| Canal | Bot | Resultado |
|-------|-----|-----------|
| Correo → Telegram (`Email_Triaged` → `send-telegram-notification`) | Bot Paciente 0 (`TELEGRAM_BOT_TOKEN` distinto al lab) | **OK** — notificaciones llegan al bot correcto |
| Mensaje escrito al bot («sigues?») | Bot Paciente 0 | **KO** — sin respuesta Tormentosa/Aiúa |

Journal AP:

```text
[telegram-watcher] gateway rc=1 update_id=551975545
```

Reproducción:

```text
cápsula tool 'telegram-gateway' no encontrada bajo SddIA/target
```

`MANIFEST.json` ola 5: 7 bins / 7 capsules — incluye `telegram-watcher` y `send-telegram-notification`, **no** `telegram-gateway`. Lab forja tiene el ELF por `cargo build` completo; bundle hermético consumidor no lo empaqueta.

**Configuración:** `TELEGRAM_BOT_TOKEN` lab ≠ Paciente 0; `TELEGRAM_ALLOWED_CHAT_ID` compartido. No hay competencia `getUpdates` entre instancias por token distinto. El fallo conversacional **no** es dual-watcher ni stop de `sddia-daemon@telegram-watcher` (legado lab).

SSOT norma: `SddIA/norms/sddia-distribution-protocol.md` § F-06 (grafo eferente del códice). Gate actual verifica `send-telegram-notification` pero no la dependencia runtime de `telegram-watcher` → proceso `telegram-gateway` → tool homónima.

---

## 1. Fricción

| ID | Síntoma | Causa raíz | Ad-hoc (no persistir) | Acción |
|----|---------|------------|----------------------|--------|
| **F-BUNDLE-06** | `gateway rc=1`; cero `TelegramMessage_Received`; sin `telegram-fallback-responder` / Mayeuta | `build-release-bundle.sh` no incluye cápsula `telegram-gateway` aunque empaqueta `telegram-watcher` | Copiar ELF forja → `SddIA_AP/SddIA/target/release/` | **DT-BUNDLE-TELEGRAM-GATEWAY:** resolver grafo telegram en bundle + gate |

Cadena esperada (lab OK, Paciente 0 cortada en **G**):

```text
telegram-watcher → execute-process telegram-gateway → tool telegram-gateway
  → .events/domain TelegramMessage_Received
  → route-domain → telegram-fallback-responder → Mayeuta → send-telegram-notification
```

Cadena correo (Paciente 0 OK — distinta dependencia):

```text
email-watcher → Email_Received → email-triage-gateway → Email_Triaged
  → send-telegram-notification  (sí en bundle)
```

---

## 2. Alcance del fix (lab / forja)

Mutación vía ciclo `bug-fix` — **no** parchear `SddIA/` en instancia consumidor.

| Artefacto | Cambio propuesto |
|-----------|------------------|
| `SddIA/scripts/build-release-bundle.sh` | Incluir `telegram-gateway` en `CAPSULE_SET` / `cargo build` cuando perfil `consumer` y `telegram-watcher` ∈ bins; escribir testigo `.sha256` |
| Gate F-06 homólogo | Tras empaquetar: `test -x …/telegram-gateway` + `telegram-gateway.md` si `telegram-watcher` presente |
| Escaneo grafo (opcional P2) | Derivar eferentes de procesos invocados por daemons (`telegram-gateway` proceso desde watcher) además de suscripciones JSON |
| `instance-creator` smoke (opcional) | Anotar en smoke topology si falta cápsula crítica telegram |

**Fuera:** tokens Telegram distintos por instancia (ya correcto); proceso `paciente0-deploy`.

---

## 3. Criterios de aceptación

- [x] `build-release-bundle.sh --profile consumer` produce ELF `SddIA/target/release/telegram-gateway` y entra en `MANIFEST.json` `capsules_resolved`.
- [x] Gate bundle falla si `telegram-watcher` ∈ bins y `telegram-gateway` ausente.
- [x] `./sddia-run.sh --process telegram-gateway --inputs '{"text":"sigues?"}'` → `success:true`, `emitted:true` (forja).
- [ ] Mensaje de prueba al bot Paciente 0 → respuesta ≤2 líneas (pendiente redeploy AP).
- [x] `docs/audits/paciente0-deploy-20260826T110203Z.md` § post-ola marcado resuelto (forja).

---

## 4. Orden de forja

1. `./sddia-run.sh --process bug-fix` → `persist_ref` `docs/fixes/bundle-consumer-telegram-gateway`.
2. Parche `build-release-bundle.sh` + gate; `cargo build` witness.
3. Rebuild bundle → `instance-creator` o wipe `--out` Paciente 0.
4. Validar G-bundle ampliado + G-telegram (prompt DEUDA §5).
5. `delivery-close-cycle` → PR único.

---

## 5. Referencias

| Ref | Uso |
|-----|-----|
| `docs/audits/paciente0-deploy-20260826T110203Z.md` | Incidente post-ola |
| `docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md` | G-bundle / G-telegram |
| `SddIA/daemons/telegram-watcher/src/main.rs` | `invoke_gateway` → `--process telegram-gateway` |
| `SddIA/engine/execute-process/src/engine/handlers/telegram_gateway.rs` | `invoke_tool(..., "telegram-gateway")` |
| `SddIA/process/telegram-fallback-responder.md` | Respuesta Tormentosa |
| `SddIA/norms/sddia-distribution-protocol.md` | F-06 grafo eferente |
