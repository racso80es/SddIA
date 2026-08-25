---
feature_name: kaizen-ignicion-soberana-centinelas
created: "2026-08-25"
process: feature
branch: feat/kaizen-ignicion-soberana-centinelas
branch_name: feat/kaizen-ignicion-soberana-centinelas
persist_ref: docs/features/kaizen-ignicion-soberana-centinelas
pbi_ref: docs/todos/done/REFACTOR - despliegue centinelas.md
document_id: PBI-KAIZEN-IGNICION-SOBERANA
uuid: "a2a69784-9dff-47ab-a0bb-aa3c576068b8"
execution_id: "7a0edc97-6a5e-4ee0-861a-894f9df6cc63"
evolution_id: "181d6291-9735-4187-a6f7-f6e56472aa3e"
global: APTO
pbi_archived: true
checks:
  AC-IC: APTO
  AC-SH: APTO
  AC-PORT: APTO
  AC-IND: APTO
  AC-TD: APTO
  AC-REBOOT: NO_APTO
  KITCHEN: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs
  - SddIA/templates/systemd/sddia-daemon@.service.template
  - SddIA/scripts/daemons/kalma2-bridge.sh
  - start-sddia.sh
  - start-sddia.md
  - SddIA/scripts/build-release-bundle.sh
  - docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt de teardown.md
  - docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
  - docs/todos/done/REFACTOR - despliegue centinelas.md
  - docs/features/kaizen-ignicion-soberana-centinelas/
  - SddIA/evolution/181d6291-9735-4187-a6f7-f6e56472aa3e.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — kaizen-ignicion-soberana-centinelas

Argos (relevo IDE). `global: APTO`. PBI en `docs/todos/done/`. `pbi_archived: true`.

## Checks

| ID | Veredicto | Evidencia |
|----|-----------|-----------|
| AC-IC | APTO | `install_systemd_templates` renderiza `sddia-event-watcher@.service` / sweeper / kalma2-bridge; test `creates_topology_and_skips_smoke` (CORE_ROOT = instance_root; fábrica no copiada cruda). `cargo test -p execute-process --lib engine::handlers::instance_creator` 3/3. |
| AC-SH | APTO | Rama `DAEMON_JURIS=systemd`: `enable --now` + `exit 0`; `wait` solo en jurisdicción `script`. `bash -n start-sddia.sh`. |
| AC-PORT | APTO | Health `GET ${KALMA_URL}/` con `SDDIA_CLIENT_PORT` (default 8765; Paciente 0 8766 vía bóveda). |
| AC-IND | APTO | Unidades nombradas distintas; email plantilla aparte. |
| AC-TD | APTO | Teardown: loop stop/disable stems `@%f`. Deploy prompt §5–6 alineado (Argos: deuda operativa). Kitchen NFT: cero superficie. |
| AC-REBOOT | NO_APTO | Linger/`WantedBy=default.target` en código; reboot host **no** ensayado esta sesión. No gate. |
| KITCHEN | APTO | Sin mutación. |

## Fuera de gate

- IOTA relay sigue `&` en jurisdicción `script`.
- Enable empírico `systemctl --user` sobre forja no ejecutado (evitar colisión lab).
