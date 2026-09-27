---
document_id: AUDIT-INSTALLER-HOST-SHORTCUTS-REGEN-20260927T150200Z
uuid: "a1c4e8f2-3b6d-4e91-9c0a-7f2d8e1b5a93"
title: "Acta — regeneración atajos ~/Aplicaciones/SddIA (Installer v3)"
created: "2026-09-27"
auditor: tekton
forge_root: /home/racso/Proyectos/SddIA
shortcuts_dest: /home/racso/Aplicaciones/SddIA
pbi_ref: PBI-OPERATIVO-INSTALLER-V3-ATAJOS-HOST
verdict: APTO
---

# Regeneración atajos escritorio — Installer v3

## Comando ejecutado

```text
/home/racso/Proyectos/SddIA/sddia-installer.sh shortcuts --dest /home/racso/Aplicaciones/SddIA
```

Salida del installer: materialización de `SddIA_Deploy.sh` y `SddIA_Eliminar_Cliente.sh`.

## Plantillas forja (sha256)

| Artefacto | sha256 |
|-----------|--------|
| `SddIA/scripts/installer/shortcuts/SddIA_Deploy.sh` | `c376b5788d6db1204aca408d54d9dc283fe4b9e2852e25be2f869bed50dc3f5c` |
| `SddIA/scripts/installer/shortcuts/SddIA_Eliminar_Cliente.sh` | `0695d585c38164d9b3bc1bb1dcd0f45c1902561328787bd975acb8c0b1b5e30c` |

## Host tras regeneración (AC-H2, AC-H3)

| Comprobación | Resultado |
|--------------|-----------|
| `SddIA_Deploy.sh` ejecutable | sí (`-rwxrwxr-x`) |
| `SddIA_Eliminar_Cliente.sh` ejecutable | sí |
| Invoca `sddia-installer-ui.sh` | sí (ambos) |
| `teardown --force` directo al motor | **no** (script Eliminar de 4 líneas, solo `exec … ui.sh teardown`) |
| sha256 materializado Deploy | `530180556b52b09a8fb9ee5b5fb090d9724f3054f8e1a8ec4e082a178d18bd90` |
| sha256 materializado Eliminar | `50b1cdb222285ac059e3874191a0581628fde61887415b51a618ea33283c4e22` |

Los hashes del host difieren de la plantilla por sustitución de `__FORGE_ROOT__` → `/home/racso/Proyectos/SddIA` (comportamiento esperado del comando `shortcuts`).

## Estado previo (legacy)

Antes de la regeneración, los atajos hacían `exec` a `sddia-installer.sh` y `SddIA_Eliminar_Cliente.sh` aplicaba `teardown --force` sin presentador ni confirmación `ELIMINAR`.

## Laudo

**APTO** — AC-H1..H4 del PBI operativo cumplidos en el host de ingeniería del Vértice Biológico.
