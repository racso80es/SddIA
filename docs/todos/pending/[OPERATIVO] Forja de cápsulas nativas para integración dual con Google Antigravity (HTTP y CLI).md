---
document_id: PBI-CAPSULES-ANTIGRAVITY-NATIVE
title: "[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI)"
format: markdown
version: "1.0.0"
created: "2026-09-02"
status: "pending"
priority: "alta"
process: feature
related:
  - SddIA/norms/capsule-json-io.md
  - SddIA/skills/skills-contract.md
  - SddIA/core/cumulo.paths.json
---

### [OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI)

#### 1. Propósito y Alcance
El objetivo de este PBI es dotar al ecosistema SddIA de la musculatura física necesaria para interactuar con la plataforma *agentic* Google Antigravity. Para garantizar un Aislamiento Paramétrico Estricto y evitar la dependencia de infraestructura de red o local, se forjarán dos cápsulas independientes (Skills/Tools). 
Estas cápsulas operarán exclusivamente como conectores (manos mecánicas) con **Ceguera Espacial** absoluta: no toman decisiones de orquestación, no deciden cuándo ejecutarse y no conocen el contexto global de SddIA.

#### 2. Especificación de Activos

**Activo 1: `skill:antigravity-http-connector`**
*   **Naturaleza:** Cápsula en Rust (compilada a binario nativo o WASI).
*   **Propósito:** Inyectar *prompts* atómicos y recuperar respuestas directamente desde la API REST/gRPC de Antigravity.
*   **Suministro Físico (Bóveda):** Exigirá la lectura de `ANTIGRAVITY_API_KEY` y `ANTIGRAVITY_API_ENDPOINT` desde la jerarquía `.SddIA/.dev/.env` antes de operar[cite: 28].
*   **Fricción de Entorno:** Cero. No requiere binarios externos ni CLI en la máquina host.

**Activo 2: `skill:antigravity-cli-executor`**
*   **Naturaleza:** Cápsula en Rust (compilada a binario nativo, ya que requiere `subprocess spawning` no soportado por WASI puro para invocar al sistema operativo)[cite: 19].
*   **Propósito:** Interfaz directa con el binario local `agy` (Antigravity CLI) para operaciones que requieran alteración nativa del *workspace* o sandboxing local.
*   **Suministro Físico (Bóveda):** Exigirá la lectura de `ANTIGRAVITY_CLI_PATH` (ej. `/usr/local/bin/agy`) desde `.SddIA/.dev/.env`[cite: 28].

#### 3. Contrato de Entrada/Salida (Rigor S+ Grade)
Ambas cápsulas deben acatar dogmáticamente la normativa `capsule-json-io.md`[cite: 28].
*   **Input (stdin):** Recibirán un JSON con el `payload` crudo (el prompt inyectado de contexto y las directrices de obliteración).
*   **Output (stdout):** Devolverán un JSON estandarizado con la estructura:
    *   `success` (booleano).
    *   `exitCode` (0 para éxito, >0 para fricción).
    *   `feedback` (Log técnico o traza de error).
    *   `result` (La respuesta pura del LLM o el estado de la alteración en el disco).
*   **Inmunidad Anti-Panic:** Todo error (caída de red HTTP, ausencia del binario CLI, *timeout* de la API) debe ser capturado internamente por Rust y devuelto como un JSON válido con `success: false` y su respectivo `exitCode`. Un `panic!` crudo que corrompa el `stdout` será considerado una falla crítica de Grado S-[cite: 19].

#### 4. Integración Futura (Out of Scope)
*   La asignación de estas cápsulas a las capacidades abstractas de SddIA (ej. `capability:llm-inference` o `capability:llm-workspace-editor`)[cite: 25] y su uso explícito por parte de agentes o acciones queda fuera del alcance de este PBI. El foco exclusivo es la construcción hermética de las herramientas físicas.

#### 5. Criterios de Aceptación (Protocolo de Acero)
*   [ ] Ambas cápsulas (`antigravity-http-connector` y `antigravity-cli-executor`) compilan correctamente en el entorno de desarrollo (Rust).
*   [ ] Ejecuciones de prueba demuestran que ambas leen correctamente los secretos de `.SddIA/.dev/.env` sin exponerlos en logs[cite: 28].
*   [ ] Un fallo provocado (ej. apagar la red o renombrar el binario CLI) devuelve un sobre JSON controlado y no un error del sistema.
*   [ ] Las cicatrices digitales (`spec.md` y `manifest.json`) de ambas entidades han sido indexadas correctamente en el catálogo del ecosistema.
