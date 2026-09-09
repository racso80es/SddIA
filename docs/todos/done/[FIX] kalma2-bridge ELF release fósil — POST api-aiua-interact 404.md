---
document_id: PBI-FIX-KALMA2-BRIDGE-AIUA-ROUTE-STALE-ELF
uuid: "bd611423-eea8-446a-a405-79cc08685e38"
title: "[FIX] kalma2-bridge ELF release fósil — POST /api/aiua/interact 404"
format: markdown
version: "1.3.0"
created: "2026-09-09"
updated: "2026-09-09"
status: "cerrado"
closed: "2026-09-09"
fix_ref: docs/fixes/kalma2-bridge-aiua-interact-stale-elf
refinement_status: refinado
refined: true
priority: alta
type: fix
process: bug-fix
dispatch: false
suggested_branch: fix/kalma2-bridge-aiua-interact-stale-elf
persist_ref_suggested: docs/fixes/kalma2-bridge-aiua-interact-stale-elf
spawned_by: AUDIT-KALMA2-WUI-TORMENTOSA-CHAT-20260909
depends_on: []
incident_ref: "Auditoría empírica WUI 2026-09-09 — 404 ruta desconocida en /api/aiua/interact"
source_audit: >-
  2026-09-09T15:08 CEST auditoría empírica kalma2-wui-tormentosa-chat-20260909.md.
  Host: kalma2-bridge PID 6151, ELF release mtime 2026-09-06 06:48:40 CEST, arranque 2026-09-07 14:34:55 CEST.
  HEAD 479536b. PR #276: head 492c673 (09:14:12 CEST); merge commit 4b212aec (09:27:15 CEST, GitHub mergedAt 2026-09-09T07:27:19Z).
  La ruta ya estaba en main.rs del HEAD; el órgano HTTP no se recicló.
  Remediación host 15:17–15:18 CEST: cargo build --release -p kalma2-bridge +
  systemctl --user restart sddia-kalma2-bridge@home-racso-Proyectos-SddIA
  → PID 1244544, ELF mtime 15:17:48 CEST, journal «lock huérfano pid=6151».
  Ruta /api/aiua/interact deja de devolver 404. Deslindado de 64f37c7f7b34 y del residual 503 de proveedor.
review_notes: >-
  v1.2.0 acertó split-brain, guarda del lanzador y deslinde 404/503/fractura, pero inyectó:
  (1) enlaces file:///home/racso/... (ceguera espacial); (2) citó todos-jurisdiction.md como
  prohibición de '/' en el filename (la norma no lo dice; es semántica POSIX);
  (3) confló commit 492c673 09:14 con el merge 4b212aec 09:27; (4) «tests de integración»
  cuando validacion del puente es crate + lab-mock y CA-5 dice «Live no gate»;
  (5) «aceptado de inmediato» vs curl post-recycle HTTP 500 en 81s; (6) CA-4 /api/interact
  como no-regresión post-recycle sin probe; (7) «formalmente cerrado» con status abierto
  y PBI aún en pending/. v1.3.0: rutas lógicas, cronología GitHub, CAs recortados a evidencia,
  cierre físico ≠ cierre documental.
architectural_constraints:
  - A-NO-DUPLICATE-LISTENERS-8765
  - A-NO-AUTO-COMPILE-HOT-PATH
  - A-SEPARATION-CHAT-AIUA
  - A-SEPARATION-404-503
  - A-NO-RETRY-BACKOFF-DA5
  - A-FRESHNESS-RESOLVER-ALREADY-ENFORCED
  - A-RECYCLE-POST-MERGE-DAEMON
  - A-LOGICAL-PATHS-NOT-FILE-URI
  - A-PHYSICAL-VS-DOCUMENTAL-CLOSURE
gates_this_wave:
  - KALMA-STALE-CA1
  - KALMA-STALE-CA2
  - KALMA-STALE-CA3
  - KALMA-STALE-CA4
  - KALMA-STALE-CA5
deferred_gates:
  - KALMA-STALE-CA6
related:
  - docs/audits/kalma2-wui-tormentosa-chat-20260909.md
  - docs/todos/pending/[OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry).md
  - docs/todos/pending/[FIX] kalma2-bridge — fractura sistémica (64f37c7f7b34).md
  - docs/todos/done/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
  - docs/todos/done/[KAIZEN] Aiúa — hallazgos auditoría live y thinking HIGH.md
  - docs/todos/kitchen/PBI-MULTI-LLM-ROUTER.md
  - docs/features/kalma2-aiua-perceptive-bridge/validacion.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/scripts/daemons/kalma2-bridge.sh
  - SddIA/scripts/common/sddia_shell_lib.sh
  - .SddIA/systemd/sddia-kalma2-bridge@.service
  - interfaces/kalma2/app.js
