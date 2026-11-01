---
global: APTO
branch: Fix_Events_Multiple
tracker_ref: AUT-61
pbi_archived: false
validated_base_sha: "2d14642644f1acd7ebfa1fa32bac0f941f817736"
validated_head_sha: "6ac2b651c54bbbab8f5b14cb1cc4eb6aaabb7fa4"
validated_tree_sha: "520520befed83c2b688eaaf303c1320b1414b6ab"
diff_digest: "sha256:0971d1c7bde471cbf30ba394620c0c3513f6cd6c3ee32f41c29969fe93134121"
---

# Validación — AUT-61

## Comando

Ejecutado desde `SddIA/`:

```bash
cargo test -p execute-process --test eda_subscriber_ecst_redb -- --test-threads=1
```

## Resultado

13 passed; 0 failed (integración router + matriz de 15 claves vía catalog y tests apretados; incluye test dedicado Aiua TQM y aislamiento de `SDDIA_AGENT_RUNTIME_COMMAND`).

Cobertura de claves con `delivery_status: success` en el test catalog y en:

- `route_domain_tracker_markdown_apply_not_blocked_by_missing_branch`
- `route_domain_memory_evolution_not_blocked_by_missing_branch`
- `route_domain_telegram_responder_receives_ecst_path`
- `route_domain_kalma2_task_queue_not_blocked_before_handler`
- `route_domain_aiua_task_queue_not_blocked_before_handler`
- `route_domain_telegram_empty_text_still_skipped` (`skipped-empty-text`, sin regresión)
- `mode_guard_isolates_host_agent_runtime_command` (lab sin spawn del host)

## Notas

- AUT-20 criterio 1 (auditoría HU) no se cierra solo con este sello; AUT-61 es kaizen hijo.
- Cierre Linear: issue AUT-61 en `done` vía tracker al merge del PR, sin fichero en `docs/todos/done/`.