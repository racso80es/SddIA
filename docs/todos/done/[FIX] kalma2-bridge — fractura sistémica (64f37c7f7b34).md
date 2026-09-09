---
document_id: PBI-FIX-FRACTURE-64f37c7f7b34
uuid: "64f37c7f-7b34-4000-8000-000000000001"
title: "[FIX] kalma2-bridge — fractura sistémica (64f37c7f7b34)"
format: markdown
version: "1.2.0"
created: "2026-09-09"
updated: "2026-09-09"
status: "cerrado"
fix_ref: docs/fixes/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
refinement_status: refinado
refined: true
priority: alta
type: fix
process: bug-fix
fracture_hash: 64f37c7f7b34
fracture_process: kalma2-bridge
fracture_kind: prosthetic_collapse
incident_ref: "System_Fracture_Detected — 64f37c7f7b34"
suggested_branch: fix/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
persist_ref_suggested: docs/fixes/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
source_audit: >-
  2026-09-09T15:36-15:45 CEST Filtro A v1.2.0 sobre v1.1.0. Hash SHA-256[:12](error_trace.trim())
  = 64f37c7f7b34 re-verificado. Emisor kalma2-bridge: handle_chat (POST /api/chat) → resolve_mayeuta_llm
  Err → emit_system_fracture(..., fracture_kind=prosthetic_collapse, attempted_action=sse_chat_stream).
  Traza literal = 'mayeuta-llm no encontrado en SddIA/target/{debug,release}'. Mitigación lab 15:02:
  SddIA/target/debug/mayeuta-llm (7.0 MiB, ELF x86-64, mtime 15:02). Release de la prótesis aún ausente.
  Daemon vivo 15:45: pid 1244544 = SddIA/target/release/kalma2-bridge (mtime 15:17). Deslindado del 404
  /api/aiua/interact (PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF) y de cbe0c30b3695 (exit 1 runtime).
review_notes: >-
  v1.0.0 stub Cúmulo válido (unclassified → process_fix; traza sin cubo léxico). v1.1.0 acertó la causa
  física y el deslinde Aiúa, pero inyectó alucinaciones de genoma y CAs irreproducibles: friction_id
  F-PROSTHETIC-COLLAPSE (no existe en plantilla Cúmulo ni en el payload del puente; el campo real es
  fracture_kind), sse_chat_stream tratado como función (es attempted_action; la función es handle_chat),
  H6/§5.2.2 como causal (el orden debug→release NO derriba el sello: ambos ELF ausentes), CA3 --lib
  (crate binario sin [lib]), CA6 live-smoke + 'cero System_Fracture_Detected en pending' (script arranca
  debug propio, exige inferencia LLM, y el predicado global es falso). v1.2.0: causa física intacta;
  taxonomía alineada al payload; CAs recortados a host vs código; Kaizen clasificador diferido.
architectural_constraints:
  - A-FRACTURE-HASH-INMUTABLE
  - A-NO-BYPASS-RAW
  - A-NO-RETRY-BACKOFF-DA5
  - A-SEPARATION-CHAT-AIUA
  - A-DETERMINISTIC-PREREQUISITES
  - A-RELEASE-PROFILE-PARITY
  - A-NO-FRICTION-ID-INVENTADO
  - A-HANDLE-CHAT-VS-ATTEMPTED-ACTION
  - A-CA-HOST-VS-PR
  - A-PREFLIGHT-WARN-NOT-EXIT
gates_this_wave:
  - KALMA-LLM-CA1
  - KALMA-LLM-CA2
  - KALMA-LLM-CA3
  - KALMA-LLM-CA4
  - KALMA-LLM-CA6
deferred_gates:
  - KALMA-LLM-CA5
