---
feature_name: sddia-domain-abstract-03-relocalizacion
created: "2026-08-09"
process: refactorization
base: main
scope: sddia-domain-abstract-03-relocalizacion
branch_name: feat/sddia-domain-abstract-03-relocalizacion
persist_ref: docs/features/sddia-domain-abstract-03-relocalizacion
pbi_ref: docs/todos/done/[REFACTOR] PBI-SDDIA-DOMAIN-ABSTRACT-03 — Relocalización física process software.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-03
parent_document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
version_spec: "1.0.0"
status: dedalo_locked
laudo: L-PACK-MULTIROOT-SIX-MOVE
agents: dedalo
correlation_id: ""
---

# Especificación — sddia-domain-abstract-03-relocalizacion

## 1. Misión técnica

Cerrar **AC-MOVE** diferido de ABSTRACT-02: relocalizar físicamente los process de membresía `codex-software-engineering` fuera de `directories.process` (Core) hacia packing del códice, con **resolución multi-root** del orquestador demostrada **antes** del move (L-RESOLVE-FIRST). Gate `DOMAIN_AUTHORITY_DENIED` intacto. Creators / entity-manager / routes / daemons **no** migran.

## 2. Laudos Dedalo

| Ref | Pregunta | Laudo |
|-----|----------|-------|
| **D1** | ¿Destino físico canónico? | **Packing códice (Core library):** `SddIA/library/codexes/codex-software-engineering/process/{name}.md`. No `.SddIA/` como hogar canónico (Vía C inyecta library; instancia solo overlay). |
| **D2** | ¿Ciclo PR se mueve? | **Sí (confirma L-SCOPE-PR).** Mover los **6** de `process_membership`. No evidencia de que deban permanecer Core-agnósticos: dependen de Git/gh/PR y ya están en el códice. |
| **D3** | ¿Mecánica de resolución? | Multi-root vía Cúmulo: array `directories.process_domain_roots` + `directories.process`. Ver §4. Sustituye hardcode `repo.join("SddIA/process")` en `resolve_process_path`. |
| **D4** | ¿Precedencia? | **Domain roots primero** (orden del array), luego Core `directories.process`. Primer hit gana. Overlay `.SddIA/local.paths.json` puede **reemplazar** el array (soberanía instancia; merge objeto de `load_paths_config`). |
| **D5** | ¿Stubs en Core? | **Prohibidos.** Tras move, los 6 `.md` **ausentes** de `SddIA/process/`. Contrato `process-contract.md` permanece en Core. |
| **D6** | ¿Índice? | Quitar filas de los 6 en `SddIA/process/index.md`. Crear `…/codex-software-engineering/process/index.md` con las 6 filas. Nota breve en índice Core: process software-lifecycle → packing códice (sin fichero ejecutable). |
| **D7** | ¿Creators / entity-manager? | **Fuera de move** (L-KEEP-CORE). Deuda conocida: `process-creator` sigue escribiendo bajo `directories.process`. Alta post-move de miembros software → path dominio (manual/forja futura). No bloquea AC de este ciclo. |
| **D8** | ¿Orden de trabajo? | **L-RESOLVE-FIRST innegociable:** T0 resolver+tests → evidencia AC-RESOLVE → T1 move+índice → smokes AC-RUN/TQM → docs. |
| **D9** | ¿Cúmulo version? | Bump `cumulo.paths.json` (p. ej. 1.5.3 → 1.6.0) al introducir `process_domain_roots`. |
| **D10** | ¿Gate ABSTRACT-02? | Sin cambio de contrato. Solo verificar post-move que deny/allow siguen verdes. |

## 3. Destino y conjunto a mover

### 3.1 Path canónico (SSOT)

| Clave Cúmulo | Valor |
|--------------|-------|
| `directories.process` | `SddIA/process` (sin cambio semántico Core) |
| `directories.process_domain_roots` | `["SddIA/library/codexes/codex-software-engineering/process"]` |

### 3.2 Archivos a relocalizar (L-SCOPE-LIFECYCLE + L-SCOPE-PR)

```text
feature.md
bug-fix.md
refactorization.md
pull-request-review.md
accept-pr.md
delivery-close-cycle.md
```

Origen: `{directories.process}/{name}.md` → Destino: `{process_domain_roots[0]}/{name}.md`. Conservar UUID/frontmatter/cuerpo; no re-forjar identidad.

### 3.3 Overlay instancia (opcional, documentar)

Ejemplo `.SddIA/local.paths.json` (reemplaza array completo si se declara la clave):

```json
{
  "directories": {
    "process_domain_roots": [
      ".SddIA/library/codexes/codex-software-engineering/process",
      "SddIA/library/codexes/codex-software-engineering/process"
    ]
  }
}
```

