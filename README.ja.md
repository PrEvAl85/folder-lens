# folder-lens — フォルダー在庫

[English](README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [简体中文](README.zh.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

**フォルダー在庫管理**のための軽量デスクトップツール：完全なスキャン、ファイルの種類別グループ化、ファイルの移動、エクスポート。Rust + Tauri 2 で開発され、単一ファイル（約10 MB）でインストールでき、クロスプラットフォーム（Windows / macOS / Linux）対応です。

ファイルが深いサブフォルダーの連なりに「埋もれている」場合に便利です。パス付きのフラットなリストで、どこに何があるかがすぐにわかります。

## スクリーンショット

![folder-lens — フォルダー在庫](https://github.com/PrEvAl85/folder-lens/releases/download/v0.3.0/Screenshot_1.png)

## 機能

- **フォルダーのスキャン** — プログレスバーと停止ボタン付きの再帰的な走査。
- **種類別のグループ化** — すべてのファイルを拡張子ごとにグループ化：種類ごとの数と合計サイズ。
- **検索と並べ替え** — 名前による即時フィルター、サイズ / 数 / 名前での並べ替え。
- **ファイル一覧** — 各種類がパス付きの完全なファイル一覧に展開されます。
- **プレビュー** — ファイルをクリックすると、右側に内容が表示されます：画像、テキストファイル、音声、PDF、Office ドキュメント（docx/xlsx/pptx）、PSD、動画。
- **ファイルの移動** — ファイルにチェックを入れて選択したフォルダーへ移動；重複は上書きされません（`(1)` サフィックスが付きます）；最後の移動は取り消せます。
- **エクスポート** — 在庫を CSV（BOM付き、区切り文字 `;`）と JSON に出力。
- **空のフォルダー** — 別途表示され、クリックで Explorer/Finder が開きます。
- **カスタマイズ可能なUI** — サイドバーとプレビューパネルはマウスでサイズ変更でき、サイズは記憶されます。

## インストール

[Releases](https://github.com/PrEvAl85/folder-lens/releases) ページからお使いのプラットフォームのインストーラーをダウンロードしてください：

| プラットフォーム | ファイル |
|-----------|------|
| Windows | `folder-lens_0.3.0_x64-setup.exe`（NSIS）または `folder-lens_0.3.0_x64_en-US.msi` |
| macOS（Apple Silicon） | `folder-lens_0.3.0_aarch64.dmg` |
| macOS（アプリ） | `folder-lens_aarch64.app.tar.gz` |
| Linux | `folder-lens_0.3.0_amd64.AppImage`、`folder-lens_0.3.0_amd64.deb`、`folder-lens-0.3.0-1.x86_64.rpm` |

> **Windows SmartScreen（署名なし EXE）。** Windows ビルドはデジタル証明書で署名されていません（署名は有料）ため、初回起動時に Windows が「Windows によって PC が保護されました」と表示することがあります。これは正常です — ファイルは安全です：
>
> - **「詳細情報」→「それでも実行する」** をクリック（一度だけ）；
> - または「インターネットからダウンロード」のマークを外す：ファイルを右クリック → **プロパティ** → **「ブロックの解除」** にチェック → OK；
> - または PowerShell で `Unblock-File folder-lens_0.3.0_x64-setup.exe` を実行。

## ビルドと実行

要件：[Rust](https://rustup.rs)（stable）、[Node.js](https://nodejs.org) ≥ 18、Tauri の[システム依存関係](https://tauri.app/start/prerequisites/)。

```sh
npm install
npm run tauri dev     # 開発
npm run tauri build   # リリースビルド（msi/nsis/deb/appimage/…）
```

## テスト

```sh
cd src-tauri && cargo test
```

カバレッジ：スキャンとグループ化、スキャンのキャンセル、上書きしない移動、移動の取り消し、CSV/JSON エクスポート、プレビュー（画像 / テキスト / 音声 / PDF / Office / PSD / 動画）。

## 技術詳細

- **スタック：** Tauri 2（Rust）+ Web UI（Vanilla JS、フレームワークなし）。
- **主要な Rust クレート：** `walkdir`、`serde`、`dunce`、`chrono`、`base64`、`zip`、`quick-xml`、`psd`、`png`、`tauri-plugin-dialog`、`tauri-plugin-opener`。
- **保存：** ローカル。データがあなたのコンピューターから出ることはありません。

---

## プロジェクトを支援する

このプロジェクトは空き時間に作成・維持されています。folder-lens が役立つなら、開発を支援してください：

- ⭐ **GitHub でスター** — [PrEvAl85/folder-lens](https://github.com/PrEvAl85/folder-lens)
- 🐛 **バグ報告とアイデア** — [Issues](https://github.com/PrEvAl85/folder-lens/issues)
- 💬 **共有** — 役に立ちそうな人に教えてください

**経済的支援：**

- ☕ **Boosty** — https://boosty.to/pws/donate
- 🍩 **DonationAlerts** — https://www.donationalerts.com/r/photowithoutstudio

**暗号通貨：**

- USDT (TRC20)：`TRcWS42MhyFRGdGSc6LqTH8CdTy6pLUMn6`
- USDT (BEP20)：`0x0905134db34d8d54abf5b60a55406821ed7b8de0`
- BTC：`17hDrZL62DBpTjK6xNCGFFG682jN9PiVF1`
- TON：`UQCzoPJlYLHSoFGmRyh_-_ox1nOMCzx3LwG79xPR5pbjs3Aq`

folder-lens をご利用いただきありがとうございます！

---

## ライセンス

このプロジェクトは **MIT ライセンス**（寛容なフリーソフトウェアライセンス）のもとで配布されています。以下に記載の著作権表示をソフトウェアのすべてのコピーまたは重要な部分に含める限り、ソフトウェアのコピーを使用・複製・変更・結合・公開・配布・サブライセンス・販売すること、および他の人にそれを許可することは自由です。

本ソフトウェアは「現状のまま」提供され、商品性、特定目的への適合性、非侵害性を含むがこれらに限定されない、明示的または黙示的な一切の保証はありません。いかなる場合も、著作者は本ソフトウェアの使用に起因または関連して発生するいかなる請求、損害、その他の責任についても責任を負いません。

完全なライセンス文は [LICENSE](LICENSE) ファイルにあります。
