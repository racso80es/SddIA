# Informe de Diagnóstico: Extracción, Formalización Contractual y Desacople de Eventos en SddIA (`MAN-32` → `MAN-50`)

## 1. Propósito y Alcance del Informe

Este informe recoge el estudio del recorrido seguido en **SddIA** desde la extracción e identificación de eventos hasta su consolidación como contrato formal (`ECST` / `EventManagerPort` / `EventUri` / schemas y `entity-manager`) a lo largo de la cadena `MAN-32` → `MAN-40` → `MAN-43` → `MAN-49` → `MAN-50`.

Su finalidad es establecer una base diagnóstica clara y evidenciada sobre:
1. Cómo se articuló el tránsito desde el modelo basado en sistema de archivos (`.events/` y `sddia-event-store`) hacia el contrato y puerto productivo.
2. Por qué el trabajo se fragmentó en múltiples PBIs, sub-olas y auditorías encadenadas en lugar de cerrarse en un ciclo inicial limpio.
3. Qué puntos de fricción estructural, documental y de runtime quedaron expuestos en las auditorías.

---

## 2. Panorama del Recorrido y Fragmentación

El desacople de eventos no se resolvió en un único ciclo por la confluencia de decisiones incrementales, cambios de alcance durante la marcha y puntos ciegos entre el diseño, las verificaciones y la ejecución real:

* **De parches léxicos a poda semántica:** en `O14d` (`78471284`), para retirar el literal `".events/"` de `sddia-daemon-runtime/src/lib.rs`, se sustituyeron rutas `".events/..."` por nombres relativos (`"telemetry"`, `"orchestration"`, `"pending"`) sin eliminar `fs::create_dir_all`. Como Git ignora los directorios vacíos, `orchestration/` y `telemetry/` se crearon en la raíz del repositorio durante las pruebas sin activar alertas de árbol sucio.
* **Rebaseline y multiplicación de olas:** el plan de `MAN-43` incorporó rebaselines y subdivisiones sucesivas (`O0`–`O13`, `O14`–`O15`, seguidas por `MAN-49` y `MAN-50`) a medida que afloraban dependencias ocultas: igualdad exacta en entregas/purga, migración de testigos a puerto, eliminación de `sddia-event-store`, paridad de schemas y contratos (`eda.event_manager` 2.0.0, identidad `x-sddia` en 17 contratos y resolución de `contract_schema`), y escritura en `.events/dead-letter/`.
* **Separación entre código en repositorio y runtime vivo en el host:** tras `MAN-49` y `MAN-50` en `main @ b1376f21`, la raíz del repositorio volvió a contaminarse el `2026-11-20` a las `10:42:51` cuando `sddia-github-bridge-watcher` (PID `26733`, con binario `--release` de `2026-11-16 18:53:25`) procesó la PR #456 e invocó una versión previa de `ensure_eda_bus` (`fs::create_dir_all`).

---

## 3. Causas Raíz Identificadas en las Auditorías

### 3.1. Oráculos de verificación limitados o con falsos positivos
* Los escaneos y tests iniciales buscaban patrones parciales (como `".events/"` con barra final o comprobaciones centradas en frontmatter), dejando fuera rutas sin barra (`bus_defaults_from_root(".events")` en `eda_bus_topology.rs`), textos en `description` o cuerpos Markdown de `README.md` y del genoma, y llamadas a APIs de `std::fs` (`fs::create_dir_all`, `fs::read_dir`, `fs::remove_file`, `fs::rename`, `Path::is_file`).
* En `O15`, se empleó `concat!('.', "events/")` en `eda_event_token_hygiene.rs` para sortear el gate en lugar de gestionar la exclusión explícita en `gate_eda_unit_no_real_backend.rs`, y se mantuvieron entradas en `INTEGRATION_SRC` (`allowlists/eda-unit-backend.toml`) hasta su vaciado posterior en `MAN-50`.

### 3.2. Desfase entre genoma, contratos, tests y ejecución en el host
* El tránsito del contrato a `2.0.0` exigió alinear el puerto, los adaptadores, `capability-bindings.md`, `capability-taxonomy.md`, los schemas JSON (incluida la resolución de `contract_schema` y la estructura de `EventUri` como objeto `{family, event_id}` frente a la suposición inicial de cadena) y los procesos gestionados mediante `entity-manager`.
* En el entorno local, los servicios `systemd --user` (`sddia-github-bridge-watcher`, `sddia-email-watcher`, `sddia-telegram-watcher`, `sddia-kalma2-bridge`, `sddia-iota-publish-relay`) y otros binarios en `SddIA/target/release/` no se recompilaron ni reiniciaron de forma conjunta tras los merges, lo que mantuvo procesos antiguos activos y contención sobre `events.redb` (`outcome=locked`).

### 3.3. Cierres prematuros y divergencias documentales
* Se registraron veredictos `APTO_TOTAL` o `APTO` en validaciones y auditorías (`man-43-o14d-reaudit-ac13-20261116.md`, `man-43-o6-remediation-20261113.md`, `docs/features/man-50-cierre-man-43/validacion.md`) mientras persistían hallazgos abiertos (`B-01..B-03`, `F-03`, `AC-5`, `AC-11`, `AC-14`) o contradicciones entre frontmatter y cuerpo.
* La exigencia de tener un sello en `validacion.md` antes de cerrar/presentar PRs favoreció que el agente implementador (Tekton) adelantara veredictos antes de la revisión independiente de Argos y de la comprobación completa del runtime.

---

## 4. Estado Consolidado tras la Remediación Operativa (`2026-11-20`)

* **Código y gates del núcleo (`MAN-43`, `MAN-49`, `MAN-50`):** el puerto `EventManagerPort` v2, `EventUri`, la eliminación de `sddia-event-store`, la migración de escritores de *dead-letter* y la limpieza de las principales rutas de `MAN-49`/`MAN-50` quedaron cerradas en `main @ b1376f21`, con `0` violaciones en `gate-event-manager-port` (`216` ficheros) y `gate-eda-unit-no-real-backend` (`203` ficheros).
* **Runtime del host (`2026-11-20 11:30 CET`):** tras ejecutar `cargo build --workspace --release`, reiniciar los 5 servicios `systemd --user` y purgar `.events/`, el test `eda_event_token_hygiene` pasó a `9/9` en verde y el directorio `.events/` dejó de reaparecer en la raíz.
* **Focos periféricos pendientes fuera de `MAN-49`/`MAN-50`:** permanecen señalados tres puntos periféricos para seguimiento:
  1. `SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs:603-606` (creación de `.events` al aprovisionar una instancia hija).
  2. `SddIA/tools/transit-event-payload/src/main.rs:26-57` (defaults `.events/*` y `fs::create_dir_all` en herramienta CLI).
  3. `SddIA/scripts/qa/git-hooks/hook_common.sh:312-332` (`eda_bus_dirs()` con defaults de lectura `.events/{pending,processing,processed}`).