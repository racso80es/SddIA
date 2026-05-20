---
uuid: "d4e5f6a7-b8c9-4012-d345-67890abcdef0"
name: "pull-request-orchestration"
version: "1.0.0"
entity_type: "norm"
jurisdiction: "dedalo"
---

# Orquestación de Pull Request (no es skill atómica)

## 1. Ley

**Crear una Pull Request** no es una habilidad atómica del Core. Es un **Proceso** diseñado por Dédalo e implementado en el ecosistema de agentes (p. ej. Tekton u orquestador equivalente).

## 2. Secuencia canónica (alto nivel)

1. **Validación previa:** Cerbero y Argos comprueban política (ramas, estado del workspace, normas en `git-operations.md`) antes de invocar skills.
2. **Publicación de commits:** invocar la skill **`git-manager`** con `operation_type` acorde (p. ej. `push`) según `SddIA/norms/skill-io-git-manager-frozen.md`.
3. **Apertura en forja:** invocar la skill **`shell-executor`** con el ejecutable **`gh`** (u otra herramienta explícitamente autorizada) y `arguments` como array, según `SddIA/norms/skill-io-shell-executor-frozen.md`.

Queda **prohibido** enrutar `gh` a través de `git-manager`.

## 3. Presentación (cierre de entrega)

La apertura de Pull Request hacia `main` se orquesta **exclusivamente** mediante el proceso **`SddIA/process/delivery-close-cycle.md`** (`process: delivery-close-cycle`), resuelto vía Cúmulo y ejecutado con `action:execute-process`.

Secuencia canónica dentro del proceso:

1. **Publicación remota:** `skill:git-manager` → `push` de `branch_name` a `origin`.
2. **Apertura en forja:** `skill:shell-executor` → `gh pr create` (o resolución de PR existente vía `gh pr view`).
3. **Sello ECST:** `action:emit-pr-presented-event` con `emitter_agent: delivery-close-cycle` y `pr_url` correlacionado cuando el contrato del evento lo admita.

Queda **prohibido**:

* Forjar una acción monolítica que combine apertura de PR y escritura en el bus (violación SRP).
* Invocar `gh pr create` en terminal o runbooks sin pasar por `delivery-close-cycle` salvo excepción documentada en esta norma.
* Enrutar `gh` a través de `git-manager`.

## 4. Merge / Aceptación (SSOT local)

Toda fusión hacia la rama principal (`main`) en el entorno local del workspace debe orquestarse **estricta y exclusivamente** mediante el proceso **`SddIA/process/accept-pr.md`** (`process: accept-pr`), resuelto vía Cúmulo y ejecutado con `action:execute-process`.

Queda **terminantemente prohibido**:

* Ejecutar manualmente `git merge` (u homólogos imperativos de fusión) por parte de cualquier agente, operador humano o asistente, fuera del flujo declarado en `accept-pr`.
* Sustituir `accept-pr` por invocaciones ad hoc de `skill:git-manager` con `operation_type: merge` sin pasar por las fases del proceso (Auditoría Genómica → Fusión Soberana → Sello Criptográfico de Fusión → Sincronización y Limpieza).
* Considerar `gh pr merge` como vía canónica de consolidación local hacia `main` salvo evolución explícita de esta norma.

La **Única Fuente de Verdad (SSOT)** para la consolidación de código en `main` es `accept-pr`.

## 5. Responsabilidades

| Rol | Responsabilidad |
| :--- | :--- |
| Dédalo | Definir el Proceso (pasos, criterios de éxito, manejo de errores). |
| Tekton (u orquestador) | Ejecutar el Proceso invocando skills en el orden y contexto acordados. |
| Cerbero / Argos | Auditar entradas contra esquemas congelados y políticas. |

## 6. Referencias

- `SddIA/process/delivery-close-cycle.md`
- `SddIA/actions/emit-pr-presented-event.md`
- `SddIA/norms/skill-io-git-manager-frozen.md`
- `SddIA/norms/skill-io-shell-executor-frozen.md`
- `SddIA/norms/git-operations.md`
- `SddIA/process/accept-pr.md`
