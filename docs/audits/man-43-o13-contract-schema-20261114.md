---
uuid: "7d1f4b2a-3c8e-4a95-b6d0-9e2f1a3c5b78"
name: man-43-o13-contract-schema
version: "1.0.0"
audit_date: "2026-11-14"
scope: "O13: baseline limpia + clase capability-contract + C-3 + identidad x-sddia de 17 contratos + Norma de Identidad Polimórfica"
verdict: "APTO_PARCIAL"
---

# MAN-43 O13 — capability-contract, C-3 e identidad de los 17 contratos

## Veredicto

**APTO_PARCIAL.** C-3 cerrado, regresión O10 y baseline O12 remediadas, 17 contratos con identidad. `APTO_TOTAL` sigue reservado a la re-auditoría AC-13 (O14).

## Norma de Identidad Polimórfica (laudo 2026-11-14)

Aplicada **antes** de ejecutar: `.cursorrules` §2 y §3, `README.md` § «Cicatriz digital y estándar atómico». Registro de evolución del estándar: `SddIA/evolution/35ec5355-a3c8-48be-9358-336026684320.md` (hash anclado con `sddia-qa evolution-rehash`; fila en `Evolution_log.md`).

Validación del laudo (detalle en `plan.md` § O13): incoherencias detectadas y resueltas: README contradecía la norma; §3 seguía marcando `.schema.json` como fósil; `entity_name` = stem del fichero; `id`/`type` derivados del sustrato B; orden `adopt` → parche (sello `Created` antes de `Updated`).

## Baseline (entorno `SDDIA*` vacío)

| Comprobación | Resultado | Evidencia |
|--------------|-----------|-----------|
| Suite lib tras corregir fixtures y hermetizar `tier_dispatch` | **824 passed; 0 failed** | `man-43-o13-baseline-exec-process-lib.log` |
| Suite lib final (con forja, paridad, identidad) | **849 passed; 0 failed; 1 ignored** | `man-43-o13-final-exec-process-lib.log` |
| `cargo check --workspace` | OK | `man-43-o13-check-workspace.log` |
| `gate-event-manager-port` | OK (214 ficheros) | consola |
| `gate-evolution` (árbol) | `EVOL_OK` | consola |
| `gate-evolution --range` (base `origin/main` **stale**, 3 días) | O13: **0 rutas O13 sin correlar** tras ampliar `relacionado` (commit `6fbbfda9`). Gate global **no verde**: 123 hallazgos `EVOL_MATERIAL_UNREGISTERED` de rutas de olas O1–O12 de la rama, previos a O13 | consola |

Regresión O10 (3 tests del validador DI) y causa del 4.º fallo corregidas; `man-43-o12-baseline-20261114.md` rectificada.

## Forja `capability-contract`

Módulo `forges/capability_contract.rs`; clase solo nativa en `entity_manager.rs` (sin creator ni fallback); clase añadida al sello en `domain_mutation.rs`. `create`, `update` por `adopt` (inserción textual idempotente, reversible byte a byte) o `schema_json_patch` (RFC 6902 `add|replace|remove`, no puede tocar `/x-sddia`, compila antes de escribir, `schema_version` debe cambiar, detecta deriva de hash), `delete` = error. 16 tests de forja + 2 de `entity_manager`.

## C-3: oráculo de paridad

`eda_contract_parity` (execute-process): peticiones mínimas construidas con los tipos de `sddia-core`, serializadas como el adaptador real (`{operation, payload}`); enum del schema == operaciones del trait (leídas de `port.rs`, 19).

- **Rojo antes del parche** (`man-43-o13-parity-red.log`): 3 de 6 tests fallaban.
- **Verde después**: 6 de 6.
- **Verificación semántica independiente** (Python, sin código del motor): `canon(antes) + parche == canon(después)`: `True`.

**Hallazgos del oráculo no previstos en el plan** (todos aditivos, en el mismo parche):

1. `record_delivery.status` no admitía `processing`, que sí existe en `DeliveryStatus`.
2. Faltaban ramas `if/then` para `list_events`, `list_deliveries`, `list_progress`, `verify_inline_ecst`, `seal_dlt`, `get_dlt_seal` (además de `list_terminal_events`).
3. **Error del plan corregido antes de implementar:** el plan fijaba `event_uri` como cadena `eda://…` con `pattern`. `EventUri` deriva `Serialize` y viaja como **objeto** `{family, event_id}` (`ExternalCapsuleEventAdapter` hace `serde_json::to_value(&req)`); la cadena no se deserializa. El schema describe el objeto.

## Identidad de los 17 contratos

Una invocación `entity-manager` por contrato (`_lab/o13-adopt-all.sh`; versiones leídas del catálogo). 16 contratos: **+5 líneas, 0 borradas** cada uno (solo el bloque `x-sddia`); `eda.event_manager.v1`: adoptado a 1.2.0 (`Domain_Entity_Created`) y parcheado a 2.0.0 (`Domain_Entity_Updated`). Guardián permanente `all_repo_capability_contracts_carry_valid_identity`: 17 compilan, uuid v4 únicos, SemVer, hash recalculable, versión = catálogo (15 filas); auxiliares `di.binding` y `llm-registry` (1.0.0) excluidos de forma explícita y comprobada. En rojo antes de adoptar (`man-43-o13-identity-red.log`).

Alineación de enum de clases (`entity_class`) por `entity-manager`: `emit-domain-mutation` 1.1.1 y `entity-manager` 1.0.3 (+ fila de la tabla de delegación). Cierra además la deriva `suite`/`daemon` que arrastraban.

Ensayo previo en copia del repo (`/tmp`) de forja, patch, enums y las 16 adopciones antes de tocar el genoma.

## DA-2 / DA-5

Sin `Write`/`StrReplace` bajo `SddIA/{library,skills,tools,daemons,process,actions,norms}/`: todo cambio de genoma por `./sddia-run.sh --process entity-manager`. Excepción de forma: `SddIA/evolution/` (no genoma; no pasa por `entity-manager` según su contrato) con hash por la CLI canónica. Sin `sleep`/polling tras acuse.

## Residual (decisión del Vértice Biológico o O14)

| # | Residual |
|---|----------|
| 1 | `evolution_contract.md` §1 sigue exceptuando el estándar atómico para `evolution/` (debilita «sin excepciones»). |
| 2 | Deuda previa del sustrato A: ningún `.md` del genoma lleva `id`/`type` (usan `name`). |
| 3 | `DualVersionManifest::builtin_v2` apunta `schema_uri` a `eda.event_manager.v2.schema.json`, que no existe (el fichero es `…v1…`, `$id` v1). |
| 4 | `capability-taxonomy` dice «`{contract}.schema.json`»; eda es la excepción (`…v1`, resuelta por `contract_schema` del binding). |
| 5 | `dlt_threshold_ok` no incluye `daemon` ni `capability-contract`: sus `Domain_Entity_Created` no se anclan en DLT (`skipped-dlt-threshold`, no falla). |
| 6 | Guardián de identidad solo corre en la suite; candidato a gate permanente en O14. |
| 7 | `SddIA/core/eda-coverage.json` acumula estado de ejecución (contratos incluidos); fuera del commit, como en O11/O12. |
| 8 | `result` por operación del schema eda: fuera de alcance. |
| 9 | `gate-evolution --range`: 123 hallazgos de olas anteriores sin registro correlado; requieren registro(s) de evolución de la rama antes del push/PR (no se inventan aquí). |
