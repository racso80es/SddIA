---
document_id: HU-MERGE-THERMODYNAMICS
title: "[OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia"
format: markdown
version: "1.0.0"
created: "2026-10-03"
status: "backlog"
priority: "alta"
process: "feature"
related:
  - SddIA/scripts/qa/git-hooks/hook_common.py
  - SddIA/process/telemetry-compliance-audit.md
  - SddIA/norms/pull-request-orchestration.md
  - SddIA/events/orchestration/index.md
---

### [OPERATIVO] Optimización Termodinámica de Aduana Git: Merge de Alta Eficiencia

#### 1. Propósito y Alcance
Erradicar la latencia estructural durante la ejecución de los *merges* y *pushes* locales mediante la aplicación estricta del Filtro C (Eficiencia / Necesidad)[cite: 12, 26]. La aduana de Git dejará de operar como un bloqueador síncrono ciego y pasará a ejecutar un triaje termodinámico, delegando la carga pesada a la asincronía del bus EDA y a la validación mediante evidencias físicas locales.

#### 2. Vectores de Optimización (Especificación)

**Fase 1: Triaje por Delta (Ejecución Condicional)**
*   **Modificación de Aduana:** Refactorizar `SddIA/scripts/qa/git-hooks/hook_common.py`[cite: 1] para que intercepte el árbol de archivos modificados (`git diff --name-only`).
*   **Lógica de Exclusión:** Si los cambios detectados pertenecen de forma exclusiva a directorios pasivos o de documentación (`docs/`, `historias/`, `README.md`), el hook debe realizar un cortocircuito (bypass) sobre las validaciones pesadas (ej. `cargo test`, `cargo build --release` o linting estricto de Python).

**Fase 2: Validación ZKP Local (Evidencias Físicas)**
*   **Desplazamiento del Cómputo:** Las auditorías pesadas requeridas para el cierre de un PR (auditorías estructurales de Argos) se trasladan a los estados `in_progress` o `in_review`[cite: 37, 38]. 
*   **Generación de Evidencia:** Tras una evaluación exitosa en background, el agente auditor escribirá un archivo JSON de certificación en el directorio `.SddIA/proofs/audits/`.
*   **Verificación Síncrona:** Durante el `pre-push` o la fusión, el hook limitará su acción a comprobar la existencia y validez criptográfica o de timestamp de dicho archivo de prueba, reduciendo el coste de validación a operaciones de E/S de milisegundos.

**Fase 3: Delegación al Bus EDA (Pipeline Asíncrono)**
*   **Poda Síncrona:** Extirpar de la cadena de hooks de Git cualquier verificación de cumplimiento que no ponga en riesgo inminente la integridad del genoma físico (ej. validaciones de formato de telemetría).
*   **Enrutamiento Reactivo:** Trasladar la ejecución del proceso `telemetry-compliance-audit`[cite: 3] y similares al bus de eventos. Estas auditorías se dispararán asíncronamente como reacción al evento `PullRequest_Merged`[cite: 14], operando en segundo plano a través de los centinelas sin bloquear la terminal del desarrollador.

#### 3. Criterios de Aceptación (Protocolo de Acero)

| ID | Criterio de Aceptación | Método de Verificación |
| :--- | :--- | :--- |
| **AC-1** | Un *merge* que solo contiene modificaciones en ficheros `.md` bajo `docs/` se ejecuta en < 1 segundo sin disparar compiladores de Rust ni linters. | Test E2E / Hook Mock |
| **AC-2** | El hook de validación síncrona lee el JSON de evidencia de Argos en `.SddIA/proofs/` en lugar de invocar una auditoría completa. | Test de Integración |
| **AC-3** | Si el JSON de evidencia es inválido, caducado, o los ficheros de código han mutado después de la creación de la prueba (hash mismatch), el hook bloquea el merge. | Red Teaming Local |
| **AC-4** | El evento `PullRequest_Merged` despacha correctamente el subproceso `telemetry-compliance-audit` en background a través del bus EDA, sin acoplarse al hilo principal de Git. | Test Bus EDA |
| **AC-5** | La alteración del hook no degrada la integridad estructural; los fallos reales de código (tests rotos en Rust) siguen bloqueando el flujo si el delta incluye modificaciones en el directorio `SddIA/` o `.rs`. | Suite Lab Core-self |

#### 4. Touchpoints Físicos a Modificar
*   `SddIA/scripts/qa/git-hooks/hook_common.py` (Lógica de triaje y lectura de evidencias).
*   `SddIA/process/accept-pr.md` (Transición de auditorías síncronas a asíncronas).
*   `SddIA/core/event-orchestration-subscriptions.json` (Suscripción al evento `PullRequest_Merged` para auditorías diferidas).
