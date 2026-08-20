#!/usr/bin/env bash
# build-release-bundle — empaquetado hermético perfil consumidor (F-06 / L-BUNDLE)
# Uso:
#   ./SddIA/scripts/build-release-bundle.sh [--out DIR] [--codex SLUG] [--profile consumer|engineering] [--skip-build]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
# shellcheck source=common/sddia_shell_lib.sh
source "$SCRIPT_DIR/common/sddia_shell_lib.sh"

OUT=""
CODEX=""
PROFILE="consumer"
SKIP_BUILD=0
PROFILE_BIN="release"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out) OUT="${2:-}"; shift 2 ;;
    --codex) CODEX="${2:-}"; shift 2 ;;
    --profile) PROFILE="${2:-}"; shift 2 ;;
    --skip-build) SKIP_BUILD=1; shift ;;
    --debug) PROFILE_BIN="debug"; shift ;;
    -h|--help)
      sed -n '2,5p' "$0"
      exit 0
      ;;
    *)
      echo "[ERROR] argumento desconocido: $1" >&2
      exit 1
      ;;
  esac
done

PROFILE="$(echo "$PROFILE" | tr '[:upper:]' '[:lower:]')"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
if [[ -z "$OUT" ]]; then
  OUT="$REPO_ROOT/dist/sddia-release-${PROFILE}-${STAMP}"
fi

# Binarios runtime consumidor (mínimo operativo + smoke).
CONSUMER_BINS=(
  execute-process
  kalma2-bridge
  event-watcher
  event-sweeper
  email-watcher
  telegram-watcher
  send-telegram-notification
)

# Capsules tool extraídas del códice / grafo eferente (F-06).
declare -A CAPSULE_SET=()
for b in "${CONSUMER_BINS[@]}"; do
  CAPSULE_SET["$b"]=1
done
CAPSULE_SET["send-telegram-notification"]=1

_add_capsule() {
  local name="$1"
  [[ -z "$name" ]] && return 0
  CAPSULE_SET["$name"]=1
}

_scan_md_for_tools() {
  local file="$1"
  [[ -f "$file" ]] || return 0
  # tool: foo | `send-telegram-notification` | tool/foo
  local hits
  hits="$(rg -oN 'tool:\s*[`"]?([a-z0-9][a-z0-9-]*)' -r '$1' "$file" 2>/dev/null || true)"
  hits+=$'\n'"$(rg -oN '`([a-z0-9]+-[a-z0-9-]+)`' -r '$1' "$file" 2>/dev/null || true)"
  while IFS= read -r hit; do
    [[ -z "$hit" ]] && continue
    if [[ -f "$REPO_ROOT/SddIA/tools/${hit}.md" ]] || [[ -d "$REPO_ROOT/SddIA/tools/${hit}" ]]; then
      _add_capsule "$hit"
    fi
  done <<< "$hits"
}

if [[ -n "$CODEX" ]]; then
  CODEX_DIR="$REPO_ROOT/SddIA/library/codexes/${CODEX}"
  if [[ ! -d "$CODEX_DIR" ]]; then
    echo "[ERROR] códice no encontrado: $CODEX_DIR" >&2
    exit 1
  fi
  while IFS= read -r -d '' md; do
    _scan_md_for_tools "$md"
  done < <(find "$CODEX_DIR" -type f -name '*.md' -print0 2>/dev/null)
fi

# Suscripciones eferentes típicas consumidor (Email_Triaged / Fracture telegram).
_scan_md_for_tools "$REPO_ROOT/SddIA/core/event-domain-subscriptions.json"

echo "[bundle] out=${OUT}"
echo "[bundle] profile=${PROFILE} cargo_profile=${PROFILE_BIN}"
echo "[bundle] capsules: ${!CAPSULE_SET[*]}"

mkdir -p "$OUT"

