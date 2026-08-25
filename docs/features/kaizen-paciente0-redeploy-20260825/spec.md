---
feature_name: kaizen-paciente0-redeploy-20260825
created: "2026-08-25"
process: feature
base: main
scope: kaizen-paciente0-redeploy-20260825
version_spec: "1.0.0"
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
uuid: "d4f13e9a-5d91-4ab8-a2f5-be2e6b8c4815"
laudo: absorber-fricciones-post-absorcion-un-pr
execution_id: "7fd0a353-d2fe-4895-8abe-d7f5b34f652c"
---

# Especificación — kaizen-paciente0-redeploy-20260825

## 1. Laudos Dedalo

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L-RESOLVE** | ¿Debug vs release? | Si ambos ELF nativos existen: usar **debug solo si su mtime > release**; en empate o debug más viejo → **release**. Un solo candidato → ese. Honor `SDDIA_EXECUTE_PROCESS_BIN` si está set (lab). | F-DEP-07. Lab `cargo build -p` produce debug fresco; debug stale no gana a release Kaizen. Cicatriz `.sha256` no se relee en cada `sddia-run` (coste); mtime del ELF es oráculo suficiente aquí. |
| **L-STUB** | ¿`local.paths.json` existente? | Existencia no basta. Si ausente, vacío o JSON objeto `{}` → copiar starter-kit (o fallback embebido). Overlay **válido** no se pisa. | F-DEP-08. Antecesor L-STARTER cubría solo «no existe». |
| **L-PIN-BUNDLE** | ¿Herencia `SDDIA_EXECUTE_PROCESS_BIN`? | En ignición **bundle** (`MANIFEST.json` presente **o** sin `SddIA/Cargo.toml`): si el pin no está bajo `REPO_ROOT` de la instancia → `unset` y resolver local. Lab (hay `Cargo.toml`) conserva el pin. | F-DEP-09. Pin de forja no es contrato de instancia. Vault instancia puede pinnear ELF del bundle (path bajo raíz). |
| **L-QA-EMIT** | ¿`Local_QA_Requested` en smoke creator? | **No emitir.** Clase exige `payload.branch`; emisor autorizado = `git-hook-pre-push`. Topology nativo + `route-domain*` (si ignición) bastan. | F-SMOKE-01. Completar `branch` inventado viola Filtro A (emisor). |
| **L-SYS** | ¿`install_user_unit`? | **Fuera de forja de este PR.** Residual `DT-SYSTEMD-USER-ENABLE`. No bloquea Done (O6). | Ceguera espacial: `~/.config/systemd/user/` es host, no Cúmulo. |
| **L-CREATOR-MD** | ¿Contrato process? | UPDATE `instance-creator` vía `entity-manager`: Topología sustituye stub `{}`; Smoke **sin** emitir `Local_QA_Requested`. Handler motor. | DA-2. |
| **L-DIST** | ¿Protocolo Vía C? | Parche bajo feature (Core `directories.norms`; no `norm-creator`): resolver orquestador L-RESOLVE; overlay stub; ignición L-PIN-BUNDLE; smoke sin Local_QA. Bump prosa; SemVer norma **1.2.1**. | Mismo locus que T6 antecesor. |
| **L-AUDIT** | ¿Audit? | Archivo **nuevo** `docs/audits/kaizen-paciente0-redeploy-20260825-residual.md`. No reescribir T6. | PBI §7 / D9. |
| **L-FORGE** | ¿Mutación? | Genoma process solo `entity-manager`. Engine + `sddia_shell_lib.sh` + `start-sddia.sh` + norma Core: Tekton bajo esta topología (DA-4). | DA-2/DA-4. |

### Rechazados

- Preferir **siempre** release (castra lab debug fresco).
- `unlink` operador / pin ELF como runbook.
- Emitir `Local_QA_Requested` con `branch` ficticio.
- Enable systemd user en creator (L-SYS).
- Auto-merge bóvedas; G5; F-TRIAGE-*; wizard.
- Reabrir F-DEP-01…04 en handler release.

## 2. Circuito objetivo

```text
[resolve]  sddia-run / start-sddia
  env pin → honor (salvo L-PIN-BUNDLE)
  debug+release → debug iff mtime(debug) > mtime(release)
  else → el que exista

[creator.Topologia]
  local.paths.json missing|empty|{} → starter-kit
  else → no-op

[creator.Smoke]
  topology checks (incl. overlay no-{})
  NO Local_QA_Requested
  skip_ignition → no route-domain*
  ignición → route-domain-event success:true

[ignition bundle]
  pin fuera de REPO_ROOT → discard
  resolve bajo instancia
```

## 3. Touchpoints

| Área | Locus | Acción |
|------|-------|--------|
| Resolver | `SddIA/scripts/common/sddia_shell_lib.sh` | L-RESOLVE |
| Ignición | `start-sddia.sh` `_ensure_orchestrator` | L-PIN-BUNDLE |
| Creator | `instance_creator.rs` | L-STUB, L-QA-EMIT |
| Contrato | `SddIA/process/instance-creator.md` | entity-manager UPDATE |
| Norma | `SddIA/norms/sddia-distribution-protocol.md` | 1.2.1 bajo feature |
| Tests | `instance_creator.rs` `#[cfg(test)]` | stub `{}`; smoke sin pending Local_QA |

## 4. Criterios (mapeo O*)

| O | Verificación |
|---|----------------|
| O1 | Ambos ELF: debug más viejo → `SDDIA_EXECUTE_PROCESS_BIN` = release |
| O2 | Creator sobre `{}` → starter-kit (`local_tools`) |
| O3 | Redeploy un creator (T6 operador) |
| O4 | `start-sddia` bundle + pin forja → log orquestador bajo instancia |
| O5 | Smoke nativo: cero JSON `Local_QA_Requested` en pending |
| O6 | N/A este PR (diferido) |
| O7 | Ya cerrado operador |
| O8–O9 | Audit + cierre documental T7 |
