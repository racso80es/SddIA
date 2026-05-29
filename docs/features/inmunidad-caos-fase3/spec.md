---
feature_name: inmunidad-caos-fase3
created: "2026-05-29"
process: feature
base: main
scope: SddIA/suites, entity-manager, suite-creator, execute-suite, cumulo.paths, execute_process_capsules, workspace_utils, scripts/qa
master_pbi_id: PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO
---

# Especificación técnica — Fase 3 · Genoma de la Suite

## 1. Contexto

Estado actual (post Fase 2):

- **3 procesos audit atómicos** operativos con handlers lab (`run_chaos_audit_process`).
- **0 artefactos Suite** en genoma (H01–H05): no `SddIA/suites/`, no `suite-creator`, enum `entity-manager` sin `suite`.
- **`invoke_subprocess_process`** lanza CLI `execute-process` sin propagación explícita de sub-`workspace_path` / `execution_id` por nodo (H14).
- **`workspace_utils`** materializa un workspace por invocación raíz; sin API de hijo (H15).
- **`survival-manifest.md`** sin contrato ni plantilla (H17).
- Eventos ECST y DLT inmunidad **fuera de alcance** (Fase 4).

Objetivo: materializar la **ED `Suite`** y el **orquestador `execute-suite`** con aislamiento verificable por nodo.

## 2. Convenciones transversales

| Atributo | Valor |
|----------|-------|
| `contract` Suite | `suites-contract v1.0.0` (nuevo) |
| `contract` orquestador | `process-contract v1.4.0` |
| `workspace_template` orquestador | `.SddIA/workspaces/{process_name}/{execution_id}/` |
| Contexto orquestador | `chaos-engineering`, `quality-assurance`, `ecosystem-evolution` |
| Atomicidad | Cada `atomic_node` referencia **un** proceso audit existente; Suite no mezcla tools |

## 3. Extensión genómica (3.A)

### 3.1 `cumulo.paths.json`

Añadir:

```json
"directories": {
  "suites": "SddIA/suites"
},
"contracts": {
  "suites": "SddIA/suites/suites-contract.md"
}
```

### 3.2 `entity-manager.md`

- Ampliar enum `entity_class`: añadir `suite`.
- Tabla delegación: `suite` → `suite-creator`.
- Mapeo `semantic_seed` → inputs `suite-creator` (§3.3).

### 3.3 `suite-creator` (nuevo proceso)

Simetría con `tool-creator` / `norm-creator`:

| Input | Descripción |
|-------|-------------|
| `suite_name` / `entity_name` | kebab-case (`core-full-stress`) |
| `suite_context` | default `chaos-engineering` |
| `execution_strategy` | `fail_fast` \| `run_all` |
| `atomic_nodes` | array de objetos (ver §4) |
| `suite_version` | default `1.0.0` |
| `suites_contract_version` | default `1.0.0` |

Fases previstas:

1. **Validación de Dominio** — Cúmulo + Cerbero; unicidad bajo `directories.suites`.
2. **Clasificación Semántica** — crypto-broker UUID; validar `atomic_nodes` no vacío.
3. **Materialización** — `{suite_name}.md` conforme `suites-contract`.
4. **Indexación** — fila en `SddIA/suites/index.md`.

Outputs handoff: `handoff_entity_uuid`, `handoff_hash_signature_new`, `handoff_version`.

Handler lab: `run_suite_forge` en `execute_process_capsules.py` (paridad `run_tool_forge`).

### 3.4 `sync-entity-index.md`

Añadir fila índice:

| `entity_class` | Ruta |
|----------------|------|
| `suite` | `SddIA/suites/index.md` |

### 3.5 `entidades-dominio-ecosistema-sddia.md`

Añadir **Suites** a la lista de entidades de dominio con referencia a `suites-contract`.

## 4. Ley de la Suite — `suites-contract.md` (3.B)

### 4.1 Frontmatter obligatorio

```yaml
uuid: "<uuid-v4>"
name: "<kebab-case>"
version: "1.0.0"
contract: "suites-contract v1.0.0"
context:
  - chaos-engineering
hash_signature: "sha256:<canon>"
execution_strategy: run_all  # fail_fast | run_all
atomic_nodes:
  - process_name: audit-thermodynamic-toll-failsoft
    expected_exit_code: 0
    timeout_ms: 120000
  - process_name: audit-telemetry-compliance-breach
    expected_exit_code: 0
    timeout_ms: 120000
  - process_name: audit-sandbox-isolation-rbac
    expected_exit_code: 0
    timeout_ms: 120000
```

