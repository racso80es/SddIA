---
uuid: "acdb6c88-f0d9-4e10-9d2f-7e4b5401a892"
name: kalma2-interact
version: "1.1.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- ecosystem-evolution
hash_signature: sha256:405f34ec7453e8669d7f6a5aef84097c0e5a1d773d5ba286e7897a9a4d9458e1
inputs:
- prompt: Texto del operador desde el cliente Kalma2
outputs:
- response: Respuesta Mayeuta (LLM o fallback determinista)
- emitted: (opcional) true si se encoló proceso vía EDA
phases:
- name: Triaje-C
  intent: "Filtrar comandos reservados (/ ! TODO: IDEA:) antes de LLM."
  delegates_to:
  - agent:mayeuta
- name: Clasificación
  intent: CLASSIFY_INTENT vía skill mayeuta-llm (CLI Cursor o heurística).
  delegates_to:
  - skill:mayeuta-llm
- name: Enrutamiento
  intent: Procesos allowlisted → evento Kalma2_Process_Requested (asíncrono) + acuse.
  delegates_to:
  - agent:mayeuta
- name: Síntesis
  intent: SYNTHESIZE vía mayeuta-llm con degradación a síntesis determinista.
  delegates_to:
  - skill:mayeuta-llm
minteo_maximo: null
porcentaje_de_exito: null
---

# kalma2-interact

Proceso **Kalma2**: recibe `prompt` del cliente web y materializa `response` mediante skill `mayeuta-llm` (CLI local) con fallback determinista compartido (`synthesize_mayeuta_response`).

## Contrato

| Input | Obligatorio |
|-------|:-----------:|
| `prompt` | Sí |

| Output | Descripción |
|--------|-------------|
| `response` | Texto para la UI |
| `emitted` | `true` si se escribió evento `Kalma2_Process_Requested` |

## Allowlist (enrutamiento asíncrono)

`bug-fix`, `feature`, `refactorization`, `task-queue-manager`

## Configuración instancia

`SDDIA_LLM_CLI_COMMAND` en `.dev/.env` (ver `.dev/.env.example`).
