---
contract_version: "1.4.0"
entity_type: "skill"
jurisdiction: "Core SddIA"
capabilities:
  - "skill-schema-governance"
  - "json-stdin-stdout-io"
  - "execution-capsule-routing"
execution_substrate: "rust-wasi"
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
* **`capabilities`**: Array obligatorio de strings que etiqueta las operaciones atómicas que resuelve la cápsula (enrutamiento semántico; ej. `uuid-generation`, `file-write`).
* ** `inputs` / `outputs`**: Esquema JSON estricto para I/O vía stdin/stdout.

## 2. Consciencia Espacial y Encapsulamiento
* El ejecutable de la entidad reside bajo `cumulo.execution_capsules.skills` → `SddIA/skills/{name}/` con artefacto compilado en `SddIA/target/` (WASI `wasm32-wasip1` o nativo `release|debug`).
* Prohibido `scripts/skills/` como ruta operativa canónica (legacy retirado en Kaizen `kaizen-rust-capsule-structure`).
* Los Skills tienen prohibido leer variables de entorno locales del usuario a menos que se inyecten explícitamente durante su ejecución, protegiendo la Táctica del Refugio.

## 3. Interfaz de Interacción (I/O JSON Estricto)
Los Skills son el "martillo ciego" del sistema. Su comunicación es puramente matemática:
* **`inputs`**: Deben recibir instrucciones exclusivamente mediante `stdin` en formato JSON estructurado.
* **`outputs`**: Deben emitir resultados exclusivamente mediante `stdout` en formato JSON (incluyendo `success`, `exitCode`, `data` o `error`).

## 4. Sustrato de Ejecución (Innegociable desde v1.3.0)

El sustrato canónico para todas las cápsulas skill es **Rust compilado a `wasm32-wasip1`**, ejecutado mediante el runtime `wasmtime`.

- **Prohibido** entregar nuevas skills como scripts Python o cualquier otro intérprete.
- El artefacto de despliegue es un binario `.wasm`; el orquestador lo invoca vía `wasmtime run`.
- **Excepción WASI → nativo:** si `wasmtime` + WASI no pueden completar la operación (ej. subprocess `git`), el orquestador usa el binario nativo `SddIA/target/{release|debug}/{name}`. **Prohibido** fallback a scripts Python.

## 5. Física del Valor y Evolución (Bloque Latente)
* `minteo_maximo`: Límite de licencias de uso o instalaciones.
* `porcentaje_de_exito`: Eficiencia termodinámica del binario (ejecuciones correctas vs. fallos de sistema).

## 5. Esquemas de entrada congelados (pre-forja)
Las skills `git-manager` y `shell-executor` deben obedecer los mensajes de stdin definidos de forma **congelada** en normas bajo `directories.norms`, referenciadas desde `SddIA/core/cumulo.paths.json` → `normative_documents.skill_io_git_manager_frozen` y `normative_documents.skill_io_shell_executor_frozen`. Cerbero y Argos validan contra esos documentos antes de la invocación.

## 6. Termodinámica declarativa (Fase 5)

Campos opcionales en frontmatter de `{name}.md`:

| Campo | Tipo | Default | Descripción |
|-------|------|---------|-------------|
| `telemetry_provided` | boolean | `false` (implícito) | La cápsula promete devolver `telemetry_receipt` en stdout |
| `telemetry_schema` | string[] | `["prompt_tokens", "completion_tokens"]` si `telemetry_provided: true` | Claves obligatorias en el recibo |

Ausencia de ambos campos → la ED no entra en auditoría de cumplimiento termodinámico.