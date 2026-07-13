---
feature_name: poda-python-rust-clientes
created: "2026-07-11"
process: feature
purpose: Poda runtime Python residual y adecuación de clientes a cápsulas Rust
---

# Clarificación — poda-python-rust-clientes

Transcript de decisiones (2026-07-11) para el PBI `PBI-REFACTOR-PODA-PYTHON-RUST`.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 (bootstrap documental; PBI declara `refactorization` pero el operador pidió feature) |
| Nombre operativo | **poda-python-rust-clientes** |
| Rama | `feat/poda-python-rust-clientes` ✅ |
| `persist_ref` | `docs/features/poda-python-rust-clientes` |
| Manifiesto | `docs/todos/pending/[REFACTOR] Poda ejecutables Python — adecuación de clientes a cápsulas Rust.md` |
| Contrato I/O | `SddIA/norms/capsule-json-io.md` |

---

## D2 — Feature vs refactorization

| Pregunta | Decisión |
|----------|----------|
| ¿Por qué `feature` si el PBI es REFACTOR? | Bootstrap vía cadena V5 estándar; paridad documental idéntica (`features-documentation-pattern`) |
| ¿Cambio ontológico? | No — entrega = poda de deuda y porte de cores; sin capacidad funcional nueva |
| Beta laboratorio | Fase 1 **ejecutada** vía `./sddia-run.sh --process feature`; fases 2–5 simuladas; fase 7 `delivery-close-cycle` falló (esperado sin `validacion.md`) |

---

## D3 — Inventario runtime Python (ruta caliente)

| Cliente | Dependencia Python | Paridad Rust existente |
|---------|--------------------|------------------------|
| `engine/python_core.rs` | `route_fractal_event_core.py`, `radamanto_batch_core.py`, `telemetry_compliance_audit_core.py` | Portar a módulos `engine/` |
| `engine/route_domain_core.rs` L348 | `execute-action.py` | Engine nativo / subprocess acción |
| `engine/phase_capsules.rs` L54 | `audit-entity-eda-coverage.py` | Nuevo módulo Rust o tool existente |
| `handlers/telegram_fallback.rs` L30 | `limbo/tools/send-telegram-notification/main.py` | `SddIA/tools/send-telegram-notification/` ✅ |
| `sddia-run.sh` | `orchestrator_resolve.py` | Resolver binario en bash |
| `scripts/tools/invoke.py` | `capsule_resolve.py`, `env_loader.py` | Subcomando Rust o binario `invoke` |
| `scripts/daemons/_exec_daemon.py` | `env_loader.py` | `sddia-daemon-runtime` + carga `.env` nativa |
| `scripts/limbo/**` (19 `.py`) | — | Purga total (cápsulas Rust ya compiladas) |

---

## D4 — Alcance v2.0.0 (2026-07-11, ampliación)

| Pregunta | Decisión |
|----------|----------|
| ¿Capa QA Python? | **Dentro del PBI** — Ola 5; eliminar `SddIA/scripts/qa/**/*.py` |
| ¿Cores QA duplicados? | **Dentro del PBI** — Ola 4; eliminar tras paridad Rust verificada |
| ¿Docs `SddIA/skills/*.md`? | **Dentro del PBI** — Ola 6; alinear a crates Rust |
| ¿Cero `.py` en repo? | **Sí** — objetivo O11; excl. `.venv/`, `.tools/` |
| ¿Cero referencias operativas? | **Sí** — objetivo O12; genoma operativo sin rutas `.py` |
| ¿Compatibilidad legacy? | **Rotura asumida** — sin shims Python |

## D5 — Fuera de alcance (sin cambio)

| Área | Motivo |
|------|--------|
| `.venv/`, `.tools/` | Dependencias de terceros, no SSOT |
| Histórico `SddIA/evolution/*.md` | Evidencia inmutable; no reescribir |

## D6 — Orden de consolidación (7 olas)

```text
Ola 1 Engine ──► Ola 2 Clientes ──► Ola 3 Limbo
      ──► Ola 4 Cores QA ──► Ola 5 Capa QA ──► Ola 6 Docs ──► Ola 7 Gate + cierre
```

## D7 — Inventario cores QA → Rust (Ola 4)

| Python (QA) | Rust (engine/daemon/tool) |
|-------------|----------------------------|
| `route_fractal_event_core.py` | `route_fractal_core.rs` |
| `radamanto_batch_core.py` | `radamanto_batch_core.rs` |
| `telemetry_compliance_audit_core.py` | `telemetry_compliance_core.rs` |
| `fix_tool_process_core.py` | `fix_tool_process_core.rs` |
| `cerbero_governance_react_core.py` | `cerbero_governance_react_core.rs` |
| `route_domain_event_core.py` | `route_domain_core.rs` |
| `execute-action.py` | `actions.rs` |
| Resto `*_core.py` en QA | cápsulas / `sddia-daemon-runtime` |

## D8 — SSOT y genoma

| Pregunta | Decisión |
|----------|----------|
| ¿Retirar `scripts_limbo` de Cúmulo? | **Sí**, tras purga física |
| ¿Mutación manual de genoma? | **Prohibida** — vía `entity-manager` cuando aplique |
| ¿Registro evolución? | Obligatorio en `SddIA/evolution/` al cierre |
