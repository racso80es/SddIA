---
feature_name: poc-interface-comunicacion
created: "2026-06-18"
process: feature
purpose: Estabilización de requisitos — PBI-POC-INTERFACE-COMUNICACION (Q1–Q4 ratificadas por operador)
version_clarify: "1.1.0"
pbi_ref: docs/todos/pending/PBI_PoC_Interface_Comuniccion.md
document_id: PBI-POC-INTERFACE-COMUNICACION
---

# Clarificación — PoC Interface Comunicación (Kalma2)

Transcript de asimilación del PBI (2026-06-18). **Q1–Q4 ratificadas** (2026-06-18) por orden del Vértice Biológico de avanzar a implementación; decisiones consolidadas en `implementation.md`.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Nombre operativo | **poc-interface-comunicacion** |
| Rama prevista | `feat/poc-interface-comunicacion` |
| `persist_ref` | `docs/features/poc-interface-comunicacion` |
| PBI | `docs/todos/pending/PBI_PoC_Interface_Comuniccion.md` |
| `document_id` | `PBI-POC-INTERFACE-COMUNICACION` |
| Inputs SSOT | `_init-feature.json` |
| Nombre semilla UI | **Kalma2** (`interfaces/kalma2` en PBI original) |

---

## D2 — Asimilación del PBI (contenido heredado)

El borrador original mezclaba tres artefactos (`spec.md`, `clarify.md`, `plan.md`) en un único fichero pending. Tras estandarización:

| Bloque original | Destino canónico | Estado |
|-----------------|------------------|--------|
| Propósito, UI, arquitectura | PBI §1–2 + `objectives.md` | ✅ Consolidado |
| Entrypoint, ceguera, estado cero | PBI §3 + esta clarificación | ✅ Consolidado |
| Fases puente / UI / validación | PBI §6 + futuro `plan.md` | ⏸ Pendiente Q1–Q4 |

**Typo de archivo:** `Comuniccion` (doble **c**) se conserva en el nombre físico del PBI para no romper enlaces; el `document_id` usa grafía canónica `COMUNICACION`.

---

## D3 — Inventario de ficheros implicados (mapa pre-spec)

### 3.1 Documentación de ciclo (genoma docs — permitido)

| Fichero | Rol | Estado |
|---------|-----|--------|
| `docs/todos/pending/PBI_PoC_Interface_Comuniccion.md` | Semilla de negocio / backlog | ✅ v1.1.0 |
| `docs/features/poc-interface-comunicacion/_init-feature.json` | Manifiesto lab / execute-process | ✅ |
| `docs/features/poc-interface-comunicacion/objectives.md` | Requisito termodinámico (Mayeuta lógico) | ✅ |
| `docs/features/poc-interface-comunicacion/clarify.md` | Este transcript | ✅ |
| `docs/features/poc-interface-comunicacion/spec.md` | Especificación técnica | ⏸ Bloqueado por Q1–Q4 |
| `docs/features/poc-interface-comunicacion/plan.md` | Línea de montaje | ⏸ Bloqueado por Q1–Q4 |

### 3.2 Runtime PoC (a forjar — Tekton, post-spec)

| Fichero | Rol semilla PBI | Conflicto / nota |
|---------|-----------------|------------------|
| `SddIA/scripts/clients/sddia-client-bridge.py` | Servidor HTTP + `POST /api/interact` + subproceso | **Q3:** ¿genoma o instancia? No existe `directories.clients` en Cúmulo |
| `interfaces/kalma2/index.html` | UI estática (Material) | **Q1:** ruta citada en cabecera PBI |
| `interfaces/kalma2/app.js` | `fetch`, bloqueo botón, render JSON | Derivado de Q1 |
| `interfaces/kalma2/style.css` | Presentación mínima | Derivado de Q1 |
| `.SddIA/client/index.html` | UI alternativa (plan original) | **Q1:** compite con `interfaces/kalma2` |
| `.SddIA/client/…` | Mismo bundle bajo instancia | Separación genoma/runtime del plan original |

### 3.3 Dependencias existentes (solo lectura — no mutar en PoC doc)