related:
  - docs/audits/kalma2-wui-tormentosa-chat-20260909.md
  - docs/todos/pending/[FIX] kalma2-bridge ELF release fósil — POST api-aiua-interact 404.md
  - docs/todos/done/[FIX] kalma2-bridge — fractura sistémica (cbe0c30b3695).md
  - docs/todos/done/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (60db1db67e49).md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/interfaces/kalma2-bridge/Cargo.toml
  - SddIA/skills/mayeuta-llm/src/main.rs
  - SddIA/skills/mayeuta-llm/Cargo.toml
  - SddIA/scripts/daemons/kalma2-bridge.sh
  - SddIA/scripts/common/sddia_shell_lib.sh
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/core/fracture_pbi.rs
  - SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
related_pbis:
  - id: PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF
    rol: "Incidente contemporáneo (2026-09-09) en el mismo órgano: 404 en POST /api/aiua/interact por ELF release de kalma2-bridge fósil (mtime 06-09, arranque 07-09, recycle 15:17/15:18 pid 1244544). Ortogonal: endpoint, handler (handle_aiua_interact) y prótesis (orquestador / aiua-stimulus-processing) distintos."
  - id: PBI-FIX-FRACTURE-cbe0c30b3695
    rol: "Fractura histórica (2026-07-20), traza 'mayeuta-llm/prótesis exit 1'. Allí el binario existía y el hijo salió 1. Aquí el preflight is_native_elf falló antes de Command::spawn."
  - id: PBI-FIX-FRACTURE-60db1db67e49
    rol: "Precedente del clasificador (2026-09-09): catch-all ya no usa el token 'failed'. Esta traza no contiene timeout|block|abort|colaps → unclassified=true, veredicto process_fix. Eso es correcto, no un fallo de 60db. Un cubo nuevo es higiene, no causa de este sello (KALMA-LLM-CA5 diferido)."
  - id: PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION
    rol: "SSOT anti-alucinación: friction_id no forma parte del genoma YAML de PBI de fractura (plantilla materialize-fracture-pbi). No reintroducir F-* inventados. Canal LLM asíncrono ≠ este bug-fix."
---

# [FIX] kalma2-bridge — fractura sistémica (64f37c7f7b34)

> **Refinamiento v1.2.0 (Filtro A).** Sello Cúmulo inmutable (`64f37c7f7b34`). Causa física: `resolve_mayeuta_llm` no halló ELF nativo y `handle_chat` emitió `System_Fracture_Detected` con `fracture_kind: prosthetic_collapse`. Mitigado en lab (debug 15:02). Deuda: artefacto release + preflight + test hermético + alineación de orden de perfiles. **No** es 404 Aiúa. **No** es `exit 1` de `cbe0c30b3695`. **No** hay `friction_id` en este evento.

---

## 1. Identidad del sello (inmutable)

| Campo | Valor | Notas |
|-------|-------|-------|
| `fracture_hash` | `64f37c7f7b34` | `SHA-256[:12]` de `error_trace.trim()` (`fracture_trace_hash`). Verificado 2026-09-09 15:39 CEST. Inmutable. |
| `fracture_process` | `kalma2-bridge` | Payload `process_name`. Demonio HTTP Kalma2 (bind default `:8765` vía `SDDIA_CLIENT_PORT`). |
| `fracture_kind` | `prosthetic_collapse` | Campo real del JSON emitido por `emit_system_fracture`. **No** existe `friction_id` en este payload ni en la plantilla Cúmulo. |
| Emisor | `kalma2-bridge` | `emitter_agent` / `agent_emitter` / `source`. |
| Acción intentada | `sse_chat_stream` | String `attempted_action`. **No** es el nombre de una función Rust. |
| Función HTTP | `handle_chat` | `POST /api/chat` → parseo `InteractReq` → `resolve_mayeuta_llm` → spawn. |
| Cápsula / Satélite | `mayeuta-llm` | Skill CLI (`SddIA/skills/mayeuta-llm`). Operación invocada: `STREAM`. |
| Endpoint | `POST /api/chat` | Consumido por WUI `#chat` / `Ctrl+Enter`. Distinto de `/api/interact` (Mayeuta sync) y `/api/aiua/interact` (Aiúa). |

