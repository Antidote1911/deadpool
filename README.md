[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/Antidote1911/deadpool/blob/master/LICENSE-MIT)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/Antidote1911/deadpool/actions/workflows/release.yml/badge.svg)](https://github.com/Antidote1911/deadpool/actions/workflows/release.yml)
[![Release](https://github.com/Antidote1911/deadpool/actions/workflows/release.yml/badge.svg)](https://github.com/Antidote1911/deadpool/actions/workflows/release.yml)
[![Latest Release](https://img.shields.io/github/v/release/Antidote1911/deadpool)](https://github.com/Antidote1911/deadpool/releases/latest)

> [Version française](README.fr.md)

# 🔑 Deadpool and Deadpool-CLI

This repository contains three components: **deadpool-core**, a Rust library for generating secure passwords; **deadpool-cli**, a command-line tool; and **deadpool**, a graphical interface — both built on top of the core library.

<img src='screenshots/deadpool_screenshot.png'/>

## Download pre-built binaries

Pre-built binaries for Linux, Windows, and macOS (Universal) are available on the [Releases page](https://github.com/Antidote1911/deadpool/releases/latest).

| Platform | Archive |
|---|---|
| Linux x86\_64 | `deadpool-{version}-linux-x86_64.AppImage` |
| Windows x86\_64 (portable) | `deadpool-{version}-win-portable.zip` |
| Windows x86\_64 (installer) | `deadpool-{version}-win-setup.exe` |
| macOS Universal (arm64 + x86\_64) | `deadpool-{version}-universal-macos.dmg` |

The Linux AppImage and the Windows portable zip each contain two binaries:
- `deadpool-cli` / `deadpool-cli.exe` — command-line tool
- `deadpool` / `deadpool.exe` — graphical interface

## Install on Arch Linux

Three PKGBUILDs are provided in `packaging/archlinux/`.

**AppImage** (`PKGBUILD-AppImage`) — downloads the pre-built AppImage, no compilation required:

```bash
mkdir -p ~/builds/deadpool-appimage && cd ~/builds/deadpool-appimage
curl -O https://raw.githubusercontent.com/Antidote1911/deadpool/master/packaging/archlinux/PKGBUILD-AppImage
mv PKGBUILD-AppImage PKGBUILD
makepkg -si
```

Installs `/opt/deadpool/deadpool.AppImage` and creates `/usr/bin/deadpool` and `/usr/bin/deadpool-cli` symlinks. Requires `fuse2`.

**Stable release** (`PKGBUILD`) — builds from the latest tagged version on GitHub:

```bash
mkdir -p ~/builds/deadpool && cd ~/builds/deadpool
curl -O https://raw.githubusercontent.com/Antidote1911/deadpool/master/packaging/archlinux/PKGBUILD
makepkg -si
```

**Git version** (`PKGBUILD-git`) — builds from the latest commit on `master`, installs as `deadpool-git` (conflicts with `deadpool`):

```bash
mkdir -p ~/builds/deadpool-git && cd ~/builds/deadpool-git
curl -O https://raw.githubusercontent.com/Antidote1911/deadpool/master/packaging/archlinux/PKGBUILD-git
mv PKGBUILD-git PKGBUILD
makepkg -si
```

Or clone the repo and use the files directly:

```bash
git clone https://github.com/Antidote1911/deadpool
cd deadpool/packaging/archlinux
# for stable:
makepkg -si -p PKGBUILD
# for git:
makepkg -si -p PKGBUILD-git
```

Both packages install:
- `/usr/bin/deadpool-cli` — command-line tool
- `/usr/bin/deadpool` — graphical interface
- A `.desktop` entry and icon for the GUI

## Usage for Deadpool crate

```rust
use deadpool_core::Pool;

let mut pool = Pool::new();
pool.extend_from_uppercase();
pool.extend_from_digits();
pool.extend_from_dashes();
pool.extend_from_string("@é=")?;
pool.exclude_chars("0Oo1iIlL5S"); // exclude ambiguous chars

let password = pool.generate(25)?;
```

## Character sets

| Flag | Characters |
|---|---|
| `-u` / `--uppercase` | `ABCDEFGHIJKLMNOPQRSTUVWXYZ` |
| `-l` / `--lowercase` | `abcdefghijklmnopqrstuvwxyz` |
| `-d` / `--digits` | `0123456789` |
| `-b` / `--braces` | `()[]{}` |
| `-p` / `--punctuation` | `.,:;` |
| `-q` / `--quotes` | `"'` |
| `--dashes` | `-/\_\|` |
| `-m` / `--math` | `!*+<=>?` |
| `--logograms` | `#$%&@^\`~` |

## Usage for deadpool-cli

The generated passwords always contain at least one character from each selected group.
Without arguments, the generated password is 10 characters long and uses lowercase letters and numbers.

### Using the CLI via the Linux AppImage

The AppImage bundles both the GUI and the CLI. The recommended way to install it is via the provided script, which downloads the latest release under a versionless name and creates two symlinks so updates never break your habits or scripts:

```bash
curl -sL https://raw.githubusercontent.com/Antidote1911/deadpool/master/packaging/linux/install.sh | bash
```

This installs into `~/.local/bin/`:
- `deadpool` — launches the graphical interface
- `deadpool-cli` — launches the command-line tool directly

```bash
deadpool-cli -ld -L 20
deadpool-cli --count 3 -L 30 -ldm --include "@éèà"
deadpool-cli --help
```

**Manual setup** — if you prefer to place the AppImage yourself:

```bash
# Save the AppImage without the version number
mv deadpool-*-linux-x86_64.AppImage ~/.local/bin/deadpool.AppImage
chmod +x ~/.local/bin/deadpool.AppImage

# Create symlinks
ln -sf deadpool.AppImage ~/.local/bin/deadpool
ln -sf deadpool.AppImage ~/.local/bin/deadpool-cli
```

The AppImage also accepts `--cli` as its first argument if you invoke it directly:

```bash
./deadpool.AppImage --cli -ld -L 20
```

### Examples

```
# equivalent to ./deadpool-cli -ld -L 10
./deadpool-cli
uabhbunf0q
```

Generate 3 passwords with 30 chars using lowercase, digits, math symbols, and include `@ é è à % M`:
```
./deadpool-cli --count 3 -L 30 -ldm --include "@éèà%M"
0c3mi<l1=Ma6xfujp>ddc3%%*n76èp
3>j+%=?5k*ubyd@p+=wior4a@qhiàu
tz6z99iwà1h!s+Mg4iv5t%@%5kenq8
```

Generate a password with 30 chars using only digits, excluding 0–5:
```
./deadpool-cli -d -L 30 --exclude 012345
879866968679799766976867796776
```

> The `--exclude` option takes precedence over `--include`. A character added with `--include` is always removed by `--exclude`.

Full help:
```
./deadpool-cli -h

🔑 Random password generator CLI

Usage: deadpool-cli [OPTIONS]

Options:
  -u, --uppercase          Use UPPERCASE letters [A-Z]
  -l, --lowercase          Use lowercase letters [a-z]
  -d, --digits             Use digits [0-9]
  -b, --braces             Use braces [()[]{}]
  -p, --punctuation        Use punctuation [.,:;]
  -q, --quotes             Use quotes ["']
      --dashes             Use dashes [-/\_|]
  -m, --math               Use math symbols [!*+<=>?]
      --logograms          Use logograms [#$%&@^`~]
  -C, --count <NUMBER>     Number of passwords to generate [default: 1]
  -L, --length <NUMBER>    Sets the required password length [default: 10]
      --output <OUTPUT>    Output to a txt file
      --exclude <EXCLUDE>  Exclude chars
      --include <INCLUDE>  Include chars
  -h, --help               Print help
  -V, --version            Print version

If you do not specify any of the [--uppercase, --lowercase, --digits] flags, then lowercase and digits will be used.
```

## Build from source

Clone the repo and build with Cargo:
```
git clone https://github.com/Antidote1911/deadpool
cd deadpool
cargo build --release
```

Binaries are written to `target/release/`.
