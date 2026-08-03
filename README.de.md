# folder-lens — Ordnerinventar

[English](README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [简体中文](README.zh.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

Ein leichtgewichtiges Desktop-Tool für die **Ordnerinventur**: vollständiges Scannen, Gruppierung der Dateien nach Typ, Verschieben von Dateien und Export. Entwickelt mit Rust + Tauri 2, Installation als einzelne Datei (~10 MB), plattformübergreifend (Windows / macOS / Linux).

Praktisch, wenn Dateien in tiefen Unterordner-Ketten „vergraben" sind: Eine flache Liste mit Pfaden zeigt sofort, wo alles liegt.

## Screenshot

![folder-lens — Ordnerinventar](https://github.com/PrEvAl85/folder-lens/releases/download/v0.3.0/Screenshot_1.png)

## Funktionen

- **Ordner scannen** — rekursiver Durchlauf mit Fortschrittsbalken und Stopp-Schaltfläche.
- **Nach Typ gruppieren** — alle Dateien nach Erweiterung gruppiert: Anzahl und Gesamtgröße pro Typ.
- **Suchen und sortieren** — sofortiger Namensfilter, Sortierung nach Größe / Anzahl / Name.
- **Dateiliste** — jeder Typ erweitert sich zu einer vollständigen Liste der Dateien mit Pfaden.
- **Vorschau** — Klick auf eine Datei zeigt den Inhalt rechts an: Bilder, Textdateien, Audio, PDF, Office-Dokumente (docx/xlsx/pptx), PSD und Videos.
- **Dateien verschieben** — markieren Sie Dateien und verschieben Sie sie in den gewählten Ordner; Duplikate werden nicht überschrieben (sie erhalten das Suffix `(1)`); das letzte Verschieben kann rückgängig gemacht werden.
- **Export** — das Inventar wird als CSV (mit BOM, Trennzeichen `;`) und JSON exportiert.
- **Leere Ordner** — werden separat angezeigt, Klick öffnet sie im Explorer/Finder.
- **Anpassbare Oberfläche** — Seitenleiste und Vorschaufenster sind mit der Maus in der Größe veränderbar, die Größen werden gemerkt.

## Installation

Laden Sie den Installer für Ihre Plattform von der [Releases](https://github.com/PrEvAl85/folder-lens/releases)-Seite:

| Plattform | Datei |
|-----------|------|
| Windows | `folder-lens_0.3.0_x64-setup.exe` (NSIS) oder `folder-lens_0.3.0_x64_en-US.msi` |
| macOS (Apple Silicon) | `folder-lens_0.3.0_aarch64.dmg` |
| macOS (App) | `folder-lens_aarch64.app.tar.gz` |
| Linux | `folder-lens_0.3.0_amd64.AppImage`, `folder-lens_0.3.0_amd64.deb`, `folder-lens-0.3.0-1.x86_64.rpm` |

> **Windows SmartScreen (unsignierte EXE).** Die Windows-Builds sind nicht mit einem digitalen Zertifikat signiert (Signierung ist kostenpflichtig), daher kann Windows beim ersten Start „Windows hat Ihren PC geschützt“ anzeigen. Das ist normal — die Datei ist sicher:
>
> - klicken Sie auf **„Weitere Informationen“ → „Trotzdem ausführen“** (einmalig);
> - oder entfernen Sie die Markierung „Aus dem Internet heruntergeladen“: Rechtsklick auf die Datei → **Eigenschaften** → Häkchen **„Blockierung aufheben“** → OK;
> - oder führen Sie in PowerShell `Unblock-File folder-lens_0.3.0_x64-setup.exe` aus.

## Build & Start

Voraussetzungen: [Rust](https://rustup.rs) (stable), [Node.js](https://nodejs.org) ≥ 18, Tauri-[Systemabhängigkeiten](https://tauri.app/start/prerequisites/).

```sh
npm install
npm run tauri dev     # Entwicklung
npm run tauri build   # Release-Build (msi/nsis/deb/appimage/…)
```

## Tests

```sh
cd src-tauri && cargo test
```

Abdeckung: Scannen und Gruppieren, Scan-Abbruch, Verschieben ohne Überschreiben, Verschieben-Rückgängig, CSV/JSON-Export, Vorschau (Bilder / Text / Audio / PDF / Office / PSD / Video).

## Technische Details

- **Stack:** Tauri 2 (Rust) + Web-UI (Vanilla JS, ohne Frameworks).
- **Wichtigste Rust-Crates:** `walkdir`, `serde`, `dunce`, `chrono`, `base64`, `zip`, `quick-xml`, `psd`, `png`, `tauri-plugin-dialog`, `tauri-plugin-opener`.
- **Speicherung:** lokal, Daten verlassen niemals Ihren Computer.

---

## Projekt unterstützen

Dieses Projekt wird in der Freizeit erstellt und gepflegt. Wenn folder-lens für Sie nützlich ist — unterstützen Sie die Entwicklung:

- ⭐ **Star auf GitHub** — [PrEvAl85/folder-lens](https://github.com/PrEvAl85/folder-lens)
- 🐛 **Fehlermeldungen und Ideen** — [Issues](https://github.com/PrEvAl85/folder-lens/issues)
- 💬 **Teilen** — erzählen Sie anderen davon, denen es nützlich sein könnte

**Finanzielle Unterstützung:**

- ☕ **Boosty** — https://boosty.to/pws/donate
- 🍩 **DonationAlerts** — https://www.donationalerts.com/r/photowithoutstudio

**Kryptowährung:**

- USDT (TRC20): `TRcWS42MhyFRGdGSc6LqTH8CdTy6pLUMn6`
- USDT (BEP20): `0x0905134db34d8d54abf5b60a55406821ed7b8de0`
- BTC: `17hDrZL62DBpTjK6xNCGFFG682jN9PiVF1`
- TON: `UQCzoPJlYLHSoFGmRyh_-_ox1nOMCzx3LwG79xPR5pbjs3Aq`

Danke, dass Sie folder-lens verwenden!

---

## Lizenz

Das Projekt wird unter der **MIT-Lizenz** verbreitet — einer freizügigen Freie-Software-Lizenz. Sie dürfen Kopien der Software frei verwenden, kopieren, modifizieren, zusammenführen, veröffentlichen, verteilen, unterlizenzieren und verkaufen sowie anderen Personen dies erlauben, vorausgesetzt, der unten aufgeführte Urheberrechtshinweis wird in allen Kopien oder wesentlichen Teilen der Software enthalten.

Die Software wird „wie besehen" bereitgestellt, ohne jegliche ausdrückliche oder stillschweigende Garantie, einschließlich, aber nicht beschränkt auf stillschweigende Garantien der Marktgängigkeit und der Eignung für einen bestimmten Zweck. Die Autoren haften in keinem Fall für Ansprüche, Schäden oder sonstige Verpflichtungen, die aus oder im Zusammenhang mit der Nutzung der Software entstehen.

Der vollständige Lizenztext befindet sich in der Datei [LICENSE](LICENSE).
