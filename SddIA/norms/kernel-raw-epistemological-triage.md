---
name: kernel-raw-epistemological-triage
version: "1.0"
nature: motor
description: Directriz TEKTON de triaje legacy (SddIA_1..4) hacia Core V5, Starter Kit y Limbo; salida JSON para parser.
---

# KERNEL RAW — Triaje epistemológico y planificación

## Parches normativos (determinismo)

- **`nature` ausente (Pasos 2 y 3):** Si `spec.json` existe pero **no** define la clave `nature` o el valor es vacío/nulo, aplicar la **misma rama** que `nature: product` (primera aparición top-down → Starter Kit en la ruta que corresponda: `principles/`, `patterns/` o `security/` bajo `.SddIA/`).
- **`timeline_sealed` (Paso 6):** Entero no negativo = **número total de ficheros regulares** (conteo recursivo) bajo `SddIA/scripts/limbo/archetypes/evolution/` **después** de completar el Paso 4. Cada fichero cuenta una vez; no incluir solo directorios.

---

## Directriz de ejecución

Asume la jurisdicción **TEKTON**.

**MISIÓN:** Secuencia de orquestación manual: triaje 100% determinista, deduplicación top-down, protección contra sobrescritura física y salida JSON estricta.

### PASO 0: Inventario de stems raíz y protección V5

- Escanea **exclusivamente** los ficheros (no directorios) en la raíz de `SddIA/norms/` y `SddIA/security/`. Extrae sus *stems* (nombres sin extensión).
- **Bloqueo mecánico:** Si el *stem* de cualquier archivo legacy coincide con uno protegido, envíalo **directamente** a `SddIA/scripts/limbo/archetypes/` (subcarpeta `norms/` o `security/` según el origen).

### PASO 1: Triaje de normas (top-down: SddIA_4 → 3 → 2 → 1)

- Itera top-down moviendo la **primera aparición** de cada ítem:
  - **A `SddIA/norms/`:** `obediencia-procesos` (`.md` y `.json`), `capsule-json-io`, `commands-via-skills-or-tools`, `paths-via-cumulo`, `entidades-dominio-ecosistema-sddia`, `git-via-skills-or-process`, `agents-principles-contract`, `touchpoints-ia`, `sddia-evolution-sync`.
  - **A `SddIA/library/norms/`:** `features-documentation-pattern`, `patterns-in-planning-implementation-execution`, `pr-acceptance-protocol`, `openapi-contract-rest-frontend`, `nextjs-hydration-client-state` (tactical-norm vía `norm-creator`; SSOT Librería).
  - **A `SddIA/scripts/limbo/archetypes/norms/`:** solo las bloqueadas del Paso 0 y duplicados no resueltos; **no** replicar normas ya en `SddIA/norms/` ni `SddIA/library/norms/`.

### PASO 2: Triaje de sabiduría (top-down)

- Itera carpetas en `principles/` y `patterns/` desde SddIA_4 a 1.
- **Regla de colisión V5:** Si la carpeta (UUID o nombre) **ya existe** en `SddIA/principles/` o `SddIA/patterns/`, mueve la copia legacy al Limbo de inmediato.
- Si no hay colisión, evalúa `spec.json`:
  - Si existe y `nature: motor` explícito → primera aparición a `SddIA/principles/` o `SddIA/patterns/`.
  - Si `nature: product`, **`nature` ausente** (parche arriba), o **`spec.json` no existe** → primera aparición a `SddIA/scripts/starter-kit/.SddIA/principles/` o `.../patterns/` según corresponda.
  - **Limbo:** apariciones subsiguientes, contratos maestros legacy (`principles-contract`, `patterns-contract`) y `README.md` legacy.

### PASO 3: Triaje de seguridad (top-down)

- Itera carpetas UUID en `security/` desde SddIA_4 a 1.
- **Misma regla de colisión V5** respecto a `SddIA/security/`.
- Evalúa `spec.json`:
  - Si `nature: motor` o `"context": "sddia-core"` → primera aparición a `SddIA/security/<uuid>/`.
  - Si `nature: product`, **`nature` ausente**, o **`spec.json` no existe** → primera aparición a `SddIA/scripts/starter-kit/.SddIA/security/<uuid>/`.
  - **Limbo:** duplicados y `security-contract` legacy (y bloqueos del Paso 0).

### PASO 4: Trazabilidad evolutiva

- Con el comando nativo del SO (`mv` en Unix, `Move-Item` en PowerShell), mueve el directorio completo `SddIA_X/evolution/` → `SddIA/scripts/limbo/archetypes/evolution/SddIA_X/` para X = 1, 2, 3, 4.

### PASO 5: Poda recursiva

- Elimina directorios vacíos de forma **recursiva** bajo las raíces `SddIA_1` … `SddIA_4` (comando nativo del SO).

### PASO 6: Reporte para parser

- Calcula `timeline_sealed` según la definición de arriba.
- Imprime por **STDOUT** únicamente JSON válido (sin fences markdown, sin texto adicional). Claves obligatorias: `sequence`, `core_norms_injected`, `skipped_protected`, `starter_kit_populated` (objeto con `principles_folders_moved`, `patterns_folders_moved`, `norms_files_moved`), `timeline_sealed` (definición arriba), `integrity_check`.

---

## Planificación operativa

| Fase | Acción | Verificación |
|------|--------|--------------|
| **Pre** | Rama o working tree conocido; opcional commit previo. | `git status` sin sorpresas críticas. |
| **0** | Listar stems raíz V5; no mover aún. | Lista escrita / checklist. |
| **1** | Normas top-down; bloqueo antes de Core/Kit. | `SddIA_X/norms/` vacío de `.md`/`.json` o solo lo acordado en Limbo. |
| **2** | Principles/patterns; colisión → Limbo primero. | Contar carpetas nuevas en Core vs Kit vs Limbo. |
| **3** | Security UUIDs; misma colisión. | Sin sobrescritura en `SddIA/security/`. |
| **4** | Mover `evolution/` completo por X. | Existe `limbo/.../evolution/SddIA_X/` con contenido. |
| **5** | Poda vacía bajo `SddIA_1..4`. | Sin árboles vacíos colgando. |
| **6** | Conteo recursivo + JSON a stdout. | Parser acepta el JSON. |

**Contadores `starter_kit_populated`:** Incrementar `norms_files_moved` por cada fichero colocado en Paso 1 bajo `.SddIA/norms/`; `principles_folders_moved` / `patterns_folders_moved` por cada **carpeta** de entidad movida en Paso 2 al Starter Kit (primera aparición válida).

**Orden:** No reordenar fases; el Paso 0 condiciona el 1; 4 antes de 5; 6 al final.

**Riesgos:** Rutas Windows vs Unix en `Move-Item`; destino `evolution/SddIA_X/` ya existente → resolver con merge o renombre determinista antes de mover.
