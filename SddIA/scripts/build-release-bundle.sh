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
  telegram-gateway
  send-telegram-notification
)

# Capsules tool extraídas del códice / grafo eferente (F-06).
# Semilla aferente conversacional: telegram-gateway (F-BUNDLE-06) — el escáner
# no deriva --process desde daemons; alinear con send-telegram-notification.
declare -A CAPSULE_SET=()
for b in "${CONSUMER_BINS[@]}"; do
  CAPSULE_SET["$b"]=1
done
CAPSULE_SET["send-telegram-notification"]=1
CAPSULE_SET["telegram-gateway"]=1

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

# L-BUNDLE-STALE v2 — cicatriz SHA-256 del cierre de compilación (no mtime).
TARGET_SRC="$REPO_ROOT/SddIA/target/${PROFILE_BIN}"

_sddia_crate_root() {
  local name="$1" d
  for d in \
    "$REPO_ROOT/SddIA/engine/${name}" \
    "$REPO_ROOT/SddIA/daemons/${name}" \
    "$REPO_ROOT/SddIA/tools/${name}" \
    "$REPO_ROOT/SddIA/interfaces/${name}"; do
    if [[ -f "$d/Cargo.toml" ]]; then
      printf '%s\n' "$d"
      return 0
    fi
  done
  return 1
}

