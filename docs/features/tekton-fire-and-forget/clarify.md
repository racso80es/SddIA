---
feature_name: tekton-fire-and-forget
created: "2026-08-16"
process: feature
purpose: Estabilización Mayeuta — veto de supervisión síncrona de Tekton y acuse CLI fire-and-forget
branch_name: feat/tekton-fire-and-forget
persist_ref: docs/features/tekton-fire-and-forget
pbi_ref: docs/todos/pending/ARQUITECTURA] Erradicación de esperas síncronas en Tekton (Patrón Fire-and-Forget).md
document_id: PBI-TEKTON-FIRE-AND-FORGET
uuid: 3ad2901a-aaf4-4631-b5df-11386b3ea997
status: blueprint_locked
mayeuta_verdict: ok
dedalo_verdict: ok
laudo: L-CLI-DETACH-ALLOWLIST
execution_id: 57dc7e51-9a48-4b98-a717-191da9070903
---

# Clarificación — tekton-fire-and-forget

Transcript Mayeuta (2026-08-16). Semilla v0 PBI-TEKTON-FIRE-AND-FORGET → requisitos estabilizados para handoff Dedalo.

Fuentes: PBI adjunto; `SddIA/core/cumulo.paths.json` v1.6.2; `SddIA/norms/external-ai-constraints.md` v1.4.0; `SddIA/agents/tekton.md` v1.1.0; `.cursorrules`; precedentes PBI-044 (`kalma2-pasarela-asincrona-eda`) y `kalma2-canal-telemetria-progreso`. Init lab: `execution_id` `57dc7e51-9a48-4b98-a717-191da9070903`.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` |
| `feature_name` | `tekton-fire-and-forget` |
| Rama | `feat/tekton-fire-and-forget` |
| `persist_ref` | `docs/features/tekton-fire-and-forget` |
| `document_id` | `PBI-TEKTON-FIRE-AND-FORGET` |
| Fase | Estabilización de Requisitos (Mayeuta) |
| Intención estable | Erradicar la supervisión síncrona de la IA obrera (Tekton/Cursor) tras invocar la Aduana; el operador dispara, recibe acuse, y el bus/centinelas continúan |

---

## D1 — Triaje de incongruencias (I1–I5)

| ID | Afirmación PBI / semilla | Hecho SSOT / precedente | Laudo Mayeuta |
|----|--------------------------|-------------------------|---------------|
| **I1** | Depósito del evento en `.SddIA/events/` | `event_bus` = `./.events`; `eda_fractal.*` = `./.events/{telemetry,orchestration,domain,progress}`; `eda_instance.customization` = `.SddIA/events` (Vía C, **no** cola) | **Prohibido** documentar o tratar `.SddIA/events/` como bus. El acuse se ancla al depósito en el bus fractal `./.events/…` según familia Cúmulo. |
| **I2** | Solape con pasarela HTTP y canal de progreso | PBI-044 = acuse HTTP 202 del **bridge** (cerrado). `kalma2-canal-telemetria-progreso` = PTC/SSE de **progreso UI** (cerrado). Este PBI = conducta de **Tekton** + retorno del **CLI operador** (`./sddia-run.sh` / `execute-process`) | **Adyacentes, no fusionar.** No reabrir socket HTTP ni hoja `progress`. |
| **I3** | Veto a `sleep` / `wait` / `while` vs herramientas IDE (`AwaitShell`, `block_until_ms`) | El síntoma es el LLM parcheando ceguera temporal. Los ticks internos de centinelas (`event-watcher` heartbeat) no son Tekton. Bloquear hasta el JSON de acuse del CLI **es** el contrato de retorno, no una espera post-inyección | **Veto acotado** (D3). No castrar daemons ni el wait del propio proceso CLI hasta el acuse. |
| **I4** | Mutar `tekton.md` y `external-ai-constraints.md` | DA-2: `directories.agents` y `directories.norms` son genoma. `.cursorrules` / `.cursor/rules/` son touchpoints que **difunden** la norma, no la sustituyen | Genoma **solo** vía `entity-manager`. Touchpoints sí en este ciclo documental, alineados a la norma motor. |
| **I5** | CLI síncrono hasta fin de fases vs «devolver control al depositar el evento» | Este init devolvió en ~1.6 s con peaje fractal ya escrito (`orchestration` `8dc4b0b2…`, `telemetry` `90b82bb5…`) **tras** ejecutar fases lab. Un `pull-request-review` / `radamanto-batch` sigue ocupando la terminal hasta completar trabajo | Vector II = **acuse al depositar intención/peaje en el bus**, sin join a watcher ni a la carga larga. **No** reimplementar la pasarela HTTP. Mecanismo = Dedalo. |

---

## D2 — Qué falla / qué no

| Afirmación | Veredicto |
|------------|-----------|
| Tekton inyecta `sleep` / polling tras `./sddia-run.sh` | **Sí** — brecha de conducta (Vector I) |
| CLI espera a `event-watcher` tras escribir el JSON de peaje | **No evidenciado** en este init; el peaje se escribe y el proceso retorna. El bloqueo real es **ejecutar la carga** en el mismo proceso |
| AC «consola inmediata» al invocar proceso largo | **Sí** — hoy falso para PPR / radamanto-batch (Vector II) |
| Encadenar fases documentales IDE (`simulated` → Mayeuta) del **mismo** `feature` | **Fuera del veto** — no es supervisión de un proceso largo ajeno; es relevo de fase del ciclo activo |
| Bridge HTTP / PTC / SSE | **Fuera** (I2) |

---

## D3 — Perímetro del veto (Vector I)

**Dentro (IA obrera / Tekton, post-inyección a la Aduana):**

- `sleep`, `timeout`, `wait` de shell cuyo objeto es «dar tiempo» al Core.
- Bucles `while` / reintentos de lectura sobre `./.events/`, `GET /api/status`, artefactos de `persist_ref` o PID de centinelas.
- `AwaitShell` / `notify_on_output` / `block_until_ms` extra **después** de haber recibido el JSON de acuse, para vigilar watcher o materialización.

**Fuera:**

- Ticks internos de daemons (`directories.daemons`).
- Backoff de cápsulas Rust (no es el hilo de Tekton).
- Bloqueo del invocador **hasta** stdout JSON + `exitCode` del CLI (eso **es** el acuse, hasta que Vector II lo haga inmediato).
- Relevo de fase IDE del proceso `feature`/`bug-fix`/`refactorization` activo cuando el CLI marcó la fase `simulated`.

**Mandato de latencia:** tras acuse (`success` + `correlation_id` / `execution_id` / peaje), Tekton declara la inyección cerrada y libera el hilo. El siguiente estímulo lo dictan Racso o Kalma2 (Vector III).

---

## D4 — Vectores soberanos estabilizados

| ID | Qué (requisito estable) | Por qué |
|----|-------------------------|---------|
| **V1** | Norma motor + contrato Tekton + touchpoints IDE declaran veto I3 y patrón Fire-and-Forget | El LLM no debe supervisar tiempo; la norma debe ser auditable (AC Auditoría) |
| **V2** | `./sddia-run.sh` / `execute-process` devuelve control al operador en el instante del depósito en `eda_fractal` / `event_bus`, sin join a centinelas ni a la carga larga | AC Prueba de Fricción; termodinámica EDA |
| **V3** | Tras acuse, no encadenar en el mismo hilo la respuesta de procesos largos; testigo = Racso / Kalma2 | Ceguera temporal; relevo de testigos |
| **V4** | Centinelas (`event-watcher` y pares) consumen el bus en background; artefactos en destino sin dependencia del hilo de la IA | AC Alineación del Bus Fractal |

---

## D5 — Alcance (dentro / fuera)

| Dentro | Fuera |
|--------|-------|
| Cláusula prohibitiva en `external-ai-constraints.md` (DA nueva), `agents/tekton.md`, difusión en `.cursorrules` / `.cursor/rules/` | Reabrir PBI-044 (HTTP 202, spawn bridge) |
| Acuse CLI operador: exit 0 + JSON de confirmación (`correlation_id` / `execution_id`) al depositar en `./.events/{family}/` | Hoja `progress`, SSE, WUI cromática |
| Prueba de fricción empírica con proceso largo (`pull-request-review` **o** `radamanto-batch`) | Castrar `sleep` de daemons |
| Documentar que Tekton no hace join a watcher ni polling post-acuse | Inventar familia ECST nueva si Dedalo puede reutilizar peaje/orquestación existente |
| Cierre documental PBI → `docs/todos/done/` + `validacion.md` en la misma rama | Commit de suciedad ajena (Radamanto / `Cursor_Obligar_Uso_ED.md`) arrastrada al checkout |

---

## D6 — Criterios de aceptación mapeados

| AC PBI | Requisito estable |
|--------|-------------------|
| Auditoría de Normas | V1 materializado y explícito (veto + Fire-and-Forget) |
| Prueba de Fricción | Invocación de proceso largo → acuse inmediato; Tekton cierra sin esperas post-acuse |
| Alineación del Bus Fractal | Watcher/background genera artefactos; hilo IA no es join |
| Cierre documental | Patrón v1.2.x: PBI en `done/` + `pbi_archived: true` en el PR |

---

## D7 — Entropía operativa (no requisito)

El `workspace-init` partió de `main` con working tree sucio (Radamanto + delete `Cursor_Obligar_Uso_ED.md`). Esa suciedad **no** forma parte de este PBI; Dedalo/Tekton no la incluyen en la entrega.

Nombre físico del PBI carece del `[` inicial (`ARQUITECTURA] …`). Cosmético; no bloquea.

---

## Handoff Dedalo

Cuerpo de `objectives.md` = `refined_requirements`. Dedalo elige mecanismo de V2 (detach vs ingest EDA vs otro) **sin** reabrir HTTP bridge y **sin** escribir `.SddIA/events/` como cola. Genoma vía `entity-manager`.
