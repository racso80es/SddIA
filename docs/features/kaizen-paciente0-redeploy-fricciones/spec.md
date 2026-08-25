---
feature_name: kaizen-paciente0-redeploy-fricciones
created: "2026-08-25"
process: feature
base: main
scope: kaizen-paciente0-redeploy-fricciones
version_spec: "1.0.0"
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
uuid: "56aff1d3-d5f6-4502-9b5b-e5a57dc718e3"
laudo: absorber-parches-core-un-pr
execution_id: "c95fa63f-be71-481b-a927-475e7c885fd0"
---

# Especificación — kaizen-paciente0-redeploy-fricciones

## 1. Laudos Dedalo

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L-CORE-ROOT** | ¿Qué sustituye `@@SDDIA_CORE_ROOT@@`? | Siempre `instance_root` (ruta absoluta de la instancia). Nunca `repo` del CLI forjador. | F-DEP-01: `install_systemd_templates` usa `repo.display()`. Invocación desde forja apuntaba ExecStart al lab. |
| **L-STARTER** | ¿`local.paths.json` vacío? | Si el archivo no existe, copiar `SddIA/scripts/starter-kit/.SddIA/local.paths.json`. Prohibido escribir `{}`. Si ya existe, no pisar. | F-DEP-04. Starter-kit ya es SSOT de overlay mínimo. |
| **L-BUNDLE-PY** | ¿Gate `.py`? | Tras copiar ELF (con o sin `--skip-build`), `strings` sobre centinelas (`event-watcher`, `event-sweeper`, `email-watcher`, `telegram-watcher`, `execute-process`, `kalma2-bridge`) **no** debe contener `execute-process.py`. Fallo → exit ≠ 0, no emitir `MANIFEST.json` útil. | F-DEP-03 causa inmediata. |
| **L-BUNDLE-STALE** | ¿`--skip-build` cuándo? | **v2 — cicatriz SHA-256, no mtime.** `--skip-build` solo si, para cada binario de `CONSUMER_BINS`, existe testigo `SddIA/target/${PROFILE_BIN}/<bin>.sha256` cuyo `source_sha256` coincide con el digest actual del **cierre de compilación local** (ver §1.1). Divergencia, testigo ausente o ELF ausente → exit ≠ 0, exigir rebuild **sin** `--skip-build`. Tras `cargo build` (camino sin skip) el script **escribe** el testigo. | mtime se altera con checkout/stash/CI; el PBI pedía hash/build-id. D12. |
| **L-START** | ¿`cargo build` en bundle? | `_ensure_orchestrator`: si existe `MANIFEST.json` en raíz instancia **o** no existe `SddIA/Cargo.toml` → no invocar cargo; resolver ELF; si falta, fail con mensaje de bundle, no de compile. Si hay `Cargo.toml` (lab) → conservar build actual. | F-DEP-02. Gate en Core `start-sddia.sh` (el bundle lo proyecta). |
| **L-SMOKE-ROUTE** | ¿Smoke `route-domain*`? | Si `skip_ignition=true` → smoke ruta **skipped** (no falso APTO de enrutado). Si ignición ejecutada → emitir 1 evento domain de laboratorio y exigir `route-domain` o `route-domain-event` `success:true`. Fallo → fase Smoke `failed`. Topology+Local_QA se mantiene como preflight. | F-DEP-06. No dualidad de QA: reusar procesos `route-domain*` existentes. |
| **L-GUARD** | ¿Cuándo extraer asunto post-LLM? | En `classify_llm`, si `extract_actionable_from_subject` devuelve `(title, datetime)` completo → `verdict=actionable` **aunque** el LLM haya dicho `passive` o `noise` o vacío. Triaje-C `noise` no llega aquí. Extracción incompleta no inventa fecha. | D3 Mayeuta + matriz §1 desempate. |
| **L-PATH-LABEL** | ¿`decision_path`? | Conservar `llm` (la fase Clasificacion corrió). Añadir en payload/proof `subject_elevation: true` si el guard elevó. No nueva clase ECST. | Cero dualidad ontológica. |
| **L-PROMPT** | ¿Prompt Clasificacion? | Añadir: reunión/cita con fecha extraíble en asunto es candidato `actionable` (extracción pendiente). No reescribir la matriz. | F-TRIAGE-02 alineación. |
| **L-INFER** | ¿Peaje cero? | Propagar `tokens_*` desde envelope cápsula (hoy ya se leen; mock → 0). Si `SDDIA_LLM_REQUIRE_INFER=1` y (`invoke` falla **o** `tokens_in+tokens_out==0`): no emitir `passive` silencioso. Orden: (1) extracción asunto; (2) si incompleta, emitir `Email_Triaged` con `classification-degraded: true` y `verdict=passive`. Default consumidor: documentar flag; no forzar=1 en Core. | O8. Consumidor opt-in. |
| **L-ONBOARD** | ¿Inventario? | `ONBOARDING.md` del bundle: claves mínimas consumidor **nombres solo** (`SDDIA_EMAIL_*`, `TELEGRAM_*`, `SDDIA_LLM_*`, `SDDIA_RUNTIME_PROFILE`, `SDDIA_SENSORIAL_JURISDICTION`, `SDDIA_CLIENT_PORT`). LLM recomendado para Clasificacion; sin LLM → Triaje-C + extracción asunto. | F-DEP-05 documental. Sin secretos. |
| **L-MATRIX** | ¿Mutar matriz? | **UPDATE** `email-triage-matrix` §1: una frase — post-Clasificacion, extracción estructural completa de asunto eleva a `actionable` salvo que Triaje-C ya hubiera cerrado `noise`. | Hacer explícito el desempate; no nueva norma. |
| **L-DIST** | ¿Mutar distribución? | **UPDATE** `sddia-distribution-protocol`: gate ELF fresco + prohibición `--skip-build` stale; `CORE_ROOT` de plantillas systemd = raíz instancia. | Vía C; no norma paralela. |
| **L-CREATOR-MD** | ¿Proceso `instance-creator`? | **UPDATE** via `entity-manager`: fase Systemd intent `CORE_ROOT=instance_root`; Topología copia starter-kit; Smoke declara preflight + `route-domain*` si ignición. Handler `instance_creator.rs` es motor (no DA-2). | Contrato de fases vs código. |
| **L-AUDIT** | ¿PBI §9? | Un `.md` bajo `paths.auditsPath` (`docs/audits/`) al cierre. No sustituye `validacion.md`. | Cúmulo. |
| **L-FORGE** | ¿Mutación? | Genoma (`process/`, `norms/`, `library/norms/`) solo `entity-manager`. Engine, scripts bundle/ignición: Tekton bajo esta topología feature (DA-4 cubierto). | DA-2/DA-4. |