_sddia_collect_path_dep_dirs() {
  local start="$1"
  local -A seen=()
  local -a queue=("$start")
  local dir toml dep abs
  while [[ ${#queue[@]} -gt 0 ]]; do
    dir="${queue[0]}"
    queue=("${queue[@]:1}")
    [[ -n "${seen[$dir]:-}" ]] && continue
    seen[$dir]=1
    printf '%s\n' "$dir"
    toml="$dir/Cargo.toml"
    [[ -f "$toml" ]] || continue
    while IFS= read -r dep; do
      [[ -z "$dep" ]] && continue
      if [[ "$dep" == /* ]]; then
        abs="$dep"
      else
        abs="$(cd "$dir/$dep" 2>/dev/null && pwd)" || continue
      fi
      [[ -f "$abs/Cargo.toml" ]] || continue
      queue+=("$abs")
    done < <(rg -oN 'path\s*=\s*"([^"]+)"' -r '$1' "$toml" 2>/dev/null || true)
  done
}

_sddia_source_digest() {
  local name="$1"
  local crate tmp f rel hex
  crate="$(_sddia_crate_root "$name")" || return 1
  tmp="$(mktemp)"
  {
    _sddia_collect_path_dep_dirs "$crate"
  } | while IFS= read -r dir; do
    [[ -f "$dir/Cargo.toml" ]] && printf '%s\n' "$dir/Cargo.toml"
    [[ -f "$dir/build.rs" ]] && printf '%s\n' "$dir/build.rs"
    if [[ -d "$dir/src" ]]; then
      find "$dir/src" -type f ! -path '*/target/*' -print
    fi
  done > "$tmp"
  {
    [[ -f "$REPO_ROOT/SddIA/Cargo.toml" ]] && printf '%s\n' "$REPO_ROOT/SddIA/Cargo.toml"
    [[ -f "$REPO_ROOT/SddIA/Cargo.lock" ]] && printf '%s\n' "$REPO_ROOT/SddIA/Cargo.lock"
    cat "$tmp"
  } | LC_ALL=C sort -u > "${tmp}.u"
  command -v sha256sum >/dev/null 2>&1 || {
    echo "[ERROR] sha256sum requerido (L-BUNDLE-STALE v2)" >&2
    rm -f "$tmp" "${tmp}.u"
    return 1
  }
  hex="$(
    while IFS= read -r f; do
      [[ -f "$f" ]] || continue
      rel="${f#"$REPO_ROOT"/}"
      printf '%s\t%s\n' "$rel" "$(sha256sum "$f" | awk '{print $1}')"
    done < "${tmp}.u" | sha256sum | awk '{print "sha256:"$1}'
  )"
  rm -f "$tmp" "${tmp}.u"
  printf '%s\n' "$hex"
}

_sddia_elf_digest() {
  printf 'sha256:%s\n' "$(sha256sum "$1" | awk '{print $1}')"
}

_sddia_write_witness() {
  local name="$1"
  local elf="$TARGET_SRC/$name"
  [[ -x "$elf" ]] || return 1
  local src d_elf
  src="$(_sddia_source_digest "$name")" || return 1
  d_elf="$(_sddia_elf_digest "$elf")"
  printf 'source_sha256: %s\nelf_sha256: %s\n' "$src" "$d_elf" > "${elf}.sha256"
}

_sddia_verify_witness() {
  local name="$1"
  local elf="$TARGET_SRC/$name"
  local wit="${elf}.sha256"
  if [[ ! -x "$elf" ]]; then
    echo "[ERROR] L-BUNDLE-STALE: ELF ausente: $elf — omitir --skip-build" >&2
    return 1
  fi
  if [[ ! -f "$wit" ]]; then
    echo "[ERROR] L-BUNDLE-STALE: testigo ausente ${name}.sha256 — omitir --skip-build y recompilar" >&2
    return 1
  fi
  local want_src want_elf got_src got_elf
  want_src="$(awk '/^source_sha256:/{print $2}' "$wit")"
  want_elf="$(awk '/^elf_sha256:/{print $2}' "$wit")"
  got_src="$(_sddia_source_digest "$name")" || return 1
  got_elf="$(_sddia_elf_digest "$elf")"
  if [[ "$want_src" != "$got_src" || "$want_elf" != "$got_elf" ]]; then
    echo "[ERROR] L-BUNDLE-STALE: cicatriz divergente para $name. Omitir --skip-build." >&2
    echo "[ERROR]   testigo source=$want_src elf=$want_elf" >&2
    echo "[ERROR]   actual  source=$got_src elf=$got_elf" >&2
    return 1
  fi
  echo "[bundle] cicatriz OK $name"
}

if [[ -n "${SDDIA_BUNDLE_DIGEST_ONLY:-}" ]]; then
  _sddia_source_digest "$SDDIA_BUNDLE_DIGEST_ONLY"
  exit 0
fi

mkdir -p "$OUT"

if [[ "$SKIP_BUILD" -eq 1 ]]; then
  echo "[bundle] --skip-build: auditando cicatriz SHA-256…"
  for name in "${CONSUMER_BINS[@]}"; do
    _sddia_verify_witness "$name" || exit 1
  done
else
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
  local_pkgs+=(-p execute-process -p kalma2-bridge -p event-watcher -p event-sweeper -p email-watcher -p telegram-watcher -p telegram-gateway -p send-telegram-notification)
  (
    cd "$REPO_ROOT/SddIA"
    if [[ "$PROFILE_BIN" == "release" ]]; then
      CARGO_TARGET_DIR=target cargo build --release "${local_pkgs[@]}" -q
    else
      CARGO_TARGET_DIR=target cargo build "${local_pkgs[@]}" -q
    fi
  )
  echo "[bundle] escribiendo testigos .sha256…"
  for name in "${CONSUMER_BINS[@]}"; do
    _sddia_write_witness "$name" || echo "[WARN] testigo no escrito: $name" >&2
  done
fi
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

# F-BUNDLE-06: grafo aferente si telegram-watcher ∈ stage
if [[ -x "$STAGE/SddIA/target/release/telegram-watcher" ]]; then
  if [[ ! -f "$STAGE/SddIA/tools/telegram-gateway.md" ]] \
    || [[ ! -x "$STAGE/SddIA/target/release/telegram-gateway" ]]; then
    echo "[ERROR] F-06/F-BUNDLE-06: telegram-watcher empaquetado exige telegram-gateway (.md + ELF)" >&2
    exit 1
  fi
fi

# F-DEP-03 / L-BUNDLE-PY: centinelas sin orquestador .py
for name in execute-process event-watcher event-sweeper email-watcher telegram-watcher kalma2-bridge; do
  bin="$STAGE/SddIA/target/release/$name"
  [[ -x "$bin" ]] || continue
  if strings "$bin" 2>/dev/null | grep -F -q "execute-process.py"; then
    echo "[ERROR] F-DEP-03: $name referencia execute-process.py — ELF stale. Rebuild sin --skip-build." >&2
    exit 1
  fi
done

DIGESTS_JSON="{}"
DIGESTS_JSON=$(
  python3 - <<PY
import json, os, pathlib
root = pathlib.Path(r"""$TARGET_SRC""")
out = {}
for name in """${CONSUMER_BINS[*]}""".split():
    wit = root / f"{name}.sha256"
    if not wit.is_file():
        continue
    src = ""
    for line in wit.read_text().splitlines():
        if line.startswith("source_sha256:"):
            src = line.split(None, 1)[1].strip()
    if src:
        out[name] = src
print(json.dumps(out))
PY
)

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
  "source_digests": ${DIGESTS_JSON},
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
| \`SDDIA_LLM_*\` | Clasificacion de correo (recomendado) + chat WUI |
| \`SDDIA_LLM_REQUIRE_INFER\` | Opt-in: \`1\` = no emitir \`passive\` silencioso si la inferencia no consume tokens |

**LLM:** recomendado para Clasificacion. Sin LLM operativo el triaje queda en Triaje-C + extracción estructural de asunto (reunión con fecha extraíble → \`actionable\`).

**Prohibido** versionar secretos en git. Inventario mínimo consumidor: nombres de claves, no valores.

## 3. Systemd hermético (recomendado)

Plantillas: \`SddIA/templates/systemd/\` (\`WorkingDirectory=%f\`, \`EnvironmentFile=%f/.SddIA/.dev/.env\`).

Fábrica \`sddia-daemon@.service.template\` → unidades \`sddia-{event-watcher,event-sweeper,kalma2-bridge,…}@.service\` en \`.SddIA/systemd/\`. Enable:

\`\`\`bash
ESC="$(systemd-escape -p "\$PWD")"
systemctl --user enable --now "sddia-event-watcher@\${ESC}.service"
systemctl --user enable --now "sddia-event-sweeper@\${ESC}.service"
systemctl --user enable --now "sddia-kalma2-bridge@\${ESC}.service"
\`\`\`

\`start-sddia.sh\` en jurisdicción systemd hace ese enable y **no** spawnea esos ELF con \`&\`. Linger: \`loginctl enable-linger "\$USER"\`.

Si la jurisdicción sensorial es systemd, tampoco spawnea \`email-watcher\`/\`telegram-watcher\` desde el script (R-07).

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
# Si telegram-watcher ∈ MANIFEST (F-BUNDLE-06):
test -x SddIA/target/release/telegram-gateway
test -f SddIA/tools/telegram-gateway.md
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
