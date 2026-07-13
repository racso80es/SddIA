---
contract_version: "1.5.0"
entity_type: "tool"
jurisdiction: "Core SddIA (Interfaz) / Workspace (Delivery)"
capabilities:
  - "tool-schema-governance"
  - "local-domain-execution-routing"
execution_substrate: "rust-wasi"
---

# Contrato de Tools (S+ Grade) — Kernel Raw (Interfaz agnóstica)

Este documento rige las **Tools** como **capacidades ejecutables de dominio** creadas por y para un **workspace/proyecto** específico (ej. seeders, runners de tests, orquestadores de entorno).

**Principio rector (Invarianza del Core):** el Core SddIA no asume plataforma, lenguaje, SO, framework, motor de BD, ni layout físico (no hay rutas tipo `scripts/tools/`, ni obligación de `.exe`, ni “implementación por defecto” normativa).  
El Core dicta únicamente la **Interfaz** y las **reglas de seguridad/observabilidad**. El **Delivery** es workspace-local y se resuelve por topología (Cúmulo) mediante `implementation_path_ref`.

## 1. Identidad Atómica (Innegociable)

Aunque sean workspace-local, las Tools heredan el rigor S+ Grade. Cada Tool debe tener una **definición** (spec) con:

- **`name`**: Identificador estable en kebab-case (alineado al resto de entidades de dominio SddIA). El sinónimo histórico **`toolId`** queda **deprecado** en specs y documentación nuevas.
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
- **Resolución por topología local** (Cúmulo): convierte `implementation_path_ref` en una ruta efectiva dentro del workspace (p. ej. `SddIA/tools/{name}/` + artefacto en `SddIA/target/`).

## 3. Consciencia espacial y encapsulamiento (Kaizen)

* El crate de la tool reside bajo `cumulo.execution_capsules.tools` → `SddIA/tools/{name}/`.
* El artefacto ejecutable se resuelve en `SddIA/target/` (`wasm32-wasip1/release|debug/{name}.wasm` o binario nativo `{name}`).
* Prohibido `SddIA/scripts/tools/` como ruta operativa canónica; las cápsulas residen en `SddIA/tools/{name}/` y se resuelven vía `compiled_capsules` en Cúmulo.
* El runtime laboratorio usa `capsule_resolve.resolve_tool_capsule()` — paridad con skills.

## 4. Ejecución y ruteo (Workspace-local)

- Las Tools se **invocan** a través de Cúmulo/Cerbero (no por comandos crudos directos sin auditoría).
- Cúmulo mantiene un **índice/topología** de tools disponibles en el workspace y resuelve `implementation_path_ref` sin duplicar rutas literales en specs.
- Ubicación sugerida (ejemplo no normativo): `.<workspace>/.SddIA/tools/<name>/` o equivalente. La ubicación real depende del proyecto y su topología.

## 5. Interfaz de Interacción

Las Tools deben respetar un estándar de comunicación **machine-readable**:

- **Entrada**: `request` (estructurado), idealmente por **stdin** o argumentos equivalentes (delivery decide).
- **Salida**: un **único envelope JSON** (por stdout o canal equivalente) con:
  - **`name`**: string — identificador kebab-case de la tool que emitió el resultado (**obligatorio** en implementaciones nuevas).
  - **`toolId`**: *(deprecado)* — alias legado del identificador; si aparece, **debe coincidir** con `name`. Los emisores nuevos no deben usar este campo.
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

## 6. Física del Valor y Evolución (Bloque Latente)
* `minteo_maximo`: Límite de uso definido por el arquitecto local.
* `porcentaje_de_exito`: Métrica auditable del rendimiento de la herramienta en el entorno local.

## 7. Termodinámica declarativa

Campos opcionales en frontmatter de `{name}.md` — paridad con `skills-contract.md` §6 y `actions-contract.md` §6:

| Campo | Tipo | Default |
|-------|------|---------|
| `telemetry_provided` | boolean | `false` |
| `telemetry_schema` | string[] | `["prompt_tokens", "completion_tokens"]` si `telemetry_provided: true` |

Cuando `telemetry_provided: true`, la cápsula promete devolver `telemetry_receipt` válido en stdout; el fan-out `telemetry-compliance-audit` cruza recibo vs contrato.

**Nota histórica:** v1.3.0 introduce termodinámica en tools para soportar el Arsenal de Entropía (`schema-corruptor`) y alinear el Peaje Termodinámico con skills/actions (programa Inmunidad / Caos S+ Grade).

## 8. Sustrato de Delivery recomendado

El sustrato canónico para nuevas tools en este workspace es **Rust compilado a `wasm32-wasip1`**, ejecutado vía `wasmtime`. Garantiza portabilidad, sandboxing y paridad con el ecosistema de skills.

- El artefacto es un `.wasm`; el orquestador lo invoca con `wasmtime run --dir=. {tool}.wasm`.
- Tools Python existentes que no usen subprocess son candidatas a migración inmediata.
- Tools que requieran subprocess (ej. invocación de `wasmtime` a otros módulos) están bloqueadas por limitación WASI hasta adopción de `wasi:cli` experimental.

## 9. Historial normativo (extracto)

- **v1.5.0** — §3 paths Rust `SddIA/tools/` + `SddIA/target/`; retiro `scripts/tools/` operativo.
- **v1.4.0** — §8 sustrato Rust/WASI canónico; `execution_substrate: rust-wasi` en frontmatter.
- **v1.3.0** — §6 termodinámica declarativa (`telemetry_provided`, `telemetry_schema`).
- **v1.2.0** — Identidad atómica: campo canónico **`name`**; **`toolId`** deprecado en definiciones.
- **v1.1.0** — Baseline previo (`toolId` en identidad y textos).