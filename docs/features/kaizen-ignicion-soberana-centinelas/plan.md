---
feature_name: kaizen-ignicion-soberana-centinelas
created: "2026-08-25"
process: feature
phases: factory-systemd,launcher-bridge,ignition-script,docs-teardown
---

# Plan — kaizen-ignicion-soberana-centinelas

1. Plantilla fábrica `@@DAEMON_NAME@@` + `%f`.
2. `install_systemd_templates`: render lista canónica; skip copia cruda de la fábrica; tests creator.
3. `kalma2-bridge.sh`.
4. `start-sddia.sh`: jurisdicción, sync user units, enable @%f, exit 0; cleanup condicional.
5. `start-sddia.md`, ONBOARDING bundle, teardown Paciente 0.
6. `implementation.md` / `execution.md` (Tekton).
