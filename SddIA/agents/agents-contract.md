---
contract_version: "1.0.0"
entity_type: "agent"
jurisdiction: "Core SddIA"
---

# Contrato de Agentes (S+ Grade)

Este documento rige la estructura, obligaciones y límites operativos de cualquier Agente (entidad de dominio) que opere dentro del ecosistema SddIA. Ningún agente podrá ser instanciado o reconocido por Cúmulo si viola las siguientes cláusulas.

## 1. Identidad Atómica (Innegociable)
Todo agente habita en su capsula (carpeta) ubicada según directrices de cúmulo con nombre '{name}'
Todo agente debe poseer un documento de definición (`{name}.md`) que declare obligatoriamente:
* **`uuid`**: Identificador único universal (v4). Inmutable a lo largo de la vida del agente.
* **`name`**: Nombre con aporte de contexto sobre la entidad.
* **`version`**: Control de versiones semántico (SemVer).
* **`hash_signature`**: (Opcional en desarrollo, obligatorio en producción) Firma criptográfica que valida la integridad de sus instrucciones base.

## 2. Consciencia Espacial (Obediencia al SSOT)
Queda estrictamente prohibido que un agente posea rutas locales o "hardcodeadas" hacia archivos del ecosistema. 
* Todo agente está obligado a leer su topología desde el mapa maestro: `cumulo.paths.json`.
* Cualquier interacción con otras entidades de dominio SddIA debe extraerse primero de las rutas definidas por Cúmulo. La alucinación de rutas se considera un fallo de integridad letal.

## 3. Interfaz de Interacción
La comunicación con el agente debe ser estandarizada. Todo agente debe declarar en su definición:
* **`inputs`**: Estructura de datos requerida para iniciar su ciclo de pensamiento.
* **`outputs`**: Artefactos esperados tras su ejecución (ej. un archivo Markdown, un JSON, un comando de terminal delegado a un skill).

## 4. Física del Valor y Evolución (Bloque Latente)
Para facilitar el Desarrollo Kaizen y la futura industrialización, el esquema del agente permite (de forma opcional en la Fase Core) la inclusión de métricas de termodinámica operativa:
* `minteo_maximo`: Límite de instancias o usos permitidos antes de requerir evolución.
* `porcentaje_de_exito`: Variable actualizable que audita la eficacia histórica del agente en sus tareas.
