---
document_id: LAB-EMAIL-TRIAGE-BATTERY
uuid: "c3e1a7d0-4b8f-4c2e-9a11-7f6d2e0b91c4"
title: "Baterías de ensayo — triaje de correo Kalma2"
format: markdown
version: "1.0.0"
created: "2026-09-07"
updated: "2026-09-07"
status: "activo"
type: lab
pbi_ref: docs/todos/pending/[KAIZEN] Triaje de correo — batería 20260907 inferencia nula.md
pbi_document_id: PBI-KAIZEN-EMAIL-TRIAGE-BATCH-20260907
matrix_ref: SddIA/library/norms/email-triage-matrix.md
matrix_version: "1.1.0"
lote_empirico: "2026-09-07T14:20:16Z/2026-09-07T14:26:22Z"
---

# Baterías de ensayo — triaje de correo Kalma2

Protocolo de inyección y oráculo de veredicto. SSOT normativo: `email-triage-matrix` v1.1.0. El lote 2026-09-07 es **evidencia**, no el contrato.

## 0. Cómo leer un caso

| Campo | Significado |
|-------|-------------|
| **Inyección** | Cómo debe entrar al IMAP. Si se viola, el caso **no mide** lo que declara. |
| **Oráculo matriz** | Veredicto / `decision_path` / eferencia **vigentes**. Gate de APTO. |
| **Deseo original** | Intención humana del fixture 20260907. Si diverge, es **GAP** (no se inventa regla). |
| **Observado 20260907** | Proof real. Peaje 0 en Clasificacion. |

Eferencia vigente: poke Telegram + ítem WUI (`/api/email-inbox`) **solo** si `verdict=actionable`. Proof siempre en `{eda_instance.proofs}/email-triaged/`. Cero IMAP STORE.

**Prohibido en oráculos:** inventar `datetime`; `actionable` por verbosidad/`URGENT`/`!!!`; ampliar L-GUARD a «factura/documentación/Computrabajo»; keyword de marca en C-SUBJECT-NOISE; llamar «cápsula de triaje» al handler nativo.

## 0bis. Protocolo de inyección (anti-sesgo)

| Modo | Qué prueba | Qué no prueba |
|------|------------|----------------|
| Reenvío desde `racso80es@gmail.com` (lote 20260907) | L-GUARD de asunto; fail-open `passive`; peaje LLM | C-LIST, C-NOREPLY, mute P, P-EXEMPT-C |
| Mensaje **original** con `List-Id` o `List-Unsubscribe` | C-LIST → `noise` `deterministic`; Clasificacion **skipped** | Semántica LLM |
| `From:` real `noreply@` / `notifications@` / `mailer-daemon@` | C-NOREPLY | Listas |
| Preferencia `explicit_user` + `priority` max\|high | P-EXEMPT-C (C no cierra) | Mute |
| Preferencia `mute` activa | P-MUTE-SENDER → `noise` `preference` | C-\* |

Un único remitente personal para todo el lote **invalida** el muro C y las reglas P.

Cabeceras que el centinela **sí** copia a `list_headers`: `List-Id`, `List-Unsubscribe`, `Precedence: bulk|list`, `Auto-Submitted` ≠ `no`. `X-Mailer` está en la matriz y **no** está en `detect_list_headers` del watcher: no usarlo como oráculo C-LIST.

Auditoría post-inyección: `.eml` en `.SddIA/inbox/{uid}.eml` + proof. No el bus volátil.

---

## 1. Lote A — fixtures 2026-09-07 (reformulados)

Origen: `baterias_de_pruebas_correos.md` (prosa). Remitente táctico del ensayo: `OSCAR PEREZ <racso80es@gmail.com>`. UIDs 5814–5819.

### TC-01 — Dualidad factura + cobro pendiente

