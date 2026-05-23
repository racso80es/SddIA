---
feature_name: pull-request-automation-dlt
created: "2026-05-23"
process: feature
purpose: Estabilización de requisitos — Oráculo Sensor DLT y activación validación PR remota
---

# Clarificación — Oráculo Sensor DLT (Activación Validación PR)

Transcript de decisiones (2026-05-23). Resuelve ambigüedades del PBI antes de blueprint Dedalo.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Rama | `feat/pull-request-automation-dlt` |
| `persist_ref` | `docs/features/pull-request-automation-dlt` |
| Manifiesto | `docs/todos/done/Activacion_Validacion_PR_DLT.md` (v2.0.0 — migrado desde PDF) |
| Bus SSOT runtime | `.events/pending/` (`cumulo.paths.json` → `eda_bus.pending`) |
| Instancia EDA | `.SddIA/events/` — personalización; no sustituye bus operativo |
| Estado PBI | Pendiente / Teórico — requiere laudo empírico (H4) para cerrar |

---

## D2 — Síntoma y vacío operativo

| Hallazgo | Decisión |
|----------|----------|
| `delivery-close-cycle` + `emit-pr-presented-event` cubren PR local (Cursor) | **Mantener** — no sustituir |
| PR creado por agente remoto (Jules) vía API GitHub | **Ceguera transaccional** — no hay sello local en bus |
| Dogma Despertador Inerte | Terminaciones periféricas (GitHub webhook/polling) → anclaje DLT → motor local despierta simétricamente |

**Conclusión:** esta feature **complementa** `pr-presented-orchestration` y alimenta la aduana `pull-request-review` ya cableada en `pull-request-review-redesign`.

---

## D3 — Separación de jurisdicciones (claves vs IA)

| Componente | Jurisdicción | Restricción |
|------------|--------------|-------------|
| Jules / IA obrera remota | GitHub API (PR material) | **Prohibido** acceso a seed/privkey |
| `github_bridge_watcher.py` (demonio local) | Lectura `.SddIA/.dev/wallet.key` + firma DLT | **Único** lector autorizado de clave |
| `iota-immutable-publisher` | Anclaje Testnet | Secretos vía bóveda `.dev/.env` / `.SddIA/.dev/.env` (`IOTA_WALLET_SECRET`) — **no** en payload JSON |

| Pregunta | Decisión |
|----------|----------|
| ¿Jules firma DLT directamente? | **No** — imposible físico sin violar soberanía |
| ¿Proxy local? | **Sí** — oráculo sensor contrasta GitHub REST → firma → Tangle → bus |

---

## D4 — Topología del bus (PBI vs runtime)

| Referencia PBI | Runtime SSOT actual | Decisión |
|----------------|---------------------|----------|
| `.SddIA/events/PullRequest_Presented.json` (conceptual instancia) | `.events/pending/<event_id>.json` vía `cumulo.paths.json` | Materializar en **bus canónico** `eda_bus.pending`; path instancia es metáfora ECST, no destino de escritura |
| `event_id` = `transaction_id` DLT | `emit-pr-presented-event` usa UUID v4 vía `crypto-broker` | **Ruta oráculo:** `event_id` = hash/digest IOTA (`transaction_digest`) — idempotencia f(x)=x |
| Emisor local Cursor | `emitter_agent: delivery-close-cycle` | **Ruta oráculo:** `origin_agent: jules` (u otro) + `signer_identity_rbac` en payload |

**Implicación:** posible evolución ECST `pull-request-presented` v1.2+ con campos opcionales del PBI (`repository`, `origin_agent`, `dlt_anchor_address`, `signer_identity_rbac`). Dedalo evalúa en `spec.md`.

---

## D5 — Hitos atómicos (PBI → entregables)

| Hito | Entregable propuesto | Criterio |
|------|---------------------|----------|
| **H1** | `SddIA/scripts/daemons/github_bridge_watcher.py` | Listener agnóstico al autor (polling GitHub API o webhook tunnel) |
| **H2** | Puente firma aislada | En `pull_request.opened`: validar contra REST GitHub → leer wallet → invocar `iota-immutable-publisher` |
| **H3** | Suscriptor DLT → bus | Tras confirmación Tangle: escribir JSON ECST en `pending/` con `event_id` = digest; `delivery_state` inicial |
| **H4** | Smoke E2E | `SDDIA_LAB_SIMULATE_REMOTE_PR=1` + simulador externo → aduana `pull-request-review` 7 fases, exit code binario |

---

## D6 — Contención de riesgos (Filtro B)

| Vector | Contramedida adoptada |
|--------|----------------------|
| Caída IOTA Testnet (3 reintentos) | Dead-letter en `.events/dead-letter/` con bandera `FALLBACK_LOCAL_SIGNATURE`; laudo Vértice Biológico |
| Payload corrupto desde agente remoto | **Validación ciega:** contrastar intención contra GitHub REST antes de firmar; descartar si diff no coincide (Filtro A) |
| Lectura clave por subproceso malicioso | `wallet.key` chmod 400; solo demonio sensor compilado lee; IAs en entornos limpios |

---

## D7 — Relación con suscriptores existentes

| Suscriptor actual (`PullRequest_Presented`) | Acción |
|---------------------------------------------|--------|
| `argos` → `pull-request-review` | **Sin cambio** — aduana reactiva al evento en bus |
| `cumulo` → `iota-immutable-publisher` | **Skip en ruta oráculo** — si `payload.dlt_anchor_address` presente, marcar `cumulo: success` sin re-invocar IOTA (`route_domain_event_core`) |

---

## D8 — Alcance explícito fuera de feature

- Webhook productivo en GitHub.com (producción) — laboratorio puede usar polling o ngrok.
- Custodia HSM enterprise de wallet — solo aislamiento `.dev` local del PBI.
- Modificar `delivery-close-cycle` flujo Cursor — fuera de alcance salvo correlación documental.

---

## D9 — Git y commits

Commits atómicos por sub-entrega: (1) docs clarify/objectives, (2) demonio H1, (3) puente DLT H2–H3, (4) smoke H4 + validación Argos.

Merge hacia `main` vía **`accept-pr`** cuando Argos emita APTO.

---

## D10 — Protocolo empírico (PBI §7)

| Paso | Acción |
|------|--------|
| 1 | `SDDIA_LAB_SIMULATE_REMOTE_PR=1` |
| 2 | PR dummy desde simulador externo (sin wallet) |
| 3 | Monitorizar daemon sensor + explorador Tangle |
| 4 | `validacion.md` con flujo completo y `delivery_state: success` |

---

## D11 — Precedencia Ola A

| Feature cerrada | Aportación a esta entrega |
|-----------------|---------------------------|
| `ampliacion-configuracion-entornos` | `env_loader` + `IOTA_WALLET_SECRET` vía bóveda |
| `pr-presented-orchestration` | Flujo local Cursor intacto |
| `pull-request-review-redesign` | Aduana 7 fases consumidora del evento |
