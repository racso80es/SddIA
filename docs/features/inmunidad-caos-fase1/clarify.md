---
feature_name: inmunidad-caos-fase1
created: "2026-05-28"
purpose: Decisiones Fase 1 y herencia del gate Fase 0
---

# Clarificación — Fase 1 (Arsenal de Entropía)

## Precondición (gate Fase 0)

Fase 0 cerrada con PR #58, `impact-analysis.md` (H01–H28), PBI v2.2.0 y `validacion.md` APTO. No se reabre análisis salvo hallazgo bloqueante durante Tekton.

## Decisiones heredadas (Fase 0)

| ID | Resolución | Uso en Fase 1 |
|----|------------|---------------|
| D0.1 | Contexto `chaos-engineering` | Todas las tools ofensivas declaran `context: chaos-engineering` |
| D0.3 | Inocuidad acotada a `workspace_path` | Cápsulas reciben `workspace_path` por stdin; helper valida destinos |
| D0.5 | Termodinámica en tools | Bump contrato antes de `schema-corruptor` |
| D0.9 | PBI en `pending/` | `validacion.md` con `pbi_archived: false` |

## Decisiones cerradas — Fase 1

| ID | Pregunta | Resolución |
|----|----------|------------|
| **D1.1** | ¿Scope de las tools caos? | **`core`** — catálogo `SddIA/tools/` + cápsulas `scripts/tools/` (laboratorio SddIA unificado) |
| **D1.2** | ¿Implementación de cápsulas? | Python bajo `SddIA/scripts/tools/{name}/` con envelope S+ (`success`, `exitCode`, `name`, `result`) |
| **D1.3** | ¿`io-choke` simula qué? | Fallo E/S al escribir en ruta **dentro** del workspace (archivo bloqueado o permiso denegado simulado vía pre-create read-only / ruta ocupada); no corrompe genoma Core |
| **D1.4** | ¿Comportamiento `schema-corruptor`? | Frontmatter `telemetry_provided: true`; stdout envelope con `telemetry_receipt` ausente, malformado o vacío según modo stdin `corruption_mode`: `empty` \| `invalid_json` \| `partial` |
| **D1.5** | ¿Comportamiento `sandbox-breacher`? | Input `workspace_path` + `escape_target` (default `../escape.txt` relativo al workspace); intenta WRITE; debe fallar vía `assert_workspace_bound` con `exitCode: 1` |
| **D1.6** | ¿Tekton y `quality-assurance`? | **No** ampliar Tekton con `chaos-engineering` en Fase 1 — solo procesos/tools caos; procesos audit Fase 2 añadirán políticas |
| **D1.7** | ¿Actualizar `policy-validator`? | **Fuera de alcance** Fase 1 — Kaizen; tools usan contexto nuevo documentado en norma |
| **D1.8** | ¿Smoke AC1.3 obligatorio en CI? | Test unitario `test_chaos_tools.py` invocando cápsulas + opcional integración compliance con flag lab |

## Contrato stdin común (tools caos)

| Campo | Obligatorio | Descripción |
|-------|:-----------:|-------------|
| `workspace_path` | Sí | Ruta absoluta inyectada por orquestador |
| `operation_id` | No | Trazabilidad smoke |

## Referencias

- Gate: `docs/features/inmunidad-caos-fase0/impact-analysis.md` (H07–H11, H22–H23)
- PBI: `docs/todos/pending/PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO.md` § Fase 1