Starter-kit: documentar clave; no obligatorio materializar pack local en este PR.

## 4. Resolución orquestador (`resolve_process_path`)

### 4.1 Algoritmo

```text
cfg = load_paths_config(repo)
roots = []
para cada rel en cfg.directories.process_domain_roots (si array):
  roots.push(repo.join(rel))
roots.push(repo.join(cfg.directories.process || "SddIA/process"))
para cada root en roots:
  si root/{process_name}.md es fichero → OK
  escanear *.md (excl. index, process-contract):
    name == process_name → OK
    aliases ∋ process_name → OK
  (canónico gana sobre alias dentro del mismo root)
fail: "Proceso no encontrado: {process_name}"
```

### 4.2 Touchpoints motor (mínimos)

| Path | Cambio |
|------|--------|
| `SddIA/core/cumulo.paths.json` | +`process_domain_roots`; bump version |
| `engine/.../core/resolver.rs` | multi-root; tests AC-RESOLVE |
| `engine/.../engine/workspace.rs` | reusar `load_paths_config` (ya existe) |

### 4.3 Touchpoints secundarios (post-resolve / con move)

| Path | Acción |
|------|--------|
| Hardcodes `SddIA/process/{feature\|…}.md` en tests/reactors | Apuntar a path dominio o a `resolve_process_path` |
| `verify_process_integrity` / `eda_coverage` / `sync_entity_index` | Core index sin los 6; cobertura EDA de UUIDs **conservada** (entidades siguen en genoma library) |
| `factory.rs` process-creator paths | **No** migrar lógica (D7); solo no asumir que los 6 viven en Core |
| Normas con path absoluto `SddIA/process/delivery-close-cycle.md` etc. | Sustituir por “resuelto vía Cúmulo / process_domain_roots” donde sea aduana |
| `codex-software-engineering.md` | Actualizar nota ABSTRACT-03 → packing path; membresía sin cambio |
| `external-ai-constraints.md` | Documentar `process_domain_roots` |

## 5. Criterios ↔ evidencia

| AC | Evidencia |
|----|-----------|
| **AC-RESOLVE** | Tests unitarios: (a) process solo en domain root resuelve; (b) `kalma2-interact` (Core) resuelve; (c) ausente → error; (d) sin hardcode único a Core. Preferible **antes** del move (fixture tempdir). |
| **AC-MOVE** | Los 6 `.md` ausentes de `SddIA/process/`; presentes en packing §3.1. |
| **AC-INDEX** | Índice Core sin filas fantasma; índice dominio con 6 filas alineadas a frontmatter. |
| **AC-RUN** | Smoke `./sddia-run.sh --process feature` (o `refactorization`) inicia sin panic con perfil/códice software; sin autoridad → `DOMAIN_AUTHORITY_DENIED`. |
| **AC-TQM** | TQM/Kalma2/`sddia-run` OK para process Core restantes y miembros relocalizados. |
| **AC-BUILD** | `cargo build -p execute-process --release` OK. |
| **AC-DOC** | Cascada + PBI → `docs/todos/done/` + `validacion.md` `global: APTO`, `pbi_archived: true` en la rama del PR. |

## 6. Fuera de alcance

- Migrar `*-creator`, `entity-manager`, routes EDA, daemons.
- Reabrir diseño gate ABSTRACT-02 salvo smoke de regresión.
- `codex-personal-assistant` / GesFer.
- Inventar destino fuera de Cúmulo / fusión local.
- Semillas Kaizen en `docs/todos/` (solo Cumulo / evento).

## 7. Handoff Tekton

1. **Prohibido** borrar/mover los 6 `.md` hasta AC-RESOLVE verde (tests resolver).
2. Implementar §4 + bump Cúmulo; luego move + índices §3/D6.
3. Ajustar hardcodes runtime/tests que apunten a path viejo de los 6.
4. Smokes AC-RUN / AC-TQM / gate; `implementation.md` + `execution.md`.
5. Cascada docs; evolution entry vinculando UUIDs process afectados; Argos.

## 8. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Move sin resolve → quiebra TQM/Kalma2 | D8 / plan T0→T1 |
| Overlay array reemplaza Core roots al mal configurar local.paths | Documentar; default Cúmulo siempre incluye packing software |
| process-creator crea software en Core | D7 deuda explícita; no AC-blocker |
| Auto-referencia: este ciclo ejecuta `refactorization` durante el move | Resolver multi-root debe estar en binario **antes** de borrar Core copy; en mismo PR: commit resolve primero o move atómico tras tests en árbol de trabajo |
