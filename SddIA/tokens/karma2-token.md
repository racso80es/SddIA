---
uuid: "7c4a9e2f-1b8d-4d6e-9f31-a02b8c4d7e01"
name: "karma2-token"
version: "1.1.0"
contract: "tokens-contract v1.2.0"
nature: motor
entity_family: token
description: "Contenedor conceptual de seguridad y contexto para ítems del ecosistema SddIA. Garantiza trazabilidad, autorización y cumplimiento de auditoría."
purpose: "Asegurar que cada ejecución tenga contexto de seguridad validado y trazable; permitir al auditor agregar y consultar interacciones entre entidades de modelo."
paths_ref: "SddIA/core/cumulo.paths.json → directories.tokens"
contract_ref: "SddIA/tokens/tokens-contract.md"
token_definition:
  fields:
    identity:
      issuer:
        type: string
        description: "ID del agente o usuario que emite la orden (equivale a invoked_by en auditoría)."
      subject:
        type: string
        description: "ID del ítem ejecutado (equivale a entity_id en auditoría)."
      role:
        type: string
        description: "Rol bajo el cual se ejecuta la acción."
    context:
      environment:
        type: string
        enum: ["dev", "test", "prod", "local"]
      session_id:
        type: string
        format: uuid
      timestamp:
        type: string
        format: date-time
    security:
      hash:
        type: string
      permissions:
        type: array
        items:
          type: string
      audit_ref:
        type: string
    lifecycle:
      created_at:
        type: string
        format: date-time
      expires_at:
        type: string
        format: date-time
    interaction_audit:
      entity_type:
        type: string
        enum: ["skill", "tool", "action", "process"]
      entity_id:
        type: string
      invoked_by:
        type: string
      timestamp:
        type: string
        format: date-time
validation_rules:
  - "Debe estar firmado por un emisor válido (Agent ID registrado)."
  - "Debe tener referencia de auditoría válida en paths.auditsPath cuando aplique."
  - "Para auditoría de interacciones: entity_type, entity_id, invoked_by y timestamp presentes."
---

# Token: Karma2Token (`karma2-token`)

Definición atómica del token Karma2Token. La ley de familia está en [tokens-contract.md](tokens-contract.md).

## Referencia rápida (legible)

### identity

| Campo | Tipo | Descripción |
|-------|------|-------------|
| issuer | string | ID del agente o usuario que emite la orden (`invoked_by` en auditoría). |
| subject | string | ID del ítem ejecutado (`entity_id` en auditoría). |
| role | string | Rol bajo el cual se ejecuta la acción. |

### context

| Campo | Tipo | Descripción |
|-------|------|-------------|
| environment | enum | `dev` \| `test` \| `prod` \| `local` |
| session_id | uuid (string) | Sesión de trabajo. |
| timestamp | date-time | Momento de creación del token / interacción. |

### security

| Campo | Tipo | Descripción |
|-------|------|-------------|
| hash | string | Firma o hash de integridad del contenido o parámetros. |
| permissions | array de string | Scopes o permisos requeridos. |
| audit_ref | string | Referencia al registro en `paths.auditsPath`. |

### lifecycle

| Campo | Tipo | Descripción |
|-------|------|-------------|
| created_at | date-time | Creación. |
| expires_at | date-time | Opcional; tokens efímeros. |

### interaction_audit

| Campo | Tipo | Descripción |
|-------|------|-------------|
| entity_type | enum | `skill` \| `tool` \| `action` \| `process` |
| entity_id | string | ID de la entidad invocada. |
| invoked_by | string | Identidad que invoca. |
| timestamp | date-time | Momento de la interacción. |

## Reglas de validación

Ver array `validation_rules` en el frontmatter.
