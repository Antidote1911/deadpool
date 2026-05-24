#!/bin/bash
set -euo pipefail

INSTALL_DIR="${HOME}/.local/bin"
APPIMAGE_NAME="deadpool.AppImage"

mkdir -p "$INSTALL_DIR"

echo "Fetching latest release..."
DOWNLOAD_URL=$(curl -sL "https://api.github.com/repos/Antidote1911/deadpool/releases/latest" \
  | grep -o '"browser_download_url": *"[^"]*linux-x86_64\.AppImage"' \
  | grep -o 'https://[^"]*')

if [ -z "$DOWNLOAD_URL" ]; then
  echo "Error: could not find AppImage download URL." >&2
  exit 1
fi

echo "Downloading $(basename "$DOWNLOAD_URL")..."
curl -L "$DOWNLOAD_URL" -o "$INSTALL_DIR/$APPIMAGE_NAME"
chmod +x "$INSTALL_DIR/$APPIMAGE_NAME"

ln -sf "$APPIMAGE_NAME" "$INSTALL_DIR/deadpool"
ln -sf "$APPIMAGE_NAME" "$INSTALL_DIR/deadpool-cli"

echo ""
echo "Installed to $INSTALL_DIR:"
echo "  deadpool      -> graphical interface"
echo "  deadpool-cli  -> command-line tool"
echo ""
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo "Note: add $INSTALL_DIR to your PATH:"
  echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
fi
