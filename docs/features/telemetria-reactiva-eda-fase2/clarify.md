---
feature_name: telemetria-reactiva-eda-fase2
created: "2026-05-27"
purpose: Decisiones Fase 2 y herencia Fases 0–1
---

# Clarificación — Fase 2

## Precondición (gate Fase 1)

Fase 1 mergeada en `main` (PR #52) con `validacion.md` APTO. Genoma fractal operativo: `event_family`, subcarpetas `telemetry/` / `orchestration/` / `domain/`, `Raw_Execution_Finished` en genoma. No se reabre Fase 1 salvo hallazgo bloqueante durante Tekton.

## Decisiones heredadas (aplican en Fase 2)

| ID | Resolución | Uso en Fase 2 |
|----|------------|---------------|
| D0.3 | `paths.workspacesRoot` en SSOT universal | Implementar en `cumulo.paths.json`; base para resolver plantillas |
| D0.6 | PBI maestro en `pending/` | `validacion.md` con `pbi_archived: false` |
| Axioma §0.3 | Rutas relativas compuestas | Cúmulo = base; proceso = `workspace_template` parcial |
| Axioma §0.3 | Persistencia encapsulada | Workspace es destino de `filesystem-manager`; no escritura directa ED |

## Decisiones cerradas — Fase 2

| ID | Pregunta | Resolución |
|----|----------|------------|
| D2.1 | ¿Bump de `process-contract`? | **Minor** → `1.4.0`: nueva sección § Workspace obligatorio (`workspace_template` en frontmatter de `{name}.md`) |
| D2.2 | ¿Sintaxis de plantilla? | Placeholders canónicos: `{process_name}` (name del proceso), `{execution_id}` (UUID v4 generado por CLI). Separador OS-agnóstico vía `pathlib`. Sin expresiones arbitrarias en v1 |
| D2.3 | ¿Dónde vive `workspace_template`? | En el **frontmatter YAML** de cada `SddIA/process/{name}.md` (no spec.json paralelo). Procesos sin plantilla → **bloqueante** tras bump de contrato |
| D2.4 | ¿Valor por defecto para procesos forja? | `".SddIA/workspaces/{process_name}/{execution_id}/"` — procesos `feature`, `bug-fix`, `refactorization` adoptan esta plantilla en Fase 2 |
| D2.5 | ¿Qué pasa con `persist_ref`? | **Convivencia:** input `persist_ref` sigue siendo la ruta **documental** de la tarea (`docs/features/...`) hasta migración narrativa Fase 6; el **Workspace operativo** es ortogonal y vive bajo `.SddIA/workspaces/`. El CLI expone ambos en estado de ejecución |
| D2.6 | ¿Deprecación `featurePath`/`fixPath`? | Añadir en SSOT como **deprecated aliases** que resuelven a `directories.documentation` + subcarpeta (`features` / `fixes`) durante transición; normas actualizadas para citar `workspacesRoot` como destino operativo |
| D2.7 | ¿Inyección en payload de evento? | Campo **`workspace_path`** (string, ruta absoluta resuelta) en `process_inputs` / micro-contexto de fase antes de delegar a agentes. Emisión formal en envelope ECST → **Fase 3**; Fase 2 cablea estado interno CLI + contratos agente |
| D2.8 | ¿Proceso smoke AC2.1? | Script lab o proceso mínimo `document-processor` (nombre provisional) con plantilla estándar; ejecutable vía `execute-process` sin depender de `docs/features/{slug}` |
| D2.9 | ¿Purga física de workspaces? | **No** en Fase 2 — carpeta impermanente pero sin GC automático; Kaizen futuro para TTL/purge post-validación Argos |
| D2.10 | ¿`.gitignore` workspaces? | Sí — añadir `.SddIA/workspaces/` al ignore de instancia si no existe; workspaces no versionados |

## Resolución compuesta de ruta (normativa)

```text
workspaces_root = cumulo.resolve("paths.workspacesRoot")
relative        = workspace_template.format(process_name=..., execution_id=...)
workspace_path  = (workspaces_root / relative).resolve()
```

Prohibido: concatenar literales `docs/features` en runtime de orquestación tras migración de §2.D.

## Payload mínimo de contexto (agentes)

| Campo | Origen | Obligatorio Fase 2 |
|-------|--------|-------------------|
| `workspace_path` | CLI post-instanciación | Sí (fases con agentes) |
| `persist_ref` | Input proceso / inferencia branch | Sí (documentación de tarea) |
| `execution_id` | CLI UUID | Sí |

Instrucciones a Tekton/Dédalo/Argos: mutar artefactos **solo** bajo `workspace_path`; prohibido citar rutas absolutas del repositorio en prompts generados por orquestador (AC2.3).

## Referencias

- Gate Fase 0: `impact-analysis.md` (H13–H17, D0.3, matriz featurePath)
- Gate Fase 1: `docs/features/telemetria-reactiva-eda-fase1/validacion.md`
- PBI: § Fase 2 (2.A–2.D)
- Origen consolidado: `docs/todos/tmp/Patsh Destino no proceso y no por cumulo.md`
