---
feature_name: paciente0-redeploy-ola10-codex-extraction
created: "2026-09-25"
process: feature
phases:
  - l0-diseno
  - l1-vault
  - l2-bundle
  - l3-creator
  - l4-ignicion
  - l5-gates
  - l6-audit-deuda
  - l7-dcc-pr
  - l8-ci-accept
branch_name: feat/paciente0-redeploy-ola10-codex-extraction
persist_ref: docs/features/paciente0-redeploy-ola10-codex-extraction
pbi_ref: docs/todos/pending/[OPERATIVO] Paciente 0 SddIA_AP — redeploy ola 10 post-extracción del códice de ingeniería de software.md
document_id: PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
uuid: "26df818b-f3f6-4a91-b228-84b065ad49df"
execution_id: "3098fa27-fb71-48e7-8ee2-aab2bee2c468"
---

# Plan — paciente0-redeploy-ola10-codex-extraction

Corte Diseño: **clarify + objectives + spec + plan + commit**. Ejecución L1–L8 en el mismo ciclo hasta PR verde y `accept-pr`.

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. Semilla `.tmp/feature-paciente0-redeploy-ola10.json`. `execution_id` `3098fa27-fb71-48e7-8ee2-aab2bee2c468`.

## Fase L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. **Stop aquí.** Prohibido bundle, creator, ignición o mutar DEUDA en L0.

## Fase L1 — Vault

Refrescar `/home/racso/Proyectos/SddIA_AP.deploy-vault`:

1. `root.dev.env` ← copia `CONFIG_SOURCE`.
2. `instance.SddIA.dev.env` ← mismas claves **menos** `SDDIA_AGENT_RUNTIME_*`; forzar `SDDIA_RUNTIME_PROFILE=consumer`, `SDDIA_SENSORIAL_JURISDICTION=systemd`; añadir `SDDIA_GEMINI_MODEL` si ausente (L-VAULT-GEMINI).
3. `constitution/` con `meta.product=SddIA_AP`; `codexes/` = `codex-kalma2-assistant` v1.0.2 desde forja.
4. `chmod 600` `*.env`. Cero secretos en git / logs / PBI.

## Fase L2 — Bundle

```bash
./SddIA/scripts/build-release-bundle.sh \
  --out /home/racso/Proyectos/SddIA_AP \
  --codex codex-kalma2-assistant \
  --profile consumer
```

Sin `--skip-build`. Capturar log completo para audit. Gate inmediato G-bundle v2.

## Fase L3 — instance-creator

```bash
export SDDIA_EXECUTE_PROCESS_BIN="${FORGE_ROOT}/SddIA/target/release/execute-process"
./sddia-run.sh --process instance-creator --inputs '{
  "instance_root": "/home/racso/Proyectos/SddIA_AP",
  "runtime_profile": "consumer",
  "vault_source": "/home/racso/Proyectos/SddIA_AP.deploy-vault",
  "skip_ignition": true
}'
```

DA-5: acuse JSON = fin. Verificar artefactos en disco (no poll EDA).

## Fase L4 — Ignición

```bash
loginctl enable-linger "$(id -un)"
cd /home/racso/Proyectos/SddIA_AP
mkdir -p .SddIA/daemons/logs
env -u SDDIA_EXECUTE_PROCESS_BIN \
  SDDIA_RUNTIME_PROFILE=consumer \
  SDDIA_SENSORIAL_JURISDICTION=systemd \
  ./start-sddia.sh >> .SddIA/daemons/logs/start-sddia.log 2>&1
```

Lab `email-watcher@…SddIA`: stop si active (R-07). Telegram lab: coexistir solo si token distinto.

## Fase L5 — Gates

Ejecutar DEUDA §5 + PBI §5. Una sola vez por gate de combustión (G3 v2). Registrar matriz §7.1. Veredicto §7.2.

## Fase L6 — Audit + DEUDA v1.7.0

Escribir `docs/audits/paciente0-deploy-{STAMP}.md`. Mutar DEUDA a v1.7.0 (documental, no genoma DA-2). `implementation.md` + `execution.md`. Mover PBI a `done/`. `validacion.md` con checks locales; `global` no APTO hasta CI.

## Fase L7 — DCC / PR

`./sddia-run.sh --process delivery-close-cycle` (`source_process: feature`, `persist_ref`, `branch_name`). DA-5 post-acuse.

## Fase L8 — CI + accept-pr

Un log de checks. Si rojo: parche local + un push (DA-6). Si verde: `validacion.md` `global: APTO` + `./sddia-run.sh --process accept-pr`. Cierre = merge en `main`.