### 4.2 Reglas contractuales

- `atomic_nodes` **no vacío**; cada `process_name` debe existir en `SddIA/process/index.md`.
- Prohibido referenciar tools directamente — solo procesos (preserva Atomicidad Diagnóstica).
- `timeout_ms` opcional; default orquestador 300000 ms si ausente.
- Hash canon: JSON ordenado de `atomic_nodes` + `execution_strategy` + `version`.

### 4.3 Catálogo

- `SddIA/suites/index.md` — columnas: Archivo | uuid | name | version | execution_strategy | node_count

## 5. Proceso `execute-suite` (3.C)

### 5.1 Propósito

Orquestar secuencia de procesos audit declarados en una Suite; aislar workspace por nodo; compilar manifiesto Argos.

### 5.2 Definición YAML (resumen)

```yaml
name: execute-suite
version: "1.0.0"
context: [chaos-engineering, quality-assurance, ecosystem-evolution]
inputs:
  - suite_id: Identificador kebab-case de la Suite (required)
  - execution_strategy: Override opcional fail_fast | run_all
outputs:
  - survival_manifest_path: Ruta relativa al manifiesto compilado
  - nodes_executed: Conteo de nodos ejecutados
phases:
  - name: Resolución Suite
    intent: Cargar spec Suite desde Cúmulo/directories.suites.
    delegates_to: [agent:cumulo]
  - name: Orquestación nodos
    intent: Por cada atomic_node, subproceso execute-process aislado.
    delegates_to: [agent:tekton, action:execute-process]
  - name: Compilación manifiesto
    intent: Argos escribe survival-manifest.md en workspace orquestador.
    delegates_to: [agent:argos]
```

### 5.3 Handler lab — `run_execute_suite`

Flujo:

1. **Bootstrap** workspace orquestador (`bootstrap_process_workspace`).
2. **Cargar Suite** — leer `{repo}/SddIA/suites/{suite_id}.md`, parsear frontmatter.
3. **Resolver estrategia** — input override o `execution_strategy` de Suite.
4. **Por cada `atomic_node`** (índice `i`):
   - Generar `child_execution_id = uuid4()`.
   - Resolver template del proceso hijo desde `process/{process_name}.md`.
   - Materializar sub-workspace:
     ```
     {orchestrator_ws}/nodes/{i:02d}-{process_name}/{child_execution_id}/
     ```
   - Invocar:
     ```python
     invoke_subprocess_process(repo, process_name, {
         "workspace_path": str(child_ws),
         "execution_id": child_execution_id,
         "parent_execution_id": state["execution_id"],
         "parent_suite_id": suite_id,
     })
     ```
   - Capturar `exit_code`, duración, rutas en `node_reports[]`.
   - Si `fail_fast` y `exit_code != expected_exit_code`: break.
5. **Fase Argos** — escribir `survival-manifest.md` (§6).
6. **Retorno** — envelope con `execution_report.nodes[]` (AC3.3).

### 5.4 Extensión `workspace_utils.py`

Nueva función (D0.6):

```python
def materialize_child_workspace(
    repo: Path,
    orchestrator_workspace: Path,
    node_index: int,
    process_name: str,
    execution_id: str,
) -> Path:
    rel = orchestrator_workspace / "nodes" / f"{node_index:02d}-{process_name}" / execution_id
    rel.mkdir(parents=True, exist_ok=True)
    return rel.resolve()
```

### 5.5 `execution_report` enriquecido

```json
{
  "process_name": "execute-suite",
  "suite_id": "core-full-stress",
  "execution_strategy": "run_all",
  "nodes": [
    {
      "index": 0,
      "process_name": "audit-thermodynamic-toll-failsoft",
      "execution_id": "<uuid>",
      "workspace_path": "<abs>",
      "expected_exit_code": 0,
      "actual_exit_code": 0,
      "duration_ms": 1234,
      "verdict": "pass"
    }
  ],
  "phases": [...]
}
```

## 6. Manifiesto de supervivencia (3.D)

Ruta: `{orchestrator_workspace_path}/survival-manifest.md`

Plantilla:

```markdown
# Survival Manifest — {suite_id}

| Campo | Valor |
|-------|-------|
| suite_id | core-full-stress |
| orchestrator_execution_id | {uuid} |
| execution_strategy | run_all |
| compiled_at | ISO-8601 |

## Nodos

| # | process_name | execution_id | workspace_path | expected | actual | verdict |
|---|--------------|--------------|----------------|----------|--------|---------|
| 0 | audit-thermodynamic-toll-failsoft | ... | ... | 0 | 0 | pass |
```