if [[ "$SKIP_BUILD" -ne 1 ]]; then
  echo "[bundle] compilando cápsulas nativas…"
  local_pkgs=()
  for name in "${!CAPSULE_SET[@]}"; do
    if [[ -f "$REPO_ROOT/SddIA/tools/${name}/Cargo.toml" ]] \
      || [[ -f "$REPO_ROOT/SddIA/daemons/${name}/Cargo.toml" ]] \
      || [[ -f "$REPO_ROOT/SddIA/engine/${name}/Cargo.toml" ]] \
      || [[ -f "$REPO_ROOT/SddIA/interfaces/${name}/Cargo.toml" ]] \
      || [[ "$name" == "execute-process" ]] \
      || [[ "$name" == "kalma2-bridge" ]]; then
      local_pkgs+=("-p" "$name")
    fi
  done
  # Paquetes con locus no trivial
  local_pkgs+=(-p execute-process -p kalma2-bridge -p event-watcher -p event-sweeper -p email-watcher -p telegram-watcher -p send-telegram-notification)
  (
    cd "$REPO_ROOT/SddIA"
    if [[ "$PROFILE_BIN" == "release" ]]; then
      CARGO_TARGET_DIR=target cargo build --release "${local_pkgs[@]}" -q
    else
      CARGO_TARGET_DIR=target cargo build "${local_pkgs[@]}" -q
    fi
  )
fi

TARGET_SRC="$REPO_ROOT/SddIA/target/${PROFILE_BIN}"
STAGE="$OUT"
mkdir -p "$STAGE/SddIA/target/${PROFILE_BIN}"
mkdir -p "$STAGE/SddIA/target/debug" "$STAGE/SddIA/target/release"

_copy_bin() {
  local name="$1"
  local src="$TARGET_SRC/$name"
  if [[ ! -x "$src" ]]; then
    # fallback debug/release
    for alt in "$REPO_ROOT/SddIA/target/release/$name" "$REPO_ROOT/SddIA/target/debug/$name"; do
      if [[ -x "$alt" ]]; then
        src="$alt"
        break
      fi
    done
  fi
  if [[ ! -x "$src" ]]; then
    echo "[WARN] binario ausente: $name" >&2
    return 1
  fi
  if ! _sddia_is_native_elf "$src"; then
    echo "[ERROR] no-ELF: $src" >&2
    return 1
  fi
  cp -f "$src" "$STAGE/SddIA/target/${PROFILE_BIN}/$name"
  # Dual link para resolvers que prueban debug|release
  cp -f "$src" "$STAGE/SddIA/target/release/$name"
  cp -f "$src" "$STAGE/SddIA/target/debug/$name"
  echo "  -> bin $name"
}

echo "[bundle] copiando binarios…"
MISSING=0
for name in "${!CAPSULE_SET[@]}"; do
  _copy_bin "$name" || MISSING=$((MISSING + 1))
done
# Siempre exigir núcleo
for must in execute-process kalma2-bridge event-watcher event-sweeper send-telegram-notification; do
  if [[ ! -x "$STAGE/SddIA/target/release/$must" ]]; then
    echo "[ERROR] binario obligatorio ausente en bundle: $must" >&2
    exit 1
  fi
done

_rsync_genome() {
  local src="$1"
  local dst="$2"
  mkdir -p "$(dirname "$dst")"
  if [[ -d "$src" ]]; then
    mkdir -p "$dst"
    rsync -a \
      --exclude 'src/' \
      --exclude 'target/' \
      --exclude '*.rs' \
      --exclude 'Cargo.toml' \
      --exclude 'Cargo.lock' \
      --exclude '.git/' \
      --exclude 'tests/' \
      --exclude 'benches/' \
      "$src/" "$dst/"
  elif [[ -f "$src" ]]; then
    mkdir -p "$(dirname "$dst")"
    cp -f "$src" "$dst"
  fi
}

