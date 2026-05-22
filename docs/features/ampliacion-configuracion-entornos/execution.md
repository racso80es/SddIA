---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
items_applied:
  - hito-0.1-env-loader
  - hito-0.2-entrypoints
  - hito-0.3-sanitizacion
---

# Ejecución — Jerarquía de Bóvedas

## Smoke Hito 0.1

```powershell
$env:PYTHONUTF8='1'
python -c "
from pathlib import Path
import os, sys, tempfile, shutil
from importlib import util
spec = util.spec_from_file_location('el', 'SddIA/scripts/qa/env_loader.py')
el = util.module_from_spec(spec); spec.loader.exec_module(el)
td = Path(tempfile.mkdtemp())
try:
    (td / '.SddIA' / '.dev').mkdir(parents=True)
    (td / '.dev' / '.env').write_text('A=global\n', encoding='utf-8')
    (td / '.SddIA' / '.dev' / '.env').write_text('A=local\nB=inst\n', encoding='utf-8')
    os.environ.pop('A', None); os.environ.pop('B', None)
    m = el.load_hierarchical_env(td)
    assert m['A']=='local' and m['B']=='inst'
    assert os.environ.get('A')=='local'
    print('OK 0.1')
finally:
    shutil.rmtree(td)
"
```

## Smoke Hito 0.2

```powershell
$env:PYTHONUTF8='1'
python SddIA/scripts/qa/execute-process.py --process feature --inputs "{\"feature_name\":\"smoke-vault\"}"
```

Esperado: envelope JSON `success` (fases simulated) sin error de env.

## Smoke Hito 0.3

```powershell
rg "dotenv\.config|path\.join\(__dirname,\s*['\"]\.env" SddIA/scripts/tools/
rg "^\.dev/|^\.SddIA/\.dev/" .gitignore
python -m json.tool SddIA/core/cumulo.paths.json > $null
```

Esperado: cero matches dotenv en tools; ambas rutas en gitignore; JSON válido.
