---
document_id: PBI-ACTIVACION-VALIDACION-PR-DLT
title: "[ARQUITECTURA] Oráculo Sensor DLT y Activación de Validación de PR"
format: markdown
version: "2.0.0"
created: "2026-05-22"
status: pendiente
priority: arquitectura-eda
feature_ref: docs/features/pull-request-automation-dlt
branch: feat/pull-request-automation-dlt
pdf_ref: docs/todos/pending/SddIA_PBI_TODO_Activacion_Validacion_PR_v2.0.0.pdf
bus_objetivo: .SddIA/events/PullRequest_Presented.json
anclaje_ssot: IOTA Rebased Testnet
jurisdiccion: Yunque Operativo (Tormentosa)
---

# [ARQUITECTURA] Oráculo Sensor DLT y Activación de Validación de PR

**Estado:** Pendiente / Teórico — en espera de laudo empírico (H4).  
**Feature activa:** `docs/features/pull-request-automation-dlt/` · rama `feat/pull-request-automation-dlt`.

---

## 1. Declaración de propósito de dominio

El ecosistema SddIA exige la convergencia absoluta de su Sistema Nervioso Orientado a Eventos (EDA) de Estado Cero. Actualmente, la intención de presentar un Pull Request (`PullRequest_Presented`) se materializa correctamente en el Libro Mayor Inmutable (DLT) cuando es detonada de forma local mediante el cliente (Cursor). Sin embargo, cuando un agente autónomo desacoplado espacialmente (Jules) interactúa de manera remota con la API del Leviatán (GitHub), el estímulo se disipa, provocando una **ceguera transaccional crítica** en la trinchera local.

Este ítem define la reestructuración del mecanismo de captura sensorial para unificar emisión y recepción de intenciones. Al forzar que **toda detección pase obligatoriamente por la Testnet de IOTA Rebased**, se erradica el sesgo heurístico. El sistema local ya no vigilará ciegamente un sistema de archivos aislado; escuchará el anclaje criptográfico global firmado por una identidad autorizada, resolviendo el desacople temporal y espacial sin violar el aislamiento paramétrico.

**Dogma del Despertador Inerte aplicado a redes descentralizadas:** los scripts físicos locales y las acciones de GitHub carecen de jurisdicción lógica sobre el estado. Actúan exclusivamente como terminaciones nerviosas periféricas que traducen la mutación del entorno material (GitHub) en un anclaje inmutable en la Tangle, permitiendo que el motor determinista local despierte de forma simétrica ante un humano o un agente remoto.

---

## 2. Paradoja criptográfica y aislamiento de claves

La integración de agentes LLM como Jules introduce una vulnerabilidad táctica inaceptable: **las claves privadas de la billetera (Wallet Seed / Private Key) jamás deben exponerse al contexto o entorno de ejecución de la IA obrera**. Las claves se custodian estrictamente bajo el perímetro seguro local en la ruta no rastreada `.SddIA/.dev/wallet.key`.

Para resolver la imposibilidad física de que Jules firme transacciones DLT directamente sin comprometer la soberanía del entorno, se establece la separación radical entre:

| Fase | Jurisdicción |
|------|--------------|
| Apertura de la intención material | GitHub (agente remoto) |
| Certificación criptográfica de la intención | Proxy local / Oráculo Local |

---

## 3. Arquitectura del flujo del sistema nervioso

```text
[Entorno Remoto: Jules/API] ──(PR Creado en GitHub)──► [Mundo Físico: GitHub Webhook/API]
                                                              │
                                                              ▼
[Bus Local .events/pending/] ◄──(Escribe Evento)── [Oráculo Local Sensor] ◄──(Detecta PR + Lee .dev/ y Firma DLT)
         │
         ▼
[Aduana pull-request-review] ──► Cerbero (RBAC) + Argos (Bloqueo Duro) ➔ S+ Grade
```

> **Nota runtime:** bus canónico SSOT = `.events/pending/` (`cumulo.paths.json` → `eda_bus.pending`). La ruta conceptual `.SddIA/events/` es personalización de instancia EDA, no sustituto del bus operativo.

---

## 4. Especificación técnica del backlog atómico (TODO)

