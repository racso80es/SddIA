---
feature_name: kaizen-ignicion-soberana-centinelas
created: "2026-08-25"
process: feature
items:
  - instance_creator_factory
  - kalma2_bridge_launcher
  - start_sddia_jurisdiction
  - teardown_paciente0
---

# Implementation — kaizen-ignicion-soberana-centinelas

| Path | Cambio |
|------|--------|
| `SddIA/templates/systemd/sddia-daemon@.service.template` | `@@DAEMON_NAME@@` + `%f`; ExecStart wrappers `scripts/daemons/` |
| `SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs` | Render lista canónica; no copiar fábrica cruda |
| `SddIA/scripts/daemons/kalma2-bridge.sh` | Lanzador ELF WUI |
| `start-sddia.sh` | Jurisdicción systemd vs script; materialize + enable `@%f`; exit 0 |
| `start-sddia.md` | v1.4.0 |
| `SddIA/scripts/build-release-bundle.sh` | ONBOARDING systemd núcleo |
| `docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt de teardown.md` | stop/disable unidades de instancia |
