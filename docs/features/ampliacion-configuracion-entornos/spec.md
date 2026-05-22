---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
base: main
scope: ola-a-hito-0-vault-hierarchy
updated: "2026-05-22"
---

# Especificación técnica — Jerarquía de Bóvedas

## 1. Contexto estratégico

La Ola A amplía su alcance operativo con la **Jerarquía de Bóvedas**: contrato federal de dónde residen secretos y configuración local vs global. Esta fase es **Hito 0** y precede a cualquier resolución de pasivo técnico pendiente.

Estado actual: secretos IOTA en `.env` ad hoc junto a `iota-immutable-publisher/`, acoplados físicamente a la cápsula.

| Bóveda | Ruta | Semántica |
|--------|------|-----------|
| Global | `.dev/.env` | Configuración compartida del clone |
| Instancia | `.SddIA/.dev/.env` | Overrides soberanos (Vía C); prevalece sobre global |

## 2. Hito 0.1 — Contrato del cargador

### 2.1 Módulo `SddIA/scripts/qa/env_loader.py`

| Función | Firma | Comportamiento |
|---------|-------|----------------|
| `parse_dotenv_file` | `(path: Path) → dict[str, str]` | `KEY=VALUE`, comentarios `#`, `export` opcional |
| `load_hierarchical_env` | `(repo_root: Path) → dict[str, str]` | Orquesta merge global → local |
| `apply_env` | `(merged: dict[str, str]) → None` | `os.environ.setdefault(k, v)` |

### 2.2 Algoritmo

```
merged = {}
if exists(repo_root / ".dev" / ".env"):
    merged.update(parse(global))
if exists(repo_root / ".SddIA" / ".dev" / ".env"):
    if global_exists and local_exists:
        log_stderr("[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env")
    merged.update(parse(local))   # local sobrescribe global en dict
apply_env(merged)
return merged
```

### 2.3 Invariantes

- **I1:** Idempotente.
- **I2:** Agnóstico de claves.
- **I3:** Logs solo stderr.
- **I4:** Parseo fail-fast (ruta + línea).

## 3. Hito 0.2 — Integración entrypoints

### 3.1 `execute-process.py` (puerta CLI)

```python
repo = repo_root()
load_hierarchical_env(repo)   # antes de run_process / shim_execute_action
```

### 3.2 `execute_process_capsules.py` (núcleo pre-cápsula) — **obligatorio**

Al inicio de `run_process(repo, process_name, process_inputs)`:

```python
load_hierarchical_env(repo)   # antes de load_process_def, fases, invocaciones
```

Garantiza env cuando el intérprete se usa como biblioteca o vía rutas indirectas.

### 3.3 Entrypoints autónomos (complementarios)

| Archivo | Punto de inserción |
|---------|-------------------|
| `execute-action.py` | Inicio `main()`, tras `repo_root()` |
| `event-watcher.py` | Tras `REPO_ROOT`, antes del bucle |

### 3.4 Propagación a subprocesos

- `shim_execute_action`, lanzadores skill/tool: `env = os.environ.copy()` — **sin** re-invocar loader.
- Cápsula Node IOTA: hereda variables del proceso Python padre.

### 3.5 `iota-immutable-publisher/index.ts`

| Cambio | Detalle |
|--------|---------|
| Eliminar | `import dotenv`, `dotenv.config({ path: join(__dirname, ".env") })` |
| Consumir | `process.env.IOTA_WALLET_SECRET`, `process.env.IOTA_ANCHOR_PACKAGE_ID` |
| Errores | Referenciar `.SddIA/.dev/.env` |

## 4. Hito 0.3 — Sanitización

### 4.1 Inventario a eliminar / deprecar

| Ruta legacy | Acción |
|-------------|--------|
| `SddIA/scripts/tools/iota-immutable-publisher/.env` | Deprecar; operador migra a bóveda instancia |
| Cualquier `dotenv.config` en `scripts/tools/` | Eliminar |

### 4.2 `.gitignore` (estado objetivo)

```gitignore
# Jerarquía de Bóvedas — secretos locales
.dev/
.SddIA/.dev/
```

Retirar entrada puntual:

```gitignore
# ELIMINAR:
SddIA/scripts/tools/iota-immutable-publisher/.env
```

### 4.3 Verificación automatizable

```bash
rg 'dotenv\.config|path\.join\(__dirname,\s*["'']\.env' SddIA/scripts/tools/
# expect: 0 matches operativos
rg '^\.dev/|^\.SddIA/\.dev/' .gitignore
# expect: both present
```

## 5. Topología SSOT (`cumulo.paths.json`)

```json
"env_hierarchy": {
  "global": ".dev/.env",
  "instance": ".SddIA/.dev/.env"
}
```

## 6. Plantilla starter-kit

`SddIA/scripts/starter-kit/.SddIA/.dev/.env.example`:

```dotenv
# Bóveda instancia — prevalece sobre .dev/.env
# IOTA_WALLET_SECRET=
# IOTA_ANCHOR_PACKAGE_ID=
# SDDIA_LAB_SIMULATE_IOTA=0
```

## 7. Criterios de aceptación (Argos)

| ID | Check | Hito |
|----|-------|------|
| CA-1 | Merge local > global en dict | 0.1 |
| CA-2 | SO no sobrescrito | 0.1 |
| CA-3 | Log exacto con ambos ficheros | 0.1 |
| CA-4 | `execute-process.py` sin bóvedas → OK | 0.2 |
| CA-5 | `run_process()` carga env (capsules) | 0.2 |
| CA-6 | IOTA funciona con bóveda instancia only | 0.2 |
| CA-7 | Cero dotenv local en tools | 0.3 |
| CA-8 | `.gitignore` bóvedas verificado | 0.3 |
| CA-9 | Cúmulo `env_hierarchy` válido | 0.3 |

## 8. Gate Ola A

| Condición | Efecto |
|-----------|--------|
| CA-1…CA-9 APTO | Desbloquea pasivos técnicos Ola A restantes |
| NO APTO | Bloqueo de hooks Hito 3, deuda CLI y faenas env-dependent |

## 9. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| `.env` legacy IOTA en operadores | Guía migración en `execution.md` |
| Watcher en ejecución | Reinicio post-deploy |
| Doble carga CLI + capsules | Idempotencia I1 |
