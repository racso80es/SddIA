---
feature_name: antigravity-connectors
created: "2026-09-03"
process: feature
purpose: Auditoría Filtro A del esqueleto Jules contra PBI-CAPSULES-ANTIGRAVITY-NATIVE v1.2.0
version_clarify: "1.0.0"
execution_id: "7d6ca13f-95e4-4b47-8457-6f37d54f3c3d"
pbi_ref: "docs/todos/pending/[OPERATIVO] Forja de cápsulas nativas para integración dual con Google Antigravity (HTTP y CLI).md"
document_id: PBI-CAPSULES-ANTIGRAVITY-NATIVE
pbi_uuid: "7f966f32-5502-4bd7-b252-44849f29f5d3"
verdict: NO_APTO
refactor_required: true
---

# Clarificación / auditoría — antigravity-connectors

Transcript: merge `main` → `feat/antigravity-connectors-8989250975201761652` (`735a224`). PBI v1.2.0 entra desde `main` (a01e066). Código de cápsulas = commits Jules `eac51a7` + `9e0c139`.

**Veredicto:** el delta de implementación **no cumple** el PBI refinado. Refactor obligatorio antes de DCC. No es cierre documental.

## D0 — Topología

| Pregunta | Hecho |
|----------|-------|
| Troncal | `main`. No hay `master`. |
| PBI | Sigue en `docs/todos/pending/`. No está en `done/`. |
| `persist_ref` | `docs/features/antigravity-connectors` (ciclo vivo). No crear `capsules-antigravity-native`. |
| Rama | `feat/antigravity-connectors-8989250975201761652` |
| Autor código | `google-labs-jules[bot]` |

## D1 — Mapa de desviación (código vs PBI v1.2.0)

| ID | PBI v1.2.0 | Código Jules | Gravedad |
|----|------------|--------------|----------|
| A1 | HTTP = **tool** `gemini-http-infer` (L1 default) | **skill** `antigravity-http-connector` | Alta — familia ED incorrecta |
| A2 | REST AI Studio `generateContent`; env `GEMINI_API_KEY` + `SDDIA_GEMINI_API_BASE_URL` | `ANTIGRAVITY_API_KEY` + `ANTIGRAVITY_API_ENDPOINT` (nombres inventados H2) | Alta — H2 reincidente |
| A3 | Cliente HTTP nativo + mock `SDDIA_LAB_MOCK_OUTBOUND` / `SDDIA_LAB_MOCK_GEMINI_URL` | `std::process::Command::new("curl")`; sin mock lab | Alta — spawn innecesario; key en argv (`ps`) |
| A4 | Comentario crate: nativo por red; exclusión WASI justificada | Comentario «WASI compatibility → curl» **y** exclusión en `build-wasi-capsules.sh` | Media — contradicción interna |
| A5 | CLI: `{agy} --print --output-format json` + prompt en argv `-p` | Spawn de path crudo; JSON del `request` por **stdin**; sin flags headless | Alta — H10; no es print mode |
| A6 | Binario: `SDDIA_AGY_PATH` o `PATH` | Solo `ANTIGRAVITY_CLI_PATH` (nombre no SSOT) | Media |
| A7 | Default `--sandbox`; skip-permissions doble opt-in | Sin flags de permiso | Alta — o TUI hang o auto-approve implícito ausente |
| A8 | Sobre `capsule-json-io` 2.0 (`meta`, `message`, `result`) | `sddia-io::emit_success/error` sin `meta` | Media — H7 |
| A9 | `context: system-operations` | `ecosystem-evolution` | Media — matriz Cerbero |
| A10 | `hash_signature` real vía creator | `sha256:` + 64 ceros | Alta — sello fósil |
| A11 | `{name}.md` indexado en `SddIA/skills/index.md` | **Ausentes** del índice | Alta — aduana índice |
| A12 | Cobertura EDA = UUID de la ED + hash real | `eda-coverage.json` fila `ae2927f5-…` `last_hash: sha256:deadbeef`. UUIDs reales: `b548b894-…` (HTTP), `85750058-…` (CLI). **Ninguno coincide.** | Crítica — cobertura falsa + 2 huérfanos |
| A13 | Cero `provides` / cero `llm:interact` | No declaran `provides` (OK) | — |
| A14 | Tests lab sin red Google | Cero tests | Alta |
| A15 | Forja `entity-manager` | Genoma escrito en commits Jules (`SddIA/skills/*`, `eda-coverage.json`) | Crítica — DA-2 |

## D2 — Qué se conserva

- Intención dual HTTP / CLI (dos crates nativos).
- Lectura de env de proceso (no abren `.dev/.env`). Anti-panic vía `emit_error` (parcial).
- Exclusión WASI en `build-wasi-capsules.sh` (el destino nativo es correcto; el medio `curl` no).
- Workspace `skills/*` ya los recoge como miembros Cargo.

## D3 — Decisiones de refactor (defaults PBI §8, sin laudo nuevo)

| ID | Decisión |
|----|----------|
| L1 | HTTP → **tool** `gemini-http-infer`. CLI permanece **skill** `antigravity-cli-executor` (re-forja, no bisturí sobre el md Jules). |
| L2 | `context` = `system-operations`. |
| L3 | No `provides`. No tocar Códice/bindings. |
| L4 | Vertex fuera. |
| L5 | Tiers PBI ortogonal. |
| L-PERSIST | Seguir este `persist_ref`. |
| L-RETIRE | Retirar `antigravity-http-connector` (skill) tras alta del tool. No dejar dos HTTP. Fila EDA `ae2927f5` = basura; no reusar. |

## D4 — Ruido colateral del merge

`.antigravity/rules/` entra desde `main` (IDE Antigravity). No es genoma Core (`directories.*`). No sustituye `.cursor/rules/` ni `external-ai-constraints.md`. Fuera del alcance de las cápsulas.

## D5 — Bloqueos de aduana previsibles

Si se presenta PR ahora: índice skills incompleto; EDA `--scan` huérfanos o cobertura `deadbeef`; `hash_signature` inválido; posible gate evolution si se toca genoma sin registro.

**Prohibido** `delivery-close-cycle` hasta Fase R del `plan.md`.
