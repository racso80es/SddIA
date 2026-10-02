---
uuid: "c8d9e0f1-a2b3-4c5d-8e9f-0a1b2c3d4e5f"
name: "skill-io-filesystem-manager-frozen"
version: "1.0.0"
entity_type: "norm"
jurisdiction: "cerbero"
freeze_status: "congelado"
applies_to_skill: "filesystem-manager"
schema_version: "2026-09-28"
---

# Esquema de entrada congelado — `filesystem-manager`

## 1. Raíz (stdin JSON)

| Campo | Tipo | Obligatorio | Descripción |
| :--- | :--- | :---: | :--- |
| `operation` | string (enum) | Sí | §2 |
| `target_path` | string | Sí | Relativo a `workspace_root`, sin `..` |
| `content` | string | Condicional | `WRITE_FILE`, `PATCH_FILE` |
| `destination_path` | string | Condicional | `MOVE_FILE` |
| `workspace_root` | string | No | Absoluto; si falta: `SDDIA_WORKSPACE_ROOT` o repo Core |

## 2. `operation` (enum)

`READ_FILE` · `WRITE_FILE` · `LIST_DIR` · `DELETE_FILE` · `CREATE_DIR` · `MOVE_FILE` · `PATCH_FILE`

## 3. Salida

`success`, `exitCode`, `data`, `error` (contrato `sddia-io` + skill `outputs`).
