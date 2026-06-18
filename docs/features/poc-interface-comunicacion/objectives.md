---
feature_name: poc-interface-comunicacion
created: "2026-06-18"
process: feature
branch_name: feat/poc-interface-comunicacion
persist_ref: docs/features/poc-interface-comunicacion
pbi_ref: docs/todos/done/PBI_PoC_Interface_Comuniccion.md
document_id: PBI-POC-INTERFACE-COMUNICACION
status: validacion_apto
related:
  - SddIA/agents/mayeuta.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/scripts/qa/env_loader.py
  - SddIA/scripts/qa/execute-process.py
  - docs/features/puente-sensorial-telegram/objectives.md
  - docs/features/telegram-fallback-responder/objectives.md
---

# Objetivos — PoC Interface Comunicación (Kalma2)

## Misión

Disponer de la **primera versión de cliente interactivo** para operar SddIA **sin Cursor como intermediario obligado**: superficie web mínima (prompt → envío → respuesta) acoplada a un **puente HTTP local** que delega en el motor del Core respetando contrato JSON stdin/stdout.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| No existe cliente web ni puente HTTP en el genoma | Forjar artefactos PoC bajo rutas acordadas en clarificación |
| Mayeuta estabiliza requisitos; no es chatbot genérico | El PBI nombra Mayeuta como target semilla — **requiere decisión** (ver `clarify.md` Q2) |
| `load_hierarchical_env` operativo | Puente debe cargar `.dev/.env` → `.SddIA/.dev/.env` antes del subproceso |
| PoC explícitamente **sin Cerbero/RBAC** | Solo localhost; deuda de seguridad documentada para evolución |
| Patrón Telegram (Capa 0 + gateway) | Referencia de desacople físico/lógico; este PoC es **síncrono HTTP**, no EDA |
| `interfaces/kalma2` citado en PBI | Nombre semilla UI; ruta canónica **pendiente** (Q1) |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | **UI mínima viable** | Tres controles: textarea prompt, botón envío, área respuesta solo lectura |
| **O2** | **Puente HTTP local** | `POST /api/interact` con body `{"prompt": string}`; respuesta JSON parseable |
| **O3** | **Despertador inerte** | Frontend sin lógica de negocio ni resolución de rutas Core |
| **O4** | **Bóvedas antes de motor** | Puente invoca `load_hierarchical_env(repo_root)` al arranque o por petición |
| **O5** | **Estado cero cliente** | Sin sesión ni historial en browser; cada envío atómico |
| **O6** | **Inmunidad doble envío** | Botón deshabilitado mientras `fetch` pendiente |
| **O7** | **Invocación motor** | Subproceso al target acordado (Q2) con E/S JSON coherente |
| **O8** | **Validación manual** | Smoke: prompt de prueba → respuesta visible en UI |
| **O9** | **Cierre documental** | Un PR: código + docs + PBI en `done/` + `validacion.md` APTO |

## Alcance de esta entrega (fase actual)

1. PBI estandarizado en `docs/todos/pending/`.
2. Documentación bajo `persist_ref`: `objectives.md`, `clarify.md`, `_init-feature.json`.
3. **Detenerse tras clarificación** — `spec.md` y `plan.md` requieren cierre de preguntas abiertas.

## Fuera de alcance (PoC)

- Cerbero, Karma2Token en el puente, autenticación multiusuario.
- WebSockets, SSE, cola de mensajes.
- Sustitución del flujo `feature` / `entity-manager` para mutación de genoma.
- Orquestador EDA y eventos de dominio para cada prompt.
- Empaquetado npm, frameworks SPA, despliegue remoto.

## Ley aplicada

- **Soberanía de rutas:** `SddIA/core/cumulo.paths.json` — no inferir paths fuera del SSOT.
- **Ceguera espacial UI:** el cliente no conoce topología del Core.
- **Agnosticismo Core:** puente parametrizable (puerto, target) vía bóveda o args CLI.
- **Triaje C (PoC):** invocación directa autorizada solo en localhost y scope PoC documentado.
