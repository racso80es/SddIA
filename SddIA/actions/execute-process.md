---
uuid: "f1e2d3c4-b5a6-4789-b012-cdef34567890"
name: "execute-process"
version: "1.1.0"
contract: "actions-contract v1.1.0"
context: "ecosystem-evolution"
capabilities:
  - "process-load-ssot"
  - "phase-graph-resolution"
  - "phase-invocation-binding"
  - "cerbero-policy-gate"
  - "capsule-fanout-skills-tools"
  - "crypto-broker-delegation"
inputs:
  - "process_name": "Identificador kebab-case del proceso a ejecutar (definición bajo `directories.process` en `cumulo.paths.json`)"
  - "process_inputs": "JSON con los parámetros exigidos por el proceso destino"
outputs:
  - "execution_report": "JSON agregado: fases recorridas, invocaciones resueltas (`phase_invocations`), stdin/stdout por cápsula, bindings aplicados y cierre"
  - "status_code": "0 si todas las fases completaron; 1 si Cerbero bloquea, falla topología cumulo o error fatal de fase"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: execute-process

## 1. Propósito
Orquestar de extremo a extremo la ejecución autónoma de un **proceso** del Core SddIA: resolver su ley vía Cúmulo, recorrer sus fases con Tekton, someter cada intención ejecutiva a Cerbero y delegar la física exclusivamente en **skills**, **tools** o **actions** autorizados. A partir de `process-contract v1.2.0`, las fases pueden declarar **`phase_invocations`**: secuencias ejecutables con stdin JSON, bindings de salida y reglas de error.

## 2. Orquestación lógica (sin terminal nativa)

1. **Carga de ley (Cúmulo):** Tekton solicita al agente **Cúmulo** (o al manifiesto SSOT `cumulo.paths.json` inyectado en su `cumulo_topology`) la ruta canónica de `directories.process` y compone la ruta física del archivo `{process_name}.md`. Si el proceso no existe o el mapa es inválido, emitir `status_code: 1` y detener.
2. **Iteración de fases (Tekton):** Tekton lee el YAML y el cuerpo del proceso, materializa la lista ordenada de `phases` y, por cada fase, identifica qué **actions**, **skills** o **tools** deben invocarse según `delegates_to`. Resuelve siempre rutas y contratos vía claves cumulo (`directories.actions`, `directories.skills`, `directories.tools`, `execution_capsules`), nunca por rutas absolutas inventadas.
3. **Invocaciones ejecutables (`phase_invocations`):** Si el proceso declara `phase_invocations`, para cada fase con bloque coincidente (`phase_name` == `phases[].name`), Tekton **no deduce** el stdin desde la prosa: ejecuta en orden cada elemento de `invocations`:
   * **Resolución de stdin:** Si existe `stdin_json`, usarlo literalmente como cuerpo UTF-8 enviado a la cápsula. Si existe `stdin_spec`, construir el objeto final: copiar `operation`, `target_type` (si aplica) y calcular `target_payload` según `target_payload.type`:
     - `canonical_json_utf8`: si existe `target_payload.from_process_input`, leer `process_inputs[...]` y serializar con `json.dumps` según `json_dumps`. Si existe `target_payload.from_process_inputs` (array de claves), construir `{k: process_inputs[k]}` con claves ordenadas alfabéticamente y serializar ese objeto. El valor enviado a la cápsula es la **cadena** Unicode resultante de `dumps` (igual que `GENERATE_SHA256` + `STRING` en `cryptography-manager`).
   * **Cerbero antes de cada cápsula:** Presentar `target_capsule` con el `context` de la cápsula destino (según definición `{name}.md` de skill/action/tool). Si el contexto **no** está en `allowed_policies` de la entidad solicitante, aplicar **delegación broker** (solo si `capsule` == `action:crypto-broker`): evaluar Cerbero usando el **contexto de la acción broker** (`quality-assurance` según `crypto-broker.md`), no el del orquestador padre. Si el broker tampoco está permitido en la política de delegación explícita del runtime, emitir `exitCode: 1` y abortar.
   * **Ejecución:** Para `action:crypto-broker`, el stdin de la sub-acción es el objeto `crypto_request` equivalente al stdin de `cryptography-manager` (mismo schema que `stdin_json` / objeto construido desde `stdin_spec`). Ejecutar la cápsula física `cryptography-manager.py` como describe `crypto-broker`. Para `skill:filesystem-manager` (modalidad LLM-Native), traducir a operaciones del IDE según contrato. Para otras cápsulas, resolver motor (LLM-Native vs binario) según su definición.
   * **Bindings:** Tras stdout JSON válido, aplicar `bind`: para cada par `ruta_en_respuesta -> variable_estado`, persistir en `execution_state` (p. ej. `data.result` → `child_process_uuid`). Las variables alimentan pasos posteriores y ensamblado de YAML.
   * **Errores:** Si `on_error` es `abort`, cualquier `exitCode != 0` de la cápsula o stderr no vacío mapeado a fallo debe detener el proceso con `status_code: 1` y registrar causa en `execution_report`.
4. **Paso por el centinela (Cerbero) — fases sin `phase_invocations` detalladas:** Antes de cada ejecución física de cápsula inferida solo desde `delegates_to`, Tekton presenta a **Cerbero** la intención (`entity_request` con políticas de la entidad solicitante, `target_capsule` con el `context` de la cápsula). Cerbero cruza contra `execution-contexts.md` (ruta vía `directories.norms`). Si el `context` de la actividad **no** está en `allowed_policies`, Cerbero responde `exitCode: 1`: abortar la acción y devolver `status_code: 1` con causa en `execution_report`.
5. **Ejecución y cierre:** Tras autorización, se emite la invocación estructurada (`skill_invocations`, `action_invocations` o homólogo), se registran en `execution_report` por cada fase: `invocation_index`, `capsule`, `stdin_resolved` (sin secretos), `stdout_json`, `bindings_applied`, `cerbero_context_used`. Al completar la última fase sin abortos, `status_code: 0`.

## 3. Contenido mínimo de `execution_report` (por fase con `phase_invocations`)
* `phase_name`
* `invocations`: lista de `{ capsule, stdin, stdout, exitCode, bind }`

## 4. Límites y referencias de agentes
* **Tekton:** definición operativa bajo `paths.directories.agents` — prohibición explícita de terminal nativa; obediencia a fases y topología inyectada.
* **Cerbero:** definición operativa bajo `paths.directories.agents` — intercepción pura; bloqueo binario si el contexto de la cápsula no está permitido para la entidad solicitante, salvo regla explícita de **crypto-broker** en §2.3.
* **Contrato de acciones:** `paths.contracts.actions` — sin acceso directo al SO; solo delegación en skills/tools/actions con ruteo cumulo.
* **Contrato de process:** `paths.contracts.process` — schema `phase_invocations` v1.2.0.
