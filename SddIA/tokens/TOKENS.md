---
uuid: "9f2a1b3c-4d5e-4f70-a890-bcdef0123456"
name: "tokens-domain-ssot"
version: "1.1.0"
nature: motor
entity_family: "tokens"
contract: "tokens-contract v1.1.0"
description: "Documento único SSOT del dominio tokens del motor SddIA — contrato normativo y definiciones de token en un solo artefacto legible."
paths_ref: "SddIA/core/cumulo.paths.json → directories.tokens"
indexed_by: "agent:cumulo"
---

# Dominio tokens (SSOT)

Este fichero concentra **toda** la normativa y las definiciones de tokens del Core. Los artefactos JSON (`tokens-contract.json`, `karma2-token/spec.json`) permanecen como **fuente machine-readable** para herramientas y agentes; aquí se expresa el mismo contenido para humanos y auditoría documental.

**Índice de catálogo:** [index.md](index.md)

---

## 1. Contrato del dominio (`tokens-contract.json`)

### 1.1 Alcance

- **Ruta canónica:** `paths.tokensPath` → `SddIA/tokens/` (minúsculas).
- **Naturaleza:** motor (Core inyectable).
- **Propósito:** Toda definición de **token** que actúe como entidad de modelo con trazabilidad y contexto de seguridad debe cumplir este contrato.

### 1.2 Artefactos por token

| Artefacto | Obligatoriedad | Uso |
|-----------|----------------|-----|
| `spec.json` bajo `SddIA/tokens/<token-id>/` | Sí (para cada token catalogado) | Metadatos, `token_definition`, `validation_rules`, `contract_ref`. |
| Este documento (`TOKENS.md`) | SSOT narrativo | Contrato + definiciones en prosa y tablas; debe mantenerse alineado con los JSON. |
| `index.md` | Catálogo | Tabla resumen y enlaces a anclas de cada token. |

### 1.3 Restricciones

1. `token_id` en **kebab-case**.
2. Rutas canónicas solo desde **Cúmulo** (`SddIA/core/cumulo.paths.json` + fusión con `.sddia/local.paths.json` cuando aplique).
3. Un token sin `spec.json` en su carpeta **no se considera completo** para consumo automatizado.
4. Los tokens con **auditoría de interacciones** deben exponer `interaction_audit` coherente con la sección 3.

### 1.4 Consumidores típicos

Acciones, skills, tools, process, patterns, principles, ítems de seguridad del motor y flujos de auditoría que lean `paths.tokensPath` o referencias a Karma2Token.

### 1.5 Auditoría de interacciones (resumen normativo)

**Objetivo:** registrar quién utilizó qué entidad de modelo y cuándo.

**Campos obligatorios en el modelo de interacción:** `entity_type`, `entity_id`, `invoked_by`, `timestamp`.

**Valores de `entity_type`:** `skill`, `tool`, `action`, `process`.

**Registro:** bajo `paths.auditsPath` (y convenciones de `accessLogFile` / subcarpetas según Cúmulo).

---

## 2. Token: Karma2Token

<a id="token-karma2-token"></a>

| Campo | Valor |
|-------|--------|
| **token_id** | `karma2-token` |
| **name** | Karma2Token |
| **version** | 1.1.0 |
| **nature** | motor |
| **spec.json** | `SddIA/tokens/karma2-token/spec.json` |

### 2.1 Descripción y propósito

**Descripción:** Contenedor conceptual de seguridad y contexto para ítems del ecosistema SddIA. Garantiza trazabilidad, autorización y cumplimiento de auditoría.

**Propósito:** Asegurar que cada ejecución tenga contexto de seguridad validado y trazable; permitir al auditor agregar y consultar interacciones entre entidades de modelo.

### 2.2 Definición de campos (`token_definition.fields`)

#### identity

| Campo | Tipo | Descripción |
|-------|------|-------------|
| issuer | string | ID del agente o usuario que emite la orden (equivale a `invoked_by` en auditoría). |
| subject | string | ID del ítem ejecutado (equivale a `entity_id` en auditoría). |
| role | string | Rol bajo el cual se ejecuta la acción. |

#### context

| Campo | Tipo | Descripción |
|-------|------|-------------|
| environment | enum | `dev` \| `test` \| `prod` \| `local` |
| session_id | uuid (string) | Sesión de trabajo. |
| timestamp | date-time | Momento de creación del token / interacción. |

#### security

| Campo | Tipo | Descripción |
|-------|------|-------------|
| hash | string | Firma o hash de integridad del contenido o parámetros. |
| permissions | array de string | Scopes o permisos requeridos. |
| audit_ref | string | Referencia al registro en `paths.auditsPath`. |

#### lifecycle

| Campo | Tipo | Descripción |
|-------|------|-------------|
| created_at | date-time | Creación. |
| expires_at | date-time | Opcional; tokens efímeros. |

#### interaction_audit

| Campo | Tipo | Descripción |
|-------|------|-------------|
| entity_type | enum | `skill` \| `tool` \| `action` \| `process` |
| entity_id | string | ID de la entidad invocada. |
| invoked_by | string | Identidad que invoca. |
| timestamp | date-time | Momento de la interacción. |

### 2.3 Reglas de validación

1. Debe estar **firmado por un emisor válido** (Agent ID registrado).
2. Debe tener **referencia de auditoría válida** en `paths.auditsPath` cuando aplique.
3. Para auditoría de interacciones: **entity_type**, **entity_id**, **invoked_by** y **timestamp** presentes.

### 2.4 Referencia de contrato

`contract_ref` en JSON: `SddIA/tokens/tokens-contract.json`.

---

## 3. Mantenimiento y alineación

- Cualquier cambio material en definiciones de token debe actualizar **este fichero**, **`index.md`**, **`tokens-contract.json`** y el **`spec.json`** del token afectado en la misma revisión.
- Nuevos tokens: añadir sección `## Token: …` con ancla, fila en `index.md`, carpeta `<token-id>/spec.json`, y sincronizar restricciones en `tokens-contract.json` si el esquema global cambia.
