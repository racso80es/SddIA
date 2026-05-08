---
action_id: finalize-process
contract_ref: actions-contract.json
flow_steps:
  - Precondiciones (proceso / tarea lista para cierre)
  - Commits atómicos vía skill git-save-snapshot
  - Evolution Logs de producto (paths.evolutionPath)
  - Evaluación de impacto SddIA (si hubo cambios bajo SddIA/) skill sddia-evolution-register + snapshot
  - Publicación y PR vía skills git-sync-remote y git-create-pr
  - 'Paso final orquestado (tarea finalizada, post-fusión en remoto): skill git-close-cycle con target_branch = rama de trabajo'
  - Auditoría según política del proyecto
  - 'post_pr: (opcional) skills Git tácticas solo si git-close-cycle no aplica o requiere intervención excepcional'
triggers_semanticos:
  - Proceso finalizado
  - Tarea finalizada
  - Cierre de ciclo de feature o fix tras validación
inputs:
  - Carpeta de la tarea (paths.featurePath o paths.fixPath, Cúmulo)
  - Rama feat/ o fix/ (no rama base protegida)
name: Finalize Process
outputs:
  - Rama en origin
  - Evolution Logs de producto actualizados
  - Pull Request hacia la rama base del remoto
  - finalize-process.md opcional en carpeta de tarea (YAML Frontmatter)
skills_orquestadas:
  - git-workspace-recon
  - git-save-snapshot
  - sddia-evolution-register
  - git-sync-remote
  - git-create-pr
  - git-close-cycle
  - git-tactical-retreat
  - verify-pr-protocol
---
# Action: finalize-process

## Propósito

La acción **finalize-process** cierra el ciclo cuando el **proceso o la tarea** están finalizados: consolida commits, actualiza Evolution Logs del producto, cumple el protocolo de evolución **SddIA** si hubo cambios bajo `./SddIA/`, **publica la rama** y abre el **Pull Request** hacia la rama base (p. ej. `main`). Solo debe ejecutarse con validación previa aceptable; si no, debe bloquearse o advertir.

## Principio (Ley COMANDOS)

- **Sin ejecución directa del SO:** la acción **no** prescribe scripts `.ps1`/`.bat` ni comandos literales (`cargo`, `git`, etc.) como paso normativo. El ejecutor invoca únicamente **skills** y **tools** registradas en Cúmulo (`paths.skillCapsules`, `paths.toolCapsules`), p. ej. mediante `scripts/skills/run-capsule-from-tekton-request.ps1` y `.tekton_request.json` en la raíz del repo.
- **No integrar en la rama base por la vía directa:** el merge es vía PR.
- **SSOT:** PR y logs referencian la carpeta de la tarea en `paths.featurePath` o `paths.fixPath`.

## Entradas

- Carpeta de la tarea (Cúmulo) con `objectives.md` y, para cierre seguro, `validacion.md` con resultado global coherente.
- Rama `feat/` o `fix/` con cambios consolidados.

## Salidas

- Rama publicada en `origin` cuando corresponda antes del PR.
- Entradas en `paths.evolutionPath` / `paths.evolutionLogFile`.
- PR creado hacia la rama base; cuerpo del PR con enlaces a documentación de la tarea y resumen de validación.
- Opcional: `finalize-process.md` en la carpeta de la tarea (frontmatter: `pr_url`, `branch`, `timestamp`).

## Orquestación de skills (orden de referencia)

1. **`git-workspace-recon`** — comprobar estado del workspace.
2. **`git-save-snapshot`** — commits pendientes, si los hay.
3. **Validación pre-PR** — si el proyecto exige **`verify-pr-protocol`**, invocar su cápsula (envelope v2); no usar `cargo run` ni comandos equivalentes como texto normativo.
4. **Evolution Logs de producto** — actualizar `paths.evolutionPath` según convención del repo.
5. **Impacto SddIA** — si el cambio tocó `./SddIA/`, ejecutar **`sddia-evolution-register`** y un **`git-save-snapshot`** adicional que incluya el registro, **antes** de subir.
6. **`git-sync-remote`** — integrar/sincronizar con remoto según política.
7. **`git-create-pr`** — publicar rama si aplica (`pushFirst`) y crear PR (`body` o `bodyFile` con objectives + validación).
8. **`git-close-cycle`** — **paso final de ejecución orquestada** cuando el disparador semántico es *tarea finalizada* y el cierre completo incluye higiene local **tras** la fusión en remoto: debe invocarse con `request.target_branch` igual al **nombre de la rama de trabajo** (`feat/` o `fix/`) detectada al inicio del cierre (p. ej. vía **`git-workspace-recon`** o el campo `branch` documentado en `finalize-process.md`). Si el PR aún no está fusionado, `-d` puede fallar: ejecutar esta skill solo cuando corresponda el post-merge o aceptar el envelope de error.

Cada invocación debe usar el contrato de cápsula (stdin/`--request-file` según `SddIA/norms/capsule-json-io.md`) o, para Tekton, el fichero `.tekton_request.json` y `run-capsule-from-tekton-request.ps1`.

## Flujo resumido

1. Verificar precondiciones (rama no es la base protegida, documentación mínima presente).
2. Consolidar trabajo con **`git-save-snapshot`**.
3. Opcional **`verify-pr-protocol`** vía skill.
4. Actualizar Evolution Log de producto.
5. Si mutó `SddIA/`: **`sddia-evolution-register`** + snapshot.
6. **`git-sync-remote`** → **`git-create-pr`**.
7. **`git-close-cycle`** — con `target_branch` = rama de trabajo (post-fusión en remoto cuando aplique).
8. Opcional: **`finalize-process.md`** y auditoría.

## Integración con otras acciones y procesos

- **`validate`:** precede al cierre; si `validacion.md` indica fallo bloqueante, no se debe publicar sin excepción explícita.
- **Proceso `feature`:** la fase 8 (Finalizar) consume esta acción; especificación en `paths.actionsPath/finalize-process/`.

## Estándares

- **Grado S+:** trazabilidad hasta PR; cumplimiento de evolución SddIA cuando aplique.
- **Acciones = orquestación:** ver `SddIA/actions/actions-contract.md`.

---
*Definición de la acción finalize-process (cierre de proceso/tarea). Sustituye el identificador histórico `finalize`.*
