---
uuid: "d8e9f0a1-b2c3-4d5e-6f7a-8b9c0d1e2f3a"
name: "execution-contexts"
version: "1.0.0"
entity_type: "norm"
jurisdiction: "cerbero"
---

# Normativa de Contextos de Ejecución (RBAC Simbiótico)

## 1. Naturaleza y Autoridad
Este documento define el ADN de los permisos de la Librería SddIA. 
* **Autoridad Ontológica:** Agente `cerbero`. Él es el único responsable de validar la coherencia de estos contextos.
* **Mantenimiento Físico:** Agente `cumulo`. Responsable de la persistencia y disponibilidad del artefacto.

## 2. Matriz de Contextos S+ Grade
Toda Entidad Operativa debe declarar su pertenencia a uno de estos contextos. Cualquier invocación a un contexto fuera de esta matriz se considera una alucinación y será bloqueada mediante `exitCode: 1`.

### 2.1. `source-control`
* **Dominio:** Soberanía sobre el espacio y tiempo del código.
* **Alcance:** Control de versiones, ramas, recuperación y sincronización.
* **Cápsulas asociadas (Ejemplos):** `git-commit`, `git-branch-manager`, `git-tactical-retreat`.

### 2.2. `filesystem-ops`
* **Dominio:** Interacción física con el entorno local.
* **Alcance:** Lectura, escritura, borrado de archivos y escaneo de directorios.
* **Cápsulas asociadas (Ejemplos):** `read-file`, `write-artifact`, `workspace-recon`.

### 2.3. `knowledge-management`
* **Dominio:** Epistemología y topología del ecosistema.
* **Alcance:** Modificación del Cúmulo (`cumulo.paths.json`), lectura de leyes y actualización de normativas.
* **Cápsulas asociadas (Ejemplos):** `update-cumulo-paths`, `index-norms`.

### 2.4. `quality-assurance`
* **Dominio:** Auditoría, rigor estructural y DevSecOps.
* **Alcance:** Ejecución de pruebas locales, linters y escaneo de vulnerabilidades.
* **Cápsulas asociadas (Ejemplos):** `run-tests-local`, `security-audit`.

### 2.5. `ecosystem-evolution`
* **Dominio:** Desarrollo Kaizen e inyección de la Librería SddIA.
* **Alcance:** Mutación de la arquitectura interna y sincronización de manifiestos.
* **Cápsulas asociadas (Ejemplos):** `sddia-evolution-register`, `sync-core-manifest`.

---
*Reporte de Integridad: Normativa forjada y registrada. Rutas actualizadas.*