### Traza de error literal (inmutable)

```
mayeuta-llm no encontrado en SddIA/target/{debug,release}
```

SSOT del sello: `SddIA/engine/execute-process/src/core/fracture_pbi.rs` → `fracture_trace_hash` = SHA-256 de `error_trace.trim()` recortado a 12 hex. Equivalente empírico:

```bash
printf '%s' 'mayeuta-llm no encontrado en SddIA/target/{debug,release}' | sha256sum
# 64f37c7f7b34ff7721e819b1e17fb688f0402028822d8374c12045b84b04b4c3
```

> **Mandato:** Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado. Prohibido alterar `fracture_hash`, `fracture_process` o la traza de error literal.

---

## 2. Hechos verificados (Filtro A v1.2.0, 2026-09-09)

Zona: CEST (UTC+2). Cruce: fuente HEAD, `file(1)`, `ps`, auditoría WUI, plantilla Cúmulo.

| Hecho | Evidencia |
|-------|-----------|
| **Integridad del hash** | `printf '%s' "<traza>" \| sha256sum` → `64f37c7f7b34ff77…`. Prefijo 12 = sello. Re-verificado 15:39. |
| **Origen en código** | `handle_chat` (`main.rs` ~L1189-1228): `resolve_mayeuta_llm(repo)` en `Err` llama `emit_system_fracture(repo, "prosthetic_collapse", &message, "sse_chat_stream")` y responde HTTP 500 con ese `message`. |
| **Resolver** | `resolve_mayeuta_llm` (~L1127-1146): (1) `$SDDIA_MAYEUTA_LLM_BIN` si `is_native_elf`; si la var está set y no es ELF → **otra** traza (`SDDIA_MAYEUTA_LLM_BIN no es ELF nativo`, otro sello). (2) `SddIA/target/debug/mayeuta-llm`. (3) `SddIA/target/release/mayeuta-llm`. Fallo de (2)+(3) = **esta** traza. Magia ELF = `\x7fELF` (existencia de fichero no-ELF no basta). |
| **Payload EDA** | `event_type: System_Fracture_Detected`, `event_family: domain`, escritura en `eda_bus.pending` (`./.events/pending` vía `cumulo.paths.json`). Campos extra respecto al contrato: `fracture_kind`, `source`. Contrato (`system-fracture-detected.md`) exige `process_name`, `error_trace`, `agent_emitter`, `attempted_action`. **No** lista `friction_id`. |
| **Plantilla Cúmulo** | `materialize_fracture_pbi.rs` `build_pbi_body`: `document_id`, `fracture_hash`, `fracture_process`, `incident_ref`, `status: abierto`. Sin `uuid`, sin `friction_id`. El `uuid` de este PBI es convención de refinamiento (sello → UUID v4-ish), no minteo Cúmulo. |
| **Mitigación lab** | `SddIA/target/debug/mayeuta-llm`: ELF x86-64, `7288152` bytes (7.0 MiB), mtime **2026-09-09 15:02**. `SddIA/target/release/mayeuta-llm`: **ausente** (15:39). |
| **Órgano vivo (15:39)** | pid **1244544**, exe `SddIA/target/release/kalma2-bridge`, arranque **15:18:25**, ELF mtime **15:17**. Con el resolver actual (debug primero) este proceso **usaría** la prótesis debug si atiende `/api/chat`. No se leyó `/proc/<pid>/environ` (`SDDIA_MAYEUTA_LLM_BIN`) — permiso denegado; no afirmar override. |
| **Auditoría WUI** | `docs/audits/kalma2-wui-tormentosa-chat-20260909.md` M1: chat WUI APTO tras compile 15:02. Probe curl ~15:08: `POST /api/chat` SSE **25 s sin bytes**; la WUI del Vértice sí completó. No equivaler «WUI APTO» a «curl SSE siempre entrega frames». |
| **Cargo** | `kalma2-bridge/Cargo.toml`: `[[bin]]` only, sin `[lib]`, **sin** dependencia de crate `mayeuta-llm`. `cargo build -p kalma2-bridge` no construye la prótesis. |
| **Lanzador** | `kalma2-bridge.sh`: resuelve **solo** el ELF de `kalma2-bridge` (`_sddia_resolve_daemon_binary`, orden **release luego debug** + chequeo fósil). Cero preflight de `mayeuta-llm`. |
| **Bus al re-auditar** | El JSON de este sello **no** permanece en `.events/` (cola volátil). El instante `14:59:05` de v1.1.0 **no es re-verificable**: mtime del PBI = refinamientos posteriores; fichero aún untracked. Conservar «~14:59 CEST» como declaración v1.1.0, no como cronómetro de la petición. |
| **Deslinde 404** | 404 = dispatcher del ELF fósil de **kalma2-bridge** sin ruta `/api/aiua/interact`. Independiente de `mayeuta-llm`. Recycle 15:18 cerró el 404 (CA de aquel PBI, no de este). |

