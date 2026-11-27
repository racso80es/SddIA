---
uuid: "a69d04b0-1d07-49ef-bcbf-6850e4a70ae2"
name: "SddIA Codex Software Engineering"
version: "1.1.0"
nature: "domain-codex"
author: "tekton-abstract-02"
target_environment: ["software-engineering", "git", "pull-request"]
aliases:
  - codex-agile-forge
certification_grade: "Pendiente"
process_membership:
  - feature
  - bug-fix
  - refactorization
  - pull-request-review
  - accept-pr
  - delivery-close-cycle
  - forge-pbi
  - refine-item
composition:
  - norm: "4c448c82-de41-460f-b24f-82a84fa5ed69"
    path: "../norms/features-documentation-pattern.md"
  - norm: "1c6af49c-3091-4648-aa54-bbf6bcb90f82"
    path: "../norms/patterns-in-planning-implementation-execution.md"
  - norm: "7c18fe07-9567-4f06-8d2b-a58e04608171"
    path: "../norms/pr-acceptance-protocol.md"
  - norm: "028fb30f-66e7-4811-bfc6-960c808b9517"
    path: "../norms/backlog-item-refinement.md"
hash_signature: "sha256:d504dcaef5a7994c8dc800860130d9ca39e75d129c745ae175f40ff127aa5d5e"
---

# SddIA Codex Software Engineering

## Estrategia de Dominio

Empaqueta el ciclo de vida de ingeniería de software (feature / bug-fix / refactorization y cierre PR) como Códice inyectable. El Core orquestador permanece agnóstico: solo concede autoridad cuando el perfil activo declara este códice (o el legado software-first `git_required: true` sin otro `codex_slug`).

Normas tácticas: documentación atómica de tareas, patrones plan→implementación→ejecución, protocolo de aceptación de PR y refinamiento unificado de backlog (`backlog-item-refinement`).

## Instrucciones de Prioridad

1. **`pr-acceptance-protocol`**: prevalece para cierre/fusión de entregas software.
2. **`features-documentation-pattern`**: un `.md` por fase; prohibido JSON paralelo de fase.
3. **`patterns-in-planning-implementation-execution`**: precedencia de `pattern_id` en implementación/ejecución.
4. **`backlog-item-refinement`**: proceso canónico `refine-item`; prohibido `refine-pbi` / `refine-hu`.

## Process membership (SSOT)

| Process | Rol |
|---------|-----|
| `feature` | Ciclo feature |
| `bug-fix` | Ciclo fix |
| `refactorization` | Ciclo refactor |
| `pull-request-review` | Aduana PPR |
| `accept-pr` | Fusión soberana |
| `delivery-close-cycle` | Cierre entrega / PR |
| `forge-pbi` | Alta PBI en tracker |
| `refine-item` | Refinamiento Mayeuta + DoR |

Relocalización física fuera de `directories.process` = ABSTRACT-03 (**L-PACK-MULTIROOT-SIX-MOVE**): packing canónico `SddIA/library/codexes/codex-software-engineering/process/` vía `directories.process_domain_roots` (Cúmulo ≥1.6.0). Move físico pendiente de AC-RESOLVE evidenciado en ciclo ABSTRACT-03.
