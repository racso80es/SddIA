---
feature_name: sddia-installer-v3-host-shortcuts
version: "1.0.0"
---

# Runbook — regenerar atajos en el host de ingeniería

## Cuándo

Tras merge de cambios en `SddIA/scripts/installer/shortcuts/` o si los `.sh` de escritorio vuelven a invocar `sddia-installer.sh` en lugar del presentador.

## Comando canónico (AC-H1)

Desde la forja (`FORGE` = raíz del clone SddIA):

```bash
cd "$FORGE"
./sddia-installer.sh shortcuts --dest "$HOME/Aplicaciones/SddIA"
```

Equivalente explícito:

```bash
/home/racso/Proyectos/SddIA/sddia-installer.sh shortcuts --dest /home/racso/Aplicaciones/SddIA
```

## Verificación rápida

```bash
grep -l 'sddia-installer-ui.sh' ~/Aplicaciones/SddIA/SddIA_*.sh
test -x ~/Aplicaciones/SddIA/SddIA_Deploy.sh
test -x ~/Aplicaciones/SddIA/SddIA_Eliminar_Cliente.sh
! grep -q 'teardown --force' ~/Aplicaciones/SddIA/SddIA_Eliminar_Cliente.sh
```

## Notas

- `~/Aplicaciones/SddIA/` no está versionado; la SSOT son las plantillas en la forja (norma `sddia-installer-contract` 1.3.0 § Atajos generados).
- `.desktop` opcionales: no forman parte de este runbook salvo petición del operador.
