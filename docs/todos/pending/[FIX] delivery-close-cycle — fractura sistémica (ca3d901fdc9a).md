---
document_id: PBI-FIX-FRACTURE-ca3d901fdc9a
uuid: "ca3d901f-dc9a-4b50-8362-21eccbf35da3"
title: "[FIX] delivery-close-cycle — fractura sistémica"
format: markdown
version: "1.1.0"
created: "2026-09-04"
updated: "2026-09-04"
status: "abierto"
refinement_status: clarified
priority: alta
process: bug-fix
type: fix
dispatch: false
fracture_hash: ca3d901fdc9a
fracture_process: delivery-close-cycle
incident_ref: "System_Fracture_Detected — ca3d901fdc9a"
execution_id: "21eccbf3-3477-4c5c-a4c7-86e651bf5da3"
branch: fix/ignition-pre-push-guard
child_pbi:
  - PBI-FIX-FRACTURE-ca3d901fdc9a-OLA1
  - PBI-FIX-FRACTURE-ca3d901fdc9a-OLA2
  - PBI-FIX-FRACTURE-ca3d901fdc9a-OLA3
friction_ids:
  - F-DCC-SNAPSHOT-FINAL
  - F-DCC-PUBLICACIN-REMOTA
  - F-DCC-EVOLUTION-GATE
  - F-DCC-INDEX-INTEGRITY
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - docs/todos/pending/[FIX] delivery-close-cycle — Ola 1 cápsula git-manager (ca3d901fdc9a).md
  - docs/todos/pending/[FIX] delivery-close-cycle — Ola 2 sddia-qa (ca3d901fdc9a).md
  - docs/todos/pending/[FIX] delivery-close-cycle — Ola 3 binario ausente no fractura (ca3d901fdc9a).md
source_audit: "2026-09-04 DCC execution_id 21eccbf3. Snapshot final y Publicación remota: cápsula git-manager ausente bajo SddIA/target. Aduana evolution e integridad índices: sddia-qa ausente. Cuatro System_Fracture_Detected emitidos. start-sddia.sh no construye git-manager ni sddia-qa. F4b no cubre binario lab ausente."
review_notes: "v1.0.0 semilla Cúmulo. v1.1.0 laudo: deuda absorbida en Ola 1–3. Padre = índice; no se implementa aquí."
---

# [FIX] delivery-close-cycle — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` |
| Emisor | `execute-process` |
| Acción intentada | `Snapshot final` |
| Rama | `fix/ignition-pre-push-guard` |
| `execution_id` | `21eccbf3-3477-4c5c-a4c7-86e651bf5da3` |

## Traza de error

```
cápsula skill 'git-manager' no encontrada bajo SddIA/target
```

Fases hermanas del mismo ciclo:

| Fase | `friction_id` | Traza | Evento |
|------|---------------|-------|--------|
| Snapshot final | `F-DCC-SNAPSHOT-FINAL` | cápsula `git-manager` ausente | `72a41559-18c9-4299-8355-de1defdc3559` |
| Publicación remota | `F-DCC-PUBLICACIN-REMOTA` | cápsula `git-manager` ausente | `aedcea75-80fd-4e47-a05e-a4fce051271d` |
| Aduana evolution | `F-DCC-EVOLUTION-GATE` | `sddia-qa` ausente | `1760faf4-1d13-486a-b8ca-c77c082d1b98` |
| Aduana integridad índices | `F-DCC-INDEX-INTEGRITY` | `sddia-qa` ausente | `65507278-56a4-4328-955b-fd44d2f13569` |

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado de la ola que desbloquee esa fase.

## Laudo — olas

Dos raíces de laboratorio, un defecto de aduana Kintsugi. No mezclar en un solo `bug-fix`.

| Ola | PBI | Friction | Causa | Desbloquea |
|-----|-----|----------|-------|------------|
| 1 | `PBI-FIX-FRACTURE-ca3d901fdc9a-OLA1` | `F-DCC-SNAPSHOT-FINAL`, `F-DCC-PUBLICACIN-REMOTA` | Ignición no produce ELF `git-manager` bajo `compiled_capsules.native_root` | Snapshot + push DCC |
| 2 | `PBI-FIX-FRACTURE-ca3d901fdc9a-OLA2` | `F-DCC-EVOLUTION-GATE`, `F-DCC-INDEX-INTEGRITY` | Ignición no produce `sddia-qa` | Aduanas Argos DCC + hook pre-push |
| 3 | `PBI-FIX-FRACTURE-ca3d901fdc9a-OLA3` | las cuatro (sobre-escalado) | Binario lab ausente ≠ colapso ontológico; F4b no suprime | Deja de emitir `System_Fracture_Detected` por receta de compile |

Orden: 1 → 2 (DCC secuencial). Ola 3 independiente; no sustituye 1 ni 2.

## Criterio de cierre (padre)

- [x] Ola 1 cerrada (PBI en `done/`, `validacion.md` APTO)
- [x] Ola 2 cerrada
- [ ] Ola 3 cerrada o DIFERIDO explícito con laudo
- [ ] Este TODO movido a `docs/todos/done/` en el PR que cierre la última ola absorbida
