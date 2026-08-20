---
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
uuid: "1c70e777-9b7f-4ad3-ada5-225ab6d141c6"
title: "[KAIZEN] Ignición Consumidor: Poda Filtro C, Empaquetado (Release Bundle) y Diagnóstico Local"
format: markdown
version: "0.2.0"
status: pending
type: kaizen
priority: alta
created: "2026-08-20"
updated: "2026-08-20"
derived_from: PBI-LAB-PACIENTE0-SDDIA-AP
tech_debt_ids:
  - DT-START-SDDIA-CONSUMER-PROFILE
  - DT-SYSTEMD-FULL-COVERAGE
friction_ids:
  - F-04
  - R-07
  - F-06
  - F-07
---

# [KAIZEN] Evolución de Despliegue: Perfil Consumidor, Empaquetado y Diagnóstico

## 0. Validación del refinamiento (anti-alucinación)

| Afirmación propuesta | Dictamen | Corrección / ancla |
|----------------------|----------|-------------------|
| Existe `sddia-daemons@<instancia>.service` | **Inexacto hoy** | Solo existe `SddIA/templates/systemd/sddia-email-watcher@.service.template`. La plantilla multi-daemon es **objetivo** de este PBI (absorbe `DT-SYSTEMD-FULL-COVERAGE`). |
| `WorkingDirectory` → raíz `.SddIA/` | **Incorrecto** | Plantilla actual: `WorkingDirectory=%f` = **raíz de instancia** (padre de `.SddIA/` y `./.events/`). Binding obligatorio = raíz de instancia, no `.SddIA/`. |
| Empaquetar solo «Cerbero, Cúmulo, Mayeuta» como nodos binarios | **Impreciso** | Son roles/agentes ontológicos, no un release set cerrado. Empaquetar: binarios runtime + cápsulas exigidas por el códice + contratos/normas necesarios. |
| Extirpar *todo* `.rs`/`.py` del artefacto | **Aspiracional / faseado** | Hoy hay lanzadores `.sh` y posibles cápsulas no-Rust. Meta: cero fuentes de ingeniería y cero deps de desarrollo; scripts de lanzamiento mínimos permitidos hasta paridad Rust. |
| `build-release-bundle.sh` ya existe | **No** | Entidad a forjar. |
| Smoke vía `eda-local-topology-test` / `Local_QA_Requested` | **Parcialmente existente** | Reutilizar/adaptar tool `.SddIA/tools/eda-local-topology-test` + evento `SddIA/events/orchestration/local-qa-requested.md`; no reinventar nombres. |
| `agenda-manager` es binario eferente como Telegram | **Inexacto** | Es **skill** (`agenda:persist`). El bundle resuelve el grafo de capacidades del códice (skills + tools), no una lista hardcodeada. |
| Tres `email-watcher` + watermark estancado | **Observado en ensayo** | R-07: `start-sddia.sh` + systemd (+ dups). Watermark también afectado por catch-up `last_uid=0` + lote «primeros 50» (F-07). |
| Falta `send-telegram-notification` en AP | **Observado** | F-06: códice exige tool; target AP sin binario → DLQ. |
| Prohibir centinelas globales compartidos | **Nuevo / coherente** | Alineado a ceguera espacial + multi-cliente; formalizar. |
| Objetivos 1–3 del borrador v0.2 + notas Racso | **Ya cubiertos / absorbidos** | Filtro C = §1; copia de core completo = §2; batería post-despliegue = §3; servicios por cliente = §4. |

**UUID / document_id:** inmutables (`1c70e777-…` / `PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C`).

---

## 1. Origen y destilación de fricción

Ensayo `PBI-LAB-PACIENTE0-SDDIA-AP` (informe §11):

