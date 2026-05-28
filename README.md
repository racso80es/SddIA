# SddIA Core: industrialización de inteligencia descentralizada

## Librería SddIA
Ecosistema de **activos técnicos tokenizables** (NFTs lógicos: definiciones versionadas, contratos y manifiestos con identidad estable) orientados a la **industrialización de la IA**: consumo reproducible, gobernanza explícita y trazabilidad entre núcleo canónico e instancias productivas.

## Ontología de Activos

| Entidad | Finalidad | Ubicación Core | Relación operativa |
|---------|-----------|----------------|-------------------|
| **Agent** | Orquestador de consciencia y responsable de una fase específica. | `paths.directories.agents` | Posee Skills y ejecuta Acciones dentro de un **Process**. |
| **Process** | Roadmap lógico de alto nivel para un objetivo macro (p. ej. feature). | `paths.directories.process` | Orquesta el relevo (*handoff*) entre distintos **Agents**. Declara `workspace_template` obligatorio (process-contract v1.4.0); el CLI materializa el Workspace bajo `paths.workspacesRoot`. |
| **Action** | Paso atómico, indivisible y auditable de ejecución. | `paths.directories.actions` | Invoca **Skills** o **Tools** para el trabajo técnico. |
| **Skill** | Capacidad técnica especializada definida por contrato. | `paths.directories.skills` | Ejecutada por **Cápsula** blindada (binario Rust o script Python bajo contrato). |
| **Tool** | Capacidad de infraestructura o utilidad de dominio. | `paths.directories.tools` | Servicios base a las **Actions** vía **Cápsula**. |
| **Event** | Contrato inmutable ECST; clasificado por `event_family` (`telemetry`, `orchestration`, `domain`). | `paths.directories.events` — genoma fractal | Clase en `{name}.md` bajo [events-contract.md](SddIA/events/events-contract.md); instancia en bus fractal (`eda_fractal`) o pipeline V3+ (`eda_bus`) según familia. Índice agregador: [SddIA/events/index.md](SddIA/events/index.md). Ver [CONSTITUTION_CORE.md](SddIA/CONSTITUTION_CORE.md) §3.1. |
| **Library_Codex** | Paquetes de normas orquestadas por dominio. | `paths.directories.library_codexes` | Agrupación de conocimiento técnico a cumplir por los **Agents**. |
| **Library_Norm** | Reglas técnicas atómicas, patrones y prohibiciones de *code-smells*. | `paths.directories.library_norms` | Cantera de la **Librería** (`SddIA/library/norms/`). **No** confundir con la normativa operativa del Core (`SddIA/norms`). |
| **Normativa de ejecución (Core)** | Contratos y normas de operación del núcleo (cápsulas, Git, triage, etc.). | `paths.directories.norms` | Árbol `SddIA/norms`; convive con la Librería; distinto alcance y clave SSOT que `library_norms`. |

### Dos canales de normativa (anti-dualidad)

| Canal | Clave en `cumulo.paths.json` | Ruta física (referencia) |
|-------|------------------------------|--------------------------|
| Operación del Core | `directories.norms` | `SddIA/norms` |
| Librería — normas atómicas | `directories.library_norms` | `SddIA/library/norms/` |

Jerarquía operativa: **Process** segmenta el objetivo en fases; cada fase asigna un **Agent** titular; el **Agent** descompone en **Actions**; las **Actions** consumen **Skills** y **Tools** materializados en cápsulas.

### Eventos: genoma, runtime e instancia

| Ruta | Rol | Contenido |
|------|-----|-----------|
| **`SddIA/events/`** | Genoma (Core) | **Trinidad de Estímulos:** Clases ECST en `{telemetry,orchestration,domain}/`; contrato maestro [`events-contract.md`](SddIA/events/events-contract.md) e índice agregador [`index.md`](SddIA/events/index.md) en raíz. Códice por familia: [`telemetry/index.md`](SddIA/events/telemetry/index.md), [`orchestration/index.md`](SddIA/events/orchestration/index.md), [`domain/index.md`](SddIA/events/domain/index.md). |
| **`/.events/`** | Runtime (bus local) | Instancias volátiles ECST: **bus fractal** (`eda_fractal`) y **pipeline dominio legacy V3+** (`eda_bus`) en coexistencia (D0.2). |
| **`.SddIA/events/`** | Instancia (proyecto) | Personalización por repositorio productivo (Vía C). **No** es cola del bus federal. |

