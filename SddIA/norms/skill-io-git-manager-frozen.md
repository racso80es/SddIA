---
uuid: "b2c3d4e5-f6a7-4890-b123-456789abcdef"
name: "skill-io-git-manager-frozen"
version: "1.2.0"
entity_type: "norm"
jurisdiction: "cerbero"
freeze_status: "congelado"
applies_to_skill: "git-manager"
schema_version: "2026-05-07"
---

# Esquema de entrada congelado — `git-manager`

**Estado:** Congelado. Cerbero y Argos deben rechazar cualquier `operation_type` o claves de `operation_payload_json` no listadas aquí.

**Alcance de ejecución:** Solo el binario **`git`** nativo del sistema. Ningún otro ejecutable (p. ej. `gh`, `npm`) puede invocarse a través de esta skill.

**SemVer 1.2.0 (PBI-KAIZEN-GIT-DIFF-LLM-SYNTHESIS):** añade `commit_summary` (subject Git + names-only first-parent acotado). Homónimo `remote`: string en `push`/`pull`/`fetch` ≠ boolean en `delete_branch` — no unificar.

## 1. Raíz del mensaje (stdin JSON)

| Campo | Tipo | Obligatorio | Descripción |
| :--- | :--- | :---: | :--- |
| `operation_type` | `string` (enum) | Sí | Uno de los valores del §2. |
| `repository_path` | `string` | Sí | Ruta absoluta del repositorio, **resuelta previamente por Cúmulo** (sin path traversal). |
| `operation_payload_json` | `object` | Sí | Carga según `operation_type`; claves extra quedan prohibidas salvo evolución explícita del presente documento. |

## 2. `operation_type` (enum estricto)

Valores permitidos, en minúsculas y exactamente:

`status` · `checkout` · `commit` · `push` · `pull` · `fetch` · `branch_list` · `get_last_commit` · `merge` · `delete_branch` · `diff_name_only` · `commit_summary`

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

### 3.8 `get_last_commit`

```json
{
  "ref": "string"
}
```

| Clave | Tipo | Obligatorio |
| :--- | :--- | :---: |
| `ref` | `string` | Sí |

### 3.9 `merge`

```json
{
  "branch_name": "string",
  "no_ff": true
}
```

| Clave | Tipo | Obligatorio |
| :--- | :--- | :---: |
| `branch_name` | `string` | Sí |
| `no_ff` | `boolean` | Sí |

### 3.10 `delete_branch`

```json
{
  "branch_name": "string",
  "remote": false,
  "force": false
}
```

| Clave | Tipo | Obligatorio | Notas |
| :--- | :--- | :---: | :--- |
| `branch_name` | `string` | Sí | Nombre de rama a eliminar |
| `remote` | **boolean** | Sí | `false` → `git branch -d/-D`; `true` → `git push origin --delete`. **No** es el string `"origin"`. |
| `force` | `boolean` | Sí | Solo afecta delete local (`-D` vs `-d`) |

### 3.11 `diff_name_only`

```json
{
  "ref_spec": "string"
}
```

| Clave | Tipo | Obligatorio |
| :--- | :--- | :---: |
| `ref_spec` | `string` | Sí |

### 3.12 `commit_summary`

```json
{
  "ref": "string",
  "max_files": 30,
  "max_subject_chars": 200
}
```

| Clave | Tipo | Obligatorio | Notas |
| :--- | :--- | :---: | :--- |
| `ref` | `string` | Sí | Token seguro. Típicamente hash 40 hex. |
| `max_files` | `integer` | Sí | 1–30 inclusive. |
| `max_subject_chars` | `integer` | Sí | 1–200 inclusive. |

Salida `data` (además de `gitStdout` / `gitStderr` / `errorSummary`): `commitHash` (40 hex), `subject` (primera línea Git `%s`, recortada), `files` (rutas, ≤ `max_files`), `totalFilesChanged` (antes de truncar), `truncated` (boolean).

Ejecución: `rev-parse --verify <ref>`; `show -s --format=%s <ref>`; first-parent `diff --name-only -M <ref>^ <ref>` tras verificar `<ref>^`. Si el padre no existe (root/shallow): `success: false`. Prohibido parche unificado y `git diff` working-tree.

## 4. Referencias

- Resolución de rutas normativas: `SddIA/core/cumulo.paths.json` → `directories.norms` / `normative_documents.skill_io_git_manager_frozen`.
- Política de nombres de rama y convenciones: `SddIA/norms/git-operations.md`.
- Orquestación de PRs (no atómica): `SddIA/norms/pull-request-orchestration.md`.
