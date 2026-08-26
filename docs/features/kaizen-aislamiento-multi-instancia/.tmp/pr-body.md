## Summary
- F-SYS-02: ExecStart=%f (fábrica y email); instance-creator no hornea path de host.
- F-DEP-10: REPO_ROOT por instancia (env / cwd .SddIA / fallback lab).
- F-CEN-PKILL: cero pkill -x; parada por PID de lock.

## Test plan
- [x] unit instance_creator + test-instance-root-resolver.sh
- [x] user units %f; ExecStart @SddIA vs @SddIA_AP
- [x] dos raíces lab (iso-b): cwd distintos; inbox SHA distintos; restart forja no mata iso-b