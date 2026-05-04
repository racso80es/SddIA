---
contract_version: "1.1.0"
entity_type: "process"
jurisdiction: "Core SddIA"
capabilities:
  - "process-schema-governance"
  - "declarative-phase-routing"
---

# Contrato de Process (S+ Grade)

Este documento rige los Procesos, los flujos de trabajo de más alto nivel que guían el ciclo de vida del desarrollo (ej. creación de features, refactorizaciones).

## 1. Identidad Atómica (Innegociable)
Todo proceso debe poseer un documento de definición `{name}.md` que declare:
* **`uuid`**: Identificador único universal (v4).
* **`name`**: Nombre con aporte de contexto sobre la entidad.
* **`version`**: Control de versiones semántico (SemVer).
* **`contract`**: Versión de contrato implementado.
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
    delegates_to: ["tipo:nombre-de-capsula"]  # ej. ["skill:cryptography-manager", "action:identity-manager"]
```

Donde `tipo` ∈ {`skill`, `tool`, `action`, `agent`} y `nombre-de-capsula` coincide con el `name` de la cápsula destino indexada bajo cumulo.
* **`outputs`**: Los artefactos finales que certifican el cierre del proceso.

## 4. Física del Valor y Evolución (Bloque Latente)
Métricas operativas para el ciclo de vida del flujo:
* `minteo_maximo`: Límite de ejecuciones de este proceso en la red.
* `porcentaje_de_exito`: Variable que audita si las iteraciones del proceso alcanzan el cierre sin colapso entrópico.