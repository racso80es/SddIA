---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
base: main
scope: runtime-config-hierarchy
---

# Especificación técnica — Jerarquía de configuración de entornos

## 1. Contexto

Hoy los secretos IOTA viven en un `.env` ad hoc junto a la cápsula Node (`SddIA/scripts/tools/iota-immutable-publisher/.env`), ignorado por git pero acoplado físicamente al tool. No existe un contrato federal de dónde colocar configuración local vs compartida del workspace.

La arquitectura SddIA adopta dos planos:

| Plano | Ruta | Semántica |
|-------|------|-----------|
| Global repo | `.dev/.env` | Configuración compartida del clone (equipo, CI local, defaults) |
| Instancia | `.SddIA/.dev/.env` | Overrides soberanos del proyecto (Vía C); prevalece sobre global |

## 2. Contrato del cargador

### 2.1 Módulo `SddIA/scripts/qa/env_loader.py`

| Función | Firma | Comportamiento |
|---------|-------|----------------|
| `parse_dotenv_file` | `(path: Path) → dict[str, str]` | Parseo línea a línea; soporta `#` comentarios; `export KEY=val` opcional |
| `load_hierarchical_env` | `(repo: Path) → dict[str, str]` | Orquesta merge y aplicación |
| `apply_env` | `(merged: dict[str, str]) → None` | `os.environ.setdefault(k, v)` por cada par |

### 2.2 Algoritmo de merge

```
merged = {}
if exists(repo / ".dev" / ".env"):
    merged.update(parse(repo / ".dev" / ".env"))
if exists(repo / ".SddIA" / ".dev" / ".env"):
    if both_exist:
        log_stderr("[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env")
    merged.update(parse(repo / ".SddIA" / ".dev" / ".env"))
apply_env(merged)  # setdefault — respeta SO
return merged
```

### 2.3 Invariantes

- **I1:** Idempotente — invocar dos veces no cambia resultado.
- **I2:** Agnóstico de claves — ningún `if key == "IOTA_*"` en el loader.
- **I3:** Sin efectos en stdout — logs solo stderr.
- **I4:** Fallo de parseo — excepción con ruta del fichero y línea (fail-fast).

## 3. Integración en entrypoints

### 3.1 `execute-process.py`

Insertar **inmediatamente después** de resolver `repo = repo_root()` y **antes** de `run_process` / `shim_execute_action`:

```python
from env_loader import load_hierarchical_env
load_hierarchical_env(repo)
```

### 3.2 `execute-action.py`

Misma llamada al inicio de `main()`, tras `repo_root()`.

### 3.3 `event-watcher.py`

Tras resolver `REPO_ROOT`, antes del bucle de consumo.

### 3.4 Subprocesos

Los entrypoints que ya cargaron el entorno propagan variables vía `os.environ.copy()` existente en `execute_process_capsules.shim_execute_action` y lanzadores de cápsulas — **sin** re-invocar loader en hijos.

## 4. Migración iota-immutable-publisher

### 4.1 Cambios en `index.ts`

- Eliminar líneas 1 y 10 (`import dotenv`, `dotenv.config(...)`).
- Mensajes de error referencian `.SddIA/.dev/.env`.

### 4.2 Variables esperadas (sin cambio semántico)

| Variable | Uso |
|----------|-----|
| `IOTA_WALLET_SECRET` | Clave privada / mnemonic |
| `IOTA_ANCHOR_PACKAGE_ID` | Opcional; cache de package publicado |
| `SDDIA_IOTA_TIMEOUT_SECONDS` | Leída por `event-watcher.py` (Python), no por cápsula |

### 4.3 Deprecación

Documentar en `implementation.md` que operadores deben **mover** contenido de `SddIA/scripts/tools/iota-immutable-publisher/.env` → `.SddIA/.dev/.env`. El fichero legacy puede borrarse localmente tras migración.

## 5. Topología SSOT

Añadir a `SddIA/core/cumulo.paths.json`:

```json
"env_hierarchy": {
  "global": ".dev/.env",
  "instance": ".SddIA/.dev/.env"
}
```

## 6. `.gitignore`

Reemplazar entrada específica de IOTA por:

```
# Configuración local jerárquica (secretos)
.dev/
.SddIA/.dev/
```

Mantener `node_modules/` y artefactos IOTA existentes.

## 7. Plantilla starter-kit

Archivo: `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example`

```dotenv
# Instancia local — prevalece sobre .dev/.env en la raíz del repo
# IOTA_WALLET_SECRET=<hex-o-mnemonic>
# IOTA_ANCHOR_PACKAGE_ID=
# SDDIA_LAB_SIMULATE_IOTA=0
```

## 8. Criterios de aceptación (Argos)

| ID | Check |
|----|-------|
| CA-1 | Unit smoke: merge local sobre global en dict intermedio |
| CA-2 | SO env no sobrescrito por fichero |
| CA-3 | Log exacto cuando ambos ficheros existen |
| CA-4 | `execute-process.py --process …` arranca sin error sin ficheros |
| CA-5 | IOTA cápsula funciona con secretos solo en `.SddIA/.dev/.env` |
| CA-6 | Cúmulo + gitignore + tool.md alineados |
| CA-7 | Cero referencias activas a `iota-immutable-publisher/.env` como SSOT |

## 9. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Operadores con `.env` legacy IOTA | Nota migración en `execution.md` |
| Daemon watcher ya en ejecución | Reinicio manual post-deploy |
| Laboratorios Vía C sin `.SddIA/` | Solo global `.dev/.env` — válido |