| Hito | Objetivo técnico | Criterio de validación estricta (Filtro A) |
|------|------------------|-------------------------------------------|
| **H1** | Diseño del demonio sensor efímero `sddia-github-bridge` | `github_bridge_watcher.py`: listener inerte local en Python (polling ligero o webhook tunnel ngrok/relay). Agnóstico al autor del PR (Cursor o Jules). |
| **H2** | Puente de firma aislada e inyección DLT | En `pull_request.opened`: demonio local extrae payload base, accede a `.SddIA/.dev/wallet.key`, compone estructura inmutable y ejecuta cápsula `iota-immutable-publisher` en Testnet. |
| **H3** | Materialización idempotente en el bus | Suscriptor DLT local verifica confirmación en Tangle. Tras firma válida del Core, escribe payload en `.events/pending/<event_id>.json` con `transaction_digest` como `event_id` raíz — idempotencia f(x)=x. |
| **H4** | Prueba de humo E2E desacoplada | Simular PR vía script externo sin privilegios locales (Simulación Jules). Verificar inyección reactiva en bus y arranque automatizado de las 7 fases de aduana `pull-request-review` con código de salida binario. |

---

## 5. Estructura del contrato del evento unificado

El archivo de intercambio inyectado en el bus local debe respetar la desnormalización absoluta del patrón Event-Carried State Transfer (ECST), absteniéndose de llamadas secundarias a la red durante la ejecución de la aduana:

```json
{
  "event_id": "iota_block_hash_62bcb6e1f9954edf...",
  "event_type": "PullRequest_Presented",
  "timestamp": "2026-05-22T07:45:00Z",
  "payload": {
    "repository": "racso80es/SddIA",
    "branch": "feat/pull-request-review-redesign",
    "pr_url": "https://github.com/racso80es/SddIA/pull/12",
    "origin_agent": "jules",
    "dlt_anchor_address": "iota1pr...",
    "signer_identity_rbac": "Vértice_Biológico_Relay"
  },
  "delivery_state": {
    "argos": "pending",
    "cumulo": "pending"
  }
}
```

**Evolución ECST requerida:** ampliar `pull-request-presented.md` v1.2+ con campos opcionales `repository`, `origin_agent`, `dlt_anchor_address`, `signer_identity_rbac` (ruta oráculo). Mantener compatibilidad con ruta local (`delivery-close-cycle` + UUID v4).

---

## 6. Matriz de contención de riesgos operativos

| Vector de riesgo | Impacto estructural | Contramedida rúnica (Filtro B) |
|------------------|---------------------|--------------------------------|
| Caída de red IOTA Testnet | Bloqueo transaccional; incapacidad de generar chispazo de entrada en bus | **Fallback local controlado:** tras 3 reintentos, log en `.events/dead-letter/` con bandera `FALLBACK_LOCAL_SIGNATURE`. Requiere laudo del Vértice Biológico. |
| Secuestro semántico del agente remoto | Payloads corruptos o URLs de PR falsificadas | **Validación ciega de oráculo:** contrastar intención contra API REST inmutable de GitHub antes de firmar hacia DLT. Si diff no coincide → descartar (Filtro A). |
| Lectura de claves por subproceso | Inyección de comandos maliciosos en herramientas locales | **Aislamiento `.dev`:** `wallet.key` con permisos `chmod 400`. Solo demonio sensor compilado lee; IAs obreras en entornos limpios. |

---

## 7. Protocolo de validación empírica

Para pasar de Pendiente/Teórico al repositorio real, el laudo del Vértice Biológico exige:

1. Exportar `SDDIA_LAB_SIMULATE_REMOTE_PR=1`.
2. Lanzar PR dummy en rama aislada desde interfaz externa simulando a Jules.
3. Monitorear salida del daemon sensor y certificar publicación del bloque inmutable en explorador Tangle.
4. Validar que `validacion.md` registre flujo completo con `delivery_state: success`.

---

## 8. Precedencia en el ecosistema

| Artefacto | Relación |
|-----------|----------|
| `pr-presented-orchestration` | Flujo **local** Cursor — no sustituir |
| `pull-request-review-redesign` | Aduana reactiva — consumidor del evento |
| `iota-immutable-publisher` | Cápsula de anclaje Testnet |
| `ampliacion-configuracion-entornos` | Jerarquía bóvedas — secretos vía `.dev/.env` + `IOTA_WALLET_SECRET` |
