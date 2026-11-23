---
uuid: "b8e4d2a1-3f6c-4a9e-8d1b-2c5f7e9a0b3d"
name: "spec-template"
version: "1.0.0"
nature: "motor"
contract: "templates-contract v1.1.0"
family: "template"
impacts_doc: false
---

# Plantilla — Especificación de feature (DIA)

Plantilla motor para forjar `spec.md` bajo `persist_ref`. Incluye **Declaración de Impacto de Artefactos (DIA)**.

## Uso

1. Copiar estructura al crear `docs/features/<feature>/spec.md`.
2. Ajustar `impacts_doc` según mutaciones estructurales previstas.
3. Completar la sección **Impacto en Documentación** si `impacts_doc: true`.

## Cuerpo mínimo sugerido

### Contexto

(Describir problema y alcance.)

### Criterios de aceptación

> **Regla Ola 0 (`KZ-PRM-01`):** Cada criterio debe declarar su **Oráculo mecánico ejecutable (Ola 0)** (test unitario/integración o subcomando `sddia-qa`) y contar con captura en rojo (`oracle-red.log` bajo `persist_ref`) antes de autorizar cambios productivos en Ola 1. Prohibido usar `grep` manual ad-hoc como único método de verificación.

| ID | Criterio | Oráculo mecánico ejecutable (Ola 0) |
|----|----------|-------------------------------------|
| CA-1 | … | `cargo test …` / `sddia-qa …` |

### Impacto en Documentación

<!-- Enumerar README/manuales a actualizar si impacts_doc: true -->
- (ninguno)
