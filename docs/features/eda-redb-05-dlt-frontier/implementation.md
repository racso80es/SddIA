---
feature_name: eda-redb-05-dlt-frontier
created: "2026-10-21"
branch_name: feat/eda-redb-05-dlt-frontier
---

# Implementación — eda-redb-05-dlt-frontier

El despacho vive en `route_domain_core` y `dlt_telemetry_anchor`. La telemetría calcula un digest local y escribe la prueba sin `copy_dlt_seal`. El ancla externa abre `events.redb` solo si `writes_store`.

`entity-manager` actualiza las clases de dominio y la norma. `events-contract` no tiene `uuid`: la forja reescribe `contract_version` y omite el sello `Domain_Entity_*`. La norma operativa está en `SddIA/norms/`; la forja cae ahí cuando no existe copia de librería, y persiste `tactical_norm_version` en el frontmatter.

Los JSON de suscripción no los toca la forja. El objeto IOTA se retira a mano en el Core, conservándolo en `PullRequest_Merged` y `System_Immunity_Certified`.
