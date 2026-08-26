---
document_id: PBI-FIX-EMAIL-WATCHER-IMAP-ACCOUNT-WATERMARK
uuid: "16239778-a5bc-4a55-8996-9301e51a6176"
title: "[FIX] email-watcher — watermark obsoleto al cambiar cuenta IMAP"
format: markdown
version: "1.0.0"
status: pending
type: bug-fix
priority: alta
process: bug-fix
persist_ref: docs/fixes/email-watcher-imap-account-watermark
created: "2026-08-26"
updated: "2026-08-26"
pbi_archived: false
incident_ref: "Auditoría IMAP forja 2026-08-26 — correo UID 5799 no detectado tras cambio de bóveda"
derived_from:
  - PBI-KALMA2-MVP-01A
  - docs/audits/paciente0-centinelas-email-sordo-20260826.md
friction_ids:
  - F-IMAP-WATERMARK-STALE
  - F-IMAP-ACCOUNT-CHANGE-SILENT
blocks_on: []
related_entities:
  - email-watcher
  - SddIA/daemons/email-watcher/src/main.rs
---

# [FIX] email-watcher — watermark obsoleto al cambiar cuenta IMAP

## 0. Contexto

Empiría **2026-08-26** (forja `SddIA`):

1. Se actualizó la bóveda IMAP (`SDDIA_EMAIL_IMAP_USER` / `SDDIA_EMAIL_IMAP_SECRET`) y se reinició `sddia-email-watcher@…`.
2. La conexión IMAP **autenticó correctamente** con la cuenta nueva.
3. Un correo de prueba (UID `5799`, asunto reunión 26/08/2026 12:50) **existía en INBOX** pero **no** generó `.eml`, `Email_Received` ni entrada en agenda.
4. Mitigación manual: reset `last_uid: 0` en `.SddIA/daemons/state/email-watcher.json` + poll `--once` → ingestión OK.

**Causa raíz:** el state conservaba `last_uid: 104466` de la **cuenta anterior** (2026-08-20). La cuenta nueva tiene UIDs `2639..5799`. El modo incremental filtra `uid > last_uid` → **cero candidatos**. El poll termina sin error visible (ceguera operativa).

No es fallo de triaje, bus ni systemd post-restart. Es **desalineación state ↔ espacio UID del buzón vigente**.

---

## 0bis. Adecuación del estímulo (anti-alucinación)

| Afirmación | Veredicto | Corrección |
|------------|-----------|------------|
| «Tras reiniciar el centinela, el watermark se recarga de bóveda» | **Falso** | `last_uid` vive en `.SddIA/daemons/state/email-watcher.json`, no en `.env`. |
| «Cambiar solo el app password invalida el watermark» | **Falso** | Misma cuenta → mismos UIDs; el watermark sigue válido. |
| «Cambiar `SDDIA_EMAIL_IMAP_USER` mantiene UIDs» | **Falso** | Cada buzón IMAP tiene su propio espacio UID. |
| «UNSEEN rescata correos con UID < last_uid» | **Falso** | `plan_poll_uids` filtra `unseen` con `u > last` (`main.rs`). |
| «Bootstrap solo ocurre con `last_uid=0` manual» | **Parcial** | Hoy sí; este FIX automatiza el reset ante cambio de identidad IMAP. |

---

## 1. Fricción

| ID | Síntoma | Causa raíz | Ad-hoc (hoy) | Acción |
|----|---------|------------|--------------|--------|
| **F-IMAP-WATERMARK-STALE** | Poll OK, cero ingestas tras cambio de cuenta | `last_uid` de buzón anterior > max UID del buzón nuevo | Reset manual JSON + `--once` | Persistir identidad IMAP en state; auto-bootstrap |
| **F-IMAP-ACCOUNT-CHANGE-SILENT** | Operador cree que el centinela «oye» tras editar bóveda | Sin log ni métrica de mismatch | Documentación oral | Log `[email-watcher] imap identity changed; resetting watermark` + evento telemetry opcional |

---

## 2. Diseño propuesto

### 2.1 Identidad IMAP en state

Extender `.SddIA/daemons/state/email-watcher.json`:

```json
{
  "mailbox": "INBOX",
  "last_uid": 5799,
  "imap_identity_sha256": "<hex>",
  "updated_at": "2026-08-26T10:53:19Z"
}
```

