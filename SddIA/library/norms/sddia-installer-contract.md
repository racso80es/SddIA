---
uuid: "b1327ef3-5f07-4fba-9073-a72c5fdf97e2"
name: "sddia-installer-contract"
version: "1.0.0"
nature: "tactical-norm"
author: "tekton"
scope: "infrastructure"
category: "architecture"
dependencies: []
hash_signature: "sha256:9bcea7e4e939e866b2116f5966293e380845636bcf1f789d719e3c6e5769f4ce"
---

## Directriz Core

Contrato bilateral del orquestador físico SddIA (Ceguera de Ejecución). Motor único: `SddIA/scripts/sddia-installer.sh`; fachada `./sddia-installer.sh`. Comandos: `deploy` | `teardown`. Prohibido segundo binario `sddia-undeploy`.

### Precedencia ROOT
`--root` > `SDDIA_INSTALL_ROOT` (env/vault) > last-resort host `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`. Misma resolución en ambos sentidos. `ESC` = `systemd-escape -p ROOT`.

### Cuatro jurisdicciones
| Canal | Comando | Perfil bundle (deploy) | ROOT típico | HTTP en motor |
| Installer default | deploy/teardown | full-node / n/a | last-resort o --root | No |
| Paciente 0 | prompt / futuro paciente0-* | consumer | SddIA_AP | Sí (ola); no en CLI installer |
| Atajo Deploy | SddIA_Deploy.sh | delega deploy | igual installer | No |
| Atajo Teardown | SddIA_Teardown.sh | delega teardown --force | igual installer | No |

Atajos bajo `/home/racso/Aplicaciones/SddIA/`; no versionados en forja.

### Códigos de salida
| Código | Sentido |
| 1 | ROOT inválido (vacío, /, $HOME, forja o bajo forja) |
| 2 | deploy sobre instancia viva sin --force |
| 3 | teardown sin --force (consentimiento = flag; --dry-run no exige force) |

### Deploy — I-DEP-*
| ID | Hecho |
| I-DEP-CLI | deploy; flags --root --vault --force --skip-build --dry-run; cero prompts |
| I-DEP-PATH | abort si ROOT es forja o bajo forja |
| I-DEP-LIVE | vivo + deploy sin --force → exit 2 |
| I-DEP-BUNDLE | build-release-bundle --out ROOT --profile full-node |
| I-DEP-CREATOR | instance-creator skip_ignition:true runtime_profile:engineering |
| I-DEP-SYS | enable --now sddia-<daemon>@ESC si launcher existe |
| I-DEP-HOST | systemctl, systemd-escape; sin --skip-build: cargo |

### Teardown — I-TEAR-*
| ID | Hecho |
| I-TEAR-CLI | teardown; --root --force --dry-run; sin --vault/--skip-build |
| I-TEAR-PATH | abort ROOT vacío, /, $HOME, forja (exit 1) |
| I-TEAR-FORCE | sin --force → exit 3 |
| I-TEAR-SIGNAL | SIGTERM luego KILL solo PIDs start-sddia o cwd/exe bajo ROOT vía /proc |
| I-TEAR-SYSTEMD | stop disable reset-failed por @ESC; no rm plantillas sddia-*@.service |
| I-TEAR-LOCK | _sddia_stop_lock_pid sobre .SddIA/daemons/status/*.lock |
| I-TEAR-WIPE | rm -rf ROOT si existe |
| I-TEAR-VAULT | no borrar forja, .dev, *.deploy-vault, *.preprod-vault |
| I-TEAR-CEGUERA | sin curl/ss; verificación: ! -e ROOT; systemctl inactive/unknown @ESC; sin proc bajo ROOT |

### Simetría
`deploy --force` sobre vivo ejecuta teardown previo (`do_teardown`) y continúa deploy.

### Perfiles
Installer default: full-node + engineering. Paciente 0 consumer: SddIA_AP solo con --root explícito en installer; teardown default ≠ SddIA_AP.

### Fuera del contrato CLI
Gates G3 HTTP, G-heartbeat, G-dlt, olas, LLM, poll :8766 en motor (traducidos a I-TEAR-CEGUERA).

## Restricciones Duras (Aduana de Fricción)

- Prohibido segundo motor o binario `sddia-undeploy` para el ciclo installer.
- Prohibido interpretar gates G* de ola Paciente 0 como post-condición de `deploy` o `teardown` del installer.
- Prohibido `curl`, `ss` o poll HTTP en `SddIA/scripts/sddia-installer.sh`.
- Prohibido `pkill -f` global en teardown.
- Prohibido borrar plantillas unit `sddia-*@.service` en `~/.config/systemd/user/` durante teardown.
- Prohibido usar ROOT=forja o wipe de forja/vaults por defecto.
- Prohibido default teardown implícito en `SddIA_AP` sin `--root` explícito.
- Prohibido archivar prompts Paciente 0 como efecto de esta norma.
