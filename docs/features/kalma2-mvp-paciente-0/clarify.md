---
feature_name: kalma2-mvp-paciente-0
created: "2026-08-17"
process: feature
purpose: "Estabilización Mayeuta — Filtro Antientrópico sobre PBI-KALMA2-MVP-01 (Centinela de correo + sincronización de activos)"
phase: Estabilización de Requisitos
agents: mayeuta
branch_name: feat/kalma2-mvp-paciente-0
persist_ref: docs/features/kalma2-mvp-paciente-0
pbi_ref: "docs/todos/pending/Ecosistema SddIA e Inyección Industrial en Paciente 0 (GesFer).md"
document_id: PBI-KALMA2-MVP-01
uuid: "d7d00838-9ee6-472f-a164-95dcba2ceb80"
status: stabilized
mayeuta_verdict: ok
open_decisions: 0
ratification_required: 2
ratification_granted: 2
ratified_by: biological-vertex
ratified_at: "2026-08-17"
delivery_split: "PBI-KALMA2-MVP-01A + PBI-KALMA2-MVP-01B"
---

# Clarificación Mayeuta — Filtro Antientrópico

## D0. Apertura

El PBI llegó como estratificación de dos documentos incompatibles: un plan maestro GesFer (4 fases, minteo IOTA real, Forge/Portal) y un refinamiento v1 Kalma2 (MVP de correo) con anexos añadidos por acreción. El "qué" estaba disperso y el "por qué" duplicado. Esta clarificación estabiliza un único vector: **autonomía sensorial periférica con la ley del triaje fuera del Core**.

## D1. Hechos verificados contra el repositorio

Toda afirmación siguiente está comprobada por inspección, no inferida.

| # | Hecho | Evidencia |
|---|-------|-----------|
| F1 | La dependencia declarada `PBI-SDDIA-DOMAIN-ABSTRACT-01` está cerrada | `docs/todos/done/[ARQUITECTURA] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución.md` |
| F2 | `sddia-client-bridge.py` **no existe** en el repo | 0 coincidencias; podado en la feature `kalma2-bridge-rust` (`validacion.md` O6) |
| F3 | El puente vigente es Rust y expone 5 rutas API | `SddIA/interfaces/kalma2-bridge/src/main.rs` → `/api/chat`, `/api/execute`, `/api/interact`, `/api/status`, `/api/progress/stream` |
| F4 | La WUI es HTML/JS plano servido como estático | `interfaces/kalma2/{index.html,app.js,style.css}` |
| F5 | `event-watcher` **ya existe** y vigila `.events/pending/`, no correo | `SddIA/daemons/event-watcher.md`, uuid `f995cc89-…` |
| F6 | No existe ninguna clase ECST de correo | `SddIA/events/domain/` — 20 clases, ninguna de email |
| F7 | No existe tool ni skill de descarga HTTP genérica | `SddIA/tools/` (9), `SddIA/skills/` (11); el único `fetch` es `git-manager` sobre git |
| F8 | `filesystem-manager` **sí** existe con las capacidades necesarias | `SddIA/skills/filesystem-manager.md` v1.1.0 (`file-write`, `create-directory`) |
| F9 | `iota-immutable-publisher` existe como tool de anclaje | `SddIA/tools/iota-immutable-publisher.md` |
| F10 | El contrato de Códice exige `composition[]` de normas con UUID y ruta | `SddIA/library/codexes/codex-contract.md` §1 |
| F11 | El contrato de Códice **no** define ningún campo DLT/NFT | mismo fichero, v1.1.0 |
| F12 | `SddIA/tokens/` es dominio de tokens de autorización (Karma2Token), no de activos NFT | `SddIA/tokens/tokens-contract.md` §1 |
| F13 | Existe jurisdicción para procesos empacados en códice | `cumulo.paths.json → directories.process_domain_roots` |
| F14 | El patrón de proceso-aduana-cognitiva ya está resuelto para otro canal | `SddIA/process/telegram-gateway.md` + `TelegramMessage_Received` |
| F15 | El DI por capacidades es el mecanismo de permuta de proveedor | `SddIA/core/capability-bindings.md` v1.4.0 |

