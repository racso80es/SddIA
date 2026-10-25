---
feature_name: ci-integrity-protoc-jobs
created: "2026-10-25"
process: bug-fix
items:
  - SddIA/scripts/qa/stage-native-debug-bins.sh
  - .github/workflows/sddia-index-qa.yml
execution_id: "1d05f105-00ba-4ecd-b2ac-f295ff6ca047"
---
# Implementación

`native-build` sigue compilando el workspace. El paso nuevo copia los ejecutables debug descubiertos y el artefacto deja de enumerar cuatro rutas. La descarga en `SddIA/target/debug` sigue siendo la que ya usan integridad e iota.

`eda-bus-e2e-smoke` instala el mismo par `protobuf-compiler` / `libprotobuf-dev` que `native-build`, justo después del checkout. Los jobs que ya estaban en verde no cambian de pasos, salvo el contenido del artefacto que consumen: ficheros de más en `target/debug`, los cuatro que ejecutan siguen en la raíz del artefacto.
