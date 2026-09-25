---
feature_name: paciente0-redeploy-ola10-codex-extraction
created: "2026-09-25"
process: feature
base: main
scope: paciente0-redeploy-ola10-codex-extraction
version_spec: "1.0.0"
document_id: PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
uuid: "26df818b-f3f6-4a91-b228-84b065ad49df"
persist_ref: docs/features/paciente0-redeploy-ola10-codex-extraction
branch_name: feat/paciente0-redeploy-ola10-codex-extraction
execution_id: "3098fa27-fb71-48e7-8ee2-aab2bee2c468"
dedalo_verdict: ok
laudos:
  - Q1
  - Q2
  - Q3
  - L-NO-PROCESS
  - L-NO-PATCH
  - L-OLA9
  - L-VAULT-GEMINI
  - L-FILTRO-C-ENV
  - L-CONST
  - L-AGY
  - L-G5
  - L-CI
  - L-DA5
---

# Especificación — paciente0-redeploy-ola10-codex-extraction

Operativo de instancia, no mutación de genoma. Entrega en forja = audit + DEUDA v1.7.0 + cascada `persist_ref` + PBI a `done/`.

## Constantes

| Clave | Valor |
|-------|-------|
| `INSTANCE_ROOT` | `/home/racso/Proyectos/SddIA_AP` |
| `CONFIG_SOURCE` | `/home/racso/Proyectos/.dev/.env` |
| `VAULT_STAGING` | `/home/racso/Proyectos/SddIA_AP.deploy-vault` |
| `PROFILE` / `CODEX` | `consumer` / `codex-kalma2-assistant` |
| `WUI_PORT` | `8766` (`SDDIA_CLIENT_PORT`) |
| `BINS` | 13 ELF post-FIX #298 |
| `FORGE_HEAD_BASELINE` | `67e6a2b` (`main` @ init) |

## Canal

```text
forja feat/… (docs) ← evidencia
  vault staging (Filtro C + SDDIA_GEMINI_MODEL; constitution/codexes locales)
  → build-release-bundle --out INSTANCE_ROOT --codex codex-kalma2-assistant --profile consumer
  → instance-creator skip_ignition (pin SDDIA_EXECUTE_PROCESS_BIN=forja/release)
  → start-sddia.sh env -u SDDIA_EXECUTE_PROCESS_BIN ; consumer ; systemd
  → gates §5 DEUDA + §5 operativo (una vez; DA-5)
  → audit + DEUDA v1.7.0 + DCC + CI + accept-pr
```

## Vault (L-VAULT-GEMINI / L-FILTRO-C-ENV / L-CONST)

`CONFIG_SOURCE` (nombres, 2026-09-25): 26 claves. Presentes: `GEMINI_API_KEY`, IMAP, Telegram, `SDDIA_CLIENT_PORT`. Ausentes: `SDDIA_GEMINI_MODEL`, `SDDIA_AGY_PATH`. Omitir en instancia: 5× `SDDIA_AGENT_RUNTIME_*`. Forzar `SDDIA_RUNTIME_PROFILE=consumer`, `SDDIA_SENSORIAL_JURISDICTION=systemd`.

`PREPROD_VAULT` ausente. Staging escribe `constitution/constitution.json` con `meta.product=SddIA_AP` (partiendo de starter-kit) y `codexes/` con `codex-kalma2-assistant` v1.0.2 desde forja. Fuera de git.

`instance.SddIA.dev.env` recibe `SDDIA_GEMINI_MODEL` (slug de catálogo vigente en host; no se commitea). `agy` en `~/.local/bin` → no pin `SDDIA_AGY_PATH` salvo que el unit `kalma2-bridge@%f` no lo resuelva (G-agy-host).

## Bundle (G-bundle v2)

`--out` directo (wipe). Sin `--skip-build` (ELF Tormentosa no están todos en `target/release` de forja). Capturar log: `[bundle] capsules:` 13 nombres, `[bundle] OK`, 0 `[ERROR] Filtro C`. `strings kalma2-bridge` ∋ `api/aiua/interact`. `MANIFEST.json.binaries` ⊇ 13.

## Creator (G1, G-llm-registry, G4 v2 perfil)

`instance-creator` v1.4.0 `skip_ignition`. Acuse: `success:true`, `llm_registry_materialized`, `domain_profile_materialized`. Verificar sin poll: `local.paths.json` ≠ `{}`; registry presente; perfil `codex-kalma2-assistant` / `git_required:false`; no `.SddIA/projects/`.

## Ignición + gates

`start-sddia.sh` jurisdicción systemd. G3b v2: 5 unidades `LogRateLimitIntervalSec=30s` / `LogRateLimitBurst=500`. G3 v2: un `POST /api/aiua/interact` (sin retry). G-eda-roots: `PBI_Forged` sintético → `no_subscriber`. G-authority: `feature` desde instancia → denegación limpia.

## Cierre documental (forja)

1. `docs/audits/paciente0-deploy-{STAMP}.md` — frontmatter DEUDA §7 + `wave: 10`, `prompt_ssot_version: 1.7.0`, `laudos_applied`, `consumer_bins: 13`.
2. DEUDA `1.6.0` → `1.7.0`: prompt (ola 10→11), §0bis errata ola 9, `BINS_CONSUMIDOR=13`, §4.2 + FIX #298, §5 gates v2, §10 refs.
3. PBI operativo → `docs/todos/done/` + `validacion.md` `pbi_archived: true` en el mismo PR.
4. Kaizen solo si §7.2 ∈ {OLA-REGRESIÓN, OLA-NUEVA-FRICCIÓN} con IDs no listados en PBI §8.

## Criterios de aceptación

Ver PBI §10: CA-PREREQ, CA-OLA10, CA-FILTRO-C, CA-EDA-ROOTS, CA-AUTHORITY, CA-OLA9-ABSORBIDA, CA-TORMENTOSA, CA-LLM-REGISTRY, CA-AUDIT, CA-DEUDA-1.7.0, CA-DA5.
