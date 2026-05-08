---
uuid: "f1e2d3c4-b5a6-4789-b012-cdef34567890"
name: "execute-process"
version: "1.1.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "process-load-ssot"
  - "phase-graph-resolution"
  - "phase-invocation-binding"
  - "cerbero-policy-gate"
  - "capsule-fanout-skills-tools"
  - "crypto-broker-delegation"
inputs:
  - "process_name": "Identificador kebab-case del proceso a ejecutar (definiciÃ³n bajo `directories.process` en `cumulo.paths.json`)"
  - "process_inputs": "JSON con los parÃ¡metros exigidos por el proceso destino"
outputs:
  - "execution_report": "JSON agregado: fases recorridas, invocaciones resueltas (`phase_invocations`), stdin/stdout por cÃ¡psula, bindings aplicados y cierre"
  - "status_code": "0 si todas las fases completaron; 1 si Cerbero bloquea, falla topologÃ­a cumulo o error fatal de fase"
minteo_maximo: null
porcentaje_de_exito: null
---

# AcciÃ³n: execute-process

## 1. PropÃ³sito
Orquestar de extremo a extremo la ejecuciÃ³n autÃ³noma de un **proceso** del Core SddIA: resolver su ley vÃ­a CÃºmulo, recorrer sus fases con Tekton, someter cada intenciÃ³n ejecutiva a Cerbero y delegar la fÃ­sica exclusivamente en **skills**, **tools** o **actions** autorizados. A partir de `process-contract v1.2.0`, las fases pueden declarar **`phase_invocations`**: secuencias ejecutables con stdin JSON, bindings de salida y reglas de error.

## 2. OrquestaciÃ³n lÃ³gica (sin terminal nativa)

1. **Carga de ley (CÃºmulo):** Tekton solicita al agente **CÃºmulo** (o al manifiesto SSOT `cumulo.paths.json` inyectado en su `cumulo_topology`) la ruta canÃ³nica de `directories.process` y compone la ruta fÃ­sica del archivo `{process_name}.md`. Si el proceso no existe o el mapa es invÃ¡lido, emitir `status_code: 1` y detener.
2. **IteraciÃ³n de fases (Tekton):** Tekton lee el YAML y el cuerpo del proceso, materializa la lista ordenada de `phases` y, por cada fase, identifica quÃ© **actions**, **skills** o **tools** deben invocarse segÃºn `delegates_to`. Resuelve siempre rutas y contratos vÃ­a claves cumulo (`directories.actions`, `directories.skills`, `directories.tools`, `execution_capsules`), nunca por rutas absolutas inventadas.
3. **Invocaciones ejecutables (`phase_invocations`):** Si el proceso declara `phase_invocations`, para cada fase con bloque coincidente (`phase_name` == `phases[].name`), Tekton **no deduce** el stdin desde la prosa: ejecuta en orden cada elemento de `invocations`:
   * **ResoluciÃ³n de stdin:** Si existe `stdin_json`, usarlo literalmente como cuerpo UTF-8 enviado a la cÃ¡psula. Si existe `stdin_spec`, construir el objeto final: copiar `operation`, `target_type` (si aplica) y calcular `target_payload` segÃºn `target_payload.type`:
     - `canonical_json_utf8`: si existe `target_payload.from_process_input`, leer `process_inputs[...]` y serializar con `json.dumps` segÃºn `json_dumps`. Si existe `target_payload.from_process_inputs` (array de claves), construir `{k: process_inputs[k]}` con claves ordenadas alfabÃ©ticamente y serializar ese objeto. El valor enviado a la cÃ¡psula es la **cadena** Unicode resultante de `dumps` (igual que `GENERATE_SHA256` + `STRING` en `cryptography-manager`).
   * **Cerbero antes de cada cÃ¡psula:** Presentar `target_capsule` con el `context` de la cÃ¡psula destino (segÃºn definiciÃ³n `{name}.md` de skill/action/tool). Si el contexto **no** estÃ¡ en `allowed_policies` de la entidad solicitante, aplicar **delegaciÃ³n broker** (solo si `capsule` == `action:crypto-broker`): evaluar Cerbero usando el **contexto de la acciÃ³n broker** (`quality-assurance` segÃºn `crypto-broker.md`), no el del orquestador padre. Si el broker tampoco estÃ¡ permitido en la polÃ­tica de delegaciÃ³n explÃ­cita del runtime, emitir `exitCode: 1` y abortar.
   * **EjecuciÃ³n:** Para `action:crypto-broker`, el stdin de la sub-acciÃ³n es el objeto `crypto_request` equivalente al stdin de `cryptography-manager` (mismo schema que `stdin_json` / objeto construido desde `stdin_spec`). Ejecutar la cÃ¡psula fÃ­sica `cryptography-manager.py` como describe `crypto-broker`. Para `skill:filesystem-manager` (modalidad LLM-Native), traducir a operaciones del IDE segÃºn contrato. Para otras cÃ¡psulas, resolver motor (LLM-Native vs binario) segÃºn su definiciÃ³n.
   * **Bindings:** Tras stdout JSON vÃ¡lido, aplicar `bind`: para cada par `ruta_en_respuesta -> variable_estado`, persistir en `execution_state` (p. ej. `data.result` â†’ `child_process_uuid`). Las variables alimentan pasos posteriores y ensamblado de YAML.
   * **Errores:** Si `on_error` es `abort`, cualquier `exitCode != 0` de la cÃ¡psula o stderr no vacÃ­o mapeado a fallo debe detener el proceso con `status_code: 1` y registrar causa en `execution_report`.