## D2. Triaje de incongruencias (fisuras lógicas detectadas)

### L-01 — Colisión de identidad del Centinela · **crítica**

El Anexo A ordena `ExecStart=event-watcher`, pero `event-watcher` es un Centinela existente que vigila el bus EDA (F5). Ignición de esa unidad = duplicar el watcher del bus apuntado a un directorio ajeno, no leer correo.

**Resolución:** entidad nueva y distinta, `email-watcher`. La plantilla systemd se especializa: `sddia-email-watcher@.service`. Prohibido reutilizar el nombre genérico `sddia-watcher@`.

### L-02 — Plantilla systemd inválida · **crítica**

`ExecStart=event-watcher` sin ruta absoluta no arranca: systemd exige path absoluto y no expande variables en el primer argumento.

**Resolución:** el genoma publica `SddIA/templates/systemd/sddia-email-watcher@.service.template` con marcador `@@SDDIA_CORE_ROOT@@`; el operador lo renderiza en la capa OS. Así ninguna ruta absoluta contamina el genoma y la unidad es ejecutable. `WorkingDirectory=%f` es correcto y se conserva: es el único punto de acoplamiento espacial.

### L-03 — Artefacto fantasma en la cadena de sincronización · **crítica**

El Hito 4 ordena que la WUI hable con `sddia-client-bridge.py`, inexistente (F2).

**Resolución:** el Hito 4 se re-apunta a `kalma2-bridge` (Rust, F3). Se añade una ruta `POST /api/sync-assets` que delega en `execute-process --process sync-client-assets` en modo fire-and-forget (202), homóloga a `/api/execute`. Cero Python nuevo.

### L-04 — El Centinela viola su propio contrato si delega · **crítica**

El PBI insinúa que el Centinela "despierta al CLI orquestador". `daemons-contract` §2 prohíbe explícitamente a un Centinela invocar `execute-process`.

**Resolución:** `email-watcher` **solo escribe** la instancia ECST `Email_Received` en `./.events/domain/` y su `Daemon_Heartbeat`. El despertar del motor lo hace `event-watcher`, que ya existe para eso. Se elimina un eslabón y se respeta el contrato: menos piezas, menos entropía.

> Nota de deuda observada (no de esta feature): `event-watcher` y `telegram-watcher` sí delegan en `execute-process`, en tensión con `daemons-contract` §2. Se registra como fricción preexistente; `email-watcher` no la propaga.

### L-05 — La ley del triaje quedaba en el Core · **grave**

El PBI exige que la lógica de triaje "no esté en el Core, sino en un activo descargable", pero luego describe un proceso de triaje sin decir dónde vive. Si nace en `SddIA/process/`, la exigencia queda incumplida y el Core deja de ser agnóstico.

**Resolución:** doble movimiento.
1. La **ley** (matriz de 3 vías, señales, umbrales) se materializa como norma táctica `SddIA/library/norms/email-triage-matrix.md`, compuesta por el Códice.
2. El **proceso** `email-triage-gateway` se empaca bajo `SddIA/library/codexes/codex-kalma2-assistant/process/`, registrando el directorio en `process_domain_roots` (F13). Precedente vigente: `codex-software-engineering` empaca `feature`, `bug-fix`, `refactorization`.

El Core conserva únicamente física: bus, enrutado, DI. Cero semántica de correo.

### L-06 — Identidad de activo sin lugar donde existir · **grave**

La Regla de Acero exige que el Códice nazca preparado para minteo, pero `codex-contract` v1.1.0 no define ningún campo DLT (F11) y `SddIA/tokens/` es otro dominio (F12). Añadir un bloque `dlt` sin amparo contractual lo convierte en Ruido de Sistema para Cúmulo.

**Resolución:** `codex-contract` → **v1.2.0**, incorporando un bloque `dlt` **opcional** (no rompe los 4 códices existentes):

```yaml
dlt:
  asset_class: "domain-codex"
  mint_status: "pre-mint"        # pre-mint | minted | revoked
  ledger: "iota-rebased-testnet"
  canonical_hash: "sha256:…"     # == hash_signature del códice
  token_id: null
  owner_vertex: "biological-vertex"
```

