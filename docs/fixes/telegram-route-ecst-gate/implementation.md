---
feature_name: telegram-route-ecst-gate
created: "2026-11-01"
tracker_ref: AUT-60
---

# Implementación — AUT-60

- `copy_tree` + copia `SddIA/events/` en `temp_repo_with_bus_skeleton`.
- `telegram_route_test_payload` con `text`, `chat_id`, `source`.
- `assert_delivery_not_ecst_gate_only` en ambos tests Telegram.
- Comprobaciones de `error` sin catálogo/payload ausente en el test de texto vacío.
- Router: `assemble_domain_process_subscriber_inputs` antes de `resolve_orchestrator_bin` para que `skipped-empty-text` no exija binario en repos temporales.
- Test con texto: `SDDIA_EXECUTE_PROCESS_BIN` apuntando al ELF debug del checkout (`lab_execute_process_bin`).
