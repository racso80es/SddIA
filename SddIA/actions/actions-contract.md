---
contract_version: "1.2.0"
entity_type: "action"
jurisdiction: "Core SddIA"
capabilities:
  - "action-schema-governance"
  - "skill-tool-orchestration"
  - "ssot-delegation-routing"
---

# Contrato de Actions (S+ Grade)

Este documento rige la estructura y límites operativos de cualquier Acción (orquestación atómica) dentro del ecosistema SddIA.

## 1. Identidad Atómica (Innegociable)
Toda acción debe definirse mediante un archivo `{name}.md` que declare obligatoriamente en su cabecera YAML:
* **`uuid`**: Identificador único universal (v4). Inmutable.
* **`name`**: Nombre con aporte de contexto sobre la entidad.
* **`version`**: Control de versiones semántico (SemVer).
* **`contract`**: Versión de contrato implementado.
* **`hash_signature`**: (Opcional en desarrollo) Firma criptográfica que valida la integridad de su lógica orquestal.
* **`context`**: Atributo obligatorio de Política de Seguridad (ej. `quality-assurance`, `ecosystem-evolution`).
* **`capabilities`**: Array obligatorio de strings que etiqueta las operaciones atómicas u orquestaciones lógicas que expone la acción (enrutamiento semántico).

## 2. Consciencia Espacial (Obediencia al SSOT)
Las acciones no ejecutan código directamente ni acceden al sistema operativo. 
* Operan exclusivamente invocando Skills o Tools.
* Todo ruteo hacia esas cápsulas debe extraerse consultando a cumulo. El hardcodeo de rutas hacia dependencias es un fallo letal.

## 2bis. Frontera con Procesos (Invariante Semántico)
Las Acciones son servicios atómicos de infraestructura (validación RBAC, enrutamiento, criptografía, meta-orquestación). Queda **TERMINANTEMENTE PROHIBIDO** catalogar como acción cualquier flujo que corresponda a **fases de negocio o del ciclo de vida del producto**. Esas responsabilidades viven en los **Procesos** (`process-contract`) y son orquestadas por los **Agentes** (Mayeuta, Dedalo, Argos, Tekton). La meta-orquestación de Procesos es responsabilidad exclusiva de `action:execute-process`.

**SSOT del glosario de términos/propósitos prohibidos como `action_name`** (lista exhaustiva de rechazo):

`['planning', 'implementation', 'execution', 'clarify', 'spec', 'validate', 'difusion', 'finalize']`

Cualquier alias, traducción o variante kebab-case con la misma intención semántica se considera intersección. La `process-creator` puede usar libremente estos términos como nombres de proceso; el bloqueo aplica únicamente al **catálogo de Acciones**.

## 3. Interfaz de Interacción (I/O JSON Estricto)
Toda acción debe declarar explícitamente en su `{name}.md`:
* **`inputs`**: Esquema JSON de los datos requeridos para iniciar la orquestación.
* **`outputs`**: Esquema JSON del resultado.

La cápsula ejecutora de la acción se comunica por `stdin`/`stdout` con un envelope canónico **idéntico al de Skills** (`skills-contract.md` § 3):

```json
{ "success": <boolean>, "exitCode": <integer>, "data": <object>, "error": <string> }
```

El campo de payload de éxito es **`data`** (no `result`). En fallo, `success: false`, `exitCode != 0`, `data: null` cuando proceda y `error` con causa textual.

## 4. Física del Valor y Evolución (Bloque Latente)
El esquema permite la inclusión de métricas de termodinámica operativa:
* `minteo_maximo`: Límite de veces que esta orquestación puede ser invocada o instanciada.
* `porcentaje_de_exito`: Variable auditable basada en cuántas veces la acción completó su ejecución sin devolver errores fatales.