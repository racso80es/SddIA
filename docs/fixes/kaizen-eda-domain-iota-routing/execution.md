---
feature_name: kaizen-eda-domain-iota-routing
created: "2026-06-12"
process: bug-fix
---

# Ejecución — kaizen-eda-domain-iota-routing

## Comandos de verificación

```bash
./SddIA/scripts/tools/iota-immutable-publisher/install-deps.sh
python3 SddIA/scripts/daemons/event-watcher.py --once
```

## Smoke bus-operator

```bash
echo '{"operation":"resolve_subscribers","operation_payload":{"event_type":"Manual_Task_Requested"}}' \
  | ./scripts/skills/bus-operator.sh
```

## Proceso SddIA

```bash
python3 SddIA/scripts/qa/execute-process.py --process bug-fix --inputs '...'
python3 SddIA/scripts/qa/execute-process.py --process delivery-close-cycle --inputs '...'
```
