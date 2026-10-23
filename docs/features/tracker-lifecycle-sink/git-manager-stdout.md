---
feature_name: tracker-lifecycle-sink
created: "2026-10-23"
process: feature
head: 1404be61e0b534bf4d81236daba86008d9dd2750
---

# Stdout — git-manager

`./sddia-run.sh --tool git-manager` operación `status`. Exit 0. `gitStdout` vacío (árbol limpio).

```json
{"exitCode":0,"result":{"data":{"errorSummary":null,"gitStderr":"","gitStdout":""},"exitCode":0,"success":true},"success":true}
```

Operación `branch_list`. Exit 0. La línea actual es `* main` @ `1404be61`. No hay ref local `feat/tracker-lifecycle-sink`: el sumidero entró por el merge del PR #367 y la documentación por el PR #371.
