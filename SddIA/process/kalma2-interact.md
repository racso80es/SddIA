---
context:
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:0267edbbdc4ce322115cbf1b4935cd92635202cb22a37e398e78f8d4af03de72
inputs:
- prompt: Texto del operador desde el cliente Kalma2
- project_slug: (Opcional) Slug del proyecto registrado en `instance.projects`; ausente → Core-self
minteo_maximo: null
name: kalma2-interact
outputs:
- response: Respuesta Mayeuta (LLM o fallback determinista)
- emitted: (opcional) true si se encoló proceso vía EDA
phases:
- delegates_to:
  - agent:mayeuta
  intent: 'Filtrar comandos reservados (/ ! TODO: IDEA:) antes de LLM.'
  name: Triaje-C
  id: triaje_c
- delegates_to:
  - skill:mayeuta-llm
  intent: CLASSIFY_INTENT vía skill mayeuta-llm (CLI Cursor o heurística).
  name: Clasificación
  requires_capability:
  - contract: llm.interact
    id: llm:interact
    version: '>=1.0.0'
  id: clasificacion
- delegates_to:
  - agent:mayeuta
  intent: Procesos allowlisted → evento Kalma2_Process_Requested (asíncrono) + acuse.
  name: Enrutamiento
  id: enrutamiento
- delegates_to:
  - skill:mayeuta-llm
  intent: SYNTHESIZE vía mayeuta-llm con degradación a síntesis determinista.
  name: Síntesis
  requires_capability:
  - contract: llm.interact
    id: llm:interact
    version: '>=1.0.0'
  id: sintesis
porcentaje_de_exito: null
uuid: acdb6c88-f0d9-4e10-9d2f-7e4b5401a892
version: 1.1.2
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# kalma2-interact

Proceso **Kalma2**: recibe `prompt` del cliente web y materializa `response` mediante skill `mayeuta-llm` (CLI local) con fallback determinista compartido (`synthesize_mayeuta_response`).

## Contrato

| Input | Obligatorio |
|-------|:-----------:|
| `prompt` | Sí |
| `project_slug` | No |

| Output | Descripción |
|--------|-------------|
| `response` | Texto para la UI |
| `emitted` | `true` si se escribió evento `Kalma2_Process_Requested` |

## Allowlist (enrutamiento asíncrono)

`bug-fix`, `feature`, `refactorization`, `task-queue-manager`

## Configuración instancia

`SDDIA_LLM_CLI_COMMAND` en `.dev/.env` (ver `.dev/.env.example`).
