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

### 2.6. `system-operations`
* **Dominio:** Ejecución de binarios y herramientas del SO / terceros.
* **Alcance:** Invocación determinista de herramientas como `gh`, `npm`, `docker`, `python`, etc., con sanitización anti-inyección y whitelist. **Excluye** el binario `git` nativo (debe enrutarse por `git-manager`).
* **Cápsulas asociadas (Ejemplos):** `shell-executor`.

### 2.7. `event-routing`
* **Jurisdicción:** Orquestación, triaje y movimiento de archivos en el bus de eventos asíncrono (EDA).
* **Autorización:** Permisos estrictos de lectura en `.docs/events/pending/` y `.docs/events/processing/`; escritura/movimiento hacia `processing/`, `processed/` o `dead-letter/` (resuelto vía `eda_bus` en `cumulo.paths.json`). Prohibida la mutación del genoma (`SddIA/events/`).
* **Cápsulas asociadas:** `action:route-domain-event`.

### 2.8. `dlt-auditing`
* **Jurisdicción:** Anclaje criptográfico inmutable en redes descentralizadas (ej. IOTA Rebased).
* **Autorización:** Permisos de solo-lectura sobre los hashes del genoma y capacidad de ejecución de la cápsula externa DLT.
* **Cápsulas asociadas:** `action:emit-pr-merged-event`, `tool:iota-immutable-publisher`.

---
*Reporte de Integridad: Normativa forjada y registrada. Rutas actualizadas.*