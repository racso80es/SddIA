---
feature_name: e1-iota-ci
created: "2026-05-24"
process: feature
items_applied:
  - run-iota-ci-smoke.py
  - route_domain_event_core.py
  - sddia-index-qa.yml
---

# Ejecución — E.1 IOTA CI

Registro empírico (2026-05-24).

## Comandos locales

```powershell
cd C:\Proyectos\SddIA
python SddIA/scripts/qa/verify-process-integrity.py
python SddIA/scripts/qa/run-iota-ci-smoke.py --simulate --json
```

### Salida simulate (2026-05-24)

```json
{
  "success": true,
  "mode": "simulate",
  "delivery_status": {"cumulo.iota-immutable-publisher": "success"},
  "transaction_digest": "lab-sim-…"
}
```

### Modo físico (operador)

Requiere `IOTA_WALLET_SECRET` o `.SddIA/.dev/wallet.key`:

```powershell
Remove-Item Env:SDDIA_LAB_SIMULATE_IOTA -ErrorAction SilentlyContinue
$env:IOTA_WALLET_SECRET = "<secret>"
python SddIA/scripts/qa/run-iota-ci-smoke.py --require-physical --json
```

Digest esperado: **sin** prefijo `lab-sim-`.

## CI GitHub Actions

| Job | Trigger | Resultado |
|-----|---------|-----------|
| `eda-iota-smoke-simulate` | Cada push/PR | `--simulate` |
| `eda-iota-physical` | Mismo repo + secret | `--require-physical` |
| `eda-iota-physical` | Secret ausente | Skip documentado (exit 0) |

## Regresión

| Check | Estado |
|-------|--------|
| `verify-process-integrity.py` | ✅ OK |
| `verify-tools-index.py` | ✅ (job existente) |