| | |
|---|---|
| **Clase** | Operativo / cobro |
| **Inyección** | Correo profesional **sin** `List-*`. No reenviar como newsletter. |
| **Asunto** | `Factura 08 agosto 2026` |
| **Cuerpo** | `Adjunto la factura del mes. Además, realizando seguimiento, sigue pendiente el pago del mes pasado.` (GRACO RECUPERACIÓ DE METALLS, S.L.) |
| **Oráculo matriz** | Sin datetime extraíble → **`passive`**. `decision_path=llm`. Cero agenda. Cero poke. |
| **Deseo original** | «Alerta de cobro pendiente sin abrir el correo.» **GAP:** no hay eferencia para `passive`; no hay campo `importe`/`estado_cobro` en `Email_Triaged`. |
| **Observado 20260907** | UID `5814` proof `d5305406-…` `passive` + `classification-degraded` (peaje 0). |
| **Variante TC-01b** | Mismo cuerpo + fecha de vencimiento explícita (`vence el 15/09/2026` o `dd/mm/yyyy` en asunto). Oráculo: LLM sano → `actionable` **solo** si extrae `title`+`datetime`; si no, `passive`. Prohibido que L-GUARD lo eleve (no hay `reunión\|cita\|meeting\|llamada`). |

### TC-02 — Gestoría 2T / cargo programado

| | |
|---|---|
| **Clase** | Administrativo / plazo |
| **Inyección** | Hilo o mensaje único de gestor; **sin** `List-*`. |
| **Asunto** | `DOCUMENTACIÓN 2T 2026 - OSCAR A. PEREZ` |
| **Cuerpo** | Solicitud 2T antes del día 15; facturas 4–6; cargo **63,00 €** previsto **20/07/2026**. |
| **Oráculo matriz** | Asunto sin L-GUARD. LLM + extracción. `20/07/2026` es datetime completo → `actionable` **si** el modelo lo extrae; si no, `passive`. Fecha pasada: la matriz **no** veta (F-TRIAGE-06). |
| **Deseo original** | «Quick Action atómico del cargo 63,00 €». **Filtro A:** `email-quick-action` = `archive\|draft\|delegate`, no extracción. El asiento lícito es **agenda** (`title`+`datetime`+`source_ref`), no un tipo «cargo». El hilo (ida y vuelta) **no** se agrupa: cada UID es un estímulo. |
| **Observado 20260907** | UID `5815` proof `45c1f26a-…` `passive` degradado (Clasificacion no corrió). |
| **Variante TC-02b** | Cargo **futuro** `dd/mm/yyyy` en el cuerpo. Misma regla de extracción. Preferible para no contaminar agenda con pasado. |

### TC-03 — Factura de suministro disponible

| | |
|---|---|
| **Clase** | Informativo |
| **Inyección** | Veolia / suministrador. Si `From:` contiene `notifications@` o `noreply@` → **no es este caso** (pasa a TC-11). |
| **Asunto** | `Ya tienes disponible tu factura de agua` |
| **Cuerpo** | Domicilio PT.FABRIQUETA 16. Importe **39,86 €**. Fecha **04-09-2026**. Publicidad APP. |
| **Oráculo matriz** | **`passive`**. Importe y fecha de factura **no** equivalen a cita. Antiverbosidad: marketing no eleva. Cero poke. |
| **Deseo original** | «Aislar 39,86 € y fecha, purgar publicidad.» **GAP:** `Email_Triaged` FORBIDDEN `snippet`/`body`; no hay payload de importe. El proof solo porta veredicto + `from`/`subject`. |
| **Observado 20260907** | UID `5816` proof `678370e9-…` `passive` degradado. Oráculo `passive` **cumple por coincidencia** del fail-open. |

### TC-04 — Boletín de empleo (Computrabajo)

| | |
|---|---|
| **Clase** | Entropía / bulk |
| **Inyección A (oráculo C)** | Mensaje **original** con `List-Id` o `List-Unsubscribe`. |
| **Inyección B (reenvío)** | Desde Gmail personal. **No** mide C-LIST. |
| **Asunto** | `Nuevos puestos que se ajustan a tu experiencia` |
| **Cuerpo** | Vacantes Administración / Programador python. |
| **Oráculo A** | **`noise`** `deterministic` `C-LIST`. Clasificacion skipped. Cero poke. Candidato a digest. |
| **Oráculo B + LLM sano** | `noise` (semántico) o `passive`; `decision_path=llm`. Digest C-\* **no** lo reclama. |
| **Deseo original** | «Purga en paso cero / Filtro C.» Solo válido con inyección A. |
| **Observado 20260907** | UID `5817` inyección B. `passive` degradado. Ensayo **inválido** para C-LIST. |