related_pbis:
  - id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
    rol: "Origen funcional. PR #276: head 492c673 (09:14 CEST), merge 4b212aec (09:27 CEST). Implementó POST /api/aiua/interact, handle_aiua_interact y #aiua-pulse. validacion.md APTO sobre crate + lab-mock; CA-5 del puente declara «Live no gate». No recicló el daemon systemd del host."
  - id: PBI-FIX-FRACTURE-64f37c7f7b34
    rol: "Incidente contemporáneo en el mismo daemon: fractura por ausencia del ELF mayeuta-llm al invocar POST /api/chat. Ortogonal: endpoint, handler y prótesis distintos."
  - id: PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE
    rol: "Residual post-recycle: 2ª pulsación WUI. Gemini upstream HTTP 503 UNAVAILABLE; el puente mapea a HTTP 500 con message http-status-503. Ruta despachada (no es 404). Sanitización epidermal sin retry (DA-5)."
  - id: PBI-MULTI-LLM-ROUTER
    rol: "Kitchen: failover multi-proveedor ante 503/429. Fuera de alcance."
---

# [FIX] kalma2-bridge ELF release fósil — POST /api/aiua/interact 404

> **Refinamiento v1.3.0 (Filtro A).** Split-brain de ciclo de vida: el daemon `kalma2-bridge` servía estáticos frescos desde `interfaces/kalma2/` (`#aiua-pulse` visible), pero el proceso `pid 6151` ejecutaba un ELF release de 2026-09-06, anterior a PR #276, y respondía `404 ruta desconocida`. **No** es defecto de `main.rs`. **No** falta guarda de frescura en el lanzador (`_sddia_resolve_daemon_binary` ya compara mtime ELF ≥ fuente del crate). Un proceso ya spawnado no reevalúa esa guarda. Remediación **física** completada en host (`pid 1244544`). El 503 de Gemini → [PBI-OPERATIVO-KALMA2-AIUA-503-SANITIZE](../pending/%5BOPERATIVO%5D%20Kalma2%20WUI%20%E2%80%94%20503%20Gemini%20sanitizado%20en%20epidermis%20%28sin%20retry%29.md). Cierre **documental** (CA-6): este PBI en `docs/todos/done/` con `status: cerrado`; `fix_ref` `docs/fixes/kalma2-bridge-aiua-interact-stale-elf`.

---

## 1. Incidente y evidencia empírica inicial

Durante la auditoría [kalma2-wui-tormentosa-chat-20260909.md](../../audits/kalma2-wui-tormentosa-chat-20260909.md), al pulsar `#aiua-pulse` (`Hablar con Tormentosa`), `#output` mostró:

```text
[error] ruta desconocida
```

Origen: `app.js` pinta `body.message` del JSON del puente. El 404 del dispatcher es `{"success":false,"message":"ruta desconocida","exit_code":1}`.

El canal `#chat` / `Ctrl+Enter` → `POST /api/chat` no falló por enrutamiento.

### Estado inicial del host (pre-recycle, ~15:08 CEST)

| Parámetro | Valor observado | Diagnóstico |
| :--- | :--- | :--- |
| Órgano | `kalma2-bridge` PID `6151` | Unidad `sddia-kalma2-bridge@home-racso-Proyectos-SddIA.service` (verificado 15:18+). |
| Binario en disco | `SddIA/target/release/kalma2-bridge` | mtime **2026-09-06 06:48:40 CEST**. |
| Arranque del proceso | **2026-09-07 14:34:55 CEST** | Uptime ~2 d; sin restart tras PR #276. |
| Código HEAD | `479536b` | `dispatch` declara `(Method::Post, "/api/aiua/interact")`. |
| PR #276 | head `492c673` 09:14:12 CEST; merge `4b212aec` 09:27:15 CEST | GitHub `mergedAt` 2026-09-09T07:27:19Z. No es «merge a las 09:14». |
| Probe HTTP | `POST /api/aiua/interact` → **404** | Cuerpo: `{"success":false,"message":"ruta desconocida","exit_code":1}`. |
| Estáticos | `interfaces/kalma2/{app.js,index.html}` | `GET /` lee disco en cada petición; `#aiua-pulse` en DOM. |

