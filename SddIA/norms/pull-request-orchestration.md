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

## 3. Merge / Aceptación (SSOT local)

Toda fusión hacia la rama principal (`main`) en el entorno local del workspace debe orquestarse **estricta y exclusivamente** mediante el proceso **`SddIA/process/accept-pr.md`** (`process: accept-pr`), resuelto vía Cúmulo y ejecutado con `action:execute-process`.

Queda **terminantemente prohibido**:

* Ejecutar manualmente `git merge` (u homólogos imperativos de fusión) por parte de cualquier agente, operador humano o asistente, fuera del flujo declarado en `accept-pr`.
* Sustituir `accept-pr` por invocaciones ad hoc de `skill:git-manager` con `operation_type: merge` sin pasar por las fases del proceso (Auditoría Genómica → Fusión Soberana → Sello Criptográfico de Fusión → Sincronización y Limpieza).
* Considerar `gh pr merge` como vía canónica de consolidación local hacia `main` salvo evolución explícita de esta norma.

La **Única Fuente de Verdad (SSOT)** para la consolidación de código en `main` es `accept-pr`.

## 4. Responsabilidades

| Rol | Responsabilidad |
| :--- | :--- |
| Dédalo | Definir el Proceso (pasos, criterios de éxito, manejo de errores). |
| Tekton (u orquestador) | Ejecutar el Proceso invocando skills en el orden y contexto acordados. |
| Cerbero / Argos | Auditar entradas contra esquemas congelados y políticas. |

## 5. Referencias

- `SddIA/norms/skill-io-git-manager-frozen.md`
- `SddIA/norms/skill-io-shell-executor-frozen.md`
- `SddIA/norms/git-operations.md`
- `SddIA/process/accept-pr.md`
