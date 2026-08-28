## Summary
- Rehab A1 instancia para `accept-pr` post re-revocación #203 (PPR #208).
- T0: motor A2 #203 reutilizado.
- Handoff merge PR #208 **fuera** de alcance.

## Test plan
- [x] `accept-pr` ∉ `revoked`/`permanent`
- [x] Stats `healthy` · `structure_valid: true`
- [x] Diff sin instancia Cerbero/Radamanto