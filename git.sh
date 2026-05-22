#!/usr/bin/env bash
# =============================================================================
#  git.sh — Assistant Git
#  Usage rapide    : ./git.sh "mon message de commit"
#  Menu interactif : ./git.sh
# =============================================================================

set -euo pipefail
IFS=$'\n\t'

# ── Couleurs ──────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
BLUE='\033[0;34m'; CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}ℹ ${RESET}$*"; }
success() { echo -e "${GREEN}✔ ${RESET}$*"; }
warn()    { echo -e "${YELLOW}⚠ ${RESET}$*"; }
error()   { echo -e "${RED}✖ ${RESET}$*" >&2; }
title()   { echo -e "\n${BOLD}${BLUE}━━━ $* ━━━${RESET}\n"; }
sep()     { echo -e "${BLUE}────────────────────────────────────────${RESET}"; }

confirm() {
  read -rp "$(echo -e "${YELLOW}? ${RESET}${1:-Confirmer ?} [o/N] ")" ans
  [[ "${ans,,}" =~ ^(o|oui|y|yes)$ ]]
}

require_git() {
  if ! git rev-parse --git-dir &>/dev/null; then
    error "Ce dossier n'est pas un dépôt Git."
    exit 1
  fi
}

# ── Gestion des dossiers à ignorer ───────────────────────────────────────────
# Liste des patterns à toujours ignorer (personnalisable)
IGNORE_PATTERNS=("target/" "node_modules/" ".env" "dist/" "__pycache__/")

setup_gitignore() {
  [[ ! -f .gitignore ]] && touch .gitignore
  for pattern in "${IGNORE_PATTERNS[@]}"; do
    if ! grep -qxF "$pattern" .gitignore 2>/dev/null; then
      printf '%s\n' "$pattern" >> .gitignore
      info "Ajouté au .gitignore : $pattern"
    fi
  done
}

untrack_ignored() {
  for pattern in "${IGNORE_PATTERNS[@]}"; do
    local dir="${pattern%/}"
    if git ls-files -- "$dir" 2>/dev/null | grep -q .; then
      warn "Retrait de l'index : ${pattern}"
      git rm -r --cached "$dir" 2>/dev/null || true
    fi
  done
}

# ── Stage intelligent ─────────────────────────────────────────────────────────
smart_add() {
  set +e
  local excludes=()
  for p in "${IGNORE_PATTERNS[@]}"; do
    excludes+=(":!${p}" ":!${p}**")
  done
  if git add . -- "${excludes[@]}" 2>/dev/null; then
    success "Fichiers stagés (dossiers ignorés exclus)."
  else
    git add -A
    for p in "${IGNORE_PATTERNS[@]}"; do
      git reset -- "$p" "${p}**" >/dev/null 2>&1 || true
    done
    success "Fichiers stagés (fallback)."
  fi
  set -e
}

# ── Bandeau branche ───────────────────────────────────────────────────────────
show_branch_banner() {
  local branch
  branch=$(git symbolic-ref --short HEAD 2>/dev/null || echo "HEAD détachée")
  local dirty=""
  git diff --quiet 2>/dev/null || dirty=" ${RED}●${RESET}"
  local remote
  remote=$(git remote get-url origin 2>/dev/null | sed 's/.*github.com[:/]//' | sed 's/\.git$//' || echo "")

  echo ""
  echo -e "${BOLD}${BLUE}╔══════════════════════════════════════╗${RESET}"
  printf "${BOLD}${BLUE}║${RESET}  🌿 %-34s${BOLD}${BLUE}║${RESET}\n" "${branch}${dirty@P}"
  [[ -n "$remote" ]] && \
  printf "${BOLD}${BLUE}║${RESET}  📦 %-34s${BOLD}${BLUE}║${RESET}\n" "$remote"
  echo -e "${BOLD}${BLUE}╚══════════════════════════════════════╝${RESET}"
  echo ""
}

