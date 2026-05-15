---
name: "codex-sync-laboratories"
nature: "feature-spec"
version: "1.0.0"
status: "implemented"
---

# Especificación: Codex Sync y Simetría Fractal

## 1. Propósito Táctico
Establecer la sincronización de la Librería SddIA hacia los laboratorios productivos (SddIA_1 a SddIA_4). El objetivo es que cada proyecto abandone el conocimiento legado (limbo/starter-kit) y pase a consumir un **Códice de Dominio** oficial emitido por el Core.

## 2. Arquitectura de Simetría Fractal
La trinchera local debe ser un reflejo exacto del motor central. La estructura física resultante en cada laboratorio DEBE ser:
`{SddIA_n}/.SddIA/library/codexes/` -> Contenedor del manifiesto del Códice.
`{SddIA_n}/.SddIA/library/norms/` -> Contenedor de las Normas Atómicas indexadas.

## 3. Matriz de Enrutamiento
- **SddIA_1 y SddIA_3 (Backend Admin):** Consumen `codex-backend-admin-splus.md`
- **SddIA_2 (Frontend Admin):** Consume `codex-frontend-admin-splus.md`
- **SddIA_4 (Frontend Product):** Consume `codex-frontend-product-splus.md`

## 4. Criterios de Aceptación (Aduana de Fricción)
- [x] Poda total de archivos residuales en la antigua ruta `.SddIA/norms/`.
- [x] Volcado físico (sin enlaces simbólicos) de Códices y Normas en la nueva estructura fractal.
- [x] Actualización de `{SddIA_n}/.SddIA/local.paths.json` apuntando a las nuevas rutas.