---

## 3. Qué hizo el stub v1.0.0 (Kintsugi / Mayeuta)

El stub auto-generado diagnosticó:
- **Causa raíz:** «Causa raíz no clasificada automáticamente para `kalma2-bridge`; requiere laudo humano.»
- **Veredicto:** `process_fix`.
- **Propuesta:** «Auditar proceso `kalma2-bridge`, acción `sse_chat_stream` y emisor `kalma2-bridge`.»

### 3.1 Conducta correcta del clasificador post-60db1db67e49

`analyze_fracture_kaizen` concatena `error_trace + attempted_action + process_name` en minúsculas. Catch-all residual: `timeout|block|abort|colaps` → `prompt_adjustment`. Esta traza **no** contiene esos tokens ni `failed`. `root_causes.is_empty()` → `unclassified = true` → fallback `process_fix` + laudo humano.

Eso es el diseño honesto tras retirar `failed` del catch-all. **No** es un defecto de clasificación de este sello.

### 3.2 Kaizen sistémico (diferido — no gate de esta ola)

Un cubo léxico `mayeuta-llm no encontrado` / `prosthetic_collapse` + `kalma2-bridge` mejoraría el texto del stub. No evita el colapso. No muta el puente. Pertenece a higiene de `enrich_fracture_pbi_kaizen.rs` (crate `execute-process`, sí tiene `[lib]` implícito vía `src/lib.rs`). **KALMA-LLM-CA5 diferido.** No mezclar persist_ref. Respetar `L-ENRICH-KINTSUGI-DETERMINISTA` (matcher léxico, cero LLM en enrich).

---

## 4. Discriminación de hipótesis

| # | Hipótesis | Evaluación | Veredicto |
|---|-----------|------------|-----------|
| **H1** | Bug de sintaxis / panic en `mayeuta-llm` | El sello se emite **antes** de `Command::spawn`. El debug ELF de 15:02 demuestra que el crate compilaba. No prueba calidad de `STREAM`/`SYNTHESIZE`/`CLASSIFY_INTENT` en runtime. | **Refutada como causa de este sello** |
| **H2** | Fallo de parseo JSON en `/api/chat` | Parseo `Err` o prompt vacío → HTTP 400 `"prompt requerido"` **sin** `emit_system_fracture`. La emisión implica `InteractReq.prompt` no vacío. | **Refutada** |
| **H3** | El subproceso falló en runtime / timeout / `exit 1` | Esas ramas usan trazas distintas (`spawn mayeuta-llm: …`, watchdog, `mayeuta-llm/prótesis exit 1`). Esta traza es exclusiva del `Err` de `resolve_mayeuta_llm` por paths default. | **Refutada** |
| **H4** | Solapamiento con 404 Tormentosa | `/api/chat` → `handle_chat` → `mayeuta-llm`. `/api/aiua/interact` → `handle_aiua_interact` → orquestador / `aiua-stimulus-processing`. | **Refutada** |
| **H5** | Causa física: prótesis satélite no compilada / no desplegada | Traza literal = ambos candidatos default fallaron `is_native_elf`. Compile debug 15:02 → chat WUI APTO (auditoría M1). | **CONFIRMADA (causa de este sello)** |
| **H6** | Orden de perfiles debug→release | Asimetría **real** frente a `_sddia_resolve_daemon_binary` (release→debug). **No causal** aquí: a ~14:59 faltaban **ambos** ELF. Si existiera solo release, el resolver actual **sí** lo encontraría (candidato 3). El orden **oculta** la ausencia de release cuando debug existe (estado post-15:02). | **Defecto latente confirmado en código; no causa del sello** |