`canonical_hash` reutiliza el hash canónico ya obligatorio: el minteo futuro no recalcula nada, solo ancla. Eficiencia termodinámica: cero cómputo nuevo.

### L-07 — Filtro C destructivo sobre el buzón del Vértice Biológico · **crítica (ética + irreversibilidad)**

El refinamiento v1 ordena para el ruido "eliminación o archivo silencioso". Un MVP con clasificador no calibrado y permiso de borrado produce pérdida irreversible de correo real. Ningún criterio de aceptación cubría ese riesgo.

**Resolución:** MVP estrictamente **read-only** sobre IMAP. El Filtro C descarta **lógicamente** (no eleva a consciencia) y deja constancia en `Email_Triaged`. La escritura IMAP se habilita en un PBI posterior, y solo tras demostrar precisión medida sobre el histórico de veredictos.

### L-08 — Secuestro semántico y coste de inferencia · **grave**

El criterio "peaje termodinámico medible" era un deseo sin mecanismo: pagar una inferencia LLM por cada correo comercial es exactamente el derroche que el criterio pretendía evitar, y la verbosidad comercial es el vector de secuestro descrito.

**Resolución:** el proceso resuelve en dos fases con salida temprana, calcado del patrón `Triaje-C` de `kalma2-interact`:

| Fase | Mecanismo | Coste |
|------|-----------|-------|
| Triaje-C | Heurística determinista (remitente, cabeceras de lista, patrones de asunto) declarada en la norma | 0 tokens |
| Clasificación | `skill:mayeuta-llm` vía capacidad `llm:interact`, **solo si Triaje-C no concluye** | acotado |

`Email_Triaged` porta `decision_path: deterministic | llm` y el coste. Sin ese campo, el criterio de aceptación no es verificable.

### L-09 — Peso y privacidad del payload en el bus · **grave**

Inyectar el cuerpo íntegro de cada correo en `./.events/domain/` infla el bus, y ese bus tiene destino de anclaje DLT en fases futuras: contenido personal en un registro inmutable es un daño irreversible.

**Resolución:** payload ligero. `Email_Received` porta `message_uid`, `mailbox`, `from`, `subject`, `received_at`, `snippet` (truncado, 512 caracteres por defecto) y `body_ref` apuntando a `{instancia}/.SddIA/inbox/{message_uid}.eml`, fuera de Git y fuera del bus. El anclaje DLT jamás ve el cuerpo.

### L-10 — Alcance estratificado e inejecutable · **grave**

El fichero mezclaba GesFer (relevado por el propio texto), minteo IOTA real, `@sddia/core` como paquete, Forge y Portal. Un PBI con cuatro productos no es planificable.

**Resolución:** alcance reducido a los cuatro hitos Kalma2. El plan maestro GesFer se conserva como **Anexo Z** con estado por fase, para no destruir trazabilidad histórica. El minteo real sale de alcance: H4 simula el ledger.

### L-11 — Proveedor de correo no especificado · **media**

"Conectado a la API de correo" no nombra proveedor. Elegir uno (Gmail API, Graph) sería alucinación y ataría el Core a un SDK.

**Resolución:** **IMAP genérico**, parametrizado en `{instancia}/.SddIA/.dev/.env`. El Centinela no conoce proveedor, solo host/puerto/credencial. Sondeo periódico configurable; IDLE queda como optimización posterior, no como requisito MVP.

### L-12 — "Skill ya forjada" sin verificar · **menor**

El Hito 4 daba por hecha la existencia de `filesystem-manager`. Verificado: existe y basta (F8). Sin acción correctiva; se elimina la incertidumbre.

### L-13 — Pivote DLT declarado pero sin mecanismo · **grave**

"Deprecar `github-raw-fetcher` y sustituir por `iota-ipfs-fetcher` sin alterar Proceso ni Acción" solo es cierto si el proveedor se resuelve por inyección, no por nombre.

**Resolución:** nueva capacidad en `capability-bindings.md`:

```yaml
- capability_id: "asset:fetch"
  contract: "asset.fetch"
  provider: "tool:github-raw-fetcher"
  provider_version: ">=1.0.0"
```

