---
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL
uuid: "6d64bcc7-b677-4c43-b239-928e279d2a04"
title: "[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub"
format: markdown
version: "2.1.0"
created: "2026-08-28"
updated: "2026-08-28"
status: "pendiente"
priority: "alta"
process: bug-fix
type: kaizen
dispatch: false
suggested_branch: fix/kaizen-aduana-evolution-local
incident_ref: "PR #209 — wasi-runtime-smoke rojo en evolution gate (delta) con 18 findings EVOL_MATERIAL_UNREGISTERED; ninguna capa local lo detectó"
friction_ids:
  - F-HOOKS-NO-INSTALADOS
  - F-PRE-PUSH-SKIP-POR-PR-ABIERTO
  - F-IF-TOUCHED-CONDICION-INVERTIDA
  - F-RANGE-BASE-SIN-FETCH
  - F-BASE-STALE-INVISIBLE
  - F-IMPACT-ASSESSMENT-STUB
  - F-IMPACT-SOLO-SOURCE-FEATURE
  - F-DCC-SIN-GATE-EVOLUTION
  - F-GRANULARIDAD-PRECOMMIT-VS-CI
architectural_constraints:
  - A-ADUANA-EN-PROCESO
  - A-HOOKS-SIN-INSTALACION-MANUAL
  - A-QA-INDEPENDIENTE-DE-PRESENTACION
  - A-PARIDAD-LOCAL-CI
  - A-DEGRADACION-DECLARADA
  - A-GENOMA-VIA-ENTITY-MANAGER
related:
  - SddIA/scripts/qa/git-hooks/install-hooks.sh
  - SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  - SddIA/scripts/qa/git-hooks/pre_commit_gate.sh
  - SddIA/scripts/qa/git-hooks/hook_common.sh
  - SddIA/tools/sddia-qa/src/gate_evolution.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/evolution/evolution_contract.md
  - .github/workflows/sddia-index-qa.yml
  - docs/todos/done/[KAIZEN] Init lab feature — bóveda reinyecta AGENT_RUNTIME y carrera de agentes.md
source_audit: "Auditoría física 2026-08-28 sobre las cuatro capas: ls .git/hooks (solo .sample), lectura de pre_push_gate.sh (orden de guardas), gate_evolution.rs (range_diff_spec, range_touches_evolution), phase_capsules.rs (capsule_delivery_impact_assessment), fases de delivery-close-cycle v1.1.1 y job wasi-runtime-smoke"
review_notes: "v2.0.0 — dos hallazgos nuevos de topología: F-PRE-PUSH-SKIP-POR-PR-ABIERTO (la guarda de idempotencia de PR desactiva el gate antes de ejecutarlo) y F-RANGE-BASE-SIN-FETCH (base del rango local ≠ base CI). Dependencia PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS cerrada. v2.1.0 — laudo de degradación táctica (fail-soft de rango) integrado en §1.4 y §5.4; AEL-CA5 reescrito (+CA5b) y AEL-CA13/CA14 añadidos. Corregido: la degradación no es binaria (el caso dominante es origin/main stale, no ausente), stdout está reservado al JSON de cápsula (advertencia a stderr) y la degradación no bloquea pero el veredicto sí."
---

# [KAIZEN] Aduana evolution local inexistente

## 1. Falla estructural y contexto

El contrato de evolución (`EVOL_MATERIAL_UNREGISTERED`) exige que toda mutación material
bajo `SddIA/` tenga correlato en `SddIA/evolution/`. La única capa que lo hizo cumplir en
PR #209 fue **CI**, y lo hizo con 18 findings de golpe. Las cuatro capas locales
diseñadas para impedirlo no operaron — no por un fallo puntual, sino porque **ninguna de
las cuatro puede detectar la clase de finding que el contrato define como principal**.

### 1.1. Evidencia por capa

| Capa | Estado verificado | Prueba física |
|------|-------------------|---------------|
| `.git/hooks/` | Vacía de hooks SddIA | Solo 15 ficheros `*.sample`; `core.hooksPath` no definido |
| pre-push → `gate-evolution --range --if-touched` | Ciego por construcción | `range_touches_evolution()` corta cuando el diff **no** toca `evolution/` |
| pre-push → guarda de presentación | Aborta antes del gate | `#branches == 0 → exit 0` precede a la invocación del gate |
| `delivery-impact-assessment` | Stub declarado | `impact: "none"`, `sddia_paths: []`, `note: "git diff omitido en stub Rust"` |
| `delivery-close-cycle` v1.1.1 | Sin fase evolution | Fases: Snapshot, Impacto SddIA, Aduana EDA, Push, PR, Sello, Higiene |