Definición operativa de **Event**: *contrato inmutable de comunicación asíncrona; señal con propósito que opera bajo coreografía pura.* Toda Clase declara `event_family`: `{ telemetry, orchestration, domain }` (enum estricto en [`events-contract.md`](SddIA/events/events-contract.md)).

Programa de referencia: [Telemetría Reactiva EDA S+ Grade](docs/features/telemetria-reactiva-eda-fase0/impact-analysis.md) (Fases 0–6).

#### Trinidad de Estímulos

| Familia | Naturaleza | Emisor autorizado | Destino runtime |
|---------|------------|-------------------|-----------------|
| `telemetry` | Ruido físico (Nivel 1) | **Solo CLI** (Peaje Termodinámico) | `./.events/telemetry/` |
| `orchestration` | Línea de montaje táctica | CLI (éxito) / auditores | `./.events/orchestration/` |
| `domain` | Verdad ontológica (Nivel 3) | Agentes Core (Cúmulo, Cerbero, Radamanto, …) | `./.events/domain/` |

> **Regla de oro:** no mezclar telemetría cruda con orquestación ni dominio en la misma ruta física.

#### Genoma fractal (`SddIA/events/`)

```
SddIA/events/
├── events-contract.md
├── index.md
├── telemetry/index.md      ← Códice de Familia
├── orchestration/index.md
└── domain/index.md
```

Simetría fractal: la topología del genoma refleja la del runtime bajo `./.events/{family}/`.

#### Bus fractal runtime (`eda_fractal`)

SSOT: [`cumulo.paths.json`](SddIA/core/cumulo.paths.json) → `eda_fractal.*`. Directorio `/.events/` en `.gitignore`.

| Ruta | Propósito | Enrutador | Suscripciones |
|------|-----------|-----------|---------------|
| `./.events/telemetry/` | Alta frecuencia; purga post-consenso suscriptores | [`route-telemetry`](SddIA/process/route-telemetry.md) | [`event-telemetry-subscriptions.json`](SddIA/core/event-telemetry-subscriptions.json) |
| `./.events/orchestration/` | Latencia mínima; línea de montaje | [`route-orchestration`](SddIA/process/route-orchestration.md) | [`event-orchestration-subscriptions.json`](SddIA/core/event-orchestration-subscriptions.json) |
| `./.events/domain/` | Gobernanza; Cerbero / Cúmulo / procesos reactivos | [`route-domain`](SddIA/process/route-domain.md) | [`event-domain-subscriptions.json`](SddIA/core/event-domain-subscriptions.json) |

Telemetría con **fan-out** (p. ej. `radamanto-batch` + `telemetry-compliance-audit`): cada consumidor sella `delivery_state`; la purga física del JSON pertenece solo a la infraestructura (`route-telemetry` o `event-sweeper`).

#### Pipeline dominio legacy (V3+)

Coexistencia gradual (D0.2): eventos dominio legacy y flujos como `PullRequest_Presented` → `pull-request-review` siguen el pipeline monolítico sobre `eda_bus.*`:

```
.events/pending/                          ← entrada (padre inmutable hasta sweeper)
.events/processing/
.events/processing/subscribers/           ← testigos en vuelo
.events/processed/
.events/processed/subscribers/            ← testigos OK
.events/dead-letter/
.events/dead-letter/subscribers/          ← testigos KO (p. ej. ecst-gate)
```

```mermaid
flowchart LR
  EM[Emisor] --> P[pending]
  P --> W[event-watcher]
  W --> RDE[process route-domain-event]
  RDE --> PROC[processing + testigos]
  PROC --> OUT[processed / dead-letter]
  OUT --> SW[event-sweeper]
  SW --> PURGE[purga pending]
```