4. **Paso por el centinela (Cerbero) â€” fases sin `phase_invocations` detalladas:** Antes de cada ejecuciÃ³n fÃ­sica de cÃ¡psula inferida solo desde `delegates_to`, Tekton presenta a **Cerbero** la intenciÃ³n (`entity_request` con polÃ­ticas de la entidad solicitante, `target_capsule` con el `context` de la cÃ¡psula). Cerbero cruza contra `execution-contexts.md` (ruta vÃ­a `directories.norms`). Si el `context` de la actividad **no** estÃ¡ en `allowed_policies`, Cerbero responde `exitCode: 1`: abortar la acciÃ³n y devolver `status_code: 1` con causa en `execution_report`.
5. **EjecuciÃ³n y cierre:** Tras autorizaciÃ³n, se emite la invocaciÃ³n estructurada (`skill_invocations`, `action_invocations` o homÃ³logo), se registran en `execution_report` por cada fase: `invocation_index`, `capsule`, `stdin_resolved` (sin secretos), `stdout_json`, `bindings_applied`, `cerbero_context_used`. Al completar la Ãºltima fase sin abortos, `status_code: 0`.

## 3. Contenido mÃ­nimo de `execution_report` (por fase con `phase_invocations`)
* `phase_name`
* `invocations`: lista de `{ capsule, stdin, stdout, exitCode, bind }`

## 4. LÃ­mites y referencias de agentes
* **Tekton:** definiciÃ³n operativa bajo `paths.directories.agents` â€” prohibiciÃ³n explÃ­cita de terminal nativa; obediencia a fases y topologÃ­a inyectada.
* **Cerbero:** definiciÃ³n operativa bajo `paths.directories.agents` â€” intercepciÃ³n pura; bloqueo binario si el contexto de la cÃ¡psula no estÃ¡ permitido para la entidad solicitante, salvo regla explÃ­cita de **crypto-broker** en Â§2.3.
* **Contrato de acciones:** `paths.contracts.actions` â€” sin acceso directo al SO; solo delegaciÃ³n en skills/tools/actions con ruteo cumulo.
* **Contrato de process:** `paths.contracts.process` â€” schema `phase_invocations` v1.2.0.
