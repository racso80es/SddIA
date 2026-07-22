# Contrato de invocación: envelope JSON (skills y cápsulas)

**schema_version:** `2.0`  
**Ámbito:** petición/respuesta en stdin/stdout (o variables documentadas) para binarios en `paths.skillCapsules` / tools homólogas.

## Petición

Un único objeto JSON (una línea recomendada; se acepta JSON multilínea en stdin).

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `meta` | object | Metadatos obligatorios. |
| `meta.schemaVersion` | string | Debe ser `"2.0"`. (Alias aceptado en deserialización: `schema_version`.) |
| `meta.entityKind` | string | `"skill"` \| `"tool"`. (Alias: `entity_kind`.) |
| `meta.entityId` | string | kebab-case; para skills coincide con `skill_id`; para tools coincide con **`name`** según `tools-contract.md` v1.2.0 (alias histórico `toolId` solo en payloads legados). (Alias: `entity_id`.) |
| `meta.token` | object | Opcional; Karma2Token si el contrato lo exige. |
| `request` | object | Cuerpo libre definido en `paths.skillsDefinitionPath/<skill-id>/spec.md`. |
| `di_binding` | object | **Opcional (PBI-042 Hito 2).** Binding DI resuelto por el runtime antes de stdin. Hermano de `request`. |

### Campo opcional `di_binding`

Inyectado por `capability_di_resolver` cuando la fase declara `requires_capability`. Paths lógicos vía Cúmulo (nunca absolutos de host).

| Subcampo | Tipo | Obligatorio | Descripción |
|----------|------|-------------|-------------|
| `capability_id` | string | sí | Homologado en `capability-taxonomy`. |
| `contract` | string | sí | Clave de schema (`doc.closure`, …). |
| `contract_schema_ref` | string | sí | Ref lógica (`capability_contracts/{contract}`). |
| `provider` | string | sí | Identidad canónica (`skill:…` \| `action:…`). |
| `provider_ref` | string | sí | Ref lógica al `{name}.md` del proveedor. |
| `resolved_version` | string | sí | Versión del `provides` del proveedor. |
| `binding_ssot` | string | sí | Clave Cúmulo del mapa (`capability_di.bindings`). |

**Entrada alternativa:** variable de entorno `SDDIA_CAPSULE_REQUEST` con el mismo JSON. Si `SDDIA_SKIP_STDIN=1`, no se lee stdin.

**Archivo:** `--request-file <ruta>` (argumento del `.exe`) para pruebas locales.

## Respuesta

Una sola línea JSON en stdout.

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `meta` | object | `schemaVersion`, `entityKind`, `entityId` (eco lógico). |
| `success` | boolean | Éxito de la operación de negocio. |
| `exitCode` | number | Código de salida del proceso; **debe ser 0 si y solo si** `success === true`. |
| `message` | string | Mensaje humano breve. |
| `feedback` | string | Opcional; pistas adicionales. |
| `result` | object | Payload estructurado por skill. |
| `durationMs` | number | Opcional; duración en ms. |

## Coherencia

- `exitCode === 0` ⟺ `success === true`.
- Errores de validación de envelope: `success: false`, `exitCode` ≠ 0.

## Validación post-ejecución (PBI-042 Hito 3 — R8)

Tras invocación de cápsula con `di_binding` presente, `execute-process` valida el **payload real** de salida (stdout JSON) contra `{contract}.schema.json` en `directories.capability_contracts`. Fallo → abort de fase + DLQ `./.events/dead-letter` con código `CONTRACT_OUTPUT_SCHEMA_MISMATCH`. Skip lab: `SDDIA_LAB_SKIP_CAPABILITY_DI=1` (coherente con resolve/gate).

## Revalidación envelope Cerbero (PBI-042 Hito 4 — R9)

Tras gate APTO y RBAC allow, `cerbero_di_envelope` valida cada objeto `di_binding` empaquetado contra `di.binding.schema.json` en `directories.capability_contracts` y cruza coherencia con `ResolvedBinding` + fila `capability-bindings.md`. Fallo → abort pre-inject + DLQ con `CERBERO_ENVELOPE_SCHEMA_MISMATCH` o `CERBERO_DI_BINDING_INCOHERENT`. Orden: `resolve → gate → cerbero_rbac → cerbero_envelope → inject`. Skip lab: `SDDIA_LAB_SKIP_CAPABILITY_DI=1`.

## Implementación de referencia

Rust: `scripts/skills-rs/src/capsule_v2.rs` (Cúmulo: `paths.skillsRustPath`).

---
*Norma SSOT de I/O JSON para cápsulas. Mantener alineado con `SddIA/skills/skills-contract.json` y `SddIA/tools/tools-contract.md` (tools: `meta.entityId` = `name`, v1.2.0).*
