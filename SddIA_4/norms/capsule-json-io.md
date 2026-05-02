# Contrato de invocación: envelope JSON (skills y cápsulas)

**schema_version:** `2.0`  
**Ámbito:** petición/respuesta en stdin/stdout (o variables documentadas) para binarios en `paths.skillCapsules` / tools homólogas.

## Petición

Un único objeto JSON (una línea recomendada; se acepta JSON multilínea en stdin).

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `meta` | object | Metadatos obligatorios. |
| `meta.schemaVersion` | string | Debe ser `"2.0"`. (Alias aceptado en deserialización: `schema_version`.) |
| `meta.entityKind` | string | `"skill"` para skills. (Alias: `entity_kind`.) |
| `meta.entityId` | string | kebab-case; coincide con `skill_id`. (Alias: `entity_id`.) |
| `meta.token` | object | Opcional; Karma2Token si el contrato lo exige. |
| `request` | object | Cuerpo libre definido en `paths.skillsDefinitionPath/<skill-id>/spec.md`. |

**Entrada alternativa:** variable de entorno `GESFER_CAPSULE_REQUEST` con el mismo JSON. Si `GESFER_SKIP_STDIN=1`, no se lee stdin.

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

## Implementación de referencia

Rust: `scripts/skills-rs/src/capsule_v2.rs` (Cúmulo: `paths.skillsRustPath`).

---
*Norma SSOT de I/O JSON para cápsulas. Mantener alineado con `SddIA/skills/skills-contract.json`.*