Generado por handler fase Argos; no requiere LLM en lab.

## 7. Instancia `core-full-stress.md` (3.E)

Suite referencia que encadena los tres procesos Fase 2:

| Nodo | Proceso | Vector |
|------|---------|--------|
| 0 | `audit-thermodynamic-toll-failsoft` | Peaje fail-soft |
| 1 | `audit-telemetry-compliance-breach` | Compliance breach |
| 2 | `audit-sandbox-isolation-rbac` | Sandbox RBAC |

`execution_strategy: run_all` — campaña de asedio completa antes de manifiesto.

Forja vía `suite-creator` o materialización directa + indexación (lab).

## 8. Integración `execute_process_capsules.py`

| Función | Responsabilidad |
|---------|-----------------|
| `load_suite_spec(repo, suite_id)` | Parse frontmatter Suite |
| `run_execute_suite(...)` | Orquestador completo |
| `compile_survival_manifest(...)` | Escritura Markdown manifiesto |
| `run_suite_forge(...)` | Forja vía entity-manager path |
| Rama en `run_process()` | `if canonical == "execute-suite"` |

Constante auxiliar:

```python
SUITE_ORCHESTRATOR_PROCESSES = frozenset({"execute-suite"})
```

## 9. Tests QA

Nuevo `SddIA/scripts/qa/test_execute_suite.py`:

| Test | AC |
|------|-----|
| `test_entity_manager_accepts_suite_class` | AC3.1 — smoke semantic_seed mínimo |
| `test_execute_suite_core_full_stress_smoke` | AC3.2 — orquestador + manifiesto existe |
| `test_execute_suite_isolated_sub_workspaces` | AC3.3 — paths distintos en `nodes[]` |
| `test_execute_suite_fail_fast_aborts` | D3.7 — fixture Suite mock con nodo fallido |
| `test_core_full_stress_suite_spec_valid` | Validación frontmatter + 3 procesos referenciados |

Fixture plantilla: `docs/features/inmunidad-caos-fase3/_smoke-execute-suite-core-full-stress.json`

## 10. Touchpoints (resumen)

| Artefacto | Operación |
|-----------|-----------|
| `SddIA/core/cumulo.paths.json` | +directories.suites, +contracts.suites |
| `SddIA/suites/suites-contract.md` | nuevo |
| `SddIA/suites/index.md` | nuevo |
| `SddIA/suites/core-full-stress.md` | nuevo |
| `SddIA/process/suite-creator.md` | nuevo |
| `SddIA/process/execute-suite.md` | nuevo |
| `SddIA/process/entity-manager.md` | enum + tabla + seed map |
| `SddIA/process/index.md` | +2 filas (suite-creator, execute-suite) |
| `SddIA/actions/sync-entity-index.md` | fila suite |
| `SddIA/norms/entidades-dominio-ecosistema-sddia.md` | mención Suite |
| `SddIA/scripts/qa/workspace_utils.py` | `materialize_child_workspace` |
| `SddIA/scripts/qa/execute_process_capsules.py` | handlers suite |
| `SddIA/scripts/qa/test_execute_suite.py` | nuevo |
| `SddIA/core/eda-coverage.json` | upsert entidades nuevas |
| `docs/features/inmunidad-caos-fase3/_smoke-*.json` | plantilla smoke |

## 11. Criterios de aceptación (trazabilidad)

| AC PBI | Verificador spec |
|--------|------------------|
| AC3.1 | §3 entity-manager + suite-creator + test entity class |
| AC3.2 | §5 handler + §7 core-full-stress + smoke manifiesto |
| AC3.3 | §5.4–5.5 sub-workspaces + test aislamiento |

## 12. Riesgos técnicos

| Riesgo | Mitigación |
|--------|------------|
| Subproceso hereda workspace padre por bug CLI | Pasar `workspace_path` explícito en `child_inputs`; test AC3.3 |
| Suite referencia proceso inexistente | Validación suite-creator + test spec |
| Gate EDA huérfanos al indexar | Upsert coverage o backfill documentado (D3.14) |
| Timeout hijo bloquea orquestador | `timeout_ms` por nodo; subprocess con timeout en lab |
| `run_all` paralelo prematuro | Secuencial en Fase 3; Kaizen concurrencia real post-merge |