---

## 5. Causa estructural

### 5.1 Desacoplamiento Cargo sin preflight de satélite

`kalma2-bridge` spawnea satélites (`execute-process`, `mayeuta-llm`) que **no** son crates-dependencia. `cargo build -p kalma2-bridge` (ni el lanzador) materializa `mayeuta-llm`. Host limpio / `cargo clean` → el demonio arranca y colapsa al primer `POST /api/chat`.

El mismo patrón existe en `resolve_orchestrator` (también debug→release). **Fuera de este sello** (el 404 Aiúa es ELF del puente, no del orquestador).

### 5.2 Asimetría de perfiles (latente, no causal)

| Resolver | Orden |
|----------|-------|
| `_sddia_resolve_daemon_binary` | **release** → debug (+ mtime ≥ fuente) |
| `resolve_mayeuta_llm` | **debug** → release (sin chequeo fósil) |

Efectos reales:
1. Con ambos perfiles de la prótesis presentes, un daemon **release** ejecuta prótesis **debug**.
2. Con solo debug presente (estado 15:39), el daemon release enmascara la deuda release.
3. **Falso (v1.1.0 §5.2.2):** «si no hay `mayeuta-llm --release` el bridge colapsa aunque exista release del propio bridge». El colapso requiere que **tampoco** exista debug (o que el override env no sea ELF). El release del puente **nunca** implica la prótesis; son paquetes distintos. Eso es §5.1, no el orden.

### 5.3 Tests

Existe `resolve_orchestrator_finds_debug_binary` (~L2404): depende del ELF **real** en `SddIA/target`. **No** existe test de `resolve_mayeuta_llm`. Un test que copie ese patrón fallará en CI/host sin `cargo build -p mayeuta-llm` — el mismo agujero de producción. El test de esta ola debe ser **hermético** (tempdir + magia `\x7fELF` y/o `SDDIA_MAYEUTA_LLM_BIN`).

### 5.4 Clasificador

Sin cubo para esta traza → stub genérico. Conducta prevista. Higiene ≠ causa (§3.2).

---

## 6. Alcance (`bug-fix`)

### Dentro (esta ola)

1. **Host / laboratorio (no va en el PR):** `cargo build --release -p mayeuta-llm` con `CARGO_TARGET_DIR` del workspace → `SddIA/target/release/mayeuta-llm` ELF ejecutable. `target/` está gitignored.
2. **Código — orden:** `resolve_mayeuta_llm` busca **release luego debug**, paridad con `_sddia_resolve_daemon_binary`. No exigir chequeo fósil mtime en esta ola (el lanzador ya lo hace para el puente, no para satélites).
3. **Código — preflight del lanzador:** aviso en stderr **antes** del `exec` si no hay ELF de `mayeuta-llm` (override env o `target/{release,debug}`), con el comando de compile. **No `exit 1`:** el puente sirve estáticos, `/api/status` y `/api/aiua/interact`; tumbar el órgano entero rompería Tormentosa.
4. **Código — test hermético** de resolución (override env + fixture ELF; caso negativo → mensaje exacto de esta traza).
5. **Verificación de no-regresión de ESTE sello:** `POST /api/chat` con prompt no vacío **no** responde 500 cuyo `message` sea la traza literal. No exigir stream LLM completo.

