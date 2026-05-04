---
contract_version: "1.1.0"
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

## 3. Interfaz de Interacción
Toda acción debe declarar explícitamente en su `{name}.md`:
* **`inputs`**: Esquema JSON de los datos requeridos para iniciar la orquestación (ej. contexto del repositorio, instrucciones de un agente).
* **`outputs`**: Esquema JSON del resultado devuelto al finalizar, incluyendo estados de éxito o error.

## 4. Física del Valor y Evolución (Bloque Latente)
El esquema permite la inclusión de métricas de termodinámica operativa:
* `minteo_maximo`: Límite de veces que esta orquestación puede ser invocada o instanciada.
* `porcentaje_de_exito`: Variable auditable basada en cuántas veces la acción completó su ejecución sin devolver errores fatales.