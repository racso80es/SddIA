---
document_id: PBI-ACTIVACION-VALIDACION-PR-DLT
title: "[ARQUITECTURA] Oráculo Sensor DLT y Activación de Validación de PR"
format: markdown
version: "2.0.0"
created: "2026-05-22"
status: completado
closed: "2026-05-23"
priority: arquitectura-eda
feature_ref: docs/features/pull-request-automation-dlt
validacion_ref: docs/features/pull-request-automation-dlt/validacion.md
branch: feat/pull-request-automation-dlt
bus_objetivo: .events/pending/PullRequest_Presented.json
anclaje_ssot: IOTA Rebased Testnet
jurisdiccion: Yunque Operativo (Tormentosa)
source_format: markdown
migrated_from: docs/todos/pending/SddIA_PBI_TODO_Activacion_Validacion_PR_v2.0.0.pdf
---

# [ARQUITECTURA] Oráculo Sensor DLT y Activación de Validación de PR

| Campo | Valor |
|-------|-------|
| **ID Feature** | `docs/features/pull-request-automation-dlt` |
| **Fecha creación** | 2026-05-22 |
| **Estatus** | ✅ Completado (2026-05-23) |
| **Jurisdicción** | Yunque Operativo (Tormentosa) |
| **Bus objetivo (runtime)** | `.events/pending/` (`cumulo.paths.json` → `eda_bus.pending`) |
| **Anclaje SSOT** | IOTA Rebased Testnet |

> **Nota migración:** transcripción canónica del PBI PDF v2.0.0. El bus operativo SSOT es `.events/pending/<event_id>.json`; la ruta conceptual `.SddIA/events/` es personalización de instancia EDA.

---

## 1. Declaración de propósito de dominio

El ecosistema SddIA exige la convergencia absoluta de su Sistema Nervioso Orientado a Eventos (EDA) de Estado Cero. Actualmente, la intención de presentar un Pull Request (`PullRequest_Presented`) se materializa correctamente en el Libro Mayor Inmutable (DLT) cuando es detonada de forma local mediante el cliente (Cursor). Sin embargo, cuando un agente autónomo desacoplado espacialmente (Jules) interactúa de manera remota con la API del Leviatán (GitHub), el estímulo se disipa, provocando una **ceguera transaccional crítica** en la trinchera local.

Este ítem del backlog (PBI) define la reestructuración del mecanismo de captura sensorial para unificar emisión y recepción de intenciones. Al forzar que **toda detección pase obligatoriamente por la Testnet de IOTA Rebased**, se erradica el sesgo heurístico. El sistema local ya no vigilará ciegamente un sistema de archivos aislado; escuchará el anclaje criptográfico global firmado por una identidad autorizada, resolviendo el desacople temporal y espacial sin violar el aislamiento paramétrico.

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

---

## 4. Especificación técnica del backlog atómico (TODO)

| Hito | Objetivo técnico | Criterio de validación estricta (Filtro A) | Entrega |
|------|------------------|-------------------------------------------|---------|
| **H1** | Demonio sensor efímero `sddia-github-bridge` | `github_bridge_watcher.py`: listener agnóstico al autor (Cursor o Jules) | ✅ |
| **H2** | Puente firma aislada e inyección DLT | Solo demonio accede a `.SddIA/.dev/wallet.key`; invoca `iota-immutable-publisher` | ✅ |
| **H3** | Materialización idempotente en el bus | `transaction_digest` como `event_id` en `.events/pending/` | ✅ |
| **H4** | Prueba humo E2E desacoplada | Simulación Jules → aduana `pull-request-review` 7 fases | ✅ |

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

**Implementación:** ECST evolucionado a v1.2.0 en `SddIA/events/pull-request-presented.md`; ruta oráculo marca `delivery_state.cumulo: success` al materializar (DLT ya anclado en H2).

---

## 6. Matriz de contención de riesgos operativos

| Vector de riesgo | Impacto estructural | Contramedida rúnica (Filtro B) |
|------------------|---------------------|--------------------------------|
| Caída de red IOTA Testnet | Bloqueo transaccional; incapacidad de generar chispazo en bus | Fallback local: tras 3 reintentos, log en `.events/dead-letter/` con `FALLBACK_LOCAL_SIGNATURE`. Laudo Vértice Biológico. |
| Secuestro semántico del agente remoto | Payloads corruptos o URLs falsificadas | Validación ciega: contrastar contra API REST GitHub antes de firmar (Filtro A) |
| Lectura de claves por subproceso | Inyección maliciosa en herramientas locales | `wallet.key` chmod 400; solo demonio sensor lee; IAs en entornos limpios |

---

## 7. Protocolo de validación empírica

Laudo del Vértice Biológico (ejecutado 2026-05-23):

1. Exportar `SDDIA_LAB_SIMULATE_REMOTE_PR=1`.
2. Lanzar PR dummy vía `simulate_remote_pr.py` (simulación Jules).
3. Monitorizar `github_bridge_watcher.py --once` y digest IOTA (lab: `SDDIA_LAB_SIMULATE_IOTA=1`).
4. Validar `validacion.md` con `global: APTO` y `delivery_state: success` en aduana directa.

**Evidencia:** `docs/features/pull-request-automation-dlt/validacion.md`

---

## 8. Cierre

| Artefacto | Ubicación |
|-----------|-----------|
| Feature | `docs/features/pull-request-automation-dlt/` |
| Validación | `validacion.md` (`pbi_archived: true`) |
| PBI archivado | `docs/todos/done/Activacion_Validacion_PR_DLT.md` |
| PDF origen | Retirado — sustituido por este Markdown |