### Rechazados

- F-TRIAGE-03 inbox `passive` (PBI separado).
- Wizard `DT-CONFIG-UX-ONBOARDING`.
- Consolidar parches 2026-08-24 como runbook canónico.
- Nueva clase ECST para reunión.
- Forzar `SDDIA_LLM_REQUIRE_INFER=1` en toda instancia.
- Smoke `route-domain*` cuando `skip_ignition` (falso positivo).
- Escribir `{}` como overlay de paths.
- **mtime / `find -newer` como oráculo de frescura** (L-BUNDLE-STALE v1).
- **MD5** como cicatriz (solo SHA-256; paridad `hash_signature` Core).
- Hashear únicamente `src/` + `Cargo.toml` del crate hoja (ciego a `path =` workspace).

### 1.1 Contrato cicatriz (L-BUNDLE-STALE v2)

**Actor:** `SddIA/scripts/build-release-bundle.sh` (invoca `cargo`; **no** el orquestador `execute-process`).

**Algoritmo de digest** (determinista):

1. Resolver raíz del crate del binario (`engine/|daemons/|tools/|interfaces/<name>/`).
2. Cierre de archivos = `Cargo.toml` + `src/` + `build.rs` si existe + dependencias `path =` **recursivas** (hoy: `sddia-io`, `sddia-daemon-runtime`; `sddia-core` si aparece) + `SddIA/Cargo.lock` + `SddIA/Cargo.toml` workspace.
3. `find` (no `target/`), `LC_ALL=C sort`, `sha256sum` por fichero, SHA-256 del listado `path<TAB>hex`. Prefijo de valor: `sha256:<hex64>`.
4. Prohibido `md5sum`. Prohibido `mtime` / `-newer`.

**Testigo** (`SddIA/target/${PROFILE_BIN}/<bin>.sha256`), escrito **solo** tras `cargo build` exitoso del propio script:

```text
source_sha256: sha256:<hex>
elf_sha256: sha256:<hex-del-ELF>
```

`elf_sha256` evita un ELF sustituido a mano con testigo huérfano de otra compilación.

**Skip-build (fail-closed):**

