---
document_id: PBI-POC-INTERFACE-COMUNICACION
title: "[ARQUITECTURA] PoC — Cliente Interactivo desacoplado de Cursor (Kalma2)"
format: markdown
version: "1.1.0"
created: "2026-06-18"
status: pending
priority: alta
process: feature
branch_name: feat/poc-interface-comunicacion
feature_ref: docs/features/poc-interface-comunicacion
pbi_ref: docs/todos/pending/PBI_PoC_Interface_Comuniccion.md
---

# PBI-POC: Cliente Interactivo desacoplado de Cursor (Kalma2)

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-POC-INTERFACE-COMUNICACION` |
| **Estatus** | 🟢 En implementación — Ola W1 |
| **Feature** | [`docs/features/poc-interface-comunicacion/`](../../features/poc-interface-comunicacion/) |
| **Rama prevista** | `feat/poc-interface-comunicacion` |
| **Path UI (semilla)** | `./interfaces/kalma2` |
| **Alcance** | PoC — UI mínima + puente HTTP local → motor SddIA |

## 1. Contexto y visión

Primera versión de **cliente interactivo** para operar SddIA **sin depender del IDE Cursor** como intermediario obligado. El hito habilita la **Soberanía de Dominio**: el Vértice Biológico inyecta intención desde una superficie propia.

**Entregable mínimo (MVP PoC):**

- Caja de texto multilínea para el prompt.
- Botón de envío ("Forjar" / "Enviar").
- Caja de solo lectura para respuesta o log de salida.

**Criterio tecnológico PoC:** máxima simplicidad de implementación con funcionalidad real (sin npm, bundlers ni frameworks pesados).

## 2. Arquitectura propuesta (semilla)

| Capa | Tecnología | Responsabilidad |
|------|------------|-----------------|
| **Material (frontend)** | HTML5, CSS básico, Vanilla JS | Captura prompt; `fetch` al puente; renderiza respuesta |
| **Puente físico (backend)** | Python — servidor HTTP ligero (`http.server` o FastAPI) | Recibe POST JSON; invoca ejecutable de interacción; devuelve JSON |
| **Motor** | Cápsula / proceso SddIA (semilla: Mayeuta) | Procesa prompt vía contrato E/S JSON (`capsule-json-io` / `execute-process`) |

```text
[Navegador] ──POST /api/interact──► [sddia-client-bridge.py]
                                           │
                                           ▼
                                    [subproceso motor SddIA]
                                           │
                                           ▼
                                    [respuesta JSON → UI]
```

## 3. Restricciones de diseño (no negociables en PoC)

| ID | Restricción |
|----|-------------|
| **R1** | Cliente web = **Despertador Inerte**: cero lógica de negocio; no resuelve rutas del Core |
| **R2** | **Estado cero** en el cliente: cada envío es petición atómica; persistencia en workspace (`.SddIA/workspaces/`) |
| **R3** | PoC **sin** orquestador general ni peaje RBAC Cerbero — invocación directa al target acordado en clarificación |
| **R4** | Carga de bóvedas (`.dev/.env` → `.SddIA/.dev/.env`) antes de invocar el motor |
| **R5** | Bind **localhost** exclusivamente; sin exposición pública |

## 4. Touchpoints físicos (semilla — sujetos a clarificación)

| Artefacto | Ruta semilla | Notas |
|-----------|--------------|-------|
| Puente HTTP | `SddIA/scripts/clients/sddia-client-bridge.py` | Receptor `POST /api/interact` con `{"prompt":"..."}` |
| UI estática | `./interfaces/kalma2/` **o** `.SddIA/client/` | **Conflicto documentado** — resolver en `clarify.md` |
| Bóvedas | `.dev/.env`, `.SddIA/.dev/.env` | Patrón `load_hierarchical_env` (`env_loader.py`) |

## 5. Criterios de aceptación (DoD PoC)

| ID | Criterio |
|----|----------|
| **AC1** | UI renderiza prompt, botón y área de respuesta en navegador local |
| **AC2** | `POST /api/interact` acepta JSON `{"prompt": string}` y responde JSON estructurado |
| **AC3** | Puente carga jerarquía de bóvedas antes del subproceso |
| **AC4** | Botón bloqueado durante petición (inmunidad doble envío) |
| **AC5** | Prompt de prueba produce salida visible en UI sin colapsar el hilo del servidor |
| **AC6** | Cierre documental: un PR con código + docs + PBI en `done/` + `validacion.md` APTO |

## 6. Fases de implementación (borrador)

1. **Puente físico:** servidor HTTP + endpoint + subproceso al motor.
2. **Interfaz material:** HTML + JS con `fetch` asíncrono.
3. **Validación termodinámica:** smoke manual desde navegador.

Detalle táctico en `docs/features/poc-interface-comunicacion/plan.md` (post-clarificación).

## 7. Fuera de alcance (PoC)

- Autenticación / RBAC Cerbero.
- Sesiones, historial de chat o WebSockets.
- Empaquetado npm / SPA / PWA.
- Integración EDA (eventos de dominio) salvo evolución explícita posterior.
- Sustituir Cursor para forja de genoma (sigue gobernado por `external-ai-constraints`).

## 8. Referencias

- `SddIA/agents/mayeuta.md` — agente semilla (rol real: estabilización de requisitos).
- `SddIA/norms/capsule-json-io.md` — contrato E/S JSON.
- `SddIA/scripts/qa/env_loader.py` — jerarquía de bóvedas.
- `docs/features/puente-sensorial-telegram/` — patrón puente Capa 0 + proceso.
- `docs/features/telegram-fallback-responder/` — invocación síntesis Mayeuta en lab.

## 9. Deuda / ambigüedades abiertas

Trasladadas a `docs/features/poc-interface-comunicacion/clarify.md`:

- Ruta canónica UI: `interfaces/kalma2` vs `.SddIA/client/`.
- Target de invocación: Mayeuta vs otro agente/proceso vs stub lab.
- Stack backend: `http.server` vs FastAPI.
- Ubicación puente: genoma (`SddIA/scripts/clients/`) vs solo instancia.
