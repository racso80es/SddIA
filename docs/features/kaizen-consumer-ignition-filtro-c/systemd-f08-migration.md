---
feature_name: kaizen-consumer-ignition-filtro-c
created: "2026-08-20"
process: feature
purpose: Migración F-08 — sddia-daemon@ lab-fijo → patrón %f
---

# Migración systemd multi-cliente (F-08)

## Estado lab actual

Unidades user en `~/.config/systemd/user/` pueden tener `WorkingDirectory` cableado al lab. Patrón canónico: plantillas en `SddIA/templates/systemd/`:

- `sddia-email-watcher@.service.template` (`WorkingDirectory=%f`)
- `sddia-daemon@.service.template` (`WorkingDirectory=%f`, `EnvironmentFile=%f/.SddIA/.dev/.env`)

`instance-creator` materializa unidades renderizadas en `{instancia}/.SddIA/systemd/`.

## Procedimiento operador (host)

1. Deshabilitar unidad lab-fija si colisiona:
   `systemctl --user disable --now 'sddia-daemon@event-watcher.service'` (ajustar nombre).
2. Instalar plantilla parametrizada (copiar desde `.SddIA/systemd/` o templates Core) a `~/.config/systemd/user/`.
3. Enable por instancia:
   `systemctl --user enable --now 'sddia-email-watcher@/home/…/SddIA_AP.service'`
   (sintaxis `%f` = ruta instancia; ver `man systemd.unit` / Instantiated units).
4. Exportar `SDDIA_SENSORIAL_JURISDICTION=systemd` en bóveda de instancia para R-07.
5. Verificar: `systemctl --user show -p WorkingDirectory <unit>` distinto por cliente; cero locks cruzados en `.SddIA/daemons/status/`.

## Gate O7

≥2 instancias en el mismo host con WD y `EnvironmentFile` distintos; sin PIDs/credenciales cruzados.