echo "[bundle] genoma sin fuentes de ingeniería…"
# Core operativo
for path in \
  SddIA/core \
  SddIA/CONSTITUTION_CORE.md \
  SddIA/norms \
  SddIA/process \
  SddIA/actions \
  SddIA/agents \
  SddIA/events \
  SddIA/skills \
  SddIA/templates \
  SddIA/scripts \
  SddIA/daemons \
  SddIA/tools \
  SddIA/library \
  SddIA/sddia-daemon-runtime \
  interfaces/kalma2 \
  start-sddia.sh \
  start-sddia.md \
  sddia-run.sh
do
  if [[ -e "$REPO_ROOT/$path" ]]; then
    _rsync_genome "$REPO_ROOT/$path" "$STAGE/$path"
  fi
done

# Purga residual de fuentes si rsync dejó algo
find "$STAGE/SddIA" \( -name '*.rs' -o -name 'Cargo.toml' -o -name 'Cargo.lock' \) -type f -delete 2>/dev/null || true
while IFS= read -r -d '' d; do
  rm -rf "$d"
done < <(find "$STAGE/SddIA" -type d -name src -print0 2>/dev/null)

# Perfil consumidor: no incluir github-bridge binario/lanzador en paquete hermético
if [[ "$PROFILE" == "consumer" || "$PROFILE" == "consumidor" ]]; then
  rm -f "$STAGE/SddIA/target/release/github-bridge-watcher" \
        "$STAGE/SddIA/target/debug/github-bridge-watcher" \
        "$STAGE/SddIA/scripts/daemons/github-bridge-watcher.sh" 2>/dev/null || true
fi

# Códice solicitado (ya copiado vía library; anclar slug en manifiesto)
CODEX_PRESENT=0
if [[ -n "$CODEX" && -d "$STAGE/SddIA/library/codexes/${CODEX}" ]]; then
  CODEX_PRESENT=1
fi

# Verificar F-06: cápsula eferente presente
if [[ ! -f "$STAGE/SddIA/tools/send-telegram-notification.md" ]] \
  || [[ ! -x "$STAGE/SddIA/target/release/send-telegram-notification" ]]; then
  echo "[ERROR] F-06: send-telegram-notification no verificable en bundle" >&2
  exit 1
fi

BINS_JSON=$(python3 - <<PY
import json, os
root = r"""$STAGE/SddIA/target/release"""
names = sorted(f for f in os.listdir(root) if os.path.isfile(os.path.join(root, f)) and os.access(os.path.join(root, f), os.X_OK))
print(json.dumps(names))
PY
)

CAPS_LIST="$(printf '%s\n' "${!CAPSULE_SET[@]}" | sort | python3 -c 'import sys,json; print(json.dumps([l.strip() for l in sys.stdin if l.strip()]))')"

cat > "$STAGE/MANIFEST.json" <<EOF
{
  "schema_version": "1.0.0",
  "created_at": "${STAMP}",
  "profile": "${PROFILE}",
  "codex": $( [[ -n "$CODEX" ]] && printf '"%s"' "$CODEX" || echo null ),
  "codex_present": $( [[ "$CODEX_PRESENT" -eq 1 ]] && echo true || echo false ),
  "cargo_profile": "${PROFILE_BIN}",
  "binaries": ${BINS_JSON},
  "capsules_resolved": ${CAPS_LIST},
  "excludes": ["*.rs", "Cargo.toml", "src/", "docs/features", "target build tree sources"],
  "filtro_c": $( [[ "$PROFILE" == "consumer" || "$PROFILE" == "consumidor" ]] && echo true || echo false ),
  "generator": "SddIA/scripts/build-release-bundle.sh"
}
EOF

# ONBOARDING.md — paridad con artefacto (no wizard UX)
cat > "$STAGE/ONBOARDING.md" <<EOF
# ONBOARDING — SddIA Release Bundle (${PROFILE})

