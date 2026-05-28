---
feature_name: inmunidad-caos-fase1
created: "2026-05-28"
process: feature
base: main
scope: execution-contexts, tools-contract, scripts/qa, SddIA/tools, scripts/tools
master_pbi_id: PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO
---

# Especificación técnica — Fase 1 · Arsenal de Entropía

## 1. Contexto

Estado actual (post Fase 0):

- **3 tools** catalogadas en Core (`eda-lab-smoke-may20`, `iota-immutable-publisher`, `markdown-table-editor`).
- `tools-contract` v1.2.0 **sin** § termodinámica (H07).
- `execution-contexts.md` con 8 contextos; **sin** `chaos-engineering` (H08).
- Peaje Termodinámico fail-soft **operativo** (`run_thermodynamic_toll`, D3.13).
- Fan-out `telemetry-compliance-audit` **operativo** desde `Raw_Execution_Finished`.
- `filesystem-manager` confina al repo; **sin** enforcement determinista por `workspace_path` inyectado (H10–H11).

Objetivo: materializar el **Arsenal de Entropía** sin procesos audit ni Suite (Fases 2–3).

## 2. Contexto RBAC `chaos-engineering` (1.A)

### 2.1 Nueva entrada en `execution-contexts.md`

```yaml
### 2.9. `chaos-engineering`
* **Dominio:** Ingeniería del Caos controlada e inocua.
* **Alcance:** Tools ofensivas que estresan contratos dentro del `workspace_path` inyectado.
* **Cápsulas asociadas:** `io-choke`, `schema-corruptor`, `sandbox-breacher`.
* **Restricción:** Prohibida escritura/lectura fuera del `workspace_path` declarado en stdin.
```

Bump `version` de la norma si el frontmatter lo exige.

### 2.2 Políticas agentes (Fase 1 mínima)

| Agente | Cambio |
|--------|--------|
| Tekton | **Sin cambio** en Fase 1 (D1.6) |
| Argos | Documentar en touchpoint: validará tools caos en Fase 2 |

## 3. Contrato `tools-contract.md` v1.3.0 (1.B)

Añadir §6 (paridad `skills-contract.md` / `actions-contract.md`):

| Campo | Tipo | Default |
|-------|------|---------|
| `telemetry_provided` | boolean | `false` |
| `telemetry_schema` | string[] | `["prompt_tokens", "completion_tokens"]` si `true` |

- Bump `contract_version: "1.3.0"` en frontmatter del contrato.
- Tools existentes: sin cambio de comportamiento (`telemetry_provided` implícito false).
- `schema-corruptor`: `telemetry_provided: true` explícito.

## 4. Helper Inocuidad — `assert_workspace_bound` (1.C)

### 4.1 Ubicación

`SddIA/scripts/qa/chaos_workspace_utils.py` (módulo compartido tools + tests).

### 4.2 API

```python
def assert_workspace_bound(
    repo: Path,
    target: Path,
    workspace_path: Path,
) -> tuple[bool, str | None]:
    """True si target resuelto está bajo workspace_path. Patrón fix_tool_process_core."""
```

- Rechazar path traversal (`..`).
- `target.resolve().relative_to(workspace_path.resolve())` — misma semántica que `assert_sandbox_write`.

### 4.3 Norma

Añadir párrafo en `SddIA/norms/touchpoints-ia.md` o nota en `paths-via-cumulo.md`: tools con `context: chaos-engineering` deben validar destinos con helper antes de I/O.

## 5. Tools ofensivas (1.D)

### 5.1 Convenciones comunes

| Atributo | Valor |
|----------|-------|
| `scope` forja | `core` vía `tool-creator` o Tekton directo |
| `context` | `chaos-engineering` |
| `domain_origin` | `SddIA` |
| `contract` | `tools-contract v1.3.0` |
| `implementation_path_ref` | `scripts/tools/{name}` |

Envelope stdout (tools-contract §4):

```json
{
  "name": "<tool-name>",
  "success": true,
  "exitCode": 0,
  "message": "...",
  "result": { },
  "telemetry_receipt": { }
}
```

### 5.2 `io-choke`

| Aspecto | Detalle |
|---------|---------|
| **Propósito** | Simular fallo físico al escribir en workspace |
| **stdin** | `workspace_path`, `target_file` (default `.io-choke-target`) |
| **Lógica** | Crear archivo read-only o simular `OSError` controlado al WRITE |
| **Éxito tool** | `success: true` si el fallo simulado ocurrió (la tool cumple su misión ofensiva) |
| **Vector Fase 2** | Proceso padre debe completar con Peaje fail-soft |

### 5.3 `schema-corruptor`

| Aspecto | Detalle |
|---------|---------|
| **Propósito** | Violación contrato telemetría |
| **Frontmatter** | `telemetry_provided: true`, `telemetry_schema: ["prompt_tokens", "completion_tokens"]` |
| **stdin** | `corruption_mode`: `empty` \| `invalid_json` \| `partial` |
| **stdout** | Envelope sin recibo válido según modo |
| **Smoke** | Tras ejecución en lab, fan-out compliance emite breach (AC1.3) |

### 5.4 `sandbox-breacher`

| Aspecto | Detalle |
|---------|---------|
| **Propósito** | Intento de escape del workspace |
| **stdin** | `workspace_path`, `escape_target` (relativo, default `../breach-marker.txt`) |
| **Lógica** | Resolver destino; `assert_workspace_bound` → si falla, retornar `success: false`, `exitCode: 1`, mensaje claro |
| **Éxito defensa** | Tool reporta bloqueo (no escribe fuera) |

## 6. Índice y regresión

- Actualizar `SddIA/tools/index.md` — 3 filas nuevas con capabilities distintivas.
- Test `SddIA/scripts/qa/test_chaos_tools.py`:
  - bound helper acepta/rechaza paths
  - cada cápsula responde envelope mínimo
  - `schema-corruptor` modo `empty` no incluye recibo válido

## 7. Touchpoints (resumen)

| Artefacto | Operación |
|-----------|-----------|
| `SddIA/norms/execution-contexts.md` | §2.9 |
| `SddIA/tools/tools-contract.md` | v1.3.0 §6 |
| `SddIA/scripts/qa/chaos_workspace_utils.py` | nuevo |
| `SddIA/scripts/qa/test_chaos_tools.py` | nuevo |
| `SddIA/scripts/tools/io-choke/` | cápsula |
| `SddIA/scripts/tools/schema-corruptor/` | cápsula |
| `SddIA/scripts/tools/sandbox-breacher/` | cápsula |
| `SddIA/tools/{io-choke,schema-corruptor,sandbox-breacher}.md` | specs |
| `SddIA/tools/index.md` | catálogo |
| `SddIA/norms/touchpoints-ia.md` | nota Inocuidad (mínima) |

## 8. Criterios de aceptación (trazabilidad)

| AC PBI | Verificador spec |
|--------|------------------|
| AC1.1 | §2 + §5 + §6 índice |
| AC1.2 | §4 + §5.4 |
| AC1.3 | §5.3 + test/smoke compliance |