| Paso | Componente | Responsabilidad |
|------|------------|-----------------|
| 1 | Emisores (`emit-domain-mutation`, `emit-pr-presented-event`, …) | Escriben ECST en `.events/pending/` |
| 2 | `event-watcher.py` | Monitoriza `pending/`; delega en `route-domain-event` |
| 3 | **`route-domain-event`** | Gate ECST; fan-out async; purga `pending/` al consenso |
| 4 | Suscriptores | Trabajo de dominio; testigos con `result_status` |
| 5 | `event-sweeper.py` | Purga residual; alerta Kaizen si hay `dead-letter/` |

**Invocación manual (laboratorio):**

```bash
python SddIA/scripts/qa/execute-process.py --process route-domain-event \
  --inputs '{"event_file_path":".events/pending/<event_id>.json"}'
```

Modo sync: `SDDIA_LAB_ROUTE_SYNC=1`. Plantilla Vía C: [`SddIA/templates/eda-instance-events/README.md`](SddIA/templates/eda-instance-events/README.md). Features: [refactor-topologia-eventos-ola-c-v3](docs/features/refactor-topologia-eventos-ola-c-v3/), [telemetria-reactiva-eda-fase3](docs/features/telemetria-reactiva-eda-fase3/).

### Configuración: Jerarquía de Bóvedas

Secretos y variables de entorno del runtime se cargan desde **bóvedas locales** (no versionadas), no desde `.env` dispersos en subdirectorios de cápsulas. SSOT de rutas: clave `env_hierarchy` en [SddIA/core/cumulo.paths.json](SddIA/core/cumulo.paths.json).

| Ruta | Bóveda | Rol | Contenido |
|------|--------|-----|-----------|
| **`.dev/.env`** | Global (repo) | Valores compartidos del clone | Defaults de equipo, CI local, variables comunes al workspace |
| **`.SddIA/.dev/.env`** | Instancia (proyecto) | Overrides soberanos (Vía C) | Secretos y configuración táctica; **prevalece** sobre la bóveda global |

**Precedencia al arrancar entrypoints:**

1. **Entorno del SO** — máxima prioridad; los ficheros no sobrescriben variables ya definidas.
2. **`.dev/.env`** — rellena el diccionario intermedio.
3. **`.SddIA/.dev/.env`** — sobrescribe claves del paso anterior en el diccionario; se vuelca a `os.environ` con `setdefault`.

Si existen **ambas** bóvedas, el runtime registra en stderr: `[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env`.

> **Nota EDA:** `PYTHONUTF8=1` u otras variables en la bóveda global **no sustituyen** el flujo canónico de presentación/merge (`delivery-close-cycle` → `PullRequest_Presented` → `accept-pr` → `PullRequest_Merged`). Ver `SddIA/norms/pull-request-orchestration.md`.

**Entrypoints que cargan la jerarquía** (vía `SddIA/scripts/qa/env_loader.py`) **antes** de invocar cápsulas:

| Entrypoint | Punto de carga |
|------------|----------------|
| `SddIA/scripts/qa/execute-process.py` | Tras resolver raíz del repo |
| `SddIA/scripts/qa/execute_process_capsules.py` | Inicio de `run_process()` |
| `SddIA/scripts/qa/execute-action.py` | Inicio de `main()` |
| `SddIA/scripts/daemons/event-watcher.py` | Inicio de `main()` |
| `SddIA/scripts/daemons/event-sweeper.py` | Inicio de `main()` |

Las cápsulas (p. ej. `iota-immutable-publisher`) **consumen** `process.env` / `os.environ` ya inyectado; **prohibido** `dotenv` local en el directorio del tool.

**Plantillas:** `.dev/.env.example` (global) y `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example` (instancia). Copiar a `.dev/.env` y `.SddIA/.dev/.env` en la raíz del workspace. Variables habituales en instancia: `IOTA_WALLET_SECRET`, `IOTA_ANCHOR_PACKAGE_ID`; en global: `SDDIA_ENV`, flags `SDDIA_LAB_*`, `PYTHONUTF8`, `SDDIA_IOTA_TIMEOUT_SECONDS`.