Generado automáticamente por \`build-release-bundle\` (${STAMP}).
**Paridad:** este documento describe exactamente el contenido de este directorio (\`MANIFEST.json\`).

## 1. Contenido del paquete

| Ítem | Ruta |
|------|------|
| Orquestador | \`SddIA/target/release/execute-process\` |
| WUI + bridge | \`interfaces/kalma2/\` + \`SddIA/target/release/kalma2-bridge\` |
| Ignición | \`start-sddia.sh\` / \`sddia-run.sh\` |
| Manifiesto | \`MANIFEST.json\` |
| Códice | $( [[ -n "$CODEX" ]] && echo "\`SddIA/library/codexes/${CODEX}/\`" || echo "_(no anclado; usar sync-client-assets)_" ) |

Binarios incluidos: $(python3 -c "import json; print(', '.join(json.load(open('$STAGE/MANIFEST.json'))['binaries']))")

## 2. Variables de bóveda (instancia)

Crear \`{instancia}/.SddIA/.dev/.env\` (prevalece sobre \`.dev/.env\` raíz):

| Clave | Uso |
|-------|-----|
| \`SDDIA_RUNTIME_PROFILE\` | \`consumer\` (Filtro C) o \`engineering\` |
| \`SDDIA_SENSORIAL_JURISDICTION\` | \`systemd\` si email/telegram viven en unidades \`@%f\` (R-07) |
| \`SDDIA_CLIENT_PORT\` | Puerto Kalma2 (ej. 8766) |
| \`SDDIA_EMAIL_*\` | IMAP (opcional) |
| \`TELEGRAM_BOT_TOKEN\` / \`TELEGRAM_ALLOWED_CHAT_ID\` | Eferente |
| \`SDDIA_LLM_*\` | Chat WUI |

**Prohibido** versionar secretos en git.

## 3. Systemd hermético (recomendado)

Plantillas: \`SddIA/templates/systemd/\` (\`WorkingDirectory=%f\`, \`EnvironmentFile=%f/.SddIA/.dev/.env\`).

Si la jurisdicción sensorial es systemd, \`start-sddia.sh\` **no** spawnea \`email-watcher\`/\`telegram-watcher\`.

## 4. Ignición

\`\`\`bash
export SDDIA_RUNTIME_PROFILE=consumer
# opcional: export SDDIA_SENSORIAL_JURISDICTION=systemd
./start-sddia.sh
\`\`\`

O vía proceso (cuando exista en el genoma del bundle):

\`\`\`bash
./sddia-run.sh --process instance-creator --inputs '{...}'
\`\`\`

## 5. Verificación rápida (F-06)

\`\`\`bash
test -x SddIA/target/release/send-telegram-notification
test -f SddIA/tools/send-telegram-notification.md
./sddia-run.sh --process workspace-smoke --inputs '{}'   # o eda-local-topology-test / Local_QA_Requested
\`\`\`

## 6. Inmutabilidad Vía C

La carpeta \`SddIA/\` inyectada es **regenerable**. No parchear in-place; reinyectar bundle o upstream.
Ver norma \`SddIA/norms/sddia-distribution-protocol.md\`.

## 7. Perfil consumidor (Filtro C)

- Sin \`github-bridge-watcher\` en este paquete (\`profile=consumer\`).
- WUI: Forjar Proceso deshabilitado si \`SDDIA_RUNTIME_PROFILE=consumer\`.
- Fracture: acciones de forja documental se omiten en runtime consumer.
EOF

# Gate integridad: cero fuentes
RS_LEFT="$(find "$STAGE" -name '*.rs' -type f 2>/dev/null | wc -l | tr -d ' ')"
if [[ "$RS_LEFT" != "0" ]]; then
  echo "[ERROR] bundle contiene $RS_LEFT archivos .rs" >&2
  exit 1
fi

echo "[bundle] OK → $OUT"
echo "[bundle] ONBOARDING.md + MANIFEST.json escritos"
python3 -c "import json; m=json.load(open('$STAGE/MANIFEST.json')); print('[bundle] bins=', len(m['binaries']), 'capsules=', len(m['capsules_resolved']))"
