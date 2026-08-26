---
feature_name: bundle-consumer-telegram-gateway
created: "2026-08-26"
process: bug-fix
base: main
scope: f-bundle-06-telegram-gateway-capsule
branch_name: fix/bundle-consumer-telegram-gateway
persist_ref: docs/fixes/bundle-consumer-telegram-gateway
pbi_document_id: PBI-FIX-BUNDLE-TELEGRAM-GATEWAY
friction_id: F-BUNDLE-06
---

# Especificación — F-BUNDLE-06: `telegram-gateway` en bundle consumidor

## Diagnóstico

| Campo | Valor |
|-------|-------|
| Síntoma Paciente 0 | `telegram-watcher` journal: `gateway rc=1`; cero `TelegramMessage_Received` |
| Error runtime | cápsula tool `telegram-gateway` no encontrada bajo `SddIA/target` |
| Evidencia ola 5 | `MANIFEST.json` `20260826T110203Z`: 7 bins / 7 capsules — incluye `telegram-watcher` + `send-telegram-notification`, **no** `telegram-gateway` |
| Canal OK (control) | Correo → `send-telegram-notification` (sí empaquetado) |
| Canal KO | Chat → `telegram-watcher` → proceso `telegram-gateway` → tool `telegram-gateway` |

Cadena cortada en **G** (tool ausente):

```text
telegram-watcher → execute-process --process telegram-gateway
  → tool:telegram-gateway  ✗ (ELF/md no en bundle)
  → .events/domain TelegramMessage_Received
```

## Causa raíz

`SddIA/scripts/build-release-bundle.sh` semilla `CONSUMER_BINS` / `CAPSULE_SET` con:

- `telegram-watcher` (daemon)
- `send-telegram-notification` (tool eferente vía suscripciones domain)

El escáner F-06 (`_scan_md_for_tools` sobre códice + `event-domain-subscriptions.json`) **no** deriva dependencias aferentes de daemons (`--process telegram-gateway` / `delegates_to: tool:telegram-gateway`). Resultado: el grafo eferente de notificaciones queda cubierto; el grafo aferente conversacional no.

La tool y el crate existen en forja (`SddIA/tools/telegram-gateway`, `SddIA/tools/telegram-gateway.md`, proceso `SddIA/process/telegram-gateway.md`); el defecto es **solo de empaquetado hermético**, no de genoma ausente.

## Decisión de diseño (P0)

Alinear `telegram-gateway` con el patrón ya usado por `send-telegram-notification`:

1. **Semilla explícita** en `CONSUMER_BINS` (o `_add_capsule` inmediato post-semilla si se prefiere no listarlo como “bin runtime” semántico — equivalente operativo: debe entrar en `CAPSULE_SET`, `cargo -p`, copia ELF y testigo `.sha256`).
2. **Gate fail-closed F-06 ampliado:** si el stage contiene ELF `telegram-watcher`, exigir:
   - `SddIA/tools/telegram-gateway.md` presente
   - ELF ejecutable `SddIA/target/release/telegram-gateway` (y dual debug/release como el resto)
3. **Cargo:** `-p telegram-gateway` en el build del generador (la iteración sobre `CAPSULE_SET` ya lo cubre si el crate está bajo `SddIA/tools/`; eliminar dependencia de la línea hardcodeada incompleta o ampliarla para no divergir).
4. **Testigo:** escribir/verificar `{telegram-gateway}.sha256` en el mismo ciclo que el resto de `CONSUMER_BINS` / cápsulas críticas (L-BUNDLE-STALE).
5. **Manifiesto:** `telegram-gateway` ∈ `binaries` y `capsules_resolved`.

### Norma (documental, mismo PR)

Actualizar `SddIA/norms/sddia-distribution-protocol.md` § Resolución de cápsulas (F-06): gate mínimo = `send-telegram-notification` **y**, si `telegram-watcher` ∈ paquete, `telegram-gateway` (`.md` + binario).

### Fuera de alcance P0

| Ítem | Motivo |
|------|--------|
| Escaneo genérico de eferentes desde fuentes daemon (`--process` en Rust) | P2 / DT; no bloquea cierre F-BUNDLE-06 |
| Parche en instancia `SddIA_AP` | Prohibido; fix solo en forja vía bundle |
| Tokens Telegram / dual-watcher | Ya descartados por auditoría ola 5 |
| Proceso `paciente0-deploy` | Feature distinta |

## Touchpoints (Tekton)

| Path | Mutación |
|------|----------|
| `SddIA/scripts/build-release-bundle.sh` | Semilla + cargo + gate + testigo |
| `SddIA/norms/sddia-distribution-protocol.md` | Documentar gate F-06 ampliado |
| Prompt/DEUDA G-bundle (opc. si se toca en mismo ciclo) | Ya exige ELF+md; no requiere cambio si el generador falla cerrado |

**No mutar** genoma de `tool:telegram-gateway` / `process:telegram-gateway` / daemon (sin defectos funcionales en lab).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `build-release-bundle.sh --profile consumer` (sin `--skip-build` si hace falta) produce ELF `SddIA/target/release/telegram-gateway` en el stage y lo lista en `MANIFEST.json` (`binaries` ∪ `capsules_resolved`). |
| CA2 | Gate del script aborta con error F-06/F-BUNDLE-06 si `telegram-watcher` está en stage y falta ELF o `.md` de `telegram-gateway`. |
| CA3 | Testigo `telegram-gateway.sha256` escrito; `--skip-build` con cicatriz válida no omite la cápsula. |
| CA4 | Redeploy Paciente 0 **sin** copia manual de ELF: `./sddia-run.sh --process telegram-gateway --inputs '{"text":"sigues?"}'` → `success:true`, `emitted:true`. |
| CA5 | Mensaje de prueba al bot instancia → journal **sin** `gateway rc=1`; evento domain o respuesta Tormentosa/Aiúa / `telegram-fallback-responder` (G-telegram). |
| CA6 | Cierre documental en rama: PBI → `docs/todos/done/`; `validacion.md` `global: APTO`, `pbi_archived: true` en el mismo PR. |

## Verificación sugerida (Argos)

1. Diff script: presencia de `telegram-gateway` en semilla/gate.
2. Smoke local forja: generar bundle a `dist/` temporal; `test -x …/telegram-gateway`; `jq` sobre `MANIFEST.json`.
3. Negativo: quitar temporalmente la cápsula del stage simulado → gate falla (si se instrumenta en ejecución; si no, revisión estática del `if` fail-closed).
4. Post-merge redeploy: contrastar F-BUNDLE-06 cerrado en audit Paciente 0.

## Blueprint de proceso

No aplica. Defecto acotado a generador de bundle + norma F-06; no se instancia proceso nuevo ni `plan.md`.
