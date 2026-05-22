[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. NO EXPLANATIONS, ONLY CODE.]

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
