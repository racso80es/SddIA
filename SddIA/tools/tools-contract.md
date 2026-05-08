---
contract_version: "1.1.0"
entity_type: "tool"
jurisdiction: "Core SddIA (Interfaz) / Workspace (Delivery)"
capabilities:
  - "tool-schema-governance"
  - "local-domain-execution-routing"
---

# Contrato de Tools (S+ Grade) — Kernel Raw (Interfaz agnóstica)

Este documento rige las **Tools** como **capacidades ejecutables de dominio** creadas por y para un **workspace/proyecto** específico (ej. seeders, runners de tests, orquestadores de entorno).

**Principio rector (Invarianza del Core):** el Core SddIA no asume plataforma, lenguaje, SO, framework, motor de BD, ni layout físico (no hay rutas tipo `scripts/tools/`, ni obligación de `.exe`, ni “implementación por defecto” normativa).  
El Core dicta únicamente la **Interfaz** y las **reglas de seguridad/observabilidad**. El **Delivery** es workspace-local y se resuelve por topología (Cúmulo) mediante `implementation_path_ref`.

## 1. Identidad Atómica (Innegociable)

Aunque sean workspace-local, las Tools heredan el rigor S+ Grade. Cada Tool debe tener una **definición** (spec) con:

- **`toolId`**: Identificador estable en kebab-case.
- **`uuid`**: Identificador único universal (v4) de la definición.
- **`version`**: SemVer de la Tool.
- **`contract_ref`**: Referencia a este contrato (ruta lógica).
- **`domain_origin`**: Proyecto/contexto al que pertenece (workspace).
- **`context`**: Política/ámbito de seguridad (p. ej. `quality-assurance`, `ecosystem-evolution`).
- **`capabilities`**: Etiquetas semánticas de operación (ruteo/selección).
- **`implementation_path_ref`**: Referencia abstracta (no ruta literal) que Cúmulo resuelve al artefacto de ejecución en el workspace.

## 2. Interfaz vs Delivery (Separación estricta)

- **Interfaz (Core, normativa):** qué entra/sale, cómo se reporta feedback, cómo se codifican los errores, y reglas de secretos.
- **Delivery (Workspace, no normativo en Core):** lenguaje, binario/script, layout de carpetas, empaquetado y estrategia de distribución.

El **único puente** entre ambos es:

- **`implementation_path_ref`** (en la definición): puntero abstracto a la implementación.
- **Resolución por topología local** (Cúmulo): convierte `implementation_path_ref` en una ruta efectiva dentro del workspace (p. ej. `.<algo>/.sddia/tools/<toolId>/...`).

## 3. Ejecución y ruteo (Workspace-local)

- Las Tools se **invocan** a través de Cúmulo/Cerbero (no por comandos crudos directos sin auditoría).
- Cúmulo mantiene un **índice/topología** de tools disponibles en el workspace y resuelve `implementation_path_ref` sin duplicar rutas literales en specs.
- Ubicación sugerida (ejemplo no normativo): `.<workspace>/.sddia/tools/<toolId>/` o equivalente. La ubicación real depende del proyecto y su topología.

## 4. Interfaz de Interacción

Las Tools deben respetar un estándar de comunicación **machine-readable**:

- **Entrada**: `request` (estructurado), idealmente por **stdin** o argumentos equivalentes (delivery decide).
- **Salida**: un **único envelope JSON** (por stdout o canal equivalente) con:
  - **`success`**: boolean.
  - **`exitCode`**: number (0 solo si `success=true`).
  - **`message`**: string breve (no sensible).
  - **`feedback[]`**: eventos trazables (fase, nivel, timestamp, message; opcional detail/duration).
  - **`result`**: object (payload específico de la tool; **reemplaza** cualquier uso histórico de `data`).
  - **`error`**: object opcional cuando `success=false` (tipo/código/causa, sin secretos).
  - **`duration_ms`**: number opcional.

**Reglas de seguridad de secretos (estrictas):**

- Los secretos/tokens **no** deben aparecer en `message`, `feedback`, `result` ni `error`.
- La inyección de secretos debe ocurrir vía **entorno efímero** o mecanismos equivalentes del workspace (delivery), con borrado/expiración.

## 5. Física del Valor y Evolución (Bloque Latente)
* `minteo_maximo`: Límite de uso definido por el arquitecto local.
* `porcentaje_de_exito`: Métrica auditable del rendimiento de la herramienta en el entorno local.