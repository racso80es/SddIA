---
feature_name: tracker-item-document-codec
created: "2026-10-26"
process: refactorization
phases:
  - id: P1
    name: Codec motor
    status: done
  - id: P2
    name: refine-pbi Linear
    status: done
  - id: P3
    name: Cascada documental + CI
    status: in_progress
  - id: P4
    name: Operaciones en cápsula (opcional)
    status: pending
---

# Plan — tracker-item-document-codec

## P1 — Codec motor (done)

- `tracker_item_document.rs`, `normalize_item_markdown`, tests fence Linear.
- `refine.rs` resolución remota/local.

## P2 — refine-pbi (done)

- `refine-pbi` sobre AUT-15 acusó `refinado`.

## P3 — Entrega (in_progress)

- Rama `refactor/tracker-item-document-codec`, `validacion.md`, PR + CI.

## P4 — Iteración posterior

- Extender `tracker.lifecycle` y adapters con parse/render nativos.
