---
feature_name: start-sddia-native-contract
created: "2026-07-13"
process: bug-fix
phases:
  - diagnose-profile-drift
  - enforce-native-contract
  - validate-startup
---

# Plan — contrato nativo de `start-sddia`

1. Comparar referencias Python y fechas de artefactos debug/release.
2. Unificar la selección debug-first y validar que cada ejecutable sea ELF.
3. Probar el gate de centinelas obligatorios, los overrides no nativos y la ignición completa.
4. Archivar PBI y abrir un único PR con validación APTO.