El commit `5f67ce0` («cosas») es evidencia colateral: un commit sin mensaje canónico
atravesó pre-commit sin resistencia.

### 1.2. La inversión de `--if-touched` (F-IF-TOUCHED-CONDICION-INVERTIDA)

```rust
// gate_evolution.rs:365
if if_touched && !all && !range_touches_evolution(evo, &paths) { /* skipped: EVOL_OK */ }
```

El predicado se evalúa sobre `SddIA/evolution/`, no sobre el material. Pero
`EVOL_MATERIAL_UNREGISTERED` **sólo puede ocurrir cuando `evolution/` no está tocado**.
El flag es por tanto una tautología de ceguera: en el 100 % de los casos que el gate
existe para atrapar, el gate se salta a sí mismo y retorna `success: true`. No es un bug
de borde; es la anulación del gate en su caso de uso central.

### 1.3. La guarda de presentación desactiva el gate (F-PRE-PUSH-SKIP-POR-PR-ABIERTO)

`pre_push_gate.sh` recolecta ramas, descarta las que ya tienen PR `OPEN`/`MERGED`
(`should_skip_pre_push_present`) y **sale con 0 si la lista queda vacía** — antes de
llegar a `resolve_sddia_qa` y al gate (líneas 44-52).

Consecuencia directa sobre el incidente: en cuanto el PR #209 estuvo abierto, **todo push
posterior de corrección quedó exento de QA local**. El bucle real de trabajo (abrir PR →
iterar → empujar) es exactamente el régimen en el que la aduana desaparece. La guarda
existe para no re-presentar el PR; su efecto colateral es apagar la verificación.

### 1.4. Deriva de base local ↔ CI (F-RANGE-BASE-SIN-FETCH, F-BASE-STALE-INVISIBLE)

| Ejecutor | Base del rango |
|----------|----------------|
| CI (`wasi-runtime-smoke`) | `actions/checkout` con `fetch-depth: 0` + `git fetch --no-tags origin main` **antes** de `gate-evolution --range` |
| pre-push local | `range_diff_spec()` → primer ref que exista: `origin/main` (posiblemente stale) o `main` local |

CI resuelve una base fresca sobre un clon completo, y lo hace **fuera** del gate: el
`fetch` es un paso del workflow, no código de `sddia-qa`. El gate local no dispone de esa
garantía y no la reclama.

El error del análisis inicial fue tratar la degradación como binaria («hay red» /
«no hay red»). La escalera real tiene tres estados, y el intermedio es el dominante:

| Estado | Base efectiva | Frecuencia local | Visibilidad hoy |
|--------|---------------|------------------|-----------------|
| `synced` | `origin/main` recién fetcheado | Solo en CI | — |
| `stale` | `origin/main` existente pero antiguo | **Caso habitual** | Cero: indistinguible de `synced` |
| `local` | `main` local (sin `origin/main`) | Rara (clon sin remoto) | Cero |

`range_diff_spec()` prefiere `origin/main` si el ref existe, y ese ref **casi siempre
existe**. Por tanto el fallback que hoy ocurre en la práctica no es «caer a `main`
local»: es **operar sobre un `origin/main` viejo creyendo estar sincronizado**. Un
diseño de degradación que solo cubra la caída de red deja intacto el agujero real.

Dirección del error en modo degradado — ambas direcciones son posibles, y conviene no
suponer que la degradación solo sobre-bloquea:

- **Sobre-bloqueo** (base más antigua → `merge-base` más atrás → rango ensanchado con
  material ya mergeado). Dirección tolerable en una aduana.
