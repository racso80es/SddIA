---
contract_version: "1.0.0"
entity_type: "tool"
jurisdiction: "Dominio Local / Proyecto"
---

# Contrato de Tools (S+ Grade)

Este documento rige las Tools: herramientas de dominio específicas creadas por y para un proyecto particular (ej. scripts de base de datos locales, test runners a medida).

## 1. Identidad Atómica (Innegociable)
Toda tool habita en su capsula (carpeta) ubicada según directrices de cúmulo con nombre '{name}'
Aunque sean locales, las Tools heredan el rigor S+ Grade. Deben poseer un `{name}.md` con:
* **`uuid`**: Identificador único universal (v4).
* **`name`**: Nombre con aporte de contexto sobre la entidad.
* **`version`**: SemVer.
* **`contrato`**: Versión de contrato implementado.
* **`domain_origin`**: Declaración del proyecto o contexto específico al que pertenecen.

## 2. Consciencia Espacial (Ruteo Dinámico)
* Al ser inyectadas en la estructura SddIA, Cúmulo actualizará el `cumulo.paths.json` local para indexarlas.
* Las Tools se invocan a través de Cúmulo, previniendo que los agentes utilicen comandos de terminal directos e inauditables.

## 3. Interfaz de Interacción
Al igual que las Skills, deben respetar el estándar de comunicación:
* **`inputs`**: Datos requeridos vía argumentos o stdin estructurado.
* **`outputs`**: Resultados, logs o códigos de salida estructurados para que las Actions puedan interpretarlos sin alucinaciones.

## 4. Física del Valor y Evolución (Bloque Latente)
* `minteo_maximo`: Límite de uso definido por el arquitecto local.
* `porcentaje_de_exito`: Métrica auditable del rendimiento de la herramienta en el entorno local.