---
document_id: PBI-FIX-FRACTURE-c291912fe9a5
title: "[FIX] accept-pr — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-10-30"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: c291912fe9a5
fracture_process: accept-pr
incident_ref: "System_Fracture_Detected — c291912fe9a5"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/audits/runtime-seal-tracker-outbound-20261030
  - branch: docs/runtime-seal-tracker-outbound-20261030
---

# [FIX] accept-pr — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `accept-pr` |
| Emisor | `tekton` |
| Acción intentada | `Fusion Soberana` |

## Traza de error

```
diff_seal_mismatch: docs/features/_fixture-done-gate-lab/validacion.md falta validated_base_sha. La higiene de accept-pr borro la rama remota docs/runtime-seal-tracker-outbound-20261030 con la fusion no ejecutada. execution_id d8310429-7fd7-4b0d-861a-c39e18e1eae5.
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async · relevo IDE; Shell/`./sddia-run.sh` Rejected → sin stdout nativo `enrich-fracture-pbi-kaizen`)*

- **Clasificación:** `structured` (lectura genoma; no cubo léxico del catálogo para `diff_seal_mismatch`)
- **Firma traza:** `diff_seal_mismatch` + path fixture + higiene remota post-aborto
- **evolution_verdict:** `process_fix`

### Diagnóstico de causa raíz

1. **Aborto de Fusión Soberana (primario):** `validate_for_accept_pr_merge` barre **todos** los `docs/{features,fixes,refactors}/**/validacion.md` del tip de `source_branch` (`list_persist_refs_with_validacion`). El fixture `docs/features/_fixture-done-gate-lab/validacion.md` declara `global: APTO` **sin** `validated_base_sha` / campos de sello → el gate exige sello completo y falla con `diff_seal_mismatch` antes del merge. El `persist_ref` de auditoría real (`docs/audits/…`) **no** entra en ese ls-tree; el colapso lo dispara el fixture lab en el árbol.
2. **Efecto colateral reportado (secundario, a verificar empíricamente):** la traza afirma borrado remoto de `docs/runtime-seal-tracker-outbound-20261030` con fusión no ejecutada. En el genoma, `delete_branch` higiene vive solo en fase **Sincronización y Limpieza** tras push de `main` exitoso. Si el aborto fue en Fusión Soberana, la higiene **no** debería correr → o bien hay violación del invariante `L-HYGIENE-SOFT` / umbral `merge_commit_hash` en orquestación/handoff, o la narrativa de traza mezcla un delete de otro camino. Requiere evidencia `git-manager` / report de `execution_id` `d8310429-…` (Shell peaje pendiente).
3. **Contexto satélite:** Argos ya selló `docs/audits/runtime-seal-tracker-outbound-20261030/validacion.md` como `NO_APTO` / `integridad_bloqueada` (cascada incompleta, tip ≠ sello previo). Eso no explica el path fixture en el error, pero sí bloquea un reintento limpio de accept-pr sobre esa rama.

### Veredicto evolutivo

**Corrección de proceso oficial** (`process_fix`)

### Propuestas

- **Corrección de proceso oficial:** Excluir persistencias lab/`_fixture-*` del barrido de `list_persist_refs_with_validacion` (o prohibir `global: APTO` en fixtures sin sello completo); fail-closed: higiene remota solo si `accept_pr_physical_threshold_crossed` (`merge_commit_hash` no vacío).
- **Corrección de proceso oficial:** Auditar handoff PPR/`execution_id` `d8310429-7fd7-4b0d-861a-c39e18e1eae5` para confirmar si `delete_branch` remoto se invocó post-`diff_seal_mismatch`; si sí, regresión de orquestación.
- **Operación (tras laudo):** Restaurar rama remota vía `skill:git-manager` (no bypass raw); no reinyectar `accept-pr` hasta limpiar fixture o re-sellar validaciones en alcance y peaje Shell/git-manager.

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar. Prohibido bypass (`gh`/`git`/`curl`) hasta laudo.

## Criterio de cierre

- [ ] Causa raíz resuelta
- [ ] Caso añadido a `fracture-corpus` (firma esperada o `unclassified` justificado)
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este ítem cerrado en el sumidero del proveedor
