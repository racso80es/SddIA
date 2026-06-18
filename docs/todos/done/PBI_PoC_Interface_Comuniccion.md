---
document_id: PBI-POC-INTERFACE-COMUNICACION
title: "[ARQUITECTURA] PoC — Cliente Interactivo desacoplado de Cursor (Kalma2)"
format: markdown
version: "1.2.0"
created: "2026-06-18"
status: done
priority: alta
process: feature
branch_name: feat/poc-interface-comunicacion
feature_ref: docs/features/poc-interface-comunicacion
validacion_ref: docs/features/poc-interface-comunicacion/validacion.md
pbi_ref: docs/todos/done/PBI_PoC_Interface_Comuniccion.md
---

# PBI-POC: Cliente Interactivo desacoplado de Cursor (Kalma2)

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-POC-INTERFACE-COMUNICACION` |
| **Estatus** | ✅ W1 entregado — [`validacion.md`](../../features/poc-interface-comunicacion/validacion.md) APTO |
| **Feature** | [`docs/features/poc-interface-comunicacion/`](../../features/poc-interface-comunicacion/) |
| **Rama** | `feat/poc-interface-comunicacion` |
| **Path UI** | `interfaces/kalma2/` |
| **Puente** | `.SddIA/client/sddia-client-bridge.py` |

## 1. Contexto y visión

Primera versión de **cliente interactivo** para operar SddIA **sin depender del IDE Cursor** como intermediario obligado. El hito habilita la **Soberanía de Dominio**: el Vértice Biológico inyecta intención desde una superficie propia.

**Entregable mínimo (MVP PoC):**

- Caja de texto multilínea para el prompt.
- Botón de envío ("Forjar" / "Enviar").
- Caja de solo lectura para respuesta o log de salida.

**Criterio tecnológico PoC:** máxima simplicidad de implementación con funcionalidad real (sin npm, bundlers ni frameworks pesados).

## 2. Arquitectura entregada (W1)

| Capa | Tecnología | Ruta |
|------|------------|------|
| **Material (frontend)** | HTML5, CSS, Vanilla JS | `interfaces/kalma2/` |
| **Puente físico (backend)** | Python `http.server` stdlib | `.SddIA/client/sddia-client-bridge.py` |
| **Motor (Ola 1)** | Stub eco en `invoke_engine()` | Deuda W3: `kalma2-interact` |

```text
[Navegador] ──POST /api/interact──► [sddia-client-bridge.py]
                                           │
                                           ▼
                                    [invoke_engine — eco PoC]
                                           │
                                           ▼
                                    [respuesta JSON → UI]
```

## 3. Restricciones de diseño

| ID | Restricción | Estado W1 |
|----|-------------|-----------|
| **R1** | Cliente = Despertador Inerte | ✅ |
| **R2** | Estado cero en browser | ✅ |
| **R3** | Sin Cerbero/RBAC (localhost) | ✅ |
| **R4** | Bóvedas antes de motor | ✅ |
| **R5** | Bind 127.0.0.1 | ✅ |

## 4. Touchpoints físicos (resueltos)

| Artefacto | Ruta final |
|-----------|------------|
| Puente HTTP | `.SddIA/client/sddia-client-bridge.py` |
| UI estática | `interfaces/kalma2/` |
| Smoke | `docs/features/poc-interface-comunicacion/_smoke-kalma2-interact.json` |
| Bóvedas | `.dev/.env`, `.SddIA/.dev/.env` |

## 5. Criterios de aceptación

| ID | Criterio | W1 |
|----|----------|-----|
| **AC1** | UI prompt + botón + respuesta | ✅ |
| **AC2** | POST `/api/interact` JSON | ✅ |
| **AC3** | Bóvedas cargadas | ✅ |
| **AC4** | Botón bloqueado en envío | ✅ |
| **AC5** | Smoke prompt → salida visible | ✅ |
| **AC6** | Cierre documental PR | ✅ |

## 6. Deuda post-PoC (W3)

- Proceso genoma `kalma2-interact` vía `entity-manager`.
- Sustituir eco en `invoke_engine()` por subprocess real.
- Evaluar promoción del puente a `SddIA/scripts/clients/`.

## 7. Referencias

- [`docs/features/poc-interface-comunicacion/`](../../features/poc-interface-comunicacion/)
- `SddIA/norms/capsule-json-io.md`
- `SddIA/scripts/qa/env_loader.py`
