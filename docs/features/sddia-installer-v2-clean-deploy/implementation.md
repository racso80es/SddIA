---
feature_name: sddia-installer-v2-clean-deploy
created: "2026-09-26"
process: feature
branch_name: feat/sddia-installer-v2-clean-deploy
persist_ref: docs/features/sddia-installer-v2-clean-deploy
execution_id: "8ff78e98-6e4f-4859-9ca9-60ba718703ea"
---

# Implementación — sddia-installer-v2-clean-deploy

| Área | Cambio |
|------|--------|
| Motor | `SddIA/scripts/sddia-installer.sh` v2: vault compuesto, registro, puerto, unidades condicionales, plantillas sha |
| Fachada | `./sddia-installer.sh`: verify + eventos domain |
| systemd | Plantilla daemon + `_precondition.sh` |
| Rust | `instance_health_verify`, `instance_creator` perfil engineering, kalma2 exit 78 |
| Genoma | `instance-health-verify`, eventos Instance_*, norm 1.1.0 vía EM |
| Core | `instance.host_registry`, suscripciones IOTA |