# ── Commit + Push ─────────────────────────────────────────────────────────────
commit_and_push() {
  local msg="${1:-}"
  local branch
  branch=$(git rev-parse --abbrev-ref HEAD)

  setup_gitignore
  untrack_ignored
  smart_add

  if git diff --cached --quiet; then
    warn "Rien à committer — synchronisation uniquement."
    git pull --rebase origin "$branch"
    git push origin "$branch"
    success "Dépôt à jour."
    return
  fi

  if [[ -z "$msg" ]]; then
    read -rp "$(echo -e "${YELLOW}? ${RESET}Message de commit : ")" msg
    [[ -z "$msg" ]] && msg="Update"
  fi

  git commit -m "$msg"
  success "Commit : \"$msg\""

  info "Pull --rebase depuis origin/${branch}..."
  git pull --rebase origin "$branch"

  info "Push vers origin/${branch}..."
  git push origin "$branch"

  success "Tout est pushé ✓"
}

# ── Actions du menu ───────────────────────────────────────────────────────────
do_status() {
  title "État du dépôt"
  local branch
  branch=$(git symbolic-ref --short HEAD 2>/dev/null || echo "HEAD détachée")
  local remote
  remote=$(git remote get-url origin 2>/dev/null || echo "(pas de remote)")
  echo -e "  ${BOLD}Branche :${RESET} ${GREEN}${branch}${RESET}"
  echo -e "  ${BOLD}Remote  :${RESET} ${remote}"
  echo ""
  git status -sb
}

do_tag() {
  title "Créer un tag"
  info "Tags existants :"
  git tag --sort=-version:refname | head -10 || echo "  (aucun)"
  echo ""
  read -rp "$(echo -e "${YELLOW}? ${RESET}Nom du tag (ex: v1.2.3) : ")" tag
  [[ -z "$tag" ]] && { error "Nom vide — annulé."; return; }
  if git rev-parse "$tag" &>/dev/null 2>&1; then
    error "Le tag '${tag}' existe déjà."; return
  fi
  read -rp "$(echo -e "${YELLOW}? ${RESET}Message du tag (vide = tag léger) : ")" tmsg
  if [[ -n "$tmsg" ]]; then git tag -a "$tag" -m "$tmsg"; else git tag "$tag"; fi
  success "Tag '${tag}' créé."
  if confirm "Pusher ce tag sur origin ?"; then
    git push origin "$tag"
    success "Tag '${tag}' pushé."
  fi
}

do_pull() {
  title "Pull"
  local branch
  branch=$(git symbolic-ref --short HEAD)
  echo -e "  1) pull (merge)\n  2) pull --rebase\n"
  read -rp "$(echo -e "${YELLOW}? ${RESET}Choix [2] : ")" c
  case "${c:-2}" in
    1) git pull origin "$branch" && success "Pull effectué." ;;
    *) git pull --rebase origin "$branch" && success "Pull --rebase effectué." ;;
  esac
}

do_branch() {
  title "Branches"
  echo -e "  1) Lister\n  2) Créer\n  3) Changer\n  4) Supprimer\n  5) Retour\n"
  read -rp "$(echo -e "${YELLOW}? ${RESET}Choix : ")" c
  case "$c" in
    1) git branch -vva ;;
    2)
      read -rp "$(echo -e "${YELLOW}? ${RESET}Nom de la nouvelle branche : ")" nb
      [[ -z "$nb" ]] && { error "Nom vide."; return; }
      git checkout -b "$nb" && success "Branche '${nb}' créée."
      if confirm "Pusher et créer l'upstream ?"; then
        git push --set-upstream origin "$nb"
      fi ;;
    3)
      git branch -a
      read -rp "$(echo -e "${YELLOW}? ${RESET}Nom de la branche : ")" sb
      git checkout "$sb" && success "Basculé sur '${sb}'." ;;
    4)
      git branch -vva
      read -rp "$(echo -e "${YELLOW}? ${RESET}Branche à supprimer : ")" db
      confirm "Supprimer '${db}' localement ?" && git branch -d "$db"
      confirm "Supprimer '${db}' sur origin aussi ?" && git push origin --delete "$db" ;;
    *) return ;;
  esac
}

