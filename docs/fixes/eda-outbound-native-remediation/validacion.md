---
feature_name: eda-outbound-native-remediation
created: "2026-07-13"
process: bug-fix
branch: fix/eda-outbound-native-remediation
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[FIX] EDA — remediar suscriptores IOTA y Telegram con red nativa.md
checks:
  CA1-lab-doubles-success: pass
  CA2-config-missing-classified: pass
  CA3-router-consensus: pass
  CA4-native-over-wasi: pass
  CA5-reaudit-no-new-stubs: pass
git_changes:
  - SddIA/sddia-io/src/outbound_lab.rs
  - SddIA/sddia-io/src/lib.rs
  - SddIA/sddia-io/Cargo.toml
  - SddIA/tools/send-telegram-notification/
  - SddIA/tools/iota-immutable-publisher/
  - docs/fixes/eda-outbound-native-remediation/
  - docs/todos/done/[FIX] EDA — remediar suscriptores IOTA y Telegram con red nativa.md
---

# Validación — eda-outbound-native-remediation

**Veredicto global: APTO** — cierre documental en rama, listo para PR.

| CA | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA1 | Evento lab con dobles locales → éxito Telegram + IOTA sin red | ✅ | `route-domain-event` + `SDDIA_LAB_MOCK_OUTBOUND=1` / `SDDIA_LAB_SIMULATE_IOTA=1` → `delivery_status` los tres suscriptores `success`, `sweep: purged` |
| CA2 | Credenciales ausentes → error clasificado sin secretos | ✅ | `config-missing: TELEGRAM_BOT_TOKEN...` / `config-missing: IOTA_WALLET_SECRET` |
| CA3 | Router no purga fuera de consenso | ✅ | Fan-out `PullRequest_Presented` lab purgado solo tras éxito de suscriptores |
| CA4 | Resolución nativa (`prefer_wasm: false`) | ✅ | Tras `cargo build --release` de ambos tools; perfiles `release` preceden `debug` en SSOT |
| CA5 | Sin nuevos DL por stubs WASI en rutas corregidas | ✅ | Re-enrutado post-release sin testigos KO; DL históricos conservados (fuera de alcance) |

## Comandos de verificación (2026-07-13)

```bash
cd SddIA && cargo build --release -p send-telegram-notification -p iota-immutable-publisher
cd SddIA && cargo test -p sddia-io -p send-telegram-notification -p iota-immutable-publisher

export SDDIA_LAB_MOCK_OUTBOUND=1 SDDIA_LAB_SIMULATE_IOTA=1
./sddia-run.sh --process route-domain-event --inputs '{"event_file_path":".events/pending/<uuid>.json"}'

./sddia-run.sh --process event-bus-audit --inputs '{"emit_kaizen_alert":false}'
```

## Hallazgo operativo

El resolvedor de cápsulas prioriza `SddIA/target/release/` antes que `debug`. Tras modificar tools, **recompilar release** es obligatorio para que `execute-process` / `route-domain-event` usen los binarios nuevos.

## Re-auditoría bus

| Métrica | Valor |
|---------|-------|
| pending | 0 |
| stale_pending | 0 |
| dead-letter (histórico) | 9 cabeceras + testigos previos |

Los dead-letters históricos con mensaje WASI permanecen como cicatriz Kaizen; no se purgaron (spec § fuera de alcance).

## Pendiente cierre

- ~~PR único en `fix/eda-outbound-native-remediation`~~
- ~~Mover PBI a `docs/todos/done/` con `pbi_archived: true` en el mismo PR~~

## Hallazgo operativo ejecutado

```bash
cd SddIA && cargo build --release -p sddia-io -p send-telegram-notification -p iota-immutable-publisher
```

Verificado: binarios `target/release/*` contienen strings `lab-mock-outbound` / `config-missing` (artefactos jun 13, no jun 15).
