---
document_id: PBI-MIGRACION-RUST-WASI
title: "[OPERATIVO] PBI: Migración de Cápsulas Ejecutables (Skills/Tools) a Rust (WASI)"
format: markdown
version: "1.0.0"
created: "2026-05-01"
status: done
priority: alta
process: feature
closed: "2026-06-11"
branch_name: feat/migracion-rust-wasi-certificacion
feature_ref: docs/features/migracion-rust-wasi
---

# [OPERATIVO] PBI: Migración de Cápsulas Ejecutables (Skills/Tools) a Rust (WASI)

## 1. Contexto Arquitectónico (Estado S+ Grade)
La evolución del entorno hacia el paradigma WASI (identificado en la rama de ignición) exige la obsolescencia del sustrato Python para las cápsulas de ejecución (Skills y Tools). Este PBI dirige la transmutación hacia Rust, garantizando encapsulamiento estricto, rendimiento termodinámico superior y tipado fuerte en la entrada/salida (I/O) de datos JSON, alineándose con la soberanía estructural de SddIA.

## 2. Objetivos Principales
1. **Inicialización Estándar:** Ejecutar el ciclo de vida completo del proceso `feature` (spec, clarify, plan, implementation, validation).
2. **Análisis de Impacto (Aduana Preventiva):** Mapear todas las dependencias actuales de los scripts Python en procesos, eventos, observadores (*watchers*) y orquestadores.
3. **Actualización de Contratos:** Adecuar las normas de la Constitución, `README.md` y los contratos (`skills-contract.md`, `tools-contract.md`) para reflejar el nuevo estándar de compilación y ejecución.
4. **Forja en Rust:** Reescribir la lógica de los ejecutables actuales a Rust, asegurando paridad funcional estricta respecto a sus predecesores en Python.
5. **Adaptación de Interfaces Físicas:** Modificar todos los ficheros `.bat`, `.sh` y `.ps1` afectados para invocar los nuevos binarios o el runtime WASI correspondiente.
6. **Poda Ontológica (Purga):** Eliminación total de ficheros `.py`, entornos virtuales, configuraciones de dependencias (`requirements.txt`) y referencias obsoletas.

## 3. Elementos Adicionales a Contemplar (Fricción Evolutiva)
Para garantizar la integridad y escalabilidad del ecosistema, el Yunque Rúnico dicta añadir las siguientes consideraciones a la ejecución:

* **Configuración de Cargo Workspace:** Estructurar el directorio de `skills` y `tools` bajo un único *Cargo Workspace*. Esto optimizará drásticamente los tiempos de compilación al compartir el árbol de dependencias (`target/`) y simplificará la gestión de versiones.
* **Crate de Utilidades Core (sddia-io):** Crear una librería interna en Rust que centralice la lectura de `stdin`, la escritura en `stdout` y la serialización del esquema de respuesta estándar (`success`, `exitCode`, `feedback`, `result`). Se rechaza la duplicidad de este bloque lógico en cada herramienta.
* **Integración Continua (CI/CD):** Actualizar los flujos de trabajo en `.github/workflows/` para inyectar los pasos de `cargo build` y `cargo test`, sustituyendo las validaciones o linters previos de Python.
* **Telemetría y Manejo de Errores (Safety Net):** Garantizar que los pánicos de Rust (`panics`) o errores de desempaquetado (`Unwrap`) sean capturados y devueltos al bus de eventos como un JSON válido con `exitCode > 0` y `success: false`. El fallo de una tool no debe romper el formato de salida esperado por el agente invocador (Cerbero/Argos).
* **Paridad Documental (Códices):** Auditar que los *runbooks* y la documentación de las features mantengan la simetría fractal con los nuevos binarios generados.

## 4. Criterios de Aceptación (Protocolo de Acero)
- [ ] La carpeta `.SddIA/docs/features/` contiene todos los artefactos del ciclo de vida de la feature debidamente actualizados.
- [ ] Ninguna skill o tool en el repositorio requiere un intérprete de Python para su ejecución.
- [ ] La suite completa de *smoke tests* y validaciones E2E se ejecuta con éxito consumiendo los artefactos en Rust/WASI.
- [ ] El comando `cargo build --workspace` compila todos los artefactos sin advertencias lógicas o de seguridad.
- [ ] Los contratos reflejan innegociablemente a Rust/WASI como el sustrato único para la ejecución de cápsulas.