do_stash() {
  title "Stash"
  echo -e "  1) Sauvegarder\n  2) Lister\n  3) Restaurer (pop)\n  4) Retour\n"
  read -rp "$(echo -e "${YELLOW}? ${RESET}Choix : ")" c
  case "$c" in
    1)
      read -rp "$(echo -e "${YELLOW}? ${RESET}Message (optionnel) : ")" sm
      if [[ -n "$sm" ]]; then git stash push -m "$sm"; else git stash push; fi
      success "Stash sauvegardé." ;;
    2) git stash list ;;
    3) git stash pop && success "Stash restauré." ;;
    *) return ;;
  esac
}

do_log() {
  title "Historique"
  git log --oneline --graph --decorate --color --all | head -30
}

do_init() {
  title "Initialiser un dépôt"
  if git rev-parse --git-dir &>/dev/null 2>&1; then
    warn "Un dépôt Git existe déjà ici."; return
  fi
  git init && success "Dépôt initialisé."
  read -rp "$(echo -e "${YELLOW}? ${RESET}URL du remote origin (vide pour ignorer) : ")" rurl
  if [[ -n "$rurl" ]]; then
    git remote add origin "$rurl" && success "Remote 'origin' configuré."
  fi
}

# ── Reset total : fichiers conservés, historique effacé ───────────────────────
do_full_reset() {
  title "⚠️  Reset total du dépôt"

  local branch remote
  branch=$(git symbolic-ref --short HEAD 2>/dev/null || echo "main")
  remote=$(git remote get-url origin 2>/dev/null || echo "")

  echo -e "  ${RED}${BOLD}ATTENTION — Cette opération est IRRÉVERSIBLE.${RESET}"
  echo ""
  echo -e "  Ce qui va se passer :"
  echo -e "  ${GREEN}✔${RESET} Tes fichiers actuels sont conservés"
  echo -e "  ${RED}✖${RESET} Tout l'historique Git local est supprimé"
  [[ -n "$remote" ]] && \
  echo -e "  ${RED}✖${RESET} L'historique sur GitHub est écrasé (force push)"
  echo -e "  ${GREEN}✔${RESET} Un seul nouveau commit initial sera créé"
  echo ""
  echo -e "  Branche : ${GREEN}${branch}${RESET}"
  [[ -n "$remote" ]] && echo -e "  Remote  : ${remote}"
  echo ""
  sep
  echo ""
  echo -e "${RED}Tape exactement${RESET} ${BOLD}RESET${RESET} ${RED}pour confirmer (ou Entrée pour annuler) :${RESET}"
  read -rp "  → " confirm_word
  if [[ "$confirm_word" != "RESET" ]]; then
    warn "Annulé."
    return
  fi

  read -rp "$(echo -e "${YELLOW}? ${RESET}Message du commit initial [\"Initial commit\"] : ")" init_msg
  init_msg="${init_msg:-Initial commit}"

  echo ""
  info "Suppression du dossier .git..."
  rm -rf .git

  info "Réinitialisation du dépôt..."
  git init -b "$branch" 2>/dev/null || { git init && git checkout -b "$branch" 2>/dev/null || true; }

  info "Ajout de tous les fichiers..."
  setup_gitignore
  smart_add

  git commit -m "$init_msg"
  success "Nouveau commit initial créé : \"${init_msg}\""

  if [[ -n "$remote" ]]; then
    git remote add origin "$remote"
    info "Force push vers origin/${branch}..."
    git push --force origin "$branch"
    success "Dépôt remis à zéro sur GitHub ✓"
  else
    warn "Pas de remote configuré — reset local uniquement."
  fi

  echo ""
  success "✅ Reset terminé. Historique effacé, fichiers intacts."
}


