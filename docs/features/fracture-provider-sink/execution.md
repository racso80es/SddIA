---
feature_name: fracture-provider-sink
created: "2026-10-30"
process: refactorization
items_applied:
  - T1
  - T2
tracker_ref: AUT-30
branch: refactor/fracture-provider-sink
---

# Ejecución

## T1

Forjado. El test `self_hosted_linear_ignores_orphan_markdown` crea `LAB-*` sin slug, no añade markdown y el segundo pase es `already_open`. `unconfigured_does_not_write_markdown` no deja ficheros.

## T2

Forjado. Enrich de fichero sigue emitiendo `Fracture_Clarification_Requested` cuando el caso es unclassified. El deny de dominio deposita `System_Fracture_Detected`. `event-watcher` compila.

## Nota de laboratorio

`cargo test` en sandbox no carga la bóveda: el store por defecto es redb y los tests que cuentan JSON en `.events/` necesitan `SDDIA_EVENT_STORE=filesystem`. El ELF de `linear-tracker-adapter` hay que copiarlo a `SddIA/target/debug/` si el target de Cargo queda en la caché del sandbox.
