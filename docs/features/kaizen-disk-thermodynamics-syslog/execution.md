---
feature_name: kaizen-disk-thermodynamics-syslog
created: "2026-09-13"
process: feature
branch_name: feat/kaizen-disk-thermodynamics-syslog
persist_ref: docs/features/kaizen-disk-thermodynamics-syslog
execution_id: "e3bbb4f8-b27c-4833-9750-d855b504da8b"
document_id: PBI-KAIZEN-DISK-THERMODYNAMICS-SYSLOG
items_applied:
  - silence-hot-path
  - unit-tests
  - systemd-rate-limit
  - smoke-factory-grep
  - evolution-register
---

# Ejecución — kaizen-disk-thermodynamics-syslog

## Init

`SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `e3bbb4f8-b27c-4833-9750-d855b504da8b`. Commit planificación `2986c08`. Stash `wip-unrelated-pre-kaizen-disk` no entra.

## Tests locales

```text
cd SddIA && cargo test --offline -p event-watcher
# 6 passed
bash SddIA/scripts/qa/test-instance-root-resolver.sh
# OK instance-root-resolver
```

## Runbook host (operador; no genoma)

Unidad ejemplo de este host: `sddia-event-watcher@home-racso-Proyectos-SddIA.service`.

1. `sudo truncate -s 0 /var/log/syslog && sudo rm -f /var/log/syslog.1 && sudo systemctl restart rsyslog`
2. `sudo journalctl --vacuum-size=200M`
3. Tras merge + `cargo build --release -p event-watcher`: `systemctl --user daemon-reload` y restart de la unidad activa.
4. Techos recomendados (fuera del PR): journald `SystemMaxUse=300M` `SystemMaxFileSize=50M`; logrotate syslog `size 500M`.
