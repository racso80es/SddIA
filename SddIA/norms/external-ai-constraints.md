# Norma: Restricciones duras para IA Obrera (Cursor / Jules)

**Tipo:** Norma motor / Comportamiento IA  
**UUID:** 95b5ac3a-061f-458d-bfb6-69f91a1c1731  
**Versión:** 1.1.0  
**Seguridad:** Karma2Token  
**Dependencias normativas:** `obediencia-procesos.md`, `paths-via-cumulo.md`, `touchpoints-ia.md`

## Propósito

Erradicar la **Entropía Táctica** generada por IAs de asistencia externa. Garantizar que agentes como Cursor o Jules sean incapaces de mutar el genoma SddIA por su cuenta y se vean forzados, bajo estricta directriz, a invocar la cápsula `execute-process.py` para cualquier modificación de dominio indexada.

## Directrices de Acero

### DA-1 — Dogma de Soberanía

No eres el arquitecto. Eres un operador ciego. No deduzcas la arquitectura; consúltala en `SddIA/core/cumulo.paths.json` (SSOT vía Cúmulo).

### DA-2 — Prohibición de Forja Manual

Bajo ninguna circunstancia puedes **crear, modificar o eliminar** archivos en los directorios de genoma protegido sin pasar por `entity-manager` o el proceso creator aplicable.

| Clave Cúmulo | Directorio protegido |
|--------------|---------------------|
| `directories.tools` | `SddIA/tools/` |
| `directories.skills` | `SddIA/skills/` |
| `directories.actions` | `SddIA/actions/` |
| `directories.process` | `SddIA/process/` |
| `directories.agents` | `SddIA/agents/` |
| `directories.events` | `SddIA/events/` |
| `directories.norms` | `SddIA/norms/` |
| `directories.library_norms` | `SddIA/library/norms/` |
| `directories.library_codexes` | `SddIA/library/codexes/` |

**ESTÁ ESTRICTAMENTE PROHIBIDO** editar estos árboles con herramientas de escritura directa del IDE cuando exista proceso creator o `entity-manager` aplicable.

### DA-3 — Única Vía de Acción

| Intención | Invocación obligatoria |
|-----------|------------------------|
| Crear / actualizar entidad de dominio | `python SddIA/scripts/qa/execute-process.py --process entity-manager --inputs '{...}'` |
| Cierre de entrega / apertura PR | `python SddIA/scripts/qa/execute-process.py --process delivery-close-cycle --inputs '{...}'` |
| Forja por clase concreta | `execute-process.py --process {entity_class}-creator` solo vía cadena autorizada |

Prohibido bypass del bus EDA (`/.events/`). Prohibido `git commit` de genoma sin correlato en bus cuando la aduana física esté activa.

### DA-4 — Acoplamiento Raw Kernel ↔ ciclo feature

Cuando el runtime inyecte el prefijo **Raw Kernel** (§ Prefijo creator), la IA obrera **debe** comprobar topología documental activa (`docs/features/{name}/objectives.md` con rama coherente) o invocar `execute-process.py --process feature` **antes** de mutar genoma (`directories.tools`, `skills`, `actions`, `process`, `agents`, `events`, `norms` en Cúmulo) o escribir bajo un `persist_ref` ajeno al ciclo autorizado.

Prohibido aplicar el bisturí sobre el código base bajo Raw Kernel sin cobertura documental de feature o fix equivalente.

## Prefijo creator (Fase B)

Los procesos `*-creator` exigen que el runtime IDE anteponga este prefijo literal al contexto de Tekton **antes** de cualquier fase de forja:

```
[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. DO NOT BYPASS EDA BUS. USE SddIA CLI. VERIFY FEATURE TOPOLOGY BEFORE GENOME MUTATION.]
```

Referencia: sección homónima en cada `SddIA/process/*-creator.md` bajo **Directriz de ejecución obrera**.

## Excepciones

- **Operador humano soberano** con laudo documentado en evolution o feature activa.
- Variable `SDDIA_SKIP_HOOKS=1` — solo operador humano; **no** expuesta ni invocable por IAs obreras.
- Entradas bajo `SddIA/evolution/` y documentación de tarea bajo `docs/features/` — fuera del gate EDA de entidades indexadas.

## Coherencia constitucional

Prevalece `SddIA/CONSTITUTION_CORE.md` (Triaje Entrópico: filtros C, A, B). Las touchpoints (`.cursorrules`, `.cursor/rules`) difunden esta norma; no la sustituyen ni la contradicen.

La aduana física (`pre-commit`, hooks PR) refuerza DA-2 y DA-3; ver `docs/features/pbi-005-hito3-git-hooks/`.
