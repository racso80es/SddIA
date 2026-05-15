---
uuid: "7c18fe07-9567-4f06-8d2b-a58e04608171"
name: "pr-acceptance-protocol"
version: "1.0.0"
nature: "tactical-norm"
author: "norm-creator"
scope: "agnostic"
category: "workflow"
dependencies: []
---

## Directriz Core

Antes de crear o aceptar cualquier Pull Request deben cumplirse condiciones obligatorias de calidad y gobernanza. Aplica a agentes en la acción `finalize-process`, desarrolladores locales antes de publicar rama, y pipelines CI/CD como condición de bloqueo.

### Requisitos obligatorios

1. **Nomenclatura:** ramas y convenciones de naming del repositorio validadas según `SddIA/norms/git-operations.md` y scripts o reglas de nomenclatura del proyecto (p. ej. `feat/<kebab-case>`, `fix/<kebab-case>`). Commits alineados con Conventional Commits u otra política declarada en el proyecto.
2. **Lint:** el proyecto pasa el linter configurado sin errores (comando definido en el manifiesto o scripts del repositorio).
3. **Build:** el proyecto compila o empaqueta sin errores (comando de build del stack activo).
4. **Tests:** la suite de pruebas del proyecto pasa sin fallos.
5. **Ejecución:** los checks anteriores corren en el workflow CI del repositorio; opcionalmente se ejecutan de forma local y trazada vía skills autorizadas (`shell-executor`) o procesos SddIA, nunca eludiendo la auditoría previa de Cerbero/Argos.

### Orquestación de cierre

La apertura de PR no es skill atómica: sigue el proceso en `SddIA/norms/pull-request-orchestration.md` — validación previa, publicación vía `git-manager`, apertura en forja vía `shell-executor` con `gh` u herramienta autorizada. Prohibido enrutar `gh` a través de `git-manager`.

### Violaciones

El incumplimiento implica rechazo automático por CI, bloqueo de `finalize-process` por el agente, y advertencia en el entorno local.

## Restricciones Duras (Aduana de Fricción)

- Prohibido crear o fusionar un PR si lint, build o tests del proyecto fallan en el estado publicado.
- Prohibido publicar rama o abrir PR sin cumplir nomenclatura de ramas definida en `git-operations.md` y política del repositorio.
- Prohibido omitir la secuencia de validación previa (Cerbero/Argos) antes de invocar `git-manager` o `shell-executor` en cierre de tarea.
- Prohibido invocar `gh pr create` u operaciones de forja directamente en terminal sin pasar por `shell-executor` y esquema congelado cuando el flujo es orquestado por SddIA.
- Prohibido eludir el workflow CI del repositorio como condición de aceptación (bypass de checks obligatorios).
- Prohibido ejecutar `finalize-process` como completado si algún requisito obligatorio de esta norma no está verificado y registrado en `validacion.md`.
