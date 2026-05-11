---
uuid: "b2c4e6d8-1a3f-4e5b-9c7d-8e1f2a3b4c5d"
name: "motor-capsule-execution-boundaries"
version: "1.0.0"
nature: motor
contract: "security-contract v1.0.0"
---

# Límite de ejecución y aislamiento de cápsulas

## Objetivo
Impedir que la orquestación invoque binarios o scripts fuera de las rutas canónicas resueltas tras la fusión `cumulo.paths.json` + `.SddIA/local.paths.json`.

## Reglas
1. Toda invocación de skill/tool debe usar `paths.skillCapsules` / `paths.toolCapsules` (o equivalentes post-fusión).
2. Entradas que contengan concatenación de rutas desde texto libre del usuario deben validarse contra allowlists.
3. Tekton y agentes obreros no deben propagar credenciales o secretos a logs de cápsula.
