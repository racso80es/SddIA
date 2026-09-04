---
document_id: PBI-FIX-FRACTURE-ca3d901fdc9a-OLA1
uuid: "0e7d8cd1-b57e-462f-8ae8-fb082db6f38d"
title: "[FIX] delivery-close-cycle — Ola 1 cápsula git-manager"
format: markdown
version: "1.0.0"
created: "2026-09-04"
updated: "2026-09-04"
status: cerrado
refinement_status: clarified
priority: alta
process: bug-fix
type: fix
dispatch: false
parent_pbi: PBI-FIX-FRACTURE-ca3d901fdc9a
parent_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (ca3d901fdc9a).md
suggested_branch: fix/dcc-git-manager-capsule-lab
persist_ref_suggested: docs/fixes/dcc-git-manager-capsule-lab
fracture_hash: ca3d901fdc9a
fracture_process: delivery-close-cycle
incident_ref: "System_Fracture_Detected — ca3d901fdc9a"
friction_ids:
  - F-DCC-SNAPSHOT-FINAL
  - F-DCC-PUBLICACIN-REMOTA
architectural_constraints:
  - A-DCC-GIT-SOLO-CAPSULE
  - A-IGNITION-PRODUCE-SKILL-ELF
  - A-NO-BYPASS-RAW-GIT
execution_file_lock:
  - start-sddia.sh
  - SddIA/scripts/common/sddia_shell_lib.sh
gates_this_wave:
  - DCC-GM-CA1
  - DCC-GM-CA2
  - DCC-GM-CA3
related:
  - SddIA/skills/git-manager.md
  - SddIA/skills/git-manager/src/main.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/core/cumulo.paths.json
  - docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (ca3d901fdc9a).md
source_audit: "2026-09-04. resolve_capsule(repo, git-manager) falla: no hay ELF bajo SddIA/target/{debug,release}/git-manager ni wasm sellado. start-sddia.sh release_pkgs omite git-manager. --seal-capsules names no incluye git-manager. Genoma git-manager.md documenta cargo build -p git-manager → SddIA/target/debug/git-manager."
review_notes: "Ola 1 = toolchain de cápsula nativa en ignición. No mutar genoma skills/ (DA-2). Ola 2/3 fuera."
---

# [FIX] delivery-close-cycle — Ola 1 cápsula git-manager

Absorbe `F-DCC-SNAPSHOT-FINAL` y `F-DCC-PUBLICACIN-REMOTA` del padre `PBI-FIX-FRACTURE-ca3d901fdc9a`. Mismo `fracture_hash`.

## Linaje

| Campo | Valor |
|-------|--------|
| Padre | `PBI-FIX-FRACTURE-ca3d901fdc9a` |
| Eventos | `72a41559-18c9-4299-8355-de1defdc3559`, `aedcea75-80fd-4e47-a05e-a4fce051271d` |
| Hueco | DCC exige `skill:git-manager`; ignición no lo materializa |

## 0. Laudo Ola 1

**Intención:** tras `_ensure_orchestrator`, existe ELF nativo `git-manager` bajo `compiled_capsules.native_root` (`SddIA/target/{release,debug}/git-manager`) resoluble por `capsules.rs`. Snapshot final y Publicación remota dejan de fallar por cápsula ausente.

### 0.1. Hechos (Filtro A)

| Tesis | Veredicto | Base |
|-------|-----------|------|
| Error literal `cápsula skill 'git-manager' no encontrada bajo SddIA/target` | **Hecho** | acuse DCC `failed_phase: Snapshot final`; `capsules.rs` `resolve_capsule` |
| Ignición no construye el crate | **Hecho** | `start-sddia.sh` `release_pkgs`: orquestador + daemons/tools de canal; **sin** `git-manager` |
| `--seal-capsules` no sella git-manager | **Hecho** | `names` = iota/event/telegram; skill Git fuera |
| Contrato skill documenta binario debug | **Hecho** | `SddIA/skills/git-manager.md`: `cargo build -p git-manager` → `SddIA/target/debug/git-manager` |
| Bypass `git`/`gh` raw | **Prohibido** | DA-3, Kintsugi, `delivery-close-cycle` § Publicación remota |

### 0.2. Decisiones

1. Añadir `git-manager` a `release_pkgs` de ignición (y debug equivalente si el resolver prefiere debug más nuevo — paridad F-DEP-07 del orquestador).
2. Incluir `git-manager` en `--seal-capsules` `names` si el sello es el contrato de testigo; si el resolver nativo basta con ELF, el sello es complemento no sustituto.
3. Genoma `{name}.md` de la skill: **no** en esta ola (DA-2). Solo scripts de lab / ignición.
4. Verificación: `resolve_capsule` encuentra native; un `delivery-close-cycle` de laboratorio pasa Snapshot final sin ese error (Publicación remota puede seguir bloqueada por Ola 2 si `sddia-qa` falta — no mezclar CA).

## 1. Alcance

| Archivo | Cambio |
|---------|--------|
| `start-sddia.sh` | `-p git-manager` en release (y debug si aplica); `names` de seal |
| `SddIA/scripts/common/sddia_shell_lib.sh` | solo si hay helper de resolución de cápsula skill; si no, no tocar |

### Fuera de esta ola

- Compilar `sddia-qa` (Ola 2).
- Suprimir fractura por binario ausente (Ola 3).
- Mutar `SddIA/skills/git-manager.md` / índices.
- Credencial GitHub / `workflow` scope (DA-7; fricción distinta).

## 2. Criterios de aceptación (Ola 1)

| ID | Criterio | Verificación |
|----|----------|--------------|
| DCC-GM-CA1 | Tras ignición lab, existe ELF ejecutable `SddIA/target/release/git-manager` o `debug/` equivalente. | `file` + `_sddia_is_native_elf` |
| DCC-GM-CA2 | `execute-process` resuelve `skill:git-manager` sin error `no encontrada bajo SddIA/target`. | invocación mínima git-manager (p. ej. status) vía orquestador |
| DCC-GM-CA3 | DCC llega más allá de Snapshot final cuando el resto del ciclo está íntegro. Si Ola 2 abierta, fallo permitido solo en aduanas `sddia-qa`, no en Snapshot. | acuse JSON: `failed_phase` ≠ `Snapshot final` por esta traza |

## Criterio de cierre

- [x] DCC-GM-CA1…CA3
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO en `docs/todos/done/` en la rama del PR