### Fuera

- Frontend (`interfaces/kalma2/app.js`, `index.html`) y protocolo SSE.
- `/api/aiua/interact`, recycle del puente, sanitización 503 Gemini (otros PBI).
- Mutar `SddIA/skills/mayeuta-llm` (genoma; DA-2) salvo compile.
- Cubo léxico en `enrich_fracture_pbi_kaizen.rs` (**CA5 diferido**).
- Alinear `resolve_orchestrator` al orden release-first (mismo olor; otro sello si se abre).
- Reintentos / backoff en el handler (DA-5).
- Bypass raw.
- `kalma2-chat-sse-live-smoke.sh` como gate: arranca `target/debug/kalma2-bridge` propio, si `:8765` ocupado usa `18765`, exige inferencia real (`SDDIA_LLM_REQUIRE_INFER`). No prueba el órgano release ni este sello.
- Predicado «cero `System_Fracture_Detected` en `.events/pending/`»: la cola es compartida y volátil; `kalma2-sse-fracture-smoke.sh` **emite** fracturas a propósito (`sse_watchdog`).

---

## 7. Afirmaciones descartadas (v1.0.0 y v1.1.0)

| Afirmación | Clasificación | Corrección |
|------------|---------------|------------|
| «Causa raíz no clasificada, requiere laudo humano» | *Vacío descriptivo del stub, no error* | Causa física: ningún candidato default pasó `is_native_elf`. El clasificador hizo lo correcto. |
| `friction_id: F-PROSTHETIC-COLLAPSE` | *Alucinación de genoma (v1.1.0)* | Payload = `fracture_kind: prosthetic_collapse`. Plantilla Cúmulo y contrato del evento no declaran `friction_id`. No acuñar `F-*` en este PBI. |
| «`sse_chat_stream` llama a `resolve_mayeuta_llm`» | *Inexactitud de símbolo (v1.1.0)* | Función = `handle_chat`. `sse_chat_stream` = `attempted_action`. |
| «H6 confirmada como causa» / «sin release de la prótesis el bridge colapsa» | *Inversión causal (v1.1.0)* | Este sello = ambos perfiles ausentes. El orden es deuda latente de paridad. |
| «El chat está caído» | *Falso post-15:02 (WUI)* | Canal WUI APTO con debug. Probe curl SSE no es oráculo de frames. Deuda = release + ciclo de vida. |
| «64f37c7f7b34 causa el 404 Tormentosa» | *Contigüidad temporal* | ELF fósil del **puente**. Ver PBI 404 + auditoría. |
| «mayeuta-llm crasheó» | *Confusión con cbe0c30b3695* | No hubo spawn. |
| «Compilar kalma2-bridge restaura el chat» | *Dependencia Cargo inexistente* | Hace falta `cargo build [-p mayeuta-llm]`. |
| «CA3: `cargo test --lib`» | *Inexactitud de crate (v1.1.0)* | `kalma2-bridge` es binario sin `src/lib.rs`. Comando: `--bin kalma2-bridge`. |
| «CA6: live-smoke + cero fracturas en pending» | *CA irreproducible / predicado global falso (v1.1.0)* | Ver §6 Fuera. |
| «Instante 14:59:05 = POST cursado» | *Cronómetro no re-verificable* | Declaración v1.1.0; evento no persistido; mtime del md sobrescrito. |
| «Binario 7.2 MB» | *Redondeo* | `7288152` bytes = 7.0 MiB. |
| «`failed` en catch-all habría alucinado prompt_adjustment» | *Contrafáctico innecesario* | Esta traza no contiene `failed`. El catch-all vigente es `colaps` etc. |

---

## 8. Criterios de aceptación (Gates de esta ola)

