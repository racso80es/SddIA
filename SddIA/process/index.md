# Índice de process (Core SddIA)

Contrato de familia: `process-contract.md` (no constituye un proceso ejecutable catalogado en esta tabla).

> **ABSTRACT-03:** process software-lifecycle + ciclo PR viven en packing `SddIA/library/codexes/codex-software-engineering/process/` (`directories.process_domain_roots`). No son filas ejecutables de este índice Core.

| Name | UUID | Versión | Context | Aliases | Descripción |
|------|------|---------|---------|---------|-------------|
| query-ecosystem-health | 2b337302-e794-46b8-ad4e-f65bafd21c94 | 1.0.0 | quality-assurance | — | Fusiona map-snapshot × territorio Argos/Radamanto/Cerbero y  |
| compile-ecosystem-map-snapshot | e7f09165-c445-49ae-965d-41abb4738679 | 1.0.0 | ecosystem-evolution | — | Precompila inventario esperado (tools/skills/daemons) en map |
| instance-creator | dead5ca7-c0b9-42ef-aad6-171991fb524f | 1.3.0 | ecosystem-evolution, filesystem-ops, system-operations | — | Despliegue hermético de instancia consumidor: topología .Sdd |
| evolution-audit | 8f4b09da-e277-4fc2-9890-8a363fa8a96f | 1.0.0 | quality-assurance, filesystem-ops | — | Auditoría periódica del registro evolution: inventario, relevancia, validación y persistencia oficial. |
| memory-evolution-ingest | eb50d05d-c8d8-4cb7-a7ed-4d296971cbe2 | 1.1.1 | ecosystem-evolution, event-routing | — | Ingesta Domain_Entity_Telemetry_Captured → vector_store/evolution |
| user-preference-ingest | d4e5f6a7-b8c9-4d0e-1f2a-3b4c5ab005 | 1.0.0 | ecosystem-evolution, knowledge-management | — | Ingesta User_Preference_Change_Requested → store preferencias |
| event-bus-audit | 8d577a50-055a-40b9-b7e2-93e2d2415796 | 1.0.1 | quality-assurance | — | Auditoría empírica on-demand del bus EDA: escaneo ./.events, |
| kalma2-interact | acdb6c88-f0d9-4e10-9d2f-7e4b5401a892 | 1.1.1 | ecosystem-evolution | — | Proceso PoC Kalma2: síntesis Mayeuta lab ante prompt del cli |
| daemon-heartbeat-audit | f45bda9d-40d9-471e-82a1-b9404b5a0dfd | 1.0.1 | quality-assurance | — | Triaje Argos: latido térmico Centinelas; System_Fracture_Det |
| daemon-kill-switch | b0de6585-11fc-4b3c-8b19-ad6b727d820e | 1.0.1 | system-operations | — | Kill-Switch global: purga SIGTERM/SIGKILL de todos los Centi |
| governance-daemon-manager | 5a89793a-ba98-4b4f-9287-43c087e312df | 1.0.1 | system-operations | — | Actuador OS puro: start, status, kill de Centinelas indexado |
| daemon-creator | c172f130-532f-4714-be4e-fcd80b84a5dc | 1.0.1 | ecosystem-evolution | — | Proceso maestro para estandarizar y automatizar la creación de nuevos Centinelas (daemons) en el Core SddIA. |
| governance-daemon-manager | 5a89793a-ba98-4b4f-9287-43c087e312df | 1.0.1 | system-operations, ecosystem-evolution | — | Actuador OS puro: start, status, kill de Centinelas indexados bajo directories.daemons. |
| daemon-kill-switch | b0de6585-11fc-4b3c-8b19-ad6b727d820e | 1.0.1 | system-operations, ecosystem-evolution | — | Kill-Switch global: purga SIGTERM/SIGKILL de todos los Centinelas al apagar el Core. |
| daemon-heartbeat-audit | f45bda9d-40d9-471e-82a1-b9404b5a0dfd | 1.0.1 | quality-assurance, event-routing | — | Triaje Argos: latido térmico Centinelas; System_Fracture_Detected tras 3 ciclos omitidos. |
| process-creator | 7c2d9e41-88a3-4f6b-9c12-4def01a2b3c4 | 1.0.0 | ecosystem-evolution | — | Proceso maestro para instanciar nuevos procesos en el Core SddIA y mantener el índice del directorio `process`. |
| skill-creator | b8c3d1e2-f4a5-4a6b-8c7d-0e1f2a3b4c5d | 1.0.0 | ecosystem-evolution | — | Proceso maestro para estandarizar y automatizar la creación de nuevas skills (definición física y lógica) en el Core SddIA. |
| action-creator | d0e1f2a3-b4c5-46d7-e8f9-0a1b2c3d4e5f | 1.0.0 | ecosystem-evolution | — | Proceso maestro para instanciar nuevas acciones (orquestaciones lógicas) en el Core SddIA y mantener el índice del directorio `actions`. |
| tool-creator | c4355159-b6ea-4201-973a-a08db5ce8156 | 1.1.0 | ecosystem-evolution | — | Forja de cápsulas tool con destino bifurcado por `scope` (`core` → `SddIA/tools/`; `local` → `.SddIA/tools/`) e índice sincronizado en cada ámbito. |
| agent-creator | e7d5087c-6d47-4890-9602-34962496b3bb | 1.0.0 | ecosystem-evolution | — | Proceso maestro para instanciar nuevas identidades operativas (Agentes) en el Core SddIA y mantener el índice soberano bajo `agents`. |
| task-queue-manager | 608ae470-4db2-4ae6-8bb8-7aa5949c208a | 1.0.0 | ecosystem-evolution, filesystem-ops | automatic_task | Meta-orquestación de **cola de tareas** para el Core SddIA. Expone el alias canónico legacy **`automatic_task`** (`process-contract v1.3.0`) hacia el mismo archivo físico. |
| sddia-difusion | de142ec3-4022-4ac1-bcf4-1b8490cabf9d | 1.0.0 | ecosystem-evolution | — | Proceso que cubre la **deuda** registrada al purgar la antigua acción de difusión: materializa la propagación controlada del Core SddIA hacia `.cursor/rules`, `.github` y homólogos, sin violar la frontera Acciones/Procesos. |
| norm-creator | a132a6fc-52c8-4795-8c68-a2897d456588 | 1.2.1 | ecosystem-evolution, knowledge-management | — | Creator de **`tactical-norm`** (`Library_Norm`): triaje, clasificación + UUID, destilación, materialización e **indexación** de `library_norms/index.md` según **`norms-contract.md`**. |
| codex-creator | dd9e13b2-fc07-40d2-95f5-b50ebd535a9e | 1.1.1 | ecosystem-evolution, knowledge-management | — | Creator de **`domain-codex`** (`Library_Codex`): inventario de normas, identidad, estrategia, materialización e **indexación** de `library_codexes/index.md` según **`codex-contract.md`**. |
| entity-manager | 62f08bbd-e9ce-479d-8d1b-792684e1bd26 | 1.0.2 | ecosystem-evolution | — | Gestor de Entidad: fachada sobre *-creator (piloto skill + event) con sello universal emit-domain-mutation. |
| event-creator | b28194d9-62a8-4cbc-9cbd-237e51e44333 | 1.2.0 | ecosystem-evolution | — | Proceso maestro para instanciar Clases de Evento ECST en `SddIA/events/` y mantener el índice del genoma. |
| route-domain-event | c8e91f2a-4b6d-4e1a-9f03-2d7e5a684b10 | 1.0.1 | event-routing, ecosystem-evolution | — | Orquestador bus EDA V3+: fan-out suscriptores, topología simétrica, testigos y cabeceras por estado. |
| route-telemetry | b2c3d4e5-f6a7-4890-b1c2-d3e4f5a6b7c8 | 1.0.1 | event-routing, ecosystem-evolution | — | Enrutador bus fractal `./.events/telemetry/` → Radamanto (`radamanto-batch`). |
| route-orchestration | c3d4e5f6-a7b8-4901-c2d3-e4f5a6b7c8d9 | 1.0.1 | event-routing, ecosystem-evolution | — | Enrutador bus fractal `./.events/orchestration/`. |
| route-domain | d4e5f6a7-b8c9-4012-d3e4-f5a6b7c8d9e0 | 1.0.1 | event-routing, ecosystem-evolution | — | Enrutador bus fractal `./.events/domain/` (coexiste con V3+ pending). |
| telemetry-batch-stub | f1e2d3c4-b5a6-4789-8c0d-1e2f3a4b5c6d | 1.0.1 | event-routing, quality-assurance | — | **Deprecated** — sustituido por `radamanto-batch` (Fase 4). |
| radamanto-batch | 2a3b4c5d-6e7f-4a8b-9c0d-1e2f3a4b5c6d | 1.1.1 | event-routing, quality-assurance, ecosystem-evolution | — | Batch Radamanto: Self-Healing Domain_Entity_* + Telemetry_Captured. |
| telemetry-compliance-audit | b3c4d5e6-f7a8-4901-b2c3-d4e5f6a7b8c9 | 1.0.1 | event-routing, quality-assurance | — | Auditoría cumplimiento termodinámico; fan-out telemetría (Fase 5). |
| cerbero-governance-react | 3b4c5d6e-7f8a-4b9c-0d1e-2f3a4b5c6d7e | 1.0.1 | event-routing, knowledge-management | — | RBAC reactivo Self-Healing. |
| fix-tool-process | 4c5d6e7f-8a9b-4c0d-1e2f-3a4b5c6d7e8f | 1.0.1 | ecosystem-evolution, filesystem-ops, quality-assurance | — | Reparación sandbox; Argos `structure_valid` sin redención. |
| workspace-smoke | c4e8a1b2-3f5d-4a9c-8e7b-2d1f0a9b6c3e | 1.0.1 | quality-assurance | — | Smoke laboratorio: instanciación workspace dinámico (Telemetría Fase 2). |
| audit-thermodynamic-toll-failsoft | 2e8cd8cd-e0cd-4b0e-ae78-09150ab9c266 | 1.0.1 | chaos-engineering, quality-assurance | — | Audit Caos: `io-choke` + fail-soft Peaje Termodinámico (D3.13). |
| audit-telemetry-compliance-breach | fd2f075c-5d01-4b54-8b26-67678417e22b | 1.0.1 | chaos-engineering, quality-assurance, event-routing | — | Audit Caos: `schema-corruptor` → `Telemetry_Compliance_Breached`. |
| audit-sandbox-isolation-rbac | 242d937d-a0da-4d36-ab89-c0fbbc18c868 | 1.0.1 | chaos-engineering, quality-assurance | — | Audit Caos: `sandbox-breacher` + bloqueo Inocuidad workspace. |
| suite-creator | f3a1b2c3-d4e5-46f7-8901-234567890abc | 1.0.1 | ecosystem-evolution | — | Creator de **Suite** (ED Caos): validación, materialización e indexación bajo `suites/`. |
| execute-suite | a1b2c3d4-e5f6-4789-a012-3456789abcde | 1.0.1 | chaos-engineering, quality-assurance, ecosystem-evolution | — | Orquestador de Suites: sub-workspaces aislados por nodo + manifiesto Argos. |
| telegram-gateway | f5a6b7c8-d9e0-4f1a-b2c3-d4e5f6a7b8c9 | 1.0.1 | ecosystem-evolution, external-ingest | — | Aduana texto Telegram → eventos domain (`TelegramMessage_Received`, `Manual_Task_Requested`, `Kaizen_Idea_Captured`). |
| telegram-fallback-responder | c9d0e1f2-a3b4-4c5d-6e7f-8a9b0c1d2e3f | 1.0.1 | ecosystem-evolution, external-ingest | — | Triaje inverso Telegram: Filtro C → Mayeuta → `send-telegram-notification`. |
| sync-client-assets | 0f6bf2ff-a067-46fb-9175-ee97e6a5dcd8 | 1.0.0 | ecosystem-evolution, knowledge-management | — | Sincronización unidireccional repositorio maestro → instancia cliente: 4 fases (Manifiesto-Local, Reclamacion, Aduana-Integridad, Inyeccion). Aduana SHA-256 pre-escritura. |