`download-remote-asset` declara `requires_capability: asset:fetch` y nunca nombra la cápsula. El pivote es una línea en el binding. Con esto el criterio de aceptación pasa a ser verificable por construcción.

## D3. Decisiones estabilizadas

| ID | Decisión | Origen |
|----|----------|--------|
| D-01 | Centinela nuevo `email-watcher`, sin reutilizar `event-watcher` | L-01 |
| D-02 | Template systemd con marcador `@@SDDIA_CORE_ROOT@@` en `SddIA/templates/systemd/` | L-02 |
| D-03 | `POST /api/sync-assets` en `kalma2-bridge`; cero Python | L-03 |
| D-04 | El Centinela solo emite ECST; el enrutado lo hace `event-watcher` | L-04 |
| D-05 | Ley en `library/norms/email-triage-matrix.md`; proceso empacado en el códice | L-05 |
| D-06 | `codex-contract` v1.2.0 con bloque `dlt` opcional | L-06 |
| D-07 | MVP IMAP read-only; cero mutación del buzón | L-07 |
| D-08 | Triaje bifásico con salida temprana determinista + `decision_path` en el evento | L-08 |
| D-09 | Payload ligero con `body_ref`; cuerpo fuera del bus y de Git | L-09 |
| D-10 | Alcance = 4 hitos Kalma2; GesFer a Anexo Z | L-10 |
| D-11 | IMAP genérico por entorno de instancia | L-11 |
| D-12 | Reutilizar `filesystem-manager` sin cambios | L-12 |
| D-13 | Capacidad DI `asset:fetch` como punto único de pivote | L-13 |

## D4. Verificación de Ceguera Espacial

| Vector de acoplamiento | Estado |
|-----------------------|--------|
| Ruta del cliente en `SddIA/` | Ninguna. Único acoplamiento: `WorkingDirectory=%f`, inyectado por systemd en el encendido |
| Credenciales de correo | Solo en `{instancia}/.SddIA/.dev/.env` |
| Semántica de triaje en el Core | Ninguna: norma en Librería, proceso empacado en códice |
| Nombre de la cápsula de descarga en proceso/acción | Ninguno: resuelto por `asset:fetch` |
| Cuerpo del correo en genoma o bus | Ninguno: `body_ref` a `.SddIA/inbox/` |
| Origen remoto en la acción de negocio | Ninguno: `download-remote-asset` ignora la procedencia |

**Veredicto:** desacople total. El Core opera a ciegas sobre el Códice inyectado.

## D5. Ratificación del Vértice Biológico — concedida (2026-08-17)

Dos puntos alteraban SSOT compartido. Ambos ratificados; gate G1 desbloqueado.

| # | Punto | Impacto | Estado |
|---|-------|---------|--------|
| R-01 | Elevar `codex-contract` a v1.2.0 (D-06) | Contrato de familia; afecta a los 4 códices existentes solo como campo opcional | **Ratificado** |
| R-02 | Añadir `codex-kalma2-assistant/process/` a `process_domain_roots` en `cumulo.paths.json` (D-05) | SSOT de topología; precedente existente con `codex-software-engineering` | **Ratificado** |

### D5.1 Desglose de entrega ratificado

Entrega en **dos PBIs secuenciales**, cada uno con PR único y cierre documental propio conforme a `task-closure-documental`:

| PBI | Alcance | Feature |
|-----|---------|---------|
| `PBI-KALMA2-MVP-01A` | T0–T5 + aduana sensorial | `kalma2-mvp-sensorial-email` |
| `PBI-KALMA2-MVP-01B` | T6–T8 + aduana de sincronización | `kalma2-mvp-sync-activos` |

Ambos consumen esta clarificación y `spec.md` como dossier compartido: la especificación del genoma **no se duplica** por ola.

## D6. Handoff a Dédalo

Entradas cerradas: 13 fisuras resueltas, 13 decisiones, 11 entidades a forjar, 4 mutaciones de SSOT, 2 ratificaciones.

Restricciones que Dédalo no puede relajar: contrato de Centinela (ceguera lógica), read-only IMAP, salida temprana determinista antes de LLM, DI para la cápsula de descarga, payload ligero.

Continúa en `spec.md` (genoma) y `plan.md` (línea de montaje).
