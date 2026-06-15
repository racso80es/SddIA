---
feature_name: centinela-soberania-ejecucion
created: "2026-06-15"
process: feature
branch_name: feat/centinela-soberania-ejecucion
persist_ref: docs/features/centinela-soberania-ejecucion
---

# Ejecución — centinela-soberania-ejecucion

```bash
python3 SddIA/scripts/qa/execute-process.py --process governance-daemon-manager --inputs '{"operation":"start","daemon_id":"event-watcher","repository_path":"<abs>"}'

python3 SddIA/scripts/qa/execute-process.py --process daemon-kill-switch --inputs '{"repository_path":"<abs>"}'
```

| Sello EDA | event_id |
|-----------|----------|
| `Domain_Entity_Created` (daemon-creator) | `7009cd90-d36f-4c53-b4eb-e063c906be3d` |
| `Domain_Entity_Created` (governance-daemon-manager) | `86f44d56-a892-4035-a718-3c8aab7e5866` |
| `Domain_Entity_Created` (daemon-kill-switch) | `c420a839-ae26-4be8-aa5c-8edfabe9f8ee` |
| `Domain_Entity_Created` (daemon-heartbeat) | `d65b2fad-44f7-44c1-853b-4f47907d7587` |
