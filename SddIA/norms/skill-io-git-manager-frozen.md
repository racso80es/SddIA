---
uuid: "b2c3d4e5-f6a7-4890-b123-456789abcdef"
name: "skill-io-git-manager-frozen"
version: "1.0.0"
entity_type: "norm"
jurisdiction: "cerbero"
freeze_status: "congelado"
applies_to_skill: "git-manager"
schema_version: "2026-05-07"
---

# Esquema de entrada congelado — `git-manager`

**Estado:** Congelado antes de la forja física de la cápsula. Cerbero y Argos deben rechazar cualquier `operation_type` o claves de `operation_payload_json` no listadas aquí.

**Alcance de ejecución:** Solo el binario **`git`** nativo del sistema. Ningún otro ejecutable (p. ej. `gh`, `npm`) puede invocarse a través de esta skill.

## 1. Raíz del mensaje (stdin JSON)

| Campo | Tipo | Obligatorio | Descripción |
| :--- | :--- | :---: | :--- |
| `operation_type` | `string` (enum) | Sí | Uno de los valores del §2. |
| `repository_path` | `string` | Sí | Ruta absoluta del repositorio, **resuelta previamente por Cúmulo** (sin path traversal). |
| `operation_payload_json` | `object` | Sí | Carga según `operation_type`; claves extra están prohibidas salvo evolución explícita del presente documento. |

## 2. `operation_type` (enum estricto)

Valores permitidos, en minúsculas y exactamente:

`status` · `checkout` · `commit` · `push` · `pull` · `fetch` · `branch_list`

## 3. `operation_payload_json` por operación

### 3.1 `status`

```json
{}
```

Objeto vacío. Sin claves adicionales.

### 3.2 `branch_list`

```json
{}
```

Objeto vacío. Sin claves adicionales.

### 3.3 `checkout`

```json
{
  "branch_name": "string",
  "create_if_not_exists": false
}
```

| Clave | Tipo | Obligatorio |
| :--- | :--- | :---: |
| `branch_name` | `string` | Sí |
| `create_if_not_exists` | `boolean` | Sí |

La validación de que el nombre cumple política de ramas **no** es responsabilidad de `git-manager`; ver `git-operations.md`.

### 3.4 `commit`

```json
{
  "message": "string",
  "files": ["array", "of", "strings"]
}
```

| Clave | Tipo | Obligatorio |
| :--- | :--- | :---: |
| `message` | `string` | Sí |
| `files` | `string[]` | Sí (puede ser array vacío si la política de staging lo permite en implementación; si no, Cerbero debe bloquear antes). |

### 3.5 `push`

```json
{
  "remote": "string",
  "branch": "string",
  "force": false
}
```

| Clave | Tipo | Obligatorio |
| :--- | :--- | :---: |
| `remote` | `string` | Sí |
| `branch` | `string` | Sí |
| `force` | `boolean` | Sí |

### 3.6 `pull`

```json
{
  "remote": "string",
  "branch": "string"
}
```

| Clave | Tipo | Obligatorio |
| :--- | :--- | :---: |
| `remote` | `string` | Sí |
| `branch` | `string` | Sí |

### 3.7 `fetch`

```json
{
  "remote": "string",
  "prune": false
}
```

| Clave | Tipo | Obligatorio |
| :--- | :--- | :---: |
| `remote` | `string` | Sí |
| `prune` | `boolean` | Sí |

## 4. Referencias

- Resolución de rutas normativas: `SddIA/core/cumulo.paths.json` → `directories.norms`.
- Política de nombres de rama y convenciones: `SddIA/norms/git-operations.md`.
- Orquestación de PRs (no atómica): `SddIA/norms/pull-request-orchestration.md`.
