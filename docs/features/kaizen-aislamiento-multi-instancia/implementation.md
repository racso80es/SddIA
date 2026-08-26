---
feature_name: kaizen-aislamiento-multi-instancia
created: "2026-08-26"
process: feature
items:
  - systemd_percent_f
  - instance_root_resolver
  - lock_pid_stop
---

# Implementation — kaizen-aislamiento-multi-instancia

| Path | Cambio |
|------|--------|
| `SddIA/templates/systemd/sddia-daemon@.service.template` | `ExecStart=%f/SddIA/scripts/daemons/@@DAEMON_NAME@@.sh` |
| `SddIA/templates/systemd/sddia-email-watcher@.service.template` | `ExecStart=%f/SddIA/daemons/email-watcher.sh` |
| `instance_creator.rs` | sin `@@SDDIA_CORE_ROOT@@`; tests `%f` |
| `sddia_shell_lib.sh` | `_sddia_resolve_instance_root`, `_sddia_stop_lock_pid` |
| `_run_daemon.sh` `_exec_daemon.sh` `kalma2-bridge.sh` `daemons/email-watcher.sh` | raíz instancia; 0 `pkill -x` |
| `start-sddia.sh` | materialize `%f`; cleanup lock-PID |