# ── Purge d'un fichier trop gros de tout l'historique ────────────────────────
do_purge_large_file() {
  title "Purge fichier trop gros (>100 MB)"

  echo -e "  ${YELLOW}Fichiers les plus lourds dans l'historique Git :${RESET}"
  echo ""
  # Lister les 10 plus gros objets trackés dans l'historique
  git rev-list --objects --all 2>/dev/null     | git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' 2>/dev/null     | awk '/^blob/ { printf "%s\t%s\n", $3, $4 }'     | sort -rn     | head -10     | awk '{ printf "  %8.2f MB  %s\n", $1/1024/1024, $2 }'     || echo "  (impossible de lister)"
  echo ""
  sep
  echo ""

  read -rp "$(echo -e "${YELLOW}? ${RESET}Chemin du fichier à purger (ex: assets/model.nnue) : ")" filepath
  [[ -z "$filepath" ]] && { error "Chemin vide — annulé."; return; }

  echo ""
  echo -e "  Fichier à supprimer de tout l'historique : ${RED}${filepath}${RESET}"
  echo -e "  ${YELLOW}Le fichier restera sur ton disque, mais ne sera plus dans Git.${RESET}"
  echo ""

  read -rp "$(echo -e "${YELLOW}? ${RESET}Ajouter un pattern .gitignore pour éviter que ça se reproduise ? [O/n] ")" do_ignore
  local ignore_pattern=""
  if [[ ! "${do_ignore,,}" =~ ^n ]]; then
    # Proposer d'ignorer par extension ou chemin exact
    local ext="${filepath##*.}"
    echo -e "  1) Ignorer ce fichier exact     : ${filepath}"
    [[ "$ext" != "$filepath" ]] &&     echo -e "  2) Ignorer toute l'extension    : *.${ext}"
    read -rp "$(echo -e "${YELLOW}? ${RESET}Choix [1] : ")" ichoice
    case "${ichoice:-1}" in
      2) ignore_pattern="*.${ext}" ;;
      *) ignore_pattern="$filepath" ;;
    esac
  fi

  echo ""
  echo -e "${RED}Tape exactement${RESET} ${BOLD}PURGE${RESET} ${RED}pour confirmer (IRRÉVERSIBLE sur l'historique) :${RESET}"
  read -rp "  → " confirm_word
  [[ "$confirm_word" != "PURGE" ]] && { warn "Annulé."; return; }

  echo ""
  info "Réécriture de l'historique (filter-branch)..."
  git filter-branch --force --index-filter     "git rm --cached --ignore-unmatch '${filepath}'"     --prune-empty --tag-name-filter cat -- --all

  info "Nettoyage des refs résiduelles..."
  git for-each-ref --format="delete %(refname)" refs/original 2>/dev/null | git update-ref --stdin || true
  git reflog expire --expire=now --all
  git gc --prune=now --aggressive
  success "Historique nettoyé."

  # Ajouter au .gitignore
  if [[ -n "$ignore_pattern" ]]; then
    if ! grep -qxF "$ignore_pattern" .gitignore 2>/dev/null; then
      printf '%s
' "$ignore_pattern" >> .gitignore
      git add .gitignore
      git commit -m "chore: ignore ${ignore_pattern}"
      success "Pattern ajouté au .gitignore : ${ignore_pattern}"
    fi
  fi

  # Force push
  local branch
  branch=$(git symbolic-ref --short HEAD)
  local remote
  remote=$(git remote get-url origin 2>/dev/null || echo "")

  if [[ -n "$remote" ]]; then
    if confirm "Force push vers origin/${branch} ?"; then
      git push --force origin "$branch"
      # Pusher aussi les tags si réécrits
      git push --force --tags origin 2>/dev/null || true
      success "Force push effectué ✓"
    fi
  else
    warn "Pas de remote — purge locale uniquement."
  fi

  echo ""
  success "✅ Fichier purgé de l'historique. Le push devrait maintenant fonctionner."
}