`imap_identity_sha256` = SHA-256 de la cadena normalizada:

```text
{host}|{port}|{mailbox}|{user}
```

(todo `trim`, `user`/`host`/`mailbox` en minúsculas; **sin** secret).

### 2.2 Reglas en `poll_once` (antes de `uid_search`)

1. Calcular `identity_now` desde `ImapCfg`.
2. Si el fichero state **no** tiene `imap_identity_sha256` (legado): calcularlo y persistir; **no** resetear solo por ausencia del campo.
3. Si `imap_identity_sha256` presente y **≠** `identity_now`:
   - Log stderr explícito (una línea por transición).
   - Tratar como bootstrap: `last := 0` en memoria (no borrar `.eml` históricos de otra cuenta en inbox — documentar; opcional: subdirectorio por identity en ola futura).
   - Tras poll exitoso, persistir `last_uid` + `imap_identity_sha256` nuevo.
4. **Heurística defensiva** (misma identidad): si `last_uid > 0` y el max UID del mailbox devuelto por `UID SEARCH ALL` es `< last_uid`, forzar bootstrap (`last := 0`) y log `watermark above mailbox ceiling`.

### 2.3 Fuera de alcance (este FIX)

- Migrar `.SddIA/inbox/*.eml` de cuenta anterior (UIDs 104xxx vs 5xxx coexisten hoy).
- Rotación automática de secret sin cambio de user (no requiere reset).
- Cambio de `SDDIA_EMAIL_IMAP_HOST` sin cambio de user en otro proveedor con UIDs distintos → **cubierto** por identidad (host entra en el hash).

---

## 3. Entregables

| # | Entrega | Ubicación |
|---|---------|-----------|
| D1 | Lógica identity + heurística ceiling | `SddIA/daemons/email-watcher/src/main.rs` |
| D2 | Tests unitarios: mismatch identity, ceiling, legado sin campo | `#[cfg(test)]` en mismo crate |
| D3 | Nota operador (cambio IMAP) | `start-sddia.md` § vars IMAP o `email-watcher.md` |
| D4 | Entrada evolución | `SddIA/evolution/<uuid-fix>.md` al cerrar |
| D5 | Dossier fix | `docs/fixes/email-watcher-imap-account-watermark/` (spec, validacion) |

Forja de código vía ciclo `bug-fix` + PR único (cierre documental en rama).

---

## 4. Criterios de aceptación

- [ ] Tras cambiar `SDDIA_EMAIL_IMAP_USER` en bóveda y **restart** del centinela (sin tocar JSON a mano), el **primer poll** ingesta correos del buzón nuevo (bootstrap F-07, lote ≤50).
- [ ] Tras cambiar **solo** `SDDIA_EMAIL_IMAP_SECRET` (mismo user/host/mailbox), el watermark **no** se resetea.
- [ ] State legado `{ last_uid, mailbox }` sin `imap_identity_sha256`: primer poll **no** dispara bootstrap masivo; solo persiste el campo nuevo.
- [ ] Si `last_uid` > max UID del mailbox (misma identidad corrupta), bootstrap automático + log.
- [ ] Tests `cargo test -p email-watcher` verdes.
- [ ] `validacion.md` global **APTO**, `pbi_archived: true`, PBI en `docs/todos/done/`.

---

## 5. Ensayo de regresión (lab)

Precondición: bóveda IMAP válida en `.SddIA/.dev/.env`.

1. State artificial `{ last_uid: 999999, mailbox: INBOX, imap_identity_sha256: "<cuenta A>" }`.
2. Bóveda apuntando a cuenta B (distinta user).
3. `email-watcher --once` → debe emitir ≥1 `Email_Received` de cuenta B y actualizar `last_uid` ≤ max UID real.
4. Repetir con cuenta A restaurada → nuevo reset y bootstrap.

Reproducción del incidente original: documentar UID de prueba y latencia ≤ `SDDIA_EMAIL_POLL_SECONDS` + 1 poll.

---

## 6. Kaizen

Fricción **nueva** respecto a A-06 (catch-up) y F-07 (bootstrap últimos 50): asume **continuidad del mismo buzón**. Cambio de cuenta rompe esa premisa sin guardia.

Relacionado con aislamiento multi-instancia (cada instancia tiene su state), pero el bug se manifestó en **una sola** raíz tras rotación de credenciales en bóveda.