---

## 2. Cuadro de purga forense (Filtro A)

### 2.1 Conservado de v1.2.0 (sigue válido)

| Elemento previo (v1.0.0 / v1.1.0) | Clasificación | Realidad | Resolución |
| :--- | :--- | :--- | :--- |
| Path de archivo con `/api/aiua/` | Aberración POSIX | `/` es separador de directorios. Un título HTTP literal materializa subárboles bajo `docs/todos/pending/`. | Filename plano: `POST api-aiua-interact 404.md`. Cotejo `ls` 15:49 CEST: un solo archivo, sin anidación residual. |
| Kaizen «fallar si mtime ELF < mtime main.rs» | Redundancia | `_sddia_resolve_daemon_binary` (`sddia_shell_lib.sh`) ya llama `_sddia_daemon_elf_fresh_vs_source` sobre `*.rs` + `Cargo.toml` del crate. `kalma2-bridge.sh` lo invoca en spawn. | Cero lógica nueva en el lanzador. Causa: proceso longevo no reejecuta el script. |
| «CA: un PR que mute Rust» | Incoherencia metodológica | `main.rs` / WUI del puente ya están en `main` vía PR #276. | Prohibido inventar diffs en el crate. Distinto de un PR **documental** que archive este PBI (CA-6 diferido). |
| Conflación 404 ↔ 503 Gemini | Capas | 404 = dispatcher sin pattern. 503 = proveedor, **después** de entrar a `handle_aiua_interact`. | 404 = este PBI. 503 = PBI operativo hermano. |
| Conflación con fractura `64f37c7f7b34` | Contexto | Aquella: `POST /api/chat` + ELF `mayeuta-llm` ausente. | Endpoints y prótesis distintas. |

### 2.2 Hallazgos v1.3.0 (esta pasada)

| Elemento v1.2.0 | Clasificación | Realidad SSOT | Resolución v1.3.0 |
| :--- | :--- | :--- | :--- |
| Enlaces `file:///home/racso/Proyectos/SddIA/...` como «canónicos» | Ceguera espacial | Precedente Filtro A en PBIs hermanos: viola agnosticismo del Core (`README` § independencia de instancia). | Rutas lógicas de repo / relativas desde `docs/todos/pending/`. |
| «Viola `todos-jurisdiction.md`» por el `/` en el filename | Cita normativa falsa | Esa norma rige buckets `pending/done/kitchen/…`, no el alfabeto del basename. | Citar POSIX. No imputar a la jurisdicción de todos. |
| «PR #276 merge 09:14, commit `492c673`» | Conflación commit/merge | `492c673` = head del PR (feat, 09:14:12). Merge GitHub = `4b212aec` (09:27:15 CEST). | Cronología desglosada. `492c673` permanece como ancla de CI (`validacion.md` CA-CI). |
| «Verificado en tests de integración» | Sobreclaim | `docs/features/kalma2-aiua-perceptive-bridge/validacion.md` CA-5: flatten `lab-mock`; **«Live no gate»**. | El 404 de host es exactamente el hueco que ese CA no cubrió. |
| Probe post-recycle «aceptado de inmediato» | Inexactitud | Auditoría §6: curl 15:19 → HTTP **500** en **81 s**, mismo 503 Gemini. No 404. | Aceptación = ≠404. Latencia/status = proveedor. El puente responde **500** con `message` que contiene `http-status-503`; no reenvía el status HTTP de Gemini. |
| CA-4 `POST /api/interact` post-recycle | Evidencia ausente | Probe 200 solo **pre-recycle** (auditoría §2). WUI **no** llama `/api/interact`. Matriz post-recycle: `/api/chat`. | CA-4 recortado a `/api/chat`. `/api/interact` = nota pre-recycle, no gate. |
| «Incidente formalmente cerrado» + `status: abierto` en `pending/` | Incoherencia de cierre | `features-documentation-pattern` v1.2.1: prohibido `pbi_archived: true` si el PBI sigue en `pending/`. | Dimensión física remediada. Dimensión documental abierta (CA-6 diferido). |
| `strings` «contiene `/api/aiua/interact`» | Precisión .rodata | ELF 15:17: secuencia empaquetada `.../api/sync-assets/api/aiua/interafile` + símbolo mangled `handle_aiua_interact`. | Predicado empírico: `grep -E 'aiua/intera|handle_aiua_interact'`. No afirmar C-string aislado. |
| Cadena «handler → `gemini-http-infer`» como llamada directa | Atajo | `handle_aiua_interact` spawnea `aiua-stimulus-processing`; el proceso declara `tool:gemini-http-infer`. | Dos hops. |

