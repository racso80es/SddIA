---
feature_name: kaizen-aislamiento-multi-instancia
created: "2026-08-26"
process: feature
phases: templates-percent-f,resolver,no-pkill,tests,docs
---

# Plan — kaizen-aislamiento-multi-instancia

1. Plantillas `%f`; quitar `@@SDDIA_CORE_ROOT@@` de ExecStart.
2. `instance-creator` + `start-sddia` materialize: no hornear path.
3. `_sddia_resolve_instance_root` + `_sddia_stop_lock_pid`.
4. Extirpar `pkill -x` en wrappers/cleanup.
5. Tests handler + smoke shell `test-instance-root-resolver.sh`.
6. Cascada documental; evolution; ensayo dos raíces (operador).
