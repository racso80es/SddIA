---
contract_version: "1.0.0"
entity_type: "tool"
jurisdiction: "Dominio Local / Proyecto"
---

# Contrato de Tools (S+ Grade)

Este documento rige las Tools: herramientas de dominio específicas creadas por y para un proyecto particular (ej. scripts de base de datos locales, test runners a medida).

## 1. Identidad Atómica (Innegociable)
Aunque sean locales, las Tools heredan el rigor S+ Grade. Deben poseer un `{name}.md` con:
* **`uuid`**: Identificador único universal (v4).
* **`name`**: Nombre con aporte de contexto sobre la entidad.
* **`version`**: SemVer.
* **`contract`**: Versión de contrato implementado.
* **`domain_origin`**: Declaración del proyecto o contexto específico al que pertenecen.
* **`context`**: Atributo obligatorio de Política de Seguridad (ej. `quality-assurance`, `ecosystem-evolution`).

## 2. Ejecución Local
* El ejecutable de la entidad debe residir en lo indicado por cumulo en la clave 'execution_capsules'.'tools'/{name}/.
* Invocadas exclusivamente a través de Cúmulo/Cerbero, previniendo la ejecución de comandos crudos inauditables en la terminal.

## 3. Consciencia Espacial (Ruteo Dinámico)
* Al ser inyectadas en la estructura SddIA, Cúmulo actualizará el `cumulo.paths.json` local para indexarlas.
* Las Tools se invocan a través de Cúmulo, previniendo que los agentes utilicen comandos de terminal directos e inauditables.

## 4. Interfaz de Interacción
Deben respetar el estándar de comunicación:
* **`inputs`**: Datos requeridos vía argumentos o stdin estructurado.
* **`outputs`**: Resultados, logs o códigos de salida estructurados para que las Actions puedan interpretarlos sin alucinaciones.

## 5. Física del Valor y Evolución (Bloque Latente)
* `minteo_maximo`: Límite de uso definido por el arquitecto local.
* `porcentaje_de_exito`: Métrica auditable del rendimiento de la herramienta en el entorno local.