| Fichero | Rol en el flujo |
|---------|-----------------|
| `SddIA/scripts/qa/env_loader.py` | `load_hierarchical_env(repo_root)` — bóvedas global + instancia |
| `SddIA/scripts/qa/execute-process.py` | CLI canónico; carga env al inicio; invocación de procesos |
| `SddIA/agents/mayeuta.md` | Agente **estabilización** de requisitos — no diseñado como REPL chat |
| `SddIA/norms/capsule-json-io.md` | Contrato envelope JSON stdin/stdout v2.0 |
| `SddIA/core/cumulo.paths.json` | SSOT rutas; `env_hierarchy`, `paths.workspacesRoot` |
| `SddIA/scripts/qa/telegram_fallback_responder_core.py` | Referencia lab: `synthesize_mayeuta_response` (stub determinista) |

### 3.4 Diagrama de capas (propuesta pre-decisión)

```text
┌─────────────────────────────────────────────────────────┐
│  Capa Material — HTML/JS (Q1: kalma2 vs .SddIA/client) │
│  [textarea] [Enviar] [output readonly]                  │
└──────────────────────────┬──────────────────────────────┘
                           │ fetch POST /api/interact
                           ▼
┌─────────────────────────────────────────────────────────┐
│  Puente físico — sddia-client-bridge.py (Q3, Q4)        │
│  load_hierarchical_env → subprocess → JSON response     │
└──────────────────────────┬──────────────────────────────┘
                           │ stdin/stdout JSON (Q2)
                           ▼
┌─────────────────────────────────────────────────────────┐
│  Motor SddIA — target TBD (Mayeuta / proceso / stub)    │
└─────────────────────────────────────────────────────────┘
```

---

## D4 — Decisiones cerradas (sin ambigüedad)

| ID | Tema | Decisión |
|----|------|----------|
| **D4.1** | Naturaleza del cliente | **Despertador inerte** — cero lógica de negocio en JS |
| **D4.2** | Estado | **Estado cero** en browser; workspaces en `.SddIA/workspaces/` si el motor escribe |
| **D4.3** | Seguridad PoC | **Sin Cerbero/RBAC**; bind **127.0.0.1** obligatorio |
| **D4.4** | Bóvedas | Patrón existente: `.dev/.env` → `.SddIA/.dev/.env` vía `env_loader` |
| **D4.5** | Endpoint | `POST /api/interact` body `{"prompt": "<string>"}` |
| **D4.6** | UX envío | Deshabilitar botón durante petición (inmunidad doble clic) |
| **D4.7** | Stack frontend | HTML5 + CSS + Vanilla JS — **sin** npm ni bundlers |
| **D4.8** | Alcance EDA | **Fuera** del PoC — HTTP síncrono directo, no bus `.events/` |

---

## D5 — Preguntas resueltas (ratificadas 2026-06-18)

| ID | Decisión ratificada |
|----|---------------------|
| **Q1** | **A** — bundle UI en `interfaces/kalma2/` (SSOT versionado; servido por el puente) |
| **Q2** | **D → B** — Ola 1 stub eco en puente; Ola 2 proceso genoma `kalma2-interact` (post-PoC) |
| **Q3** | **B** — puente en `.SddIA/client/sddia-client-bridge.py` (instancia; cero diff genoma) |
| **Q4** | **C** — único script stdlib (`http.server`): sirve estáticos + expone `/api/interact` |

Detalle original de opciones conservado abajo para auditoría.

### Q1 — Ruta canónica del bundle UI

| Opción | Pros | Contras |
|--------|------|---------|
| **A)** `./interfaces/kalma2/` (repo root) | Visible, nombre alineado a Kalma2; fácil abrir en navegador | Mezcla PoC en root; no bajo `.SddIA/` |
| **B)** `.SddIA/client/` | Separación genoma/runtime explícita (plan original) | Path oculto; no coincide con cabecera PBI |
| **C)** Híbrido: fuente en `interfaces/kalma2/`, runtime servido desde puente | Versionado claro + servicio unificado | Dos rutas a mantener |

**Propuesta Tekton (pendiente ratificación):** **A** para PoC — `interfaces/kalma2/` como SSOT del bundle; el puente sirve estáticos desde ahí.

---

### Q2 — Target de invocación del puente

El PBI dice «cápsula de Mayeuta». En el genoma actual, **Mayeuta estabiliza requisitos** (`raw_user_intent` → `thermodynamic_stable_requirement_md`), no responde conversacionalmente en un REPL.

