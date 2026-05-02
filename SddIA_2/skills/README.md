# Skills SddIA — Definición vs implementación

Este directorio es **paths.skillsDefinitionPath** (Cúmulo, `SddIA/agents/cumulo.json`). Separa la **capa de definición (SddIA)** de la **capa de implementación (scripts)** para desacoplar la arquitectura IA de los artefactos técnicos.

## Estructura

| Ubicación | Contenido | Propósito |
|-----------|-----------|-----------|
| **SddIA/skills/** (este directorio) | Contrato global: `skills-contract.json`, `skills-contract.md`. Por skill: subcarpeta **&lt;skill-id&gt;/** con `spec.md` y `spec.json`. | **Definición:** qué hace el skill, contrato, entradas/salidas, flujo. Consumido por agentes y procesos. |
| **scripts/skills/** | Índice `index.json` y, por skill con ejecutable, cápsula **&lt;skill-id&gt;/** (manifest, .ps1, .bat, doc, bin/). | **Implementación:** código, launchers y ejecutables. Ruta canónica: **paths.skillCapsules[&lt;skill-id&gt;]** (Cúmulo). |

La **raíz del path de implementación** la indica Cúmulo (**paths.skillsPath**, **paths.skillCapsules**). No se usan rutas literales en la definición.

## Definición por skill (SddIA/skills/&lt;skill-id&gt;/)

Cada skill tiene en este directorio una carpeta con:

- **spec.md** — Especificación legible: objetivo, entradas, salidas, flujo, reglas. Idioma: es-ES.
- **spec.json** — Metadatos machine-readable. Si el skill tiene implementación ejecutable, debe incluir **implementation_path_ref**: referencia a la ruta de implementación en Cúmulo (ej. `paths.skillCapsules.<skill-id>`).

## Implementación por defecto: Rust

**Las implementaciones por defecto de los scripts de skills (igual que en tools) han de ser en Rust.** Los ejecutables se construyen en paths.skillsRustPath (Cúmulo) y se copian a cada cápsula `bin/`. Launcher `.bat` en la cápsula invoca el `.exe` en `bin/` si existe; si no, fallback al script `.ps1`.

## Listado de skills

| skill_id | Descripción breve | Cápsula (paths.skillCapsules) |
| :--- | :--- | :--- |
| invoke-command | Interceptor de comandos de sistema (git, dotnet, npm, pwsh). | invoke-command |
| invoke-commit | Commit con parámetros directos (--message, --files, --all). | invoke-commit |
| git-workspace-recon | Inspección del workspace Git (JSON v2 / CLI). | git-workspace-recon |
| git-branch-manager | Crear o checkout de rama. | git-branch-manager |
| git-save-snapshot | Stage y commit con request JSON o CLI. | git-save-snapshot |
| git-sync-remote | Sincronización fetch/pull/push. | git-sync-remote |
| git-tactical-retreat | Reset/limpieza destructiva con confirmación. | git-tactical-retreat |
| git-create-pr | Push opcional y PR con gh. | git-create-pr |
| git-close-cycle | Checkout integración, pull/fetch prune, eliminar rama local de trabajo. | git-close-cycle |
| git-operations | Uso seguro de Git (ramas feat/fix, commits convencionales). | — |
| documentation | Estándares SSOT y gestión de documentación. | — |
| filesystem-ops | Operaciones de archivo seguras (PowerShell). | — |
| dotnet-development | Estándares .NET (build, test, logging). | — |
| frontend-build | Build Next.js (Product, Admin) fallback. | — |
| security-audit | Auditoría y hooks pre-commit/pre-push. | — |

## Guías de portabilidad (otros repos SddIA)

- **Proceso `create-skill`:** `reproducir-create-skill-en-otros-entornos-sddia.md`
- **Skills concretas (definición + cápsula):** `reproducir-skills-en-otros-entornos-sddia.md`

## Referencias

- Contrato global: `skills-contract.json`, `skills-contract.md` (en este directorio).
- Cúmulo: `SddIA/agents/cumulo.json` → **paths.skillsDefinitionPath**, **paths.skillsPath**, **paths.skillCapsules**, **paths.skillsIndexPath**.
- Índice de implementaciones: **paths.skillsIndexPath** (paths.skillsIndexPath, Cúmulo).