### TC-05 — Newsletter técnica (Codely)

| | |
|---|---|
| **Clase** | Entropía / lista |
| **Inyección** | Igual que TC-04 (A original / B reenvío). |
| **Asunto** | `¡Ya se puede crear un Event Bus con Kafka! + Fable 5.1 es muy bueno programando` |
| **Cuerpo** | Novedades Kafka / Fable 5.1. |
| **Oráculo** | Idéntico a TC-04. |
| **Observado 20260907** | UID `5818` inyección B. `passive` degradado. |

### TC-06 — Reunión: asunto estructural vs cuerpo chat

| | |
|---|---|
| **Clase** | Acción / L-GUARD |
| **Inyección** | Remitente cualquiera **sin** C-\* que cierre. Fecha en asunto `dd/mm/yyyy` [hora]. |
| **Asunto** | `Reunión con Racso el 28/08/2026 a las 18:44` |
| **Cuerpo** | `Ahora llega` / `Esooo`. |
| **Oráculo matriz (invite)** | **`actionable`** vía L-GUARD aunque LLM falle o diga `passive`. Agenda + poke + inbox. Cuerpo chat **no** veta. |
| **Deseo original** | «Letargo táctico: no interrumpir por réplicas del hilo.» **GAP de producto** (no de este oráculo): cada UID nuevo con el mismo asunto **vuelve a poke**. No hay agrupación por `In-Reply-To` / Message-ID. Ver TC-13. |
| **Observado 20260907** | UID `5819` proof `e215ebf7-…` `actionable` `subject_elevation` agenda `2b49036a-…`. Duplicado de UID `5792` (26/08). Fecha **pasada**. |

**Variante TC-06b (G5):** mismo patrón, **fecha futura**. Oráculo idéntico. Preferible para no asentar pasado.

---

## 2. Lote B — casos nuevos (cobertura de muro y huecos)

### TC-11 — C-NOREPLY de suministro

`From: Veolia <notifications@veolia.example>` (o `noreply@`). Asunto de factura de agua.  
**Oráculo:** `noise` `deterministic` `C-NOREPLY`. Clasificacion skipped. Distingue TC-03 (informativo de persona/empresa sin noreply).

### TC-12 — C-SUBJECT-NOISE

Asunto: `Newsletter semanal — view in browser`. Cuerpo irrelevante. Sin `List-*` siquiera.  
**Oráculo:** `noise` `deterministic` `C-SUBJECT-NOISE`. Patrones: `unsubscribe`, `viagra`, `newsletter`, `view in browser`.

### TC-13 — Réplica de hilo de reunión (letargo)

`In-Reply-To` + `References` del TC-06b. Asunto `Re: Reunión con Racso el {fecha futura} a las {hora}`. Cuerpo: `Esooo`.  
**Oráculo matriz vigente:** L-GUARD vuelve a elevar → **segundo** `actionable` + poke.  
**Oráculo deseo:** un solo asiento / cero poke duplicado. Registrar como F-TRIAGE-06 / letargo. **No** cambiar el oráculo vigente en este lab hasta laudo Dedalo.

### TC-14 — Urgencia comercial (antiverbosidad)

Asunto: `URGENT!!! Act now — última plaza`. Cuerpo largo, mayúsculas, sin cita ni fecha extraíble.  
**Oráculo:** **no** `actionable`. `noise` o `passive` según C/LLM. Tokens de urgencia **no** elevan.

### TC-15 — Bounce / Mailer-Daemon

`From: Mailer-Daemon <mailer-daemon@…>`.  
**Oráculo:** `noise` `C-NOREPLY` (subcadena `mailer-daemon@`). Cero poke.

### TC-16 — P-MUTE-SENDER

Remitente con preferencia activa `predicate: mute` `value.muted: true` (autoridad irrelevante para mute; C no ha cerrado). Mensaje personal sin List-\*.  
**Oráculo:** `noise` `preference` `P-MUTE-SENDER`. Clasificacion skipped.