| Opción | Descripción | Alineación PoC |
|--------|-------------|----------------|
| **A)** Proceso **`telegram-fallback-responder`** (lab stub `synthesize_mayeuta_response`) | Respuesta ≤2 líneas determinista | Rápido; no es Mayeuta real |
| **B)** Nuevo proceso PoC **`kalma2-interact`** | Wrapper JSON: input `prompt` → output `response` | Limpio; requiere forja genoma |
| **C)** Invocación IDE/agente Mayeuta vía orquestador | Respuesta real de estabilización | Puede devolver preguntas, no «chat» |
| **D)** Stub mínimo en el puente (eco JSON) | Solo validar UI+HTTP | Fase 0 antes de motor real |

**Propuesta Tekton (pendiente ratificación):** **D → B** en dos olas: stub eco en validación termodinámica; luego proceso `kalma2-interact` que delegue en síntesis tipo fallback-responder hasta exista agente conversacional canónico.

---

### Q3 — Ubicación del puente Python

| Opción | Implicación |
|--------|-------------|
| **A)** `SddIA/scripts/clients/sddia-client-bridge.py` | Versionado en repo; nueva carpeta `scripts/clients/` (no indexada Cúmulo) |
| **B)** `.SddIA/client/sddia-client-bridge.py` | 100% instancia; cero diff genoma |
| **C)** `SddIA/scripts/limbo/clients/` | Patrón limbo transitorio pre-Rust |

**Propuesta Tekton (pendiente ratificación):** **B** para PoC (mínima fricción genoma); promoción a **A** en feature posterior si el puente se estabiliza.

---

### Q4 — Stack HTTP Python

| Opción | Dependencias | Complejidad |
|--------|--------------|-------------|
| **A)** `http.server` + handler custom | stdlib | Baja; CORS manual si file:// |
| **B)** FastAPI + uvicorn | pip externo | Media; OpenAPI gratis |
| **C)** Servir UI + API en un solo script stdlib | stdlib | Baja; coherente con PoC |

**Propuesta Tekton (pendiente ratificación):** **C** — un script stdlib que sirva estáticos desde `interfaces/kalma2/` y exponga `/api/interact`.

---

## D6 — Contrato HTTP preliminar (sujeto a spec)

**Request**

```http
POST /api/interact HTTP/1.1
Host: 127.0.0.1:{PORT}
Content-Type: application/json

{"prompt": "texto del operador"}
```

**Response (éxito — forma lógica)**

```json
{
  "success": true,
  "response": "<texto o markdown>",
  "duration_ms": 0
}
```

**Response (error)**

```json
{
  "success": false,
  "message": "causa breve",
  "exit_code": 1
}
```

`PORT` por defecto propuesto: **8765** (override vía `SDDIA_CLIENT_PORT` en bóveda).

---

## D7 — Restricciones heredadas (no negociables)

- Mutación de genoma (`SddIA/tools`, `process`, `agents`, …) vía **`entity-manager` / execute-process** — no forja manual IDE (`external-ai-constraints` DA-2).
- Documentación de feature en `docs/features/` — permitida sin gate EDA.
- PoC no sustituye Cursor para editar entidades indexadas.

---

## AC0 — Arranque feature (documental)

| AC | Descripción | Estado |
|----|-------------|--------|
| AC0.1 | PBI estandarizado con frontmatter y DoD | ✅ |
| AC0.2 | `persist_ref` con `_init-feature.json` | ✅ |
| AC0.3 | `objectives.md` con trazabilidad PBI | ✅ |
| AC0.4 | `clarify.md` con mapa de ficheros y preguntas Q1–Q4 | ✅ |
| AC0.5 | Rama `feat/poc-interface-comunicacion` creada | ⏸ Pendiente operador |
| AC0.6 | Cierre Q1–Q4 ratificado | ✅ 2026-06-18 |

---

## Siguiente fase (desbloqueada)

Q1–Q4 ratificadas → **`implementation.md`** consolida touchpoints físicos, contrato y olas de forja.

1. `implementation.md` — touchpoints, rutas físicas finales, propuestas de código (✅ emitido).
2. Tekton — materialización en rama `feat/poc-interface-comunicacion` (pendiente).
3. `execution.md` + `validacion.md` — tras smoke termodinámico.
