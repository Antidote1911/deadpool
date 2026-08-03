#!/usr/bin/env bash
# =============================================================================
#  build.sh — Vérification des versions + compilation du workspace
# =============================================================================

set -euo pipefail
IFS=$'\n\t'

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
BLUE='\033[0;34m'; CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}ℹ ${RESET}$*"; }
success() { echo -e "${GREEN}✔ ${RESET}$*"; }
warn()    { echo -e "${YELLOW}⚠ ${RESET}$*"; }
error()   { echo -e "${RED}✖ ${RESET}$*" >&2; }
title()   { echo -e "\n${BOLD}${BLUE}━━━ $* ━━━${RESET}\n"; }
sep()     { echo -e "${BLUE}────────────────────────────────────────${RESET}"; }

# ── Localisation du script ────────────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# ── Lecture de la version workspace ──────────────────────────────────────────
title "Lecture de la version workspace"

ROOT_CARGO="Cargo.toml"
if [[ ! -f "$ROOT_CARGO" ]]; then
  error "Cargo.toml introuvable à la racine ($SCRIPT_DIR). Abandon."
  exit 1
fi

# Extrait la version dans la section [workspace.package]
WORKSPACE_VERSION=$(awk '
  /^\[workspace\.package\]/ { in_section=1; next }
  /^\[/                      { in_section=0 }
  in_section && /^version\s*=/ {
    match($0, /"([^"]+)"/, arr); print arr[1]; exit
  }
' "$ROOT_CARGO")

if [[ -z "$WORKSPACE_VERSION" ]]; then
  error "Impossible de lire [workspace.package] version dans $ROOT_CARGO."
  exit 1
fi

success "Version workspace : ${BOLD}${WORKSPACE_VERSION}${RESET}"

# ── Synchronisation des membres Cargo ────────────────────────────────────────
title "Membres du workspace"

MEMBER_CRATES=("deadpool-core/Cargo.toml" "deadpool-ffi/Cargo.toml" "deadpool-gui/Cargo.toml")

for crate in "${MEMBER_CRATES[@]}"; do
  if [[ ! -f "$crate" ]]; then
    warn "$crate introuvable — ignoré."
    continue
  fi

  if grep -qE '^version\.workspace\s*=\s*true' "$crate"; then
    success "$crate  →  version.workspace = true  ✓"
  elif grep -qE '^version\s*=' "$crate"; then
    local_ver=$(grep -m1 '^version\s*=' "$crate" | sed 's/.*"\(.*\)".*/\1/')
    if [[ "$local_ver" == "$WORKSPACE_VERSION" ]]; then
      success "$crate  →  version=\"${local_ver}\"  ✓"
    else
      sed -i "0,/^version\s*=\s*\"${local_ver}\"/s/^version\s*=\s*\"${local_ver}\"/version = \"${WORKSPACE_VERSION}\"/" "$crate"
      warn "$crate : ${local_ver} → ${WORKSPACE_VERSION}  (mis à jour)"
    fi
  else
    warn "$crate : aucun champ version trouvé — ignoré."
  fi
done

# ── Synchronisation des fichiers de packaging ─────────────────────────────────
title "Fichiers de packaging"

declare -A PACKAGING_FILES=(
  ["packaging/archlinux/PKGBUILD"]="pkgver"
  ["packaging/archlinux/PKGBUILD-AppImage"]="pkgver"
)

for file in "${!PACKAGING_FILES[@]}"; do
  field="${PACKAGING_FILES[$file]}"

  if [[ ! -f "$file" ]]; then
    warn "$file introuvable — ignoré."
    continue
  fi

  file_ver=$(grep -m1 "^${field}=" "$file" | cut -d= -f2 | tr -d '[:space:]')

  if [[ -z "$file_ver" ]]; then
    warn "$file : champ '${field}' introuvable — ignoré."
  elif [[ "$file_ver" == "$WORKSPACE_VERSION" ]]; then
    success "$file  →  ${field}=${file_ver}  ✓"
  else
    sed -i "s/^${field}=${file_ver}$/${field}=${WORKSPACE_VERSION}/" "$file"
    warn "$file : ${field} ${file_ver} → ${WORKSPACE_VERSION}  (mis à jour)"
  fi
done

# ── Bilan ─────────────────────────────────────────────────────────────────────
echo ""
sep
success "Toutes les versions sont synchronisées sur ${BOLD}${WORKSPACE_VERSION}${RESET}."

CORES=$(nproc 2>/dev/null || echo 4)

# ── Nettoyage ─────────────────────────────────────────────────────────────────
title "Nettoyage"

info "Suppression de target/ ..."
rm -rf target/
success "target/ supprimé."

info "Suppression de deadpool-qt/build/ ..."
rm -rf deadpool-qt/build/
success "deadpool-qt/build/ supprimé."

# ── Compilation Rust (workspace complet) ─────────────────────────────────────
title "Compilation Rust (release)"

info "Cœurs disponibles : ${CORES}"
echo ""

if cargo build --release; then
  echo ""
  success "Workspace Rust compilé."
  info "GUI  egui : ${BOLD}target/release/deadpool${RESET}"
  info "Lib  FFI  : ${BOLD}target/release/libdeadpool_ffi.a${RESET}"
else
  echo ""
  error "La compilation Rust a échoué."
  exit 1
fi

# ── Compilation Qt (deadpool-qt) ──────────────────────────────────────────────
title "Compilation Qt (deadpool-qt)"

QT_BUILD_DIR="deadpool-qt/build"

if ! command -v cmake &>/dev/null; then
  warn "cmake introuvable — compilation Qt ignorée."
else
  generator=""
  command -v ninja &>/dev/null && generator="-GNinja"

  info "Configuration CMake dans ${QT_BUILD_DIR}..."
  if cmake -B "$QT_BUILD_DIR" -S deadpool-qt \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
        ${generator}; then

    info "Compilation sur ${CORES} cœurs..."
    if cmake --build "$QT_BUILD_DIR" --parallel "$CORES"; then
      echo ""
      success "Qt GUI compilé."
      info "GUI  Qt   : ${BOLD}${QT_BUILD_DIR}/deadpool-qt${RESET}"
    else
      echo ""
      error "La compilation Qt a échoué."
      exit 1
    fi
  else
    warn "Qt6 (Widgets) introuvable — compilation Qt ignorée."
  fi
fi