| Condición | Resultado |
|-----------|-----------|
| Digest working tree == `source_sha256` **y** SHA-256 del ELF == `elf_sha256` | Alineación → empaquetar |
| Testigo ausente (p. ej. `cargo` fuera del script) | Fractura |
| Cualquier divergencia de token / rama / stash / path-dep | Fractura |
| Fractura | exit ≠ 0; no `MANIFEST.json` de bundle; mensaje: omitir `--skip-build` |

**Perfil:** testigo junto al ELF de `PROFILE_BIN` (`release` vs `debug`). No copiar `.sha256` al bundle consumidor (sin fuentes que revalidar). Opcional: eco de `source_sha256` en `MANIFEST.json` (auditoría).

**Fuera de cicatriz (deuda explícita):** versión de `rustc`/target triple. Un `rustup update` no revoca `--skip-build`. No ampliar en este ciclo.

## 2. Circuito objetivo

```text
[bundle]
  build-release-bundle
    → (opcional cargo) → copy ELF
    → GATE strings sin execute-process.py
    → GATE --skip-build: cicatriz SHA-256 alineada (testigo vs working tree)
    → MANIFEST.json + ONBOARDING (inventario + LLM)

[deploy]
  instance-creator
    Topologia: starter-kit local.paths.json (no {})
    Systemd: @@SDDIA_CORE_ROOT@@ → instance_root
    Ignicion: según skip
    Smoke: topology+Local_QA; SI ignición → route-domain* success:true

[ignition]
  start-sddia.sh
    bundle (MANIFEST | sin Cargo.toml) → resolve ELF, sin cargo
    lab (Cargo.toml) → cargo + resolve

[correo]
  Triaje-C noise → cierra
  Clasificacion LLM → guard extracción asunto
    completo → actionable + subject_elevation
    REQUIRE_INFER + peaje 0 → extract o classification-degraded
  Email_Triaged → agenda/WUI/Telegram solo actionable
```

## 3. Touchpoints

| Área | Locus | Acción |
|------|-------|--------|
| Bundle | `SddIA/scripts/build-release-bundle.sh` + ONBOARDING generado | Gates L-BUNDLE-* + inventario |
| Ignición | `start-sddia.sh` (raíz; proyectado al bundle) | L-START |
| Creator motor | `SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs` | L-CORE-ROOT, L-STARTER, L-SMOKE-ROUTE |
| Creator contrato | `SddIA/process/instance-creator.md` | UPDATE entity-manager |
| Triaje | `SddIA/engine/execute-process/src/engine/handlers/email_triage.rs` | L-GUARD, L-PATH-LABEL, L-PROMPT, L-INFER |
| Matriz | `SddIA/library/norms/email-triage-matrix.md` | UPDATE §1 |
| Distribución | `SddIA/norms/sddia-distribution-protocol.md` | UPDATE gates |
| Tests | `email_triage.rs` + `instance_creator.rs` tests | O2, O3, O6, O7 |
| Auditoría | `docs/audits/` | O10 |
| Cascada | `persist_ref` | patrón documental |

## 4. Contratos de prueba (mínimo)

| ID | Caso | Esperado |
|----|------|----------|
| T-CORE | templates con `instance_root=/tmp/inst-x`, `repo=/tmp/forge` | `ExecStart` contiene `/tmp/inst-x/SddIA/` no `/tmp/forge` |
| T-PATHS | Topología sin `local.paths.json` | archivo = contenido starter-kit, no `{}` |
| T-GUARD | mock LLM `{"verdict":"passive"}` + asunto `Reunión con Racso el 25/08/2026 a las 10:00` | `actionable` + datetime + `subject_elevation` |
| T-NOISE | Triaje-C C-LIST + mismo asunto | `noise`; no elevación |
| T-INFER | `SDDIA_LLM_REQUIRE_INFER=1`, tokens 0, asunto sin fecha | `classification-degraded` + no `actionable` |
| T-START | `MANIFEST.json` presente, `Cargo.toml` ausente, ELF presente | `_ensure_orchestrator` no llama cargo (asserción por rama / test shell) |
| T-STALE | testigo escrito; mutar un `.rs` del cierre; `--skip-build` | exit ≠ 0; mensaje exige rebuild. Restaurar + rebuild → `--skip-build` OK |
| T-STALE-ABSENT | ELF presente, `.sha256` ausente, `--skip-build` | exit ≠ 0 (fail-closed; cargo externo no cuenta como cicatriz) |

## 5. Límites

- No mutar buzón IMAP.
- No versionar `.env` ni valores de bóveda.
- Redeploy Paciente 0 (O9) es verificación de Tekton/operador sobre instancia real; no mockear el gate G5 en unit tests.