# ── Release : commit + push + tag annoté ─────────────────────────────────────
do_release() {
  title "Release — Commit + Push + Tag"

  # Version depuis le Cargo.toml workspace (ligne : version = "x.y.z")
  local cargo_version=""
  if [[ -f "Cargo.toml" ]]; then
    cargo_version=$(sed -n 's/^version\s*=\s*"\(.*\)"/\1/p' Cargo.toml | head -1)
  fi
  local suggested_tag="v${cargo_version}"

  info "Version Cargo.toml : ${BOLD}${cargo_version:-inconnue}${RESET}"
  info "Tags existants :"
  git tag --sort=-version:refname | head -5 || echo "  (aucun)"
  echo ""
  sep

  # ── Étape 1 : commit + push ──────────────────────────────────────────────────
  title "Étape 1/2 — Commit + Push"
  commit_and_push ""

  echo ""
  sep

  # ── Étape 2 : tag annoté ─────────────────────────────────────────────────────
  title "Étape 2/2 — Créer le tag de release"

  read -rp "$(echo -e "${YELLOW}? ${RESET}Nom du tag [${BOLD}${suggested_tag}${RESET}] : ")" tag
  tag="${tag:-$suggested_tag}"

  [[ -z "$tag" ]] && { error "Nom de tag vide — annulé."; return; }

  if git rev-parse "$tag" &>/dev/null 2>&1; then
    error "Le tag '${tag}' existe déjà — annulé."
    return
  fi

  read -rp "$(echo -e "${YELLOW}? ${RESET}Message du tag [Release ${tag}] : ")" tmsg
  tmsg="${tmsg:-Release ${tag}}"

  git tag -a "$tag" -m "$tmsg"
  success "Tag annoté '${tag}' créé."

  info "Push du tag vers origin..."
  git push origin "$tag"

  echo ""
  success "✅ Release '${tag}' publiée — GitHub Actions va compiler et créer la release."
}

# ── Menu interactif ───────────────────────────────────────────────────────────
main_menu() {
  while true; do
    echo ""
    if git rev-parse --git-dir &>/dev/null 2>&1; then
      show_branch_banner
    else
      echo -e "${BOLD}${BLUE}╔══════════════════════════════════════╗${RESET}"
      echo -e "${BOLD}${BLUE}║        🚀  Assistant Git             ║${RESET}"
      echo -e "${BOLD}${BLUE}╚══════════════════════════════════════╝${RESET}"
      echo ""
    fi

    echo -e "  ${BOLD}1${RESET}  Commit + Push            ${CYAN}← équivalent à ./git.sh \"msg\"${RESET}"
    echo -e "  ${BOLD}2${RESET}  Créer & pusher un tag"
    echo -e "  ${BOLD}3${RESET}  ${GREEN}Release${RESET} ${GREEN}(commit + push + tag → déclenche GitHub Actions)${RESET}"
    echo -e "  ${BOLD}4${RESET}  Pull / Synchroniser"
    echo -e "  ${BOLD}5${RESET}  Branches"
    echo -e "  ${BOLD}6${RESET}  Stash"
    echo -e "  ${BOLD}7${RESET}  Historique (log)"
    echo -e "  ${BOLD}8${RESET}  Status"
    echo -e "  ${BOLD}0${RESET}  Initialiser un nouveau dépôt"
    echo -e "  ${BOLD}r${RESET}  ${RED}Reset total${RESET} ${RED}(efface l'historique, garde les fichiers)${RESET}"
    echo -e "  ${BOLD}f${RESET}  ${YELLOW}Purger un fichier trop gros${RESET} ${YELLOW}(fichier >100MB rejeté par GitHub)${RESET}"
    echo -e "  ${BOLD}q${RESET}  Quitter"
    echo ""
    sep

    read -rp "$(echo -e "${YELLOW}➜ ${RESET}Choix : ")" choice

    case "$choice" in
      1|2|3|4|5|6|7|8|r|R|f|F) require_git ;;
    esac

    case "$choice" in
      1) commit_and_push "" ;;
      2) do_tag ;;
      3) do_release ;;
      4) do_pull ;;
      5) do_branch ;;
      6) do_stash ;;
      7) do_log ;;
      8) do_status ;;
      0) do_init ;;
      r|R) do_full_reset ;;
      f|F) do_purge_large_file ;;
      q|Q) echo -e "\n${GREEN}À bientôt !${RESET}\n"; exit 0 ;;
      *) warn "Choix invalide." ;;
    esac
  done
}

# ── Point d'entrée ────────────────────────────────────────────────────────────
if [[ $# -gt 0 ]]; then
  require_git
  show_branch_banner
  commit_and_push "$*"
else
  main_menu
fi
