---
contract_version: "1.0.0"
entity_type: "skill"
jurisdiction: "Core SddIA"
---

# Contrato de Skills (S+ Grade)

Este documento rige la creación de Skills: capacidades de ejecución universales y agnósticas al proyecto, diseñadas como cápsulas blindadas.

## 1. Identidad Atómica (Innegociable)
Toda skill debe poseer un `{name}.md` en su capsula de definición con:
* **`uuid`**: Identificador único universal (v4).
* **`name`**: Nombre con aporte de contexto sobre la entidad.
* **`version`**: SemVer.
* **`contract`**: Versión de contrato implementado.
* **`hash_signature`**: Firma del binario o script ejecutable asociado, garantizando que el código no ha sido manipulado (vital para operaciones de sistema).
* **`context`**: Atributo obligatorio que define la Política de Seguridad a la que pertenece esta herramienta (ej. `source-control`, `filesystem-ops`), leída desde la normativa de Cerbero.
* ** `inputs` / `outputs`**: Esquema JSON estricto para I/O vía stdin/stdout.

## 2. Consciencia Espacial y Encapsulamiento
* El ejecutable de la entidad debe residir en lo indicado por cumulo en la clave 'execution_capsules'.'skills'/{name}/.
* Los Skills tienen prohibido leer variables de entorno locales del usuario a menos que se inyecten explícitamente durante su ejecución, protegiendo la Táctica del Refugio.

## 3. Interfaz de Interacción (I/O JSON Estricto)
Los Skills son el "martillo ciego" del sistema. Su comunicación es puramente matemática:
* **`inputs`**: Deben recibir instrucciones exclusivamente mediante `stdin` en formato JSON estructurado.
* **`outputs`**: Deben emitir resultados exclusivamente mediante `stdout` en formato JSON (incluyendo `success`, `exitCode`, `data` o `error`).

## 4. Física del Valor y Evolución (Bloque Latente)
* `minteo_maximo`: Límite de licencias de uso o instalaciones.
* `porcentaje_de_exito`: Eficiencia termodinámica del binario (ejecuciones correctas vs. fallos de sistema).