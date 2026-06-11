---
feature_name: snapshot-friccion-laboratorio-jules
process: feature
created: "2026-06-11"
branch_name: feat/snapshot-friccion-laboratorio-jules
persist_ref: docs/features/snapshot-friccion-laboratorio-jules
base: main
scope: cierre-pbi-snapshot-friccion-jules
---

# Especificación — Cierre Snapshot Fricción Laboratorio Jules

## 1. Propósito

Materializar el remanente del PBI **Snapshot de Fricción** tras el incidente Jules (Raw Kernel + colapso del laboratorio). La feature cierra tres gaps operativos y archiva el manifiesto con trazabilidad documental completa.

## 2. Fronteras

| In scope | Out of scope |
|----------|--------------|
| Failsoft Git offline en `git-manager` | Re-migración WASI |
| Norma RAW KERNEL → feature init | Reimplementar hooks Husky |
| Skill `intent-transpiler` (forja vía entity-manager) | PoC wasm32-wasi (cerrado) |
| Archivo PBI + `validacion.md` | Poda total `requirements.txt` |

## 3. Arquitectura — O1 Git failsoft

```text
invoke_git_manager(repo, op, payload)
    └── wasmtime run git-manager.wasm
            └── on network/auth failure → envelope JSON (success: false, offline: true)
    └── fallback native git-manager.py (misma semántica)
```

**Contrato ampliado (respuesta error tolerable):**

```json
{
  "success": false,
  "exitCode": 0,
  "data": {
    "offline": true,
    "gitStderr": "fatal: could not read Username...",
    "errorSummary": "remote unavailable — local mode"
  }
}
```

El orquestador **no** eleva `RuntimeError` cuando `offline: true` y la operación es `fetch`/`pull` en contexto lab; registra warning en `git_steps`.

## 4. Arquitectura — O2 Raw Kernel gate

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/norms/external-ai-constraints.md` | Nueva directriz DA-4: prefijo RAW exige `persist_ref` o `--process feature` previo |
| Procesos `*-creator.md` | Alinear prefijo canónico con DA-4 |
| `.cursor/rules/` o touchpoint | Difundir DA-4 (sin duplicar norma completa) |

## 5. Arquitectura — O3 intent-transpiler

```text
Vértice Biológico (texto libre)
    └── skill:intent-transpiler (stdin JSON)
            └── stdout: structured_directive + target_paths + required_process
    └── Tekton / execute-process (solo tras persist_ref válido)
```

**Entidad:** `SddIA/skills/intent-transpiler.md` — forja exclusiva vía `entity-manager`.

## 6. Criterios de aceptación (S+ Grade)

| ID | Criterio | Verificación |
|----|----------|--------------|
| CA-1 | `fetch` sin credenciales no aborta `workspace-init` en rama local existente | Smoke lab sin red / credenciales inválidas |
| CA-2 | `external-ai-constraints.md` declara DA-4 RAW → feature | Diff norma + grep en creators |
| CA-3 | `intent-transpiler.md` en catálogo skills + contrato I/O | `entity-manager` + `skills/index.md` |
| CA-4 | PBI en `docs/todos/done/` | Path en rama feature |
| CA-5 | `validacion.md` APTO, `pbi_archived: true` | Argos / verify-task-closure |
| CA-6 | `verify-process-integrity.py` exit 0 | CI local |

## 7. Dependencias

- `docs/features/ia-obrera-blindaje/` — base normativa
- `docs/features/migracion-rust-wasi/clarify.md` §D8 — excepción git-manager Python/WASM
- `docs/features/husky-pre-push-blocking-route/` — aduana física ya materializada

## 8. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Failsoft enmascara errores reales de Git | Solo activo para errores de red/auth catalogados; otros errores mantienen `exitCode > 0` |
| Transpilador alucina rutas | Salida acotada a paths resueltos vía `cumulo.paths.json` |
| Scope creep WASI | Frontera explícita en `clarify.md` D2 |
