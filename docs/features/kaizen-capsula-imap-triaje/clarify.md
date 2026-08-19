---
feature_name: kaizen-capsula-imap-triaje
created: "2026-08-19"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-CAPSULA-IMAP-TRIAJE (Kaizen IMAP + triaje interactivo)
branch_name: feat/kaizen-capsula-imap-triaje
persist_ref: docs/features/kaizen-capsula-imap-triaje
pbi_ref: docs/todos/pending/PBI-KAIZEN-CAPSULA-IMAP-TRIAJE.md
document_id: PBI-KAIZEN-CAPSULA-IMAP-TRIAJE
uuid: "9c25bb52-57a4-4ede-be43-41388a7576b2"
execution_id: "14fff213-bcee-4c26-ad17-53e5e585979b"
mayeuta_verdict: ok
laudo: no-dualidad-email-triaged
---

# Clarificación — kaizen-capsula-imap-triaje

Transcript Mayeuta (2026-08-19). Semilla PBI v1.0.0 con spec/clarify/plan embebidos → requisito termodinámico. Filtro A aplicado contra genoma vigente; no se implementa a ciegas.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `kaizen-capsula-imap-triaje` |
| Rama | `feat/kaizen-capsula-imap-triaje` |
| `persist_ref` | `docs/features/kaizen-capsula-imap-triaje` |
| `document_id` | `PBI-KAIZEN-CAPSULA-IMAP-TRIAJE` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` + `SDDIA_AGENT_RUNTIME_COMMAND=` (relevo IDE; evita anidar cursor-agent) |
| `execution_id` | `14fff213-bcee-4c26-ad17-53e5e585979b` |
| Fase | Estabilización Mayeuta (esta sesión) |
| Dependencia | `kalma2-mvp-sensorial-email` (APTO) — Paciente Cero |
| Adyacente no fusionar | `PBI-ARQ-CONSCIENCIA-UNIVERSAL` (grafo de hábitos) |

---

## D1 — Dualidad `Actionable_Email_Detected` (I1)

| Semilla PBI | Hecho SSOT | Laudo |
|-------------|------------|-------|
| Forjar `Actionable_Email_Detected` | Clases `Email_Received` (`574fe330`) y `Email_Triaged` (`6a4b0e9a`) ya existen | **Prohibida dualidad ontológica** |
| «Erradicar alerta plana» | `Email_Triaged: []` en `event-domain-subscriptions.json`; Paciente 0: *«Suscriptor MVP: ninguno (consumo por proyección GET /api/status)»* | La brecha es **fan-out humano ausente**, no ausencia de clase |
| Matriz `actionable` | `email-triage-matrix` v1.0.0: vías `noise` / `passive` / `actionable` | El veredicto accionable ya es `Email_Triaged.verdict=actionable` |

**Toll:** el nombre `Actionable_Email_Detected` es etiqueta humana, no SSOT. Dedalo solo forja clase nueva si demuestra que el payload de UI (resumen + acciones rápidas) **no cabe** como extensión de `Email_Triaged` ni como estímulo de canal eferente. Default = **cero clases nuevas**.

---

## D2 — ¿Quién clasifica? (I2, I7)

| Semilla PBI | Norma vigente | Laudo |
|-------------|---------------|-------|
| «La cápsula clasifica RUIDO vs FRICCION_RELEVANTE» | `email-triage-matrix` § Restricciones: *Prohibido alojar esta matriz en `SddIA/process/` o en la cápsula del Centinela* | **Rechazado** |
| Daemon `email-watcher` | Jurisdicción: ceguera lógica; solo `Email_Received`; no interpreta veredicto | Invariante G4 Paciente 0: no relajar |
| Proceso `email-triage-gateway` | Triaje-C → Clasificacion LLM → Emision `Email_Triaged` | Sigue siendo la aduana cognitiva |

**Split del PBI:**

- **Objetivo A (Guante):** resiliencia física del centinela/cápsula IMAP (`SddIA/daemons/email-watcher/`) al contrato `capsule-json-io` — caídas de red, MIME corrupto, adjuntos; **nunca** `panic!` que mate el hilo; `success:false` + `exitCode` controlado. Sin estado estratégico.
- **Objetivo B (Umbral humano):** cuando el veredicto ya emitido es `actionable`, el sistema **eleva** al Vértice Biológico (Kalma2 y/o Telegram eferente). `noise` deja constancia en `Email_Triaged` y **no** notifica canales humanos.

---

## D3 — Matriz vs grafo de hábitos (I3)

| Semilla | SSOT | Laudo |
|---------|------|-------|
| «Matriz de contexto inyectada» | Norma `email-triage-matrix` (`3d8c7e09`) ya es la ley inyectada por el códice | Esta feature **consume** la matriz; no la reescribe salvo gap Dedalo |
| Aprendizaje local en IMAP | `PBI-ARQ-CONSCIENCIA-UNIVERSAL`: herramientas = actuadores ciegos; hábitos en Grafo | **Fuera.** Corrección de usuario → fricción genérica (PBI-ARQ); no persistir hábitos en el watcher |

---

## D4 — Catálogo de eventos (I4)

Forja de cualquier Clase ECST = `entity-manager` → `{name}.md` bajo `directories.events` + índice de familia. `events-contract.md` es contrato maestro, **no** catálogo. Semilla «registrar en events-contract.md» = alucinación de locus.

---

## D5 — Identificadores de canal (I5)

| Semilla | Canónico | Laudo |
|---------|----------|-------|
| `kalma2_interact_core` | Proceso `kalma2-interact` (`acdb6c88`); handler nativo `kalma2-interact-core` | El **proceso** es `kalma2-interact`. El handler no es entidad de suscripción |
| Handler en `telegram-watcher` | Daemon Capa 0 **aferente** (long-poll → bus). Jurisdicción: ceguera lógica | **Prohibido** convertir el watcher en consumidor de correo |
| Eferencia Telegram | Tool `send-telegram-notification`; patrón PEC/`telegram-fallback-responder` | Canal humano Telegram = herramienta eferente ciega, no el centinela |

WUI Kalma2 (`interfaces/kalma2/` + `kalma2-bridge`) es el umbral interactivo declarado. Dedalo elige proyección push vs poll enriquecido; no sustituye `GET /api/status` como veredicto terminal (laudo PBI-044).

---

## D6 — Acciones rápidas vs IMAP RO (I6)

| Semilla | Norma | Laudo |
|---------|-------|-------|
| Botón `[Archivar]` | Matriz: *Prohibido mutar el buzón IMAP como efecto de un veredicto* | Archivar **no** es efecto de `Email_Triaged` |
| Acciones rápidas | PBI: resolver fricción en un clic | Son **eventos de retorno** desde UI (Kalma2/Telegram) hacia un actuador distinto del centinela |
| `[Generar Borrador]` | No existe cápsula de redacción en este perímetro | Alcance: disparar estímulo de retorno; Dedalo no forja cliente SMTP en esta feature salvo gap explícito |

IMAP write (STORE/expunge) queda **fuera** del centinela. Si Dedalo necesita archivo real, es cápsula/tool nueva de escritura, ciclo aparte o hito explícito.

---

## D7 — Criterios de aceptación reencuadrados

| AC PBI | Reencuadre termodinámico |
|--------|--------------------------|
| Cápsula IMAP sin panic / JSON-io | **AC-A.** Watcher sobrevive fallos IMAP/MIME; hilo del orquestador intacto |
| Ruido no genera evento hacia Kalma2/Telegram | **AC-B1.** `Email_Triaged` con `verdict=noise` (y `passive`) no dispara notificación humana. La constancia de dominio **sí** se emite (matriz §1) |
| Correo relevante → notificación enriquecida | **AC-B2.** `verdict=actionable` eleva resumen táctico al umbral humano (WUI y/o Telegram eferente) |
| Acciones rápidas sin salir de plataforma | **AC-B3.** Payload de retorno (`Archivar` como intención, `Generar Borrador` como estímulo) sin mutar IMAP desde el centinela |

---

## D8 — Fuera de alcance

- Reabrir `kalma2-mvp-sensorial-email` / genoma Paciente 0 (G4/G5).
- Fusionar con `PBI-ARQ-CONSCIENCIA-UNIVERSAL`.
- Mutar `telegram-watcher` para consumir correo.
- Alojar `email-triage-matrix` en el centinela.
- Clase `Actionable_Email_Detected` por default.

---

## Handoff Dedalo

Consumir el cuerpo de `objectives.md` como `refined_requirements`. Resolver:

1. Extensión de `Email_Triaged` vs evento de canal (solo si el default D1 no basta).
2. Suscriptores concretos de `Email_Triaged` (proceso/tool eferente; **no** `telegram-watcher`).
3. Forma del payload UI (resumen + botones) sin violar FORBIDDEN de `Email_Triaged` (`body`, `snippet`).
4. Perímetro de auditoría panic/JSON-io del crate `email-watcher`.
5. Touchpoints WUI vs `send-telegram-notification` (uno, otro, o ambos).
