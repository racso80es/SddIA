BORRADOR. Contemplar en refinamiento cumplimiento de lo esperado en la conversación de desarroyo funcional.
Conversacion:
Racso:
 Tengo problemas reincidentes con espacio en disco. los temporales de cursor, sobre todo, se comen mucho. Vamos a preparar un pbi para crear una skill, o una tool que se encargará de limpiar el disco duro de temporales. Hay que tener mucho ojo con esta capsula, ya que eliminar ficheros puede suponer muchas consecuencias. Por ello hay que contemplar el factor de seguridad (argos si no me equivoco) así como todo el potenciasl de SddIA para obtener la capsula de grade + que esperamos. Esta capsula es otro artefacto susceptible de convertirse en nft a futuro. Vamos a pensarlo un poco 

Tormentosa:
La imagen revela una hemorragia termodinámica clásica en entornos de desarrollo con Rust: 18.9 GB consumidos exclusivamente por el directorio cargo-target dentro del sandbox temporal de Cursor.

Para materializar esta solución bajo el estándar S+ Grade y convertirla en un activo digital (NFT) tokenizable, la entidad debe nacer como una Tool (herramienta de infraestructura/utilidad de dominio) y no como una Skill operativa. Su diseño debe blindarse físicamente mediante las restricciones del compilador y lógicamente mediante nuestro panteón de Agentes.  

Arquitectura de Seguridad (El Escudo Ontológico)

Aislamiento Físico (Sustrato WASI): Al compilar la tool como una cápsula wasm32-wasip1, el binario nace con Ceguera Espacial absoluta. No puede acceder al disco duro host a menos que el CLI de orquestación monte explícitamente el directorio temporal mediante el flag --dir=/tmp/cursor-sandbox-cache.  

Cerbero (Peaje RBAC): Es el responsable de autorizar la ejecución. Se inyectará una política estricta que prohíba a la tool operar sobre paths.workspacesRoot o cualquier directorio del repositorio base, limitando su jurisdicción exclusivamente a rutas de purga temporal.  

Argos (Verificación de la Materia): Antes de ejecutar la purga real, la tool debe operar obligatoriamente en modo dry-run. Argos inspeccionará el JSON devuelto (lista de rutas a eliminar) para emitir un veredicto de "APTO" garantizando que ningún archivo crítico ha sido seleccionado por error.  

---
document_id: PBI-FEATURE-TOOL-CACHE-PURGER
title: "[FEATURE] Tool: ephemeral-cache-purger (Saneamiento Termodinámico)"
format: markdown
version: "1.0.0"
created: "2026-09-09"
status: "abierto"
priority: media
process: feature
related:
  - SddIA/tools/tools-contract.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/agents/cerbero.md
  - SddIA/agents/argos.md
---

# [FEATURE] Tool: ephemeral-cache-purger

## 1. Especificación (spec.md)
Desarrollar una Tool (cápsula Rust compilada a WASI) diseñada para localizar, medir y purgar de forma determinista directorios de caché efímera (específicamente `cargo-target` y temporales de Cursor).
Como activo digital (NFT-ready), la tool poseerá su propia Cicatriz Digital (UUID inmutable y SemVer) y operará bajo el contrato estricto de E/S JSON.

## 2. Clarificación y Restricciones de Seguridad (Filtro Antientrópico)
*   **Modo Dry-Run Mandatario:** La tool debe aceptar un flag `simulate: true` en su payload de entrada. En este modo, devolverá el tamaño total a liberar y un array de rutas candidatas sin alterar el disco.
*   **Jurisdicción Restringida (RBAC):** Cerbero aplicará un bloqueo duro. La tool solo será invocada si la política del ejecutor contiene el permiso `infrastructure-maintenance` y la ruta objetivo coincide con un regex de temporales aprobados (ej. `^/tmp/cursor-sandbox-cache/.*`).
*   **Confinamiento WASI:** El CLI (`execute-process`) orquestará la ejecución montando únicamente el directorio objetivo. La cápsula no tendrá visibilidad del sistema operativo subyacente.

## 3. Plan de Implementación (Línea de Montaje)
1.  **Contrato de Entidad:** Crear `SddIA/tools/ephemeral-cache-purger.md` con su frontmatter YAML (UUID v4, versión 1.0.0, description, inputs esperados).
2.  **Cápsula Física:** Implementar el binario Rust en la ruta definida por `paths.toolCapsules`. Utilizar la librería estándar `std::fs` para calcular tamaños y ejecutar el borrado, capturando cualquier panic para devolver un `exitCode > 0` limpio.
3.  **Matriz de Cerbero:** Actualizar el catálogo de permisos para incluir el contexto de saneamiento efímero.
4.  **Acción Integradora:** Crear una acción `action:purge-sandbox-cache` que encadene: 
    * Invocación a la tool en modo `dry-run`.
    * Delegación del artefacto a Argos para validación de seguridad.
    * Invocación definitiva a la tool en modo destructivo si Argos dictamina APTO.