- **Sub-bloqueo** (base más reciente que la real → `merge-base` adelantado → material del
  inicio de la rama desaparece del diff). Ocurre cuando el `main` local va **por delante**
  de `origin/main`: `accept_pr.rs` hace `checkout main` + `merge --no-ff` **en local** y
  después empuja, y `L-FAILSOFT-SYNC` (PPR #203) tolera de forma deliberada que ese push
  falle sin tumbar el ciclo. En esa ventana el `main` local contiene una fusión que
  `origin/main` no tiene: el gate local declara APTO y CI bloquea. Dirección inaceptable
  si el veredicto local se toma como suficiente.

Corolario: el modo degradado es admisible como **señal temprana**, nunca como sustituto de
CI.

### 1.5. Asimetría de granularidad (F-GRANULARIDAD-PRECOMMIT-VS-CI)

`pre_commit_gate.sh` sí invoca `gate-evolution --json` sin flags — es decir, sobre el
diff **staged**. Esa capa es más estricta que CI en el eje temporal: exige que el
registro evolution viaje en el *mismo commit* que el material, mientras CI sólo lo exige
en el *mismo rango de rama*. El resultado práctico es fricción alta (commits incrementales
bloqueados) con cobertura baja (no ve la acumulación de rama), y es la explicación más
plausible del hábito operativo que dejó los hooks sin instalar: la capa que más molesta
es la que menos protege.

## 2. Impacto

- **Aduana desplazada a CI**: la detección llega minutos tarde, en lote y en remoto.
  18 findings en un solo rojo obligan a una sesión de reparación retroactiva en lugar de
  un registro incremental durante el trabajo.
- **Riesgo de reincidencia estructural**: DA-6 prohíbe la vigilancia remota en bucle, de
  modo que cada rojo de evolution en CI consume el único intento barato de corrección.
- **Contrato evolution degradado a formalidad**: si el registro se compone al final para
  apagar CI, deja de ser bitácora causal del cambio y pasa a ser peaje administrativo.
- **Falsa sensación de blindaje**: `external-ai-constraints.md` afirma que «la aduana
  física refuerza esta norma». Hoy esa afirmación es falsa en local: no hay aduana física.
- **Instalación como punto único de fallo humano**: cualquier clon nuevo del repo nace sin
  ninguna de las capas, sin señal alguna de que le faltan.

## 3. Hipótesis de causa raíz

1. **La instalación es una capa, y las capas manuales se degradan** — `install-hooks.sh`
   copia/enlaza a `.git/hooks/`, un directorio no versionado y no verificable. Nada en el
   arranque del ecosistema comprueba su presencia. La alternativa (`core.hooksPath` sobre
   el directorio versionado) ya está documentada en
   `docs/features/pbi-005-hito3-git-hooks/implementation.md` como **«Equipo (opcional)»**:
   la opcionalidad es la causa raíz.
2. **El predicado del gate mide el sitio equivocado** — `--if-touched` pregunta por
   `evolution/` cuando debe preguntar por material genómico. El nombre del flag describe
   la implementación, no la intención; la intención («no ejecutar el gate si el cambio es
   irrelevante») exige el predicado inverso.
3. **QA subordinada a la presentación** — en `pre_push_gate.sh` el gate vive *después* de
   la lógica de idempotencia de PR. Son dos responsabilidades independientes (verificar vs
   presentar) acopladas por orden de ejecución.
4. **Autoridad de la aduana en el disparador, no en el proceso** — el gate se invoca desde
   el hook. Cualquier ruta de entrega que no pase por el hook (invocación directa de
   `delivery-close-cycle` por un agente) carece de verificación, porque el proceso
   canónico no la declara entre sus fases.
5. **Stub tolerado por doble escape** — `capsule_delivery_impact_assessment` no mira el
   diff, y además se auto-salta si `source_process != "feature"`. El filtro proviene del
   propio contrato del proceso («Si `source_process == feature`»), que es erróneo en la
   raíz: un `bug-fix` o una `refactorization` mutan `SddIA/` con idéntica frecuencia y
   contraen la misma obligación de registro. Se suma `SDDIA_LAB_SKIP_IMPACT_ASSESSMENT`
   como tercera vía de anulación.
6. **Preferencia de ref sin noción de frescura** — `range_diff_spec()` elige por
   *existencia* del ref (`origin/main`, luego `main`), no por *actualidad*. Un ref que
   existe se considera válido sin más, de modo que el sistema no tiene vocabulario para
   expresar «mi base es vieja». Sin ese vocabulario, ninguna advertencia es posible: el
   silencio no es una omisión de mensaje, es la ausencia del concepto.
7. **Deriva contrato ↔ código** — el proceso documenta anti-recursión vía
   `SDDIA_SKIP_HOOKS=1` acotado a `git-manager`; el código la resuelve con
   `SDDIA_HOOK_DELIVERY_CLOSE=1` exportado en `invoke_process` y leído por
   `in_delivery_close_cycle`. Mismo efecto, mecanismo distinto al declarado: cualquier
   razonamiento sobre dónde queda cubierta la aduana partiendo del `.md` es inválido.

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AEL-CA1 | Los hooks SddIA están activos **sin paso de instalación manual**: `core.hooksPath` apunta al directorio versionado `SddIA/scripts/qa/git-hooks`, aplicado por el bootstrap del ecosistema e idempotente. `install-hooks.sh` queda como compatibilidad, no como requisito |
| AEL-CA2 | Existe verificación de aduana viva (`sddia-qa verify-hooks --json` o equivalente) que retorna finding accionable cuando `core.hooksPath` no resuelve a los hooks del repo, con remedio literal en el mensaje |
| AEL-CA3 | El predicado de `--if-touched` se evalúa sobre **material genómico** (prefijos `SddIA/`), no sobre `SddIA/evolution/`. Si el rango toca material, el gate se ejecuta; si sólo toca `evolution/` o nada material, se salta con motivo explícito |
| AEL-CA4 | El gate de evolution en pre-push se ejecuta **antes e independientemente** de la guarda de presentación de PR: rama con PR `OPEN`/`MERGED` sigue verificándose. Ninguna ruta de retorno temprano precede al gate |
| AEL-CA5 | **Paridad consciente de base.** `gate-evolution --range` intenta alinear la base con `origin/main` bajo control estricto de latencia (fetch acotado, opt-in por flag). Si el intento expira, la red es inaccesible o el ref remoto no existe, conmuta a la mejor base disponible **declarando el modo** (`synced` \| `stale` \| `local`) en la envoltura JSON. La degradación **no** bloquea por sí misma; el veredicto sobre el material sí |
| AEL-CA5b | La degradación es **auditable, no solo anunciada**: la envoltura incluye `base_resolution: { mode, ref, spec, age_seconds, fetch_outcome }`. En modo `stale`/`local` se emite además advertencia legible **por stderr** (nunca por stdout, reservado al JSON de cápsula) |
| AEL-CA6 | La granularidad queda homologada: la exigencia de correlato evolution es **de rama** (rango), no de commit. El gate pre-commit se alinea con ese criterio o se retira, documentando la decisión — prohibido dejar dos semánticas contradictorias vivas |
| AEL-CA7 | `capsule_delivery_impact_assessment` calcula el diff real contra la base del rango, filtra prefijos de material vía `cumulo.paths.json` y propaga `sddia_paths` no vacío cuando hay mutaciones. Se elimina la nota de stub |
| AEL-CA8 | El impacto SddIA se evalúa para `feature`, `bug-fix` y `refactorization`. El filtro `source_process == "feature"` desaparece del handler **y** del contrato del proceso |
| AEL-CA9 | `delivery-close-cycle` declara la verificación de evolution como **fase del proceso** (junto a «Aduana EDA genómica»), de modo que toda ruta de entrega la atraviesa. El hook delega en el proceso; no duplica el gate |
| AEL-CA10 | La mutación del genoma (`delivery-close-cycle.md`, `hash_signature`) se realiza vía `execute-process` → `entity-manager` y `recalc-process-hash-signatures`. Cero edición manual del `.md` del proceso |
| AEL-CA11 | Tests unitarios sobre el predicado del gate: (a) material sin evolution → bloquea con `EVOL_MATERIAL_UNREGISTERED`; (b) sólo `evolution/` tocado → `EVOL_OK` sin ejecutar veredicto; (c) rango sin material → skip explícito; (d) rama con PR abierto → gate ejecutado |
| AEL-CA12 | Smoke reproducible: mutación bajo `SddIA/` sin entrada evolution → `delivery-close-cycle` bloquea localmente **antes** del push, con el mismo `reason_code` que emitiría CI. Registrado en `validacion.md` |
| AEL-CA13 | **CI no admite modo degradado.** El job exige base sincronizada (`--require-synced-base` o equivalente): `base_resolution.mode != "synced"` → finding con exit ≠ 0. La autoridad del veredicto permanece determinista aunque la capa local sea tolerante |
| AEL-CA14 | El intento de sincronía es **temporalmente acotado y no interactivo**: presupuesto de latencia explícito (2–3 s), prompts de credenciales desactivados (`GIT_TERMINAL_PROMPT=0`, `BatchMode` en SSH) y proceso hijo terminado al expirar. Test: remoto inalcanzable → el gate retorna en ≤ presupuesto + margen, con `fetch_outcome: "timeout"` |

## 5. Laudo arquitectónico (Filtro A)

Restricciones de diseño, no sugerencias. Una implementación que las infrinja se rechaza
en revisión.

### 5.1. La instalación no es una capa (A-HOOKS-SIN-INSTALACION-MANUAL)

Una aduana cuya existencia depende de que un operador recuerde ejecutar un script no es
una aduana: es una convención. El estado verificado (`.git/hooks` sólo con `.sample` tras
meses de operación) es la demostración empírica.

`core.hooksPath` apuntando al directorio **versionado** elimina la capa de instalación
completa: los dispatchers (`pre-commit`, `pre-push`, `post-merge`) ya resuelven
`REPO_ROOT` por `git rev-parse` y ya honran `SDDIA_SKIP_HOOKS`, de modo que funcionan sin
modificación desde su ubicación de origen. Prohibido resolver esto añadiendo
documentación, recordatorios o un segundo script instalador.

### 5.2. La autoridad de la aduana reside en el proceso (A-ADUANA-EN-PROCESO)

El hook es un **disparador**, no la autoridad. Si el gate vive en `pre_push_gate.sh`,
toda invocación directa de `delivery-close-cycle` (flujo agente, el habitual en este
ecosistema) queda exenta. La verificación debe declararse como fase del proceso canónico
y el hook debe limitarse a invocarlo.

Corolario anti-duplicación: una vez la fase existe, el gate desaparece de
`pre_push_gate.sh`. Dos ejecuciones del mismo veredicto en la misma operación son ruido y
divergen con el tiempo.

### 5.3. QA no se subordina a la presentación (A-QA-INDEPENDIENTE-DE-PRESENTACION)

Que un PR ya esté abierto es información sobre la *presentación*, no sobre la *calidad*
del contenido que se empuja. Ninguna guarda de idempotencia de forja puede preceder ni
condicionar la verificación. El orden correcto es: verificar siempre, presentar si toca.

### 5.4. Paridad de predicado, degradación de base (A-PARIDAD-LOCAL-CI)

Corrección del laudo v2.0.0, que exigía base idéntica y convertía la indisponibilidad de
red en finding bloqueante. Esa exigencia es inaplicable: haría que un portátil sin
conexión no pudiera commitear, y el operador respondería con `SDDIA_SKIP_HOOKS=1` —
reproduciendo la causa raíz que este PBI ataca (§1.5).

La distinción correcta separa dos objetos que el laudo anterior fundía:

- **El predicado es invariante.** Mismo criterio de material, mismos prefijos, mismos
  `reason_codes`. Aquí no hay grados: divergencia de predicado entre local y CI es
  defecto, no configuración.
- **La base es best-effort con modo declarado.** Se intenta la mejor base alcanzable
  dentro de un presupuesto de latencia y se declara cuál se usó.

Lo que sigue prohibido es el **fallback silencioso**, y su definición se endurece: no
basta con avisar cuando falta `origin/main`. Un `origin/main` presente pero antiguo es
también base degradada (§1.4) y debe declararse como tal. Hoy ese caso es indistinguible
del sincronizado, y es el habitual.

### 5.5. La degradación se declara, no se disculpa (A-DEGRADACION-DECLARADA)

Tres restricciones sobre la mecánica de fail-soft:

1. **La degradación no es un veredicto.** No bloquea, no consume `reason_codes`, no altera
   `exitCode`. Vive en `base_resolution` + `warnings`. Recíprocamente: un veredicto de
   material sin registro **bloquea igual** en modo degradado. Confundir ambos planos
   convierte «no había red» en licencia para empujar material sin registrar.
2. **stdout es del contrato, no del humano.** `emit()` imprime una única línea JSON por
   stdout (`capsule-json-io`); inyectar ahí texto de advertencia rompe a todo consumidor
   del envelope. El canal humano es stderr — que es además el que el operador ve en un
   hook.
3. **La red no entra en el gate por defecto.** El intento de `fetch` es opt-in explícito
   (flag del disparador), nunca comportamiento implícito de `--range`. Un verificador que
   toca la red sin que se le pida deja de ser reproducible, y CI ya resuelve su base en un
   paso propio del workflow: si el gate fetcheara solo, CI haría el trabajo dos veces y la
   diferencia de modos quedaría enmascarada.

Corolario de autoridad: el modo degradado es señal temprana para el operador; el veredicto
soberano sigue siendo el de CI, donde el modo degradado está prohibido (AEL-CA13).

### 5.6. Mutación de genoma por la forja (A-GENOMA-VIA-ENTITY-MANAGER)

`delivery-close-cycle.md` vive en el genoma (`SddIA/library/codexes/`). Añadir la fase
exige `execute-process` → `entity-manager` y recálculo de `hash_signature`; la edición
manual del `.md` es mutación prohibida y la detecta `verify-process-integrity`. Nótese
además la deriva §3.7: el contrato debe quedar alineado con el mecanismo real de
anti-recursión en la misma pasada.

## 6. Notas de implementación

Orden sugerido, de menor a mayor superficie:

1. **`core.hooksPath` + verificación** (CA1, CA2) — resucita las capas existentes sin
   tocar su lógica. Es el cambio de mayor retorno inmediato: convierte tres gates muertos
   en gates vivos.
2. **Predicado del gate** (CA3, CA11) — `range_touches_evolution` pasa a
   `range_touches_material` con prefijos desde `cumulo.paths.json`. Tests antes del
   cableado. Independiente del punto 3: se puede entregar por separado.
3. **Resolución de base con degradación declarada** (CA5, CA5b, CA13, CA14) —
   `range_diff_spec()` devuelve hoy solo un `String`; debe pasar a devolver la estructura
   de `base_resolution`. Restricciones físicas del entorno actual:
   - `git()` usa `Command::output()`, que **bloquea sin límite**. No hay `wait_timeout`
     ni ninguna dependencia de timeout en `sddia-qa/Cargo.toml`. Camino sin dependencias
     nuevas: `spawn()` + bucle `try_wait()` contra `Instant` + `child.kill()` al expirar.
     Evitar `timeout(1)` de coreutils: no existe en Windows y el repo soporta ese entorno
     (`install-hooks.ps1`).
   - Sin `GIT_TERMINAL_PROMPT=0` (y `BatchMode=yes` vía `GIT_SSH_COMMAND` en remotos SSH)
     el presupuesto de latencia es ficticio: git puede quedarse esperando credenciales por
     terminal, fuera del control del temporizador.
   - `age_seconds` es medible sin red: fecha del commit de `origin/main` y/o reflog de
     `refs/remotes/origin/main`. Es la única forma honesta de distinguir `stale` de
     `synced` cuando el fetch no se intentó.
   - El paso `evolution gate (delta)` del workflow ya hace `fetch` propio con
     `fetch-depth: 0`; CI **no** debe pasar el flag de sincronía, solo el de exigencia
     (`--require-synced-base`).
4. **Orden de guardas en pre-push** (CA4) — mover el gate por encima del retorno temprano
   de `#branches == 0`. Cambio de tres líneas, cierra el hueco decisivo del incidente.
5. **Handler de impacto real** (CA7, CA8) — el diff ya se sabe calcular en
   `gate_evolution.rs::diff_paths`; reutilizar, no reimplementar. Consume la misma
   `base_resolution` del punto 3: dos resoluciones de base independientes reeditarían la
   divergencia que este PBI corrige.
6. **Fase en el proceso + retirada del gate del hook** (CA9, CA10, CA6) — mayor
   superficie, requiere forja y recálculo de firma. Último por dependencia de los pasos
   anteriores.

Coordinar con el hábito operativo, no sólo con el código: la fricción de §1.5 es la que
desinstaló los hooks. Si la corrección endurece las tres capas sin resolver la
granularidad (CA6), el resultado predecible es `SDDIA_SKIP_HOOKS=1` sistémico — que la
norma prohíbe sin PBI activo, pero que la fricción termina imponiendo.

## 7. Fuera de alcance

- Los 18 registros evolution del PR #209 (ya saneados en su ciclo).
- Contrato de evolución en sí (`evolution_contract.md`, códigos de razón): se consume, no
  se modifica.
- Aduana EDA genómica y `orphan_count` (dominio Argos, capa independiente).
- Idempotencia del fan-out de fractura (`PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA`).
- Fricciones de init de laboratorio (`PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS`, cerrado en
  `docs/todos/done/`).