| ID | Fricción | Evidencia |
|----|----------|-----------|
| **F-04** | Derrame ontológico / Filtro C | `System_Fracture_Detected` → `enrich-fracture-pbi-kaizen` / `materialize-fracture-pbi` en instancia consumidor |
| **R-07** | Colisión de receptores | `start-sddia.sh` + `sddia-email-watcher@.service` (hasta 3 PIDs) sobre el mismo buzón/watermark |
| **F-06** | Binario eferente ausente | `send-telegram-notification` no en `SddIA_AP/SddIA/target/debug/` → `Email_Triaged` actionable en DL (`argos.send-telegram-notification: failed`) |
| **F-07** | Catch-up IMAP no «últimos 50» | `last_uid=0` + `MAX_UIDS_PER_POLL=50` procesa UIDs antiguos del lookback; correo nuevo no entra |
| — | Entropía de código | Trasplante copió genoma/fuente completo al paciente (notas Racso) |
| — | Ceguera post-despliegue | Validación por observación pasiva; falta smoke determinista reutilizable |

También: `start-sddia.sh` intenta `github-bridge-watcher`; WUI expone «Forjar Proceso» (`DT-START-SDDIA-CONSUMER-PROFILE`).

---

## 2. Objetivos de arquitectura

### 2.1 Poda de perfil consumidor (Filtro C)

1. `start-sddia.sh` / plantillas systemd: **no** ignitar `github-bridge-watcher` en perfil consumidor.
2. WUI Kalma2: ocultar/deshabilitar «Forjar Proceso» bajo perfil consumidor.
3. No enrutar `System_Fracture_Detected` a procesos de ingeniería si `codex-software-engineering` ausente (o flag de perfil explícito).
4. **Anti-colisión R-07 (instancia única sensorial):** si el paciente delega receptores a systemd, `start-sddia.sh` **no** levantará `email-watcher` / `telegram-watcher` en background. Candado singleton o cesión explícita de jurisdicción al OS. Un solo escritor del watermark IMAP por instancia.

### 2.2 Ignición IMAP — últimos 50

En primer arranque / catch-up de consumidor: recuperar solo los **50 correos más recientes** (UIDs más altos o UNSEEN acotado), no los «primeros 50 del lookback». Watermark anclado al máximo del lote. Alineación de `SDDIA_EMAIL_MAX_UIDS_PER_POLL` a esa semántica (código + docs).

### 2.3 Encapsulamiento físico (Release Bundle)

- Forjar `build-release-bundle` (script y/o cápsula Rust) que genere artefacto de despliegue.
- Contenido mínimo: binarios runtime (daemons, `execute-process`, bridge WUI), contratos/normas/códice necesarios, bus inerte, bóveda plantilla — **sin** fuentes de ingeniería (`.rs` de desarrollo, tests, dossiers) ni deps de build en el paquete de cliente.
- **Resolución dinámica de cápsulas (F-06):** el bundle **lee el códice inyectado** (p. ej. `codex-kalma2-assistant`) y compila/empaqueta **todas** las dependencias runtime exigidas por el grafo de capacidades (tools como `send-telegram-notification`, skills como `agenda-manager` + su implementación). Runtime completo ⇒ cero DLQ por binario ausente.
- Faseado: si alguna cápsula aún no es binario nativo, documentar excepción temporal; meta = paridad Rust.

### 2.4 Verificación de vida (smoke local)

- Adaptar/ejecutar batería post-despliegue basada en **`eda-local-topology-test`** + estímulo **`Local_QA_Requested`** (`local-qa-requested`), no inventar tool paralelo.
- Auditar integridad del bus, reacción de centinelas EDA, presencia de cápsulas del códice y carga de bóveda (sin filtrar secretos a logs).
- Gate: envelope `success: true` antes de declarar paciente listo.

### 2.5 Instanciación hermética de centinelas (multi-cliente)

