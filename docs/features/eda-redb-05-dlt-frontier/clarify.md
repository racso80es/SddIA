---
feature_name: eda-redb-05-dlt-frontier
created: "2026-10-21"
document_id: PBI-EDA-REDB-05-DLT-FRONTIER
pbi_ref: docs/todos/done/[ARQUITECTURA] EDA Redb 05 — Frontera de auditoría DLT.md
---

# Clarify — eda-redb-05-dlt-frontier

`events-contract` 1.2.0 declara `dlt_audit_scope`: `local_only` | `external_public_interest`. Una clase que no declara el enum es `local_only`.

Interés público: `PullRequest_Merged` (cumulo) y `System_Immunity_Certified` (radamanto). Conservan el suscriptor `iota-immutable-publisher`.

Alcance local: `PullRequest_Presented`, las mutaciones `Domain_Entity_*` (incluida la telemetría), los estímulos de Telegram, tarea manual, Kalma2, Kaizen, pensamiento, memoria vectorial e instancias. Se retira solo el suscriptor IOTA. El resto de suscriptores permanece.

La telemetría sigue escribiendo `.SddIA/proofs/dlt-telemetry/` con digest `local-`. No entra en `dlt_audit_seals`.

En `filesystem` el recibo externo sigue en el testigo del evento y la base no se abre. En `dual` o `redb` el sello guarda `tx_hash`, `digest` y `scope: external_public_interest`.

La aceptación (`PullRequest_Merged`) es el ancla externa. Los `Domain_Entity_*` del mismo cambio no salen a IOTA.