**Migración desde legacy:** mover contenido de `SddIA/scripts/tools/iota-immutable-publisher/.env` → `.SddIA/.dev/.env` y eliminar el fichero local de la cápsula.

Documentación de feature: [docs/features/ampliacion-configuracion-entornos/](docs/features/ampliacion-configuracion-entornos/).

## Agentes del Core (resumen)

Catálogo canónico (UUID, `allowed_policies`, versiones): `{paths.directories.agents}` según el SSOT [SddIA/core/cumulo.paths.json](SddIA/core/cumulo.paths.json) (`cumulo.paths.json`); tabla e índice en [SddIA/agents/index.md](SddIA/agents/index.md). Cada definición vive en `{name}.md` junto al contrato de familia `agents-contract.md`.

| Agente | Rol (una línea) |
|--------|------------------|
| **Cerbero** | Peaje RBAC: autoriza o bloquea invocaciones según contexto y políticas. |
| **Cúmulo** | SSOT: topología, índices y coherencia documental del Core. |
| **Tekton** | Ejecución: materializa procesos delegando en cápsulas, sin terminal cruda. |
| **Mayeuta** | Clarificación: estabiliza el *qué* y el *por qué*; no diseña procesos ni código. |
| **Dedalo** | Planificación: norm pack + blueprint de **Process** alineado a contrato y RBAC del ejecutor. |
| **Argos** | Verificación: inspector de la **materia** (artefactos, calidad estructural, tests); juicio por evidencia. |
| **Radamanto** | Actuario de **confianza**: batching de telemetría CLI, umbrales deterministas, sellado DLT de estatus de herramientas. Ver [`radamanto.md`](SddIA/agents/radamanto.md). |

**Flujo típico:** Mayeuta → Dedalo → Tekton → Argos. Cerbero actúa en cada delegación a cápsulas (Peaje RBAC); Cúmulo gobierna rutas y catálogos.

**Argos vs Radamanto:** Argos audita código y artefactos concretos; Radamanto acumula estadística agregada de telemetría y gobierna estatus macroscópico (`Tool_Degraded`, `Status_Restored`, `Tool_Deprecated`) vía DLT. Radamanto **no** evalúa diffs ni mide por sí mismo.

**Self-Healing (alto nivel):** telemetría degradada → Radamanto emite `Tool_Degraded` → Cerbero revoca RBAC → Tekton/Dédalo reparan en sandbox → Argos valida estructura → telemetría exitosa → Radamanto `Status_Restored` → Cerbero rehabilita. Tras `max_recovery_attempts` → `Tool_Deprecated`. Detalle: [telemetria-reactiva-eda-fase4](docs/features/telemetria-reactiva-eda-fase4/).

## Orquestación multi-agente y relevo por artefactos

La colaboración entre agentes no es mensajería efímera: es **línea de montaje** gobernada por el **Process**.

1. El **Process** fija qué **Agent** tiene el mando en cada fase.
2. Al arrancar, el **CLI** parsea `workspace_template` del proceso, genera `execution_id` (UUID) y materializa el **Workspace dinámico** bajo `paths.workspacesRoot` (SSOT: `.SddIA/workspaces/{process_name}/{execution_id}/`).
3. La coordenada absoluta se inyecta en el payload táctico (`workspace_path`); las Entidades de Dominio operan con **Ceguera Espacial** (no conocen rutas del repositorio).
4. El traspaso documental usa `persist_ref` en `docs/features/` o `docs/fixes/` — **ortogonal** al workspace operativo. Los aliases `featurePath` / `fixPath` apuntan a documentación, no al territorio de ejecución.
5. Las ED **no escriben en disco directamente**: invocan `filesystem-manager` vía `capsule-json-io` (stdin/stdout JSON) sobre el Workspace inyectado.
6. El agente emisor deposita **artefactos** en el Workspace; el receptor los **audita** antes de asumir la fase siguiente.