| ID | Criterio | Verificación |
|----|----------|--------------|
| **KALMA-LLM-CA1** | **Host.** Existe ELF ejecutable `SddIA/target/release/mayeuta-llm`. No es artefacto de git. | `test -x SddIA/target/release/mayeuta-llm && file SddIA/target/release/mayeuta-llm \| grep -q ELF` |
| **KALMA-LLM-CA2** | `resolve_mayeuta_llm` prioriza `SddIA/target/release/mayeuta-llm` sobre `debug`. Override `SDDIA_MAYEUTA_LLM_BIN` intacto. | Inspección del `for rel in` + test CA3 con ambos fixtures. |
| **KALMA-LLM-CA3** | Test hermético en `kalma2-bridge` (tempdir u override env; magia ELF; caso ausente = traza literal). Sin depender de `SddIA/target` real. | `cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge -- resolve_mayeuta_llm` |
| **KALMA-LLM-CA4** | `kalma2-bridge.sh` emite WARN en stderr si no hay prótesis, con `cd SddIA && cargo build --release -p mayeuta-llm`. **No** `exit 1` por esta ausencia. | Ejecutar el script con ELF del puente presente y prótesis ausente (staging); el proceso del puente arranca; stderr contiene el aviso. |
| **KALMA-LLM-CA6** | Con prótesis resoluble, `POST /api/chat` `{prompt}` no vacío **no** devuelve 500 cuya `message` sea la traza literal de este sello. | Probe HTTP al órgano bajo test (puerto del lab). No exigir tokens LLM ni «pending vacío». |

### Diferido (no bloquea cierre de esta ola)

| ID | Criterio | Nota |
|----|----------|------|
| **KALMA-LLM-CA5** | Cubo léxico en `analyze_fracture_kaizen` para esta traza → `process_fix` tipificado, `unclassified=false`. | Crate `execute-process` (`cargo test -p execute-process --lib -- analyze_fracture_kaizen`). Otro persist_ref si se despacha. |

---

## 9. Criterio de cierre

- [x] **KALMA-LLM-CA1** (host release).
- [x] **KALMA-LLM-CA2** (orden release-first).
- [x] **KALMA-LLM-CA3** (test hermético).
- [x] **KALMA-LLM-CA4** (WARN lanzador).
- [x] **KALMA-LLM-CA6** (no-regresión de esta traza).
- [ ] Argos `APTO` en `docs/fixes/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34/validacion.md` (`pbi_archived: true`). CA-CI pendiente de run verde post-PR.
- [x] Este PBI en `docs/todos/done/` en la **misma** rama del fix.

KALMA-LLM-CA5 no es checkbox de Done de este sello.

---

## 10. Conclusión Analítica y Propuesta Evolutiva

*(Síntesis corregida — Kintsugi / Filtro A v1.2.0)*

### Diagnóstico de causa raíz

1. **Ausencia de ELF satélite:** `POST /api/chat` → `handle_chat` → `resolve_mayeuta_llm`. Sin override env válido y sin `SddIA/target/{debug,release}/mayeuta-llm` nativos, el preflight emitió el sello `64f37c7f7b34` (`fracture_kind: prosthetic_collapse`, `attempted_action: sse_chat_stream`).
2. **Cadena de build:** `kalma2-bridge` no declara ni preflightea `mayeuta-llm`. El lanzador solo garantiza el ELF del puente.
3. **Deuda latente (no este sello):** orden debug→release en el resolver de la prótesis, opuesto al lanzador.

### Veredicto evolutivo

**`process_fix`**. Prohibido `prompt_adjustment`. Prohibido atribuir a Aiúa, red o `exit 1` de `cbe0c30b3695`.

### Propuestas (esta ola)

1. Compilar `mayeuta-llm` **release** en el host.
2. Invertir precedencia en `resolve_mayeuta_llm` (release → debug).
3. WARN de preflight en `kalma2-bridge.sh` (sin matar el órgano).
4. Test hermético del resolver y de la traza literal.

### Propuesta diferida

Matcher léxico en `enrich_fracture_pbi_kaizen.rs` para esta tipología. No gate.
