---
feature_name: kaizen-rust-capsule-structure
created: "2026-06-15"
updated: "2026-06-15"
process: refactorization
items_applied:
  - workspace-init
  - mayeuta-stabilization
  - dedalo-design
  - tekton-ola-1-skills
  - tekton-ola-2-tools
  - tekton-ola-3-event-watcher-p0
  - tekton-ola-3-event-sweeper-p1
  - tekton-ola-3-telegram-watcher-p2
  - tekton-ola-3-github-bridge-watcher-p3
  - tekton-k6-certification
pause_after: k6-certification
next_wave: k7-delivery-close-cycle
handoff: docs/features/kaizen-rust-capsule-structure/status.md
debt_ref: plan.md#backlog-de-deuda-técnica-post-k6
---

# Ejecución — Kaizen Cápsulas Rust

## Pausa documental (2026-06-15)

| Campo | Valor |
|-------|-------|
| **Último hito cerrado** | K6 — `validacion.md` APTO |
| **Siguiente hito** | K7 — `delivery-close-cycle` + PR |
| **Retomar en** | [`status.md`](./status.md) |
| **Deuda planificada** | [`plan.md` §Backlog](./plan.md#backlog-de-deuda-técnica-post-k6) |

El código y la documentación de certificación están en la rama `feat/kaizen-rust-capsule-structure`. **No hay trabajo de implementación pendiente** en este feature salvo el cierre de entrega (K7).

## Comando de arranque al retomar

```bash
# 1. Confirmar rama
git checkout feat/kaizen-rust-capsule-structure

# 2. (Opcional) Build smoke
cd SddIA && export CARGO_TARGET_DIR="$PWD/target"
cargo build --release -p event-watcher -p event-sweeper -p telegram-watcher -p github-bridge-watcher

# 3. Cierre
cd ..  # repo root
python3 SddIA/scripts/qa/execute-process.py \
  --process delivery-close-cycle \
  --inputs '{"source_process":"refactorization","feature_ref":"docs/features/kaizen-rust-capsule-structure"}'
```

## Smoke V1 (referencia)

```bash
SddIA/target/release/event-watcher --once
SddIA/target/release/event-sweeper --once --json
SddIA/target/release/telegram-watcher --once          # exit 2 sin TELEGRAM_* = OK
SDDIA_LAB_SIMULATE_REMOTE_PR=1 SddIA/target/release/github-bridge-watcher --once
```

Orquestación EDA (`execute-process.py`) permanece en Python — DEBT-K1, spec §9.

## Fases

| Fase | Agente | Estado |
|------|--------|--------|
| Clarificación / spec / plan | Mayeuta + Dedalo | ✅ |
| Ola 1 Skills | Tekton | ✅ |
| Ola 2 Tools | Tekton | ✅ |
| Ola 3 Daemons | Tekton | ✅ |
| K6 + Argos | Tekton + Argos | ✅ |
| **Cierre PR** | delivery-close-cycle | ⏳ |

## Deuda post-entrega

Tras merge del PR Kaizen, abordar ítems DEBT-K* por prioridad en `plan.md`. No mezclar con K7.
