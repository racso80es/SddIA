---
feature_name: kaizen-aislamiento-multi-instancia
created: "2026-08-26"
process: feature
base: main
scope: systemd-templates,launchers,start-sddia,instance-creator
---

# Spec — kaizen-aislamiento-multi-instancia

## L-SYS-02

Plantillas user: `ExecStart=%f/SddIA/scripts/daemons/@@DAEMON_NAME@@.sh` y `ExecStart=%f/SddIA/daemons/email-watcher.sh`. `install_systemd_templates` sustituye solo `@@DAEMON_NAME@@`. `_materialize_systemd_units` igual. Copiar el mismo molde forja→AP→`~/.config/systemd/user/` deja contenido idéntico; systemd expande `%f` por instancia.

## L-DEP-10

`_sddia_resolve_instance_root` en `sddia_shell_lib.sh`. Consumidores: `_run_daemon.sh`, `_exec_daemon.sh`, `kalma2-bridge.sh`, `SddIA/daemons/email-watcher.sh`. Prohibido `cd` a `SCRIPT_DIR/../..` si env o cwd instancia es válido.

## L-CEN-PKILL

Cero `pkill -x` en `_run_daemon.sh` y `start-sddia.sh` cleanup operativo. `_sddia_stop_lock_pid`. Con `INVOCATION_ID` (unidad systemd) skip stop-by-lock al arrancar (KillMode).

## L-ELF-07

Wrappers email y kalma2: release antes que debug (`_sddia_resolve_daemon_binary`).