Sin Workspace materializado y artefactos versionables, no hay handoff válido bajo este modelo. Detalle: [telemetria-reactiva-eda-fase2](docs/features/telemetria-reactiva-eda-fase2/).

## Aduana Universal (CLI)

Toda ejecución transita por el CLI (`execute-process`, `execute_process_capsules`). El **Peaje Termodinámico** (distinto del Peaje RBAC de Cerbero) intercepta cada invocación:

| Paso | Acción |
|------|--------|
| 1 | Cronómetro antes de ejecutar la cápsula |
| 2 | Al finalizar: capturar `exit_code`, `duration_ms`, `asset_id` |
| 3 | Emitir `Raw_Execution_Finished` (familia `telemetry`) en `./.events/telemetry/` |
| 4 | En éxito: emitir además evento de orquestación según blueprint del proceso |

**Fail-soft (D3.13):** fallo E/S al escribir telemetría no detiene el hilo de negocio.

**Recibos termodinámicos (opcionales):** si la cápsula devuelve `telemetry_receipt` en stdout JSON, el CLI lo anexa al payload telemetría. Omisión **no** falla la ejecución de negocio. Contratos ED declaran `telemetry_provided` / `telemetry_schema` en skills y actions.

**Auditoría de cumplimiento:** el proceso [`telemetry-compliance-audit`](SddIA/process/telemetry-compliance-audit.md) cruza recibo vs contrato ED; incumplimiento → `Telemetry_Compliance_Breached` en `./.events/domain/`. Gobernanza reactiva post-breach: pendiente (Kaizen). Detalle: [telemetria-reactiva-eda-fase5](docs/features/telemetria-reactiva-eda-fase5/).

## Estándar de entidades de dominio SddIA
**Toda** familia de entidades de dominio debe cumplir el siguiente rigor arquitectónico:
1. **Ubicación SSOT:** Dispone de su ubicación según las indicaciones del agente Cúmulo (`{paths.directories...}`).
2. **Contrato Legal:** En dicha ubicación ha de existir, innegociablemente, un contrato para la implementación de entidades `{entidad}-contract.md`.
3. **Índice de Trazabilidad:** En dicha ubicación ha de existir un índice (`index.md`) con los items correspondientes a las implementaciones de entidades existentes. El agente Cúmulo tiene la responsabilidad de la coherencia de datos indicados por cada implementación.

## Cicatriz digital y estándar atómico
**Toda** entidad de dominio SddIA nace con **Cicatriz Digital**: un único archivo de definición `{name}.md` que incluye:
- Cabecera **YAML** obligatoria (contrato de la entidad).
- **Versión SemVer** y **UUID v4 inmutable** asignado en creación (identidad estable para trazabilidad y catálogo).
- Cuerpo en Markdown con propósito y límites del activo.
Ese paquete es el prerequisito para integración en la **Librería SddIA** y su modelo de activo direccionable (capa NFT-ready).

## Desacoplamiento Core / instancia
Definición de núcleo en este repositorio; especialización, constitución táctica y **secretos** en instancia local bajo `.SddIA/` (constitución, eventos, **workspaces** en `.SddIA/workspaces/`, **bóveda `.SddIA/.dev/.env`**). SSOT rutas: [`cumulo.paths.json`](SddIA/core/cumulo.paths.json) (`paths.workspacesRoot`, `eda_fractal`, `eda_bus`). La bóveda global `.dev/.env` complementa valores compartidos del clone. Ver [Jerarquía de Bóvedas](#configuración-jerarquía-de-bóvedas). Sin lógica de negocio dispersa fuera de **Actions** orquestadas y **Cápsulas**; sin `.env` operativos en subdirectorios de tools.

## Estándar de ejecución
Lógica crítica en **Cápsulas** (preferente binario Rust; Python permitido cuando esté explicitado y bajo contrato). Contrato de E/S: JSON por stdin/stdout según `SddIA/norms/capsule-json-io.md`. Los agentes orquestan; no sustituyen a la cápsula en cómputo ni en contrato de I/O.
