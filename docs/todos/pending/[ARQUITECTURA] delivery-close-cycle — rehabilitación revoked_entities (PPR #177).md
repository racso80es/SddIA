---
document_id: PBI-PPR-177-DCC-REVOKED-REGISTRY
title: "[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #177)"
format: markdown
version: "1.0.0"
created: "2026-08-16"
updated: "2026-08-16T16:23:00Z"
status: abierto
priority: media
process: bug-fix
uuid: 9d2e4f81-6a3c-4b5e-8f17-c0a9d3e5b728
source_feature: docs/fixes/centinelas-fracture-ola-20260812
source_correlation_id: 4b770fd6-99a0-435e-af43-a153aa23e310
source_audit: docs/fixes/centinelas-fracture-ola-20260812/validacion.md
incident_ref: "RBAC_EMITTER_NOT_REVOKED:NO_APTO — delivery-close-cycle re-revocado since 2026-08-16T16:11:08Z (success_rate_below_threshold)"
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/process/delivery-close-cycle.md
  - docs/todos/done/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md
  - docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174).md
  - docs/fixes/centinelas-fracture-ola-20260812/validacion.md
sightings:
  - "PPR #177 · CID 4b770fd6… (origen seed · Cosecha Kaizen 2026-08-16T16:18:30Z)"
  - "PPR #178 · CID ca6fc6cb… · Cosecha Kaizen 2026-08-16T16:23:00Z — dedup; misma revocación since 2026-08-16T16:11:08Z"
---

# [ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #177)

## Incidente (Cosecha Kaizen · Cúmulo)

Re-revocación empírica tras cierre histórico PPR #136 (signer/emitter).

| Campo | Valor |
|-------|--------|
| Entidad | `delivery-close-cycle` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → `revoked.delivery-close-cycle` |
| `entity_type` (instancia) | `tool` (entropía: es proceso) |
| `reason` | `success_rate_below_threshold` |
| `since` | `2026-08-16T16:11:08Z` |
| ≠ incidente #136 done | #136: `abrupt_success_rate_drop` since `2026-07-13` (cerrado; check APTO residual abierto) |
| ECST origen | `4b770fd6-99a0-435e-af43-a153aa23e310` · PR #177 |
| Sighting adicional | `ca6fc6cb-4ecd-427f-9638-ae1960963cc3` · PR #178 (Cosecha Kaizen 2026-08-16T16:23:00Z — dedup; sin seed nueva) |
| Check aduana | `RBAC_EMITTER_NOT_REVOKED: NO_APTO` (Cerbero F4/F5; no bloqueante) |
| Firmante ECST | `Vertice_Biologico_Relay` (presente — E2 #136 liquidado) |

## Mandato

1. Laudo Cerbero: rehabilitar o confirmar revocación permanente según política Radamanto.
2. Alinear instancia: retirar/confirmar clave en `revoked_entities.json`; corregir `entity_type` si procede (`process` ≠ `tool`).
3. Reset/redención stats Radamanto o umbrales diferenciados para procesos multi-fase (mismo patrón S+ que PPR #174).
4. Cascada `docs/fixes/` o feature dedicada + `validacion.md` APTO + PBI → `done/`.
5. Verificar `RBAC_EMITTER_NOT_REVOKED: APTO` en aduana PPR posterior.

## Fuera de alcance

- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136 done).
- Revocación `pull-request-review` (seed ARQUITECTURA PPR #174 pending; misma ola de umbrales).
- Merge / handoff `accept-pr` del PR #177 (fuera de Cosecha).

## Refinamiento: Resolución de Impacto S+ Grade (16-08-2026)

Tras aplicar la jurisprudencia arquitectónica establecida en el PPR #174 y someter este incidente al Protocolo de Acero, se confirma que el colapso del proceso `delivery-close-cycle` obedece a la misma fractura sistémica: la evaluación lineal de un macro-proceso mediante métricas de herramienta atómica por parte de Radamanto.

Para blindar la arquitectura S+ Grade y consolidar la base operativa del ecosistema, se establecen las siguientes directrices de intervención homólogas:

### Vías de Implementación Inmediata (Mandato Activo)
1. **Auditoría de Umbrales Diferenciados (Alineación Radamanto):** Rectificación innegociable del `entity_type` a `process` en los registros de Cerbero y en `.SddIA/revoked_entities.json`. Se aplicará a `delivery-close-cycle` la política estadística de Radamanto con mayor tolerancia (`success_rate`) para absorber la latencia y la fricción inherentes a las operaciones físicas de red (Git push, interacciones con la API de GitHub) y la firma de ECST.
2. **Resiliencia de Fase en Handoff (Kintsugi Ontológico):** Inyección de un patrón de falla controlada (*fail-soft*) en la orquestación del cierre de ciclo. Si la fase de captura del `telemetry_receipt` o la validación del estado del repositorio sufre un *timeout* no crítico, el proceso no debe colapsar térmicamente. El ECST `PullRequest_Presented` debe emitirse y firmarse con `Vertice_Biologico_Relay` siempre que la intención biológica (el *commit/push*) haya cruzado el umbral físico con éxito, logrando un estado `APTO` en la aduana `RBAC_EMITTER_NOT_REVOKED`.

### Constancia de Evolución Futura (Faro Kaizen)
3. **Desacople de la Aduana de Emisión:** Queda registrada en el Cúmulo la necesidad teórica de aislar el chequeo de revocación del emisor (`RBAC_EMITTER_NOT_REVOKED`) en un centinela EDA independiente. Esto evitaría que fallos de latencia en la aduana re-revoquen el proceso completo, permitiendo que la Táctica de Inmunidad escale de forma distribuida en futuras versiones de la Librería.