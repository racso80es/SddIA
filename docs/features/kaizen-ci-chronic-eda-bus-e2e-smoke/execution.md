---
feature_name: kaizen-ci-chronic-eda-bus-e2e-smoke
created: "2026-10-21"
process: feature
items_applied:
  - exclude-event-bus-audit
  - local-discovery
  - wasi-build-exit-0
branch_name: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
persist_ref: docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke
execution_id: "0db56ca9-0fe4-4e81-96cd-88f5c52fcdd4"
document_id: PBI-KAIZEN-CI-CHRONIC-EDA-BUS-E2E-SMOKE
---

# Ejecución — kaizen-ci-chronic-eda-bus-e2e-smoke

## Descubrimiento (CA1)

La misma selección que el script, con `event-bus-audit` ya en `EXCLUDE`, devuelve 23 paquetes. `event-bus-audit` no está en la lista.

## Script local (CA2)

`SddIA/scripts/qa/build-wasi-capsules.sh` salió 101. No aparece `redb` ni `E0554`. El fallo es el build-script de `ring` 0.17.14: `failed to find tool "clang"`. Este host no tiene `clang` en `PATH`. El runner de CI sí lo tiene: los logs rojos de #356 y #358 compilaron `redb` después de `ring`.

`cargo metadata --filter-platform wasm32-wasip1`: el único dependiente directo de `redb` 2.6.4 es `sddia-event-store`. Ese crate no entra en los 23 paquetes descubiertos.

Segunda pasada con `CARGO_TARGET_DIR=SddIA/target`: el mismo exit 101 de `ring`. `sudo apt install clang` no corre: sudo pide contraseña.

Tercera pasada, `clang` 18.1.3 ya en el host, mismo `CARGO_TARGET_DIR`:

```json
{"success":true,"exitCode":0,"message":"wasi capsules built","result":{"target":"wasm32-wasip1","package_count":23}}
```

`Finished dev profile` en 21.49s. La salida no contiene `redb` ni `could not compile redb`.

## Evolution

`sddia-qa evolution-register` operación `alta`. `id_cambio` `4a6499e4-1a65-4aed-bd9f-e1be6b51b196`. `hash_integrity` `sha256:98edc8aadf3626ea436ccf18485e3c4614d804ba8c620975d0c53d30ed865dad`. El primer `delivery-close-cycle` dejó el PR [#360](https://github.com/racso80es/SddIA/pull/360) y bloqueó en `EVOL_MATERIAL_UNREGISTERED` sobre `build-wasi-capsules.sh`.
