---
feature_name: delivery-close-hook-eda-governance
created: "2026-05-22"
process: bug-fix
base: main
scope: delivery-close-hook-eda-governance
version_spec: "1.0.0"
---

# Especificación — Fix hooks EDA + gobernanza operador

## Hito 1 — Anti-recursión hook + delivery-close-cycle

### 1.1 `hook_common.py`

| Cambio | Detalle |
|--------|---------|
| `invoke_process` | Inyectar `SDDIA_HOOK_DELIVERY_CLOSE=1` en `env` del subproceso `execute-process.py` |
| `resolve_persist_ref` | Tras `docs/features/{slug}`, probar `docs/fixes/{slug}` |
| `should_skip_pre_push_present` | Añadir skip si `gh pr view` reporta `MERGED` (reutilizar lógica de `eda_bus_utils.github_pr_merged` o equivalente inline) |

### 1.2 `pre_push_gate.py`

Tras `skip_hooks()`, retornar `0` si `os.environ.get("SDDIA_HOOK_DELIVERY_CLOSE") == "1"`.

### 1.3 `execute_process_capsules.py` — `capsule_delivery_remote_push`

Si `inputs.get("source_process") == "git-hook-pre-push"`:

- Invocar `git-manager push` en subproceso con `environment` que incluya `SDDIA_SKIP_HOOKS=1` (heredando el resto del entorno).
- Documentar en comentario: skip acotado al hijo; no mutar `os.environ` del proceso padre.

Extender `invoke_git_manager` con parámetro opcional `extra_env: dict[str, str] | None`.

### 1.4 Test smoke

Rama `fix/smoke-hook` (lab): un push genera exactamente un `PullRequest_Presented` sin payloads recursivos en `tmp/`.

---

## Hito 2 — Retroactivo PR #20

| Evento | Payload mínimo |
|--------|----------------|
| `PullRequest_Presented` | `branch: feat/ampliacion-configuracion-entornos`, `pr_url: https://github.com/racso80es/SddIA/pull/20`, `emitter_agent: retroactive-fix`, `status: presented` |
| `PullRequest_Merged` | `merge_commit_hash: f0ef7bf4bb9e28e67091d70a6fba6f8fadcbf280`, `branch`, `pr_url`, `emitter_agent: retroactive-fix` |

Secuencia: emitir en `docs/events/pending/` → `event-watcher --once` → verificar `processed/`.

---

## Hito 3 — Gobernanza Universal (Ley de Jurisdicción Delegada)

Actualizar `SddIA/norms/obediencia-procesos.md` (v1.1):

> Queda estrictamente prohibido a la IA obrera utilizar comandos de terminal raw (`gh`, `git`, `curl`, etc.) para evadir un fallo en una cápsula, skill o proceso oficial (ej. `delivery-close-cycle`). El bypass manual es una violación S+ Grade.

Añadir § **Escalado ante fallo:** crear PBI en `docs/todos/pending/` antes de cualquier bypass; referencia al protocolo Kintsugi.

---

## Hito 4 — Evento Nativo de Fractura (Kintsugi EDA y Autoconocimiento)

### 4.1 Contrato

`SddIA/events/system-fracture-detected.md` — payload mínimo: `process_name`, `error_trace`, `agent_emitter`, `attempted_action`.

### 4.2 Suscripción dual (orden fan-out)

```json
"System_Fracture_Detected": [
  {
    "agent": "cumulo",
    "action": "materialize-fracture-pbi",
    "intent": "Gestor de Deuda Técnica — el Qué ha fallado."
  },
  {
    "agent": "mayeuta",
    "action": "enrich-fracture-pbi-kaizen",
    "intent": "Auditor Kaizen — el Por Qué y propuesta evolutiva."
  }
]
```

### 4.3 Reacción Cúmulo

`materialize-fracture-pbi` → PBI en `docs/todos/pending/` con placeholder Mayeuta.

### 4.4 Reacción Mayeuta

`enrich-fracture-pbi-kaizen` → sección **Conclusión Analítica y Propuesta Evolutiva** con veredicto (`new_norm` | `refactor_tool` | `prompt_adjustment` | `process_fix`).

### 4.5 Backfill Fase C

`audit-entity-eda-coverage.py --emit --correlation-id delivery-close-hook-eda-governance` para entidades Kintsugi (`orphan_count_after: 0`).

### 4.6 Protocolo operador

1. Intercepción → detener.
2. Emitir `System_Fracture_Detected` en bus.
3. Cúmulo materializa PBI (Qué).
4. Mayeuta enriquece PBI (Por Qué).
5. Notificar Vértice Biológico; sin bypass silencioso.

---

## Criterios de aceptación globales

- [ ] Argos APTO en `validacion.md`
- [ ] Smoke hook → un Presented → processed
- [ ] PR #20 retroactivo en bus
- [ ] Norma + evento + suscripción publicados
- [ ] PBI origen movido a `docs/todos/done/`
