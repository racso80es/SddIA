---
contract_version: "1.0.0"
entity_type: "action"
jurisdiction: "Core SddIA"
---

# Contrato de Actions (S+ Grade)

Este documento rige la estructura y límites operativos de cualquier Acción (orquestación atómica) dentro del ecosistema SddIA.

## 1. Identidad Atómica (Innegociable)
Toda acción habita en su capsula (carpeta) ubicada según directrices de cúmulo con nombre '{name}'
Toda acción debe definirse mediante un archivo `{name}.md` que declare obligatoriamente en su cabecera YAML:
* **`uuid`**: Identificador único universal (v4). Inmutable.
* **`name`**: Nombre con aporte de contexto sobre la entidad.
* **`version`**: Control de versiones semántico (SemVer).
* **`contrato`**: Versión de contrato implementado.
* **`hash_signature`**: (Opcional en desarrollo) Firma criptográfica que valida la integridad de su lógica orquestal.

## 2. Consciencia Espacial (Obediencia al SSOT)
Las acciones no ejecutan código directamente ni acceden al sistema operativo. 
* Operan exclusivamente invocando Skills o Tools.
* Todo ruteo hacia esas cápsulas debe extraerse del mapa maestro: `cumulo.paths.json`. El hardcodeo de rutas hacia dependencias es un fallo letal.

## 3. Interfaz de Interacción
Toda acción debe declarar explícitamente en su `{name}.md`:
* **`inputs`**: Esquema JSON de los datos requeridos para iniciar la orquestación (ej. contexto del repositorio, instrucciones de un agente).
* **`outputs`**: Esquema JSON del resultado devuelto al finalizar, incluyendo estados de éxito o error.

## 4. Física del Valor y Evolución (Bloque Latente)
El esquema permite la inclusión de métricas de termodinámica operativa:
* `minteo_maximo`: Límite de veces que esta orquestación puede ser invocada o instanciada.
* `porcentaje_de_exito`: Variable auditable basada en cuántas veces la acción completó su ejecución sin devolver errores fatales.