# Protocolo de Aceptación de Pull Requests

Este documento define las **condiciones obligatorias** que deben cumplirse antes de que cualquier Pull Request (PR) sea creado o aceptado. Este protocolo es aplicable a:
- Agentes de IA durante la acción `finalize-process`.
- Desarrolladores locales (Cursor) antes de hacer `push`.
- Sistemas de CI/CD (GitHub Actions) como condición de bloqueo.

## Requisitos Obligatorios

1.  **Nomenclatura (Naming):**
    - Ejecución exitosa de `scripts/validate-nomenclatura.ps1`.
    - Ramas: `feat/<kebab-case>`, `fix/<kebab-case>`, `feat/refactorization-<kebab-case>`.
    - Commits: Formato convencional (opcional pero recomendado).

2.  **Lint:**
    - El proyecto debe pasar el linter sin errores.
    - Comando: `npm run lint` (working-directory: `src/`).

3.  **Build:**
    - El proyecto debe compilar sin errores.
    - Comando: `npm run build` (working-directory: `src/`).

4.  **Pruebas (Tests):**
    - Todos los tests deben pasar.
    - Comando: `npm run test` (working-directory: `src/`).

5.  **Ejecución:**
    - Los checks anteriores se ejecutan en el workflow de GitHub Actions (`pr-validation.yml`).
    - Opcionalmente, puede usarse una skill o herramienta SddIA para ejecutar estos checks de forma local y trazada.

## Violaciones
Cualquier intento de eludir este protocolo resultará en:
- Rechazo automático del PR por GitHub Actions.
- Bloqueo de la acción `finalize-process` por parte del Agente.
- Advertencia en el editor local.
