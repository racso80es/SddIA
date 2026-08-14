---
feature_name: evolution-contract-index-v11
created: "2026-08-11"
process: feature
purpose: Notas de migración/compatibilidad EV-AUD-001 (sin normalización física)
---

# Migration notes — evolution-contract-index-v11

## Restaurado

| Artefacto | Clave Cúmulo | Versión |
|-----------|--------------|---------|
| `SddIA/evolution/evolution_contract.md` | `normative_documents.evolution_contract` | 1.1.0 |
| `SddIA/evolution/Evolution_log.md` | `normative_documents.evolution_log` | índice corte 61 |

## Universo del índice

Anclado a `docs/audits/evolution/2026-08-11.md` (`cut_commit: 9d9abd8…`). **61 filas**.

## Delta post-corte (no indexado en este PR)

Presentes en `directories.evolution` al momento de restauración, **fuera** del inventario del corte:

| Archivo |
|---------|
| `0c19403d-2749-4296-90fa-5551e907552a.md` |
| `83bbfdeb-4715-4915-88be-751532dc268a.md` |
| `a7c3e91f-2b4d-4e8a-9f01-6d5c8b3a1742.md` |

Acción diferida: incorporar al índice en ciclo de migración / siguiente auditoría (PBI `7bb37ff1-…` / audit periódico).

## Compatibilidad legacy

Tabla de alias y clases (`INV-A`, `INV-L`, `NOMBRE`, `UUID-INV`, `BORRADOR`, `SIN_FECHA`) en el contrato §3. **Cero** reescritura de frontmatter histórico en este PR.

## Validador

```bash
SddIA/target/debug/sddia-qa validate-evolution-contract --json \
  --universe audit-cut \
  --audit-ref docs/audits/evolution/2026-08-11.md
```

Exit 0 = clasificación completa del universo; no implica conformidad canónica de cada registro. Gate CI bloqueante = fuera de alcance (PBI `70f78d23-…`).

## Cierre (PBI `7bb37ff1-…` / EV-AUD-002-007)

La normalización física y extracción de borradores se ejecutó en `docs/features/evolution-history-normalization/`. Mapa reversible: `migration-manifest.json`. Universo oficial post-migración: 64 registros CANONICO + hito de ciclo si `evolution-register` añade alta. Corte `2026-08-11.md` permanece como arqueología; no reescribir.