### TC-17 — P-EXEMPT-C sobre boletín

Misma inyección que TC-04 A (`List-*`) **y** preferencia `explicit_user` + `priority` max\|high para ese `subject_key`.  
**Oráculo:** Triaje-C **skipped**. No cierra `actionable`. Sigue a mute (si hay) o Clasificacion. Un boletín prioritario **no** es `noise` por C-LIST.

### TC-18 — Auto-Submitted

Cabecera `Auto-Submitted: auto-generated` (no `no`).  
**Oráculo:** `noise` `C-LIST` (el handler trata Auto-Submitted ≠ `no` como cierre C-LIST).

---

## 3. Matriz de oráculos (resumen)

| ID | Inyección mínima | Verdict | Path | Poke | Agenda | Digest C-\* |
|----|------------------|---------|------|------|--------|-------------|
| TC-01 | sin List-\*, sin fecha extraíble | passive | llm | no | no | no |
| TC-01b | datetime vencimiento extraíble | actionable \| passive | llm | solo si actionable | solo si actionable | no |
| TC-02 | sin L-GUARD; datetime en cuerpo | actionable \| passive | llm | idem | idem | no |
| TC-03 | no noreply | passive | llm | no | no | no |
| TC-04 A / TC-05 A | `List-*` | noise | deterministic | no | no | **sí** |
| TC-04 B / TC-05 B | reenvío personal | noise \| passive | llm | no | no | no |
| TC-06 / TC-06b | asunto reunión+fecha | actionable | llm (L-GUARD) | **sí** | **sí** | no |
| TC-11 | `notifications@`/`noreply@` | noise | deterministic | no | no | **sí** |
| TC-12 | patrón asunto | noise | deterministic | no | no | **sí** |
| TC-13 | Re: reunión | actionable (vigente) | llm | sí (GAP letargo) | sí | no |
| TC-14 | URGENT sin fecha | ¬actionable | C o llm | no | no | según C |
| TC-15 | mailer-daemon@ | noise | deterministic | no | no | **sí** |
| TC-16 | mute P | noise | preference | no | no | no (mute excluido del digest) |
| TC-17 | List-\* + P-EXEMPT-C | no cierra C | — | según LLM | según LLM | no |
| TC-18 | Auto-Submitted | noise | deterministic | no | no | **sí** |

Peaje: con `SDDIA_LLM_REQUIRE_INFER=1`, todo caso `path=llm` **sin** L-GUARD exige `tokens_in+tokens_out > 0` o se marca `classification-degraded` (no es APTO de Clasificacion).

## 4. Orden de ejecución recomendado

1. TC-06b (G5 / L-GUARD; pasa con LLM muerto).  
2. TC-04 A, TC-05 A, TC-11, TC-12, TC-15, TC-18 (muro C; Clasificacion no debe correr).  
3. TC-03, TC-01, TC-14 (LLM; `passive` / ¬actionable).  
4. TC-01b, TC-02b (LLM + extracción).  
5. TC-13, TC-16, TC-17 (letargo / preferencias; 16–17 requieren store P).  
6. Reenvíos B **solo** para regresión de sesgo; no como prueba de C-LIST.

## 5. Relación con deuda

| Fricción | Casos que la exponen |
|----------|----------------------|
| F-TRIAGE-02 peaje 0 | Cualquier `path=llm` sin L-GUARD del lote A |
| F-TRIAGE-03 inbox solo actionable | TC-01, TC-03 (invisibles en WUI aunque el veredicto sea correcto) |
| F-TRIAGE-04 ruido como passive | TC-04 B, TC-05 B |
| F-TRIAGE-05 acto sin L-GUARD | TC-02, TC-01b |
| F-TRIAGE-06 pasado / duplicado / letargo | TC-06 vs TC-06b, TC-13, UID 5792 |

PBI titular: `PBI-KAIZEN-EMAIL-TRIAGE-BATCH-20260907`. Digest: `PBI-EMAIL-NOISE-HEURISTIC-DIGEST` (solo filas Digest C-\* = sí).
