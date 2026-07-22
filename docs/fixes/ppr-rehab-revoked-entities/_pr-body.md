## Summary

- Laudo Cerbero: rehabilita `pull-request-review` (revocado por falso positivo de latencia agent).
- Exención `latency_threshold` en Radamanto para PPR; instancia limpia (witness documental).
- Cierra seeds ARQUITECTURA PPR #124 y #125 en un solo fix.

## Test plan

- [x] `cargo test -p execute-process --lib pull_request_review_is_latency_exempt`
- [x] Assert local: PPR ∉ `revoked_entities.revoked`