1. **Prohibido** ejecutar centinelas globales o compartidos entre instancias (un proceso, N raíces).
2. Todo despliegue registra unidades systemd **parametrizadas por instancia** (evolución desde `sddia-email-watcher@` hacia cobertura completa tipo `sddia-<daemon>@<instancia>` / plantilla unificada — nombre final en forja; el borrador `sddia-daemons@` es **propuesta**, no SSOT actual) **o** procesos supervisados locales.
3. Binding obligatorio: `WorkingDirectory` = **raíz de la instancia** (`%f` hoy); `EnvironmentFile` = `%f/.SddIA/.dev/.env`. Competencia solo por credenciales de esa bóveda.
4. `start-sddia.sh` actúa **únicamente** sobre daemons de la carpeta ejecutora; cero interferencia con instancias vecinas (paths, locks, puertos, tokens).

---

## 3. Fuera de alcance

- Wizard UX de configuración (`DT-CONFIG-UX-ONBOARDING`, aplazada en ensayo).
- Sustituir Kalma2 WUI completa.
- Forja de nuevos dominios de negocio ajenos al perfil consumidor.

`DT-SYSTEMD-FULL-COVERAGE` **deja de ser “fuera”**: queda **absorbida** por §2.5 (cierre o reducción explícita al cerrar este PBI).

---

## 4. Criterios de cierre

### Filtro C / R-07

- [ ] Perfil consumidor: sin `github-bridge-watcher`, sin «Forjar Proceso» usable, sin suscriptores de forja ante Fracture.
- [ ] Gate: fracture sintética en `SddIA_AP` → cero `enrich-fracture` / `materialize-fracture` / `bug-fix` / `feature`.
- [ ] Con systemd sensorial activo: `start-sddia.sh` no spawnea segundo `email-watcher`/`telegram-watcher` (R-07 cerrado).
- [ ] Documentación perfil en starter-kit / constitución local.

### IMAP últimos 50

- [ ] Primer poll instancia limpia: como máximo los **50 UIDs más altos** (o UNSEEN acotado); watermark = max del lote; sin catch-up de meses.

### Release bundle + F-06

- [ ] `build-release-bundle` produce paquete sin fuentes de ingeniería ni árbol de desarrollo.
- [ ] Bundle incluye cápsulas/skills exigidas por el códice (verificación: `send-telegram-notification` presente tras bundle con `codex-kalma2-assistant`).
- [ ] Instancia desplegada desde el paquete: E2E actionable → Telegram + WUI sin DLQ por binario ausente.

### Smoke + multi-cliente

- [ ] Smoke (`eda-local-topology-test` / `Local_QA_Requested`) → `success: true` en paciente fresco.
- [ ] Dos instancias en el mismo host: centinelas con `WorkingDirectory` distintos; sin locks/watermarks/credenciales cruzados.
- [ ] `start-sddia.sh` en instancia A no toca procesos ni estado de instancia B.

---

## 5. Notas de forja

Mutación de genoma vía proceso `feature`/`kaizen` (no edición manual). Vincular UUID `1c70e777-9b7f-4ad3-ada5-225ab6d141c6` en `SddIA/evolution/` al cerrar.

**Orden sugerido de entrega:** (1) R-07 + IMAP últimos 50 + Filtro C runtime, (2) resolución cápsulas en bundle / build checklist F-06, (3) smoke local, (4) plantillas systemd multi-daemon + hermeticidad multi-cliente.

---

## 6. Referencias

| Ref | Uso |
|-----|-----|
| `docs/todos/done/[LABORATORIO] MVP Paciente 0 SddIA_AP.md` §11 | Fricciones F-04, R-07, ensayo |
| `SddIA/templates/systemd/sddia-email-watcher@.service.template` | SSOT systemd sensorial actual (`WorkingDirectory=%f`) |
| `start-sddia.sh` | Ignición híbrida; rama IMAP L280+ |
| `SddIA/events/orchestration/local-qa-requested.md` | ECST smoke |
| `.SddIA/tools/eda-local-topology-test.md` | Tool smoke existente |
| `SddIA/library/codexes/codex-kalma2-assistant.md` | Grafo de dependencias del bundle |
| `SddIA/tools/send-telegram-notification.md` | Caso F-06 |
| `SddIA/skills/agenda-manager.md` | Skill (no confundir con tool binario) |
