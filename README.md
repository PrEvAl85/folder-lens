# folder-lens — Folder Inventory

[English](README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [简体中文](README.zh.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

A lightweight desktop tool for **folder inventory**: full scanning, grouping files by type, moving files, and exporting. Built with Rust + Tauri 2, installs as a single file (~10 MB), cross-platform (Windows / macOS / Linux).

Handy when files are "buried" in deep chains of subfolders: a flat list with paths immediately shows where everything is.

## Screenshot

![folder-lens — folder inventory](https://github.com/PrEvAl85/folder-lens/releases/download/v0.3.0/Screenshot_1.png)

## Features

- **Scan a folder** — recursive walk with a progress bar and a stop button.
- **Group by type** — all files grouped by extension: count and total size per type.
- **Search and sort** — instant filter by name, sort by size / count / name.
- **File list** — each type expands into a full list of files with paths.
- **Preview** — clicking a file shows its contents on the right: images, text files, audio, PDF, Office documents (docx/xlsx/pptx), PSD, and video.
- **Move files** — tick files and move them to the chosen folder; duplicates are not overwritten (they get a `(1)` suffix); the last move can be undone.
- **Export** — inventory is exported to CSV (with BOM, `;` separator) and JSON.
- **Empty folders** — shown separately, click opens in Explorer/Finder.
- **Customizable UI** — sidebar and preview panel are resizable with the mouse, sizes are remembered.

## Installation

Download the installer for your platform from the [Releases](https://github.com/PrEvAl85/folder-lens/releases) page:

| Platform | File |
|-----------|------|
| Windows | `folder-lens_0.3.0_x64-setup.exe` (NSIS) or `folder-lens_0.3.0_x64_en-US.msi` |
| macOS (Apple Silicon) | `folder-lens_0.3.0_aarch64.dmg` |
| macOS (app) | `folder-lens_aarch64.app.tar.gz` |
| Linux | `folder-lens_0.3.0_amd64.AppImage`, `folder-lens_0.3.0_amd64.deb`, `folder-lens-0.3.0-1.x86_64.rpm` |

> **Windows SmartScreen (unsigned EXE).** The Windows builds are not signed with a digital certificate (code signing is paid), so Windows may show "Windows protected your PC" on first launch. This is expected — the file is safe:
>
> - click **"More info" → "Run anyway"** (one-time);
> - or remove the "downloaded from the internet" mark: right-click the file → **Properties** → check **"Unblock"** → OK;
> - or run `Unblock-File folder-lens_0.3.0_x64-setup.exe` in PowerShell.

## Build & Run

Requirements: [Rust](https://rustup.rs) (stable), [Node.js](https://nodejs.org) ≥ 18, Tauri [system dependencies](https://tauri.app/start/prerequisites/).

```sh
npm install
npm run tauri dev     # development
npm run tauri build   # release build (msi/nsis/deb/appimage/…)
```

## Tests

```sh
cd src-tauri && cargo test
```

Coverage: scanning and grouping, scan cancellation, non-overwriting move, move rollback, CSV/JSON export, preview (images / text / audio / PDF / Office / PSD / video).

## Technical Details

- **Stack:** Tauri 2 (Rust) + web UI (Vanilla JS, no frameworks).
- **Key Rust crates:** `walkdir`, `serde`, `dunce`, `chrono`, `base64`, `zip`, `quick-xml`, `psd`, `png`, `tauri-plugin-dialog`, `tauri-plugin-opener`.
- **Storage:** local, data never leaves your computer.

---

## Support the Project

This project is created and maintained in free time. If folder-lens is useful to you — help its development:

- ⭐ **Star on GitHub** — [PrEvAl85/folder-lens](https://github.com/PrEvAl85/folder-lens)
- 🐛 **Bug reports and ideas** — [Issues](https://github.com/PrEvAl85/folder-lens/issues)
- 💬 **Share** — tell others who might find it useful

**Financial support:**

- ☕ **Boosty** — https://boosty.to/pws/donate
- 🍩 **DonationAlerts** — https://www.donationalerts.com/r/photowithoutstudio

**Cryptocurrency:**

- USDT (TRC20): `TRcWS42MhyFRGdGSc6LqTH8CdTy6pLUMn6`
- USDT (BEP20): `0x0905134db34d8d54abf5b60a55406821ed7b8de0`
- BTC: `17hDrZL62DBpTjK6xNCGFFG682jN9PiVF1`
- TON: `UQCzoPJlYLHSoFGmRyh_-_ox1nOMCzx3LwG79xPR5pbjs3Aq`

Thank you for using folder-lens!

---

## License

The project is distributed under the **MIT License** — a permissive free-software license. You are free to use, copy, modify, merge, publish, distribute, sublicense, and sell copies of the software, as well as to permit others to do the same, provided that the copyright notice below is included in all copies or substantial portions of the software.

The software is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors be liable for any claim, damages or other liability arising from, out of or in connection with the software or its use.

The full license text is in the [LICENSE](LICENSE) file.