---

## 3. Causa raíz estructural

**Split-brain entre árbol Git (estáticos + fuente) y runtime del demonio.**

1. **Entrega en git, no en el órgano.** PR #276 metió la ruta y el handler en [`main.rs`](../../../SddIA/interfaces/kalma2-bridge/src/main.rs) y el botón en `interfaces/kalma2/{app.js,index.html}`. `validacion.md` cerró CAs de crate; el live del host no era gate.
2. **Servicio no reciclado.** Unidad `sddia-kalma2-bridge@home-racso-Proyectos-SddIA.service`; `pid 6151` vivo desde 2026-09-07.
3. **Disparidad GET vs POST:**
   - `GET /` → `serve_static` lee `interfaces/kalma2/` del disco → UI nueva.
   - `POST` → match del binario **en memoria**. Sin el arm `(Method::Post, "/api/aiua/interact")` cae al comodín `_ => 404 "ruta desconocida"`.
4. **Por qué no saltó la guarda.** `_sddia_resolve_daemon_binary` corre al spawn de [`kalma2-bridge.sh`](../../../SddIA/scripts/daemons/kalma2-bridge.sh). Un proceso ya instanciado no la reevalúa. Tras el merge, un `systemctl restart` **sin** rebuild habría fallado el check (ELF 06-09 < fuente 09-09); el fallo observado fue no restartar.

Plantilla de unidad en repo: `.SddIA/systemd/sddia-kalma2-bridge@.service`. Runtime cargado: `~/.config/systemd/user/sddia-kalma2-bridge@.service` (misma instancia verificada).

---

## 4. Estado empírico post-recycle (2026-09-09)

Remediación 15:17–15:18 CEST:

```bash
CARGO_TARGET_DIR=SddIA/target cargo build --release --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml -p kalma2-bridge
systemctl --user restart sddia-kalma2-bridge@home-racso-Proyectos-SddIA
```

Equivalente (auditoría §5): `cd SddIA && CARGO_TARGET_DIR=target cargo build --release -p kalma2-bridge`.

### Matriz in situ

| Criterio | Estado | Evidencia |
| :--- | :--- | :--- |
| Nuevo PID | Activo | PID `1244544`, start 15:18:25 CEST. ELF `SddIA/target/release/kalma2-bridge` mtime **15:17:48 CEST**. Journal: `lock huérfano pid=6151; recuperando`. Bind `127.0.0.1:8765`. |
| Símbolos en ELF | APTO (predicado real) | `strings` del release 15:17: mangled `handle_aiua_interact`; substring `aiua/intera` en .rodata empaquetado. |
| Probe HTTP | APTO (≠404) | Entrada a handler. Curl 15:19: HTTP **500** / 81 s por 503 Gemini (no 404). |
| WUI 1 | APTO | 1ª pulsación `#aiua-pulse` → respuesta de Tormentosa. |
| WUI 2 | APTO como **ruta**; NO como combustión | 2ª pulsación: `#output` con `http-status-503` / `UNAVAILABLE`. El puente HTTP es 500. Objeto del PBI 503. |
| `POST /api/chat` | APTO | Sin regresión de enrutamiento Mayeuta SSE. |
| `POST /api/interact` | Fuera de CA post-recycle | 200 pre-recycle. No re-sondado a las 15:18. WUI no lo usa. |

Cotejo 15:58 CEST (esta pasada): mismo PID `1244544` aún vivo; unidad user session activa.

---

## 5. Deslinde

```
                       POST /api/aiua/interact
                                │
              ┌─────────────────┴─────────────────┐
              ▼                                   ▼
   Pre-recycle PID 6151                 Post-recycle PID 1244544
   ELF 2026-09-06 06:48                 ELF 2026-09-09 15:17
              │                                   │
              ▼                                   ▼
     404 ruta desconocida              handle_aiua_interact
              │                        → aiua-stimulus-processing
   ESTE PBI (host OK)                  → tool:gemini-http-infer
                                       ┌──────────┴──────────┐
                                       ▼                     ▼
                                  1ª latido OK          2ª Gemini 503
                                                         puente HTTP 500
                                                         OTRO PBI (epidermis)
```

