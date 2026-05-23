---
document_id: PBI-AMPLIACION-CONFIGURACION-ENTORNOS
title: "[Ola A — Hito 0] Jerarquía de Bóvedas (configuración de entorno)"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "completado"
closed: "2026-05-23"
priority: ola-a-hito-0
feature_ref: docs/features/ampliacion-configuracion-entornos
validacion_ref: docs/features/ampliacion-configuracion-entornos/validacion.md
pr_ref: "https://github.com/racso80es/SddIA/pull/20"
merge_commit: "f0ef7bf"
branch: feat/ampliacion-configuracion-entornos
---

# [Ola A — Hito 0] Jerarquía de Bóvedas (configuración de entorno)

**Estado:** ✅ Completado — Hitos 0.1–0.3 implementados; Argos APTO; PBI archivado en `done/` (`validacion.md` `pbi_archived: true`, 2026-05-23).

---

CONTEXTO ESTRATÉGICO:
La Ola A incluye la **Jerarquía de Bóvedas** (configuración de entorno) como **Hito 0**. Ejecutar **prioritariamente** antes de cualquier resolución de pasivo técnico.

JERARQUÍA:
- `./.dev/.env` — Global al repo
- `./.SddIA/.dev/.env` — Local al proyecto; **prevalece** sobre global

HITO 0.1 — Cargador jerárquico
- Crear `SddIA/scripts/qa/env_loader.py` con `load_hierarchical_env(repo_root)`.
- Cargar global primero; local sobrescribe en dict intermedio.
- Log si ambos existen: `[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env`

HITO 0.2 — Entrypoints
- `execute-process.py` + `execute_process_capsules.py` (`run_process`) invocan loader **antes** de cualquier cápsula.
- `iota-immutable-publisher/index.ts`: eliminar dotenv local; consumir env inyectado.

HITO 0.3 — Sanitización
- Eliminar `.env` dispersos en subdirectorios de tools.
- `.gitignore`: `.dev/` y `.SddIA/.dev/`.

FEATURE: `docs/features/ampliacion-configuracion-entornos/`
RAMA: `feat/ampliacion-configuracion-entornos`
