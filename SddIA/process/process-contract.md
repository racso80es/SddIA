---
contract_version: "1.2.0"
entity_type: "process"
jurisdiction: "Core SddIA"
capabilities:
  - "process-schema-governance"
  - "declarative-phase-routing"
  - "phase-invocation-binding"
---

# Contrato de Process (S+ Grade)

Este documento rige los Procesos, los flujos de trabajo de más alto nivel que guían el ciclo de vida del desarrollo (ej. creación de features, refactorizaciones).

## 1. Identidad Atómica (Innegociable)
Todo proceso debe poseer un documento de definición `{name}.md` que declare:
* **`uuid`**: Identificador único universal (v4).
* **`name`**: Nombre con aporte de contexto sobre la entidad.
* **`version`**: Control de versiones semántico (SemVer).
* **`contract`**: Versión de contrato implementado (p. ej. `process-contract v1.2.0`).
* **`context`**: Listado de política de Seguridad de las que hace uso.
* **`hash_signature`**: Firma criptográfica de la integridad de sus fases (recomendado: `sha256:` + digest hexadecimal del JSON canónico UTF-8 del array `phases`, con `sort_keys=True` en serialización de cada objeto).

## 2. Consciencia Espacial (Obediencia al SSOT)
Los procesos dictan el camino, no el destino físico.
* Deben leer la ubicación de las Acciones que lo componen estrictamente desde lo indicado por cumulo.
* Deben instruir a los agentes basándose en la topología oficial de Cúmulo, nunca asumiendo estructuras de carpetas locales del usuario.

## 3. Interfaz de Interacción y Fases
La comunicación debe ser paramétrica. El `{name}.md` debe declarar:
* **`inputs`**: Datos de inicio (ej. el spec de una nueva feature).
* **`phases`**: Array inmutable de objetos; cada elemento define una fase declarativa y el enrutamiento por capacidades (sin deducción libre del LLM). Estructura estricta:

```yaml
phases:
  - name: "Nombre de fase"
    intent: "Descripción de lo que se busca"
    delegates_to: ["tipo:nombre-de-capsula"]  # ej. ["action:crypto-broker", "skill:filesystem-manager"]
```

Donde `tipo` ∈ {`skill`, `tool`, `action`, `agent`} y `nombre-de-capsula` coincide con el `name` de la cápsula destino indexada bajo cumulo.
* **`outputs`**: Los artefactos finales que certifican el cierre del proceso.

### 3.1. Invocaciones ejecutables (`phase_invocations`) — v1.2.0
Además de `delegates_to`, los procesos **forja** (creators) y cualquier proceso que requiera cápsulas no-LLM deterministas deben declarar **`phase_invocations`**: lista que amarra cada fase (`phase_name` igual al `name` de un elemento de `phases`) a una secuencia ordenada de invocaciones con **stdin JSON** hacia la cápsula y **bindings** del stdout.

**Estructura:**

```yaml
phase_invocations:
  - phase_name: "Nombre exacto de la fase"
    invocations:
      - capsule: "action:crypto-broker"   # o skill:/tool:/agent:
        stdin_json:                       # objeto literal enviado como stdin a la cápsula (cuando es estático)
          operation: "GENERATE_UUID"
          target_payload: null
        stdin_spec:                       # alternativa a stdin_json cuando el payload es dinámico
          operation: "GENERATE_SHA256"
          target_type: "STRING"
          target_payload:
            type: "canonical_json_utf8"
            from_process_input: "nombre_del_input_del_proceso"
            # alternativa: from_process_inputs: ["clave1", "clave2", ...] → objeto {clave: valor} con claves ordenadas alfabéticamente antes de json_dumps
            json_dumps:
              sort_keys: true
              separators: [",", ":"]
              ensure_ascii: false
        bind:
          "data.result": "nombre_variable_estado"   # ruta en respuesta JSON -> variable de ejecución
        on_error: "abort"                             # obligatorio: abort (sin reintentos)
```

**Reglas:**
1. **`delegates_to`** debe incluir cada `capsule` referenciado en las invocaciones de esa fase (coherencia Cerbero / fan-out).
2. **Criptografía** (UUID, SHA-256, validación de hash): prohibido declarar `skill:cryptography-manager` en `delegates_to` de procesos bajo contexto `ecosystem-evolution`; usar **`action:crypto-broker`** (context `quality-assurance`) como única puerta hacia la skill.
3. **`bind`**: claves son rutas tipo JSONPath reducido (p. ej. `data.result`). El ejecutor inyecta el valor en el estado del proceso para ensamblar artefactos (cabecera YAML, índices).
4. **`on_error`**: solo `abort` en Core v1.2.0.
5. **Canonicalización** para hashing: `canonical_json_utf8` usa `json.dumps` con los parámetros declarados en `json_dumps`. Si se usa `from_process_input`, el valor raíz es `process_inputs[nombre]`. Si se usa `from_process_inputs` (array de strings), el ejecutor construye un objeto `{k: process_inputs[k]}` para cada `k` presente, ordena las claves alfabéticamente y serializa ese objeto; encoding de salida **UTF-8 sin BOM**.

## 4. Física del Valor y Evolución (Bloque Latente)
Métricas operativas para el ciclo de vida del flujo:
* `minteo_maximo`: Límite de ejecuciones de este proceso en la red.
* `porcentaje_de_exito`: Variable que audita si las iteraciones del proceso alcanzan el cierre sin colapso entrópico.