1. **Fractura `64f37c7f7b34`:** `/api/chat` + `mayeuta-llm` ausente. Aquí: `/api/aiua/interact` + ELF `kalma2-bridge` fósil.
2. **503 Gemini:** ruta HTTP resuelta. El blob en `#output` es superficie de error, sin retry (DA-5).
3. **Código en `main`:** cero mutación de `main.rs` para cerrar el 404.

---

## 6. Criterios de aceptación (gates)

- [x] **KALMA-STALE-CA1 — Símbolo en ELF vivo:** ELF en ejecución / disco de ese PID contiene `handle_aiua_interact` (mangled) y la substring `aiua/intera` (verificado PID `1244544` / mtime 15:17:48).
- [x] **KALMA-STALE-CA2 — No 404 en endpoint:** `POST http://127.0.0.1:${SDDIA_CLIENT_PORT:-8765}/api/aiua/interact` con `{prompt}` no devuelve 404 `ruta desconocida`. 500/503 de proveedor **cumple** este CA.
- [x] **KALMA-STALE-CA3 — WUI `#aiua-pulse`:** 1ª pulsación no escribe `[error] ruta desconocida` en `#output`.
- [x] **KALMA-STALE-CA4 — No-regresión chat:** `POST /api/chat` (Mayeuta SSE) operativo post-recycle.
- [x] **KALMA-STALE-CA5 — Filename plano + enlaces lógicos:** un archivo bajo `docs/todos/pending/` sin `/` en el basename; hipervínculos de repo, no `file://` de host.
- [x] **KALMA-STALE-CA6 — Archivo documental:** este PBI en `docs/todos/done/` conservando `document_id`; PR del sello **sin** diffs de `kalma2-bridge`/`app.js`. No reescribe el puente.

---

## 7. Plan de verificación empírica

### Paso 1: PID de la unidad, no `pgrep -f`

`pgrep -f "target/release/kalma2-bridge"` casa wrappers del shell. Usar el MainPID de systemd:

```bash
UNIT=sddia-kalma2-bridge@home-racso-Proyectos-SddIA.service
PID=$(systemctl --user show -p MainPID --value "$UNIT")
ls -l --time-style=full-iso "/proc/$PID/exe" SddIA/target/release/kalma2-bridge
strings SddIA/target/release/kalma2-bridge | grep -E "aiua/intera|handle_aiua_interact"
```

*Esperado:* MainPID numérico; mtime ELF ≥ 2026-09-09 15:17; hits de grep.

### Paso 2: Sonda HTTP (predicado = ≠404)

```bash
curl -sS -i -m 120 -X POST "http://127.0.0.1:8765/api/aiua/interact" \
  -H "Content-Type: application/json" \
  -d '{"prompt":"prueba de latido sddia"}'
```

*Esperado:* status HTTP ≠ 404; cuerpo sin `"message":"ruta desconocida"`. 200 con envelope de Aiúa **o** 500 con `http-status-503` son ambos APTO para este PBI. Timeout 120 s: el probe 15:19 tardó 81 s.

### Paso 3: Resolvedor (spawn, no proceso vivo)

```bash
bash -c 'source SddIA/scripts/common/sddia_shell_lib.sh && _sddia_resolve_daemon_binary "$(pwd)" kalma2-bridge'
```

*Esperado:* path a `SddIA/target/release/kalma2-bridge`, exit 0, si el ELF de disco ≥ fuente del crate. No prueba el proceso ya residente.

---

## 8. Lecciones y criterio de cierre

1. **Recycle post-merge:** crate bajo demonio systemd exige `cargo build` (perfil que ejecuta el lanzador) **y** `systemctl --user restart sddia-<daemon>@…`. CAs de crate no sustituyen el órgano vivo. El puente perceptivo lo dejó explícito: live no era gate.
2. **Basename POSIX:** ningún PBI con `/` en el nombre de archivo.
3. **Ceguera espacial:** hipervínculos de repo, no `file://` de host.
4. **Cierre de este sello:**
   - **Físico (hecho):** PID `1244544`, ruta ≠404, CA-1…CA-5.
   - **Documental (CA-6):** PBI en `docs/todos/done/`, `status: cerrado`. `dispatch: false` respecto a reescritura del puente. El PR de archivo no inventa código Rust.
5. **Fuera:** sanitizar 503; failover multi-LLM; fractura `mayeuta-llm`.
