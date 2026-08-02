# folder-lens — 文件夹清单

[English](README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [简体中文](README.zh.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

一款轻量级桌面工具，用于**文件夹清单**：完整扫描、按类型分组文件、移动文件和导出。基于 Rust + Tauri 2 开发，单文件安装（约 10 MB），跨平台（Windows / macOS / Linux）。

当文件「深埋」在层层子文件夹中时非常方便：带路径的扁平列表能立即显示所有内容的位置。

## 截图

![folder-lens — 文件夹清单](https://github.com/PrEvAl85/folder-lens/releases/download/v0.1.0/Screenshot_1.png)

## 功能

- **扫描文件夹** — 递归遍历，带进度条和停止按钮。
- **按类型分组** — 所有文件按扩展名分组：每种类型的数量和总大小。
- **搜索和排序** — 按名称即时筛选，按大小 / 数量 / 名称排序。
- **文件列表** — 每种类型展开为带路径的完整文件列表。
- **预览** — 点击文件在右侧显示内容：图片、文本文件和视频。
- **移动文件** — 勾选文件并移动到所选文件夹；不覆盖重复文件（它们会获得 `(1)` 后缀）；最后一次移动可以撤销。
- **导出** — 清单导出为 CSV（带 BOM，分隔符 `;`）和 JSON。
- **空文件夹** — 单独显示，点击可在 Explorer/Finder 中打开。
- **可定制界面** — 侧边栏和预览面板可用鼠标调整大小，尺寸会被记住。

## 安装

从 [Releases](https://github.com/PrEvAl85/folder-lens/releases) 页面下载适合您平台的安装包：

| 平台 | 文件 |
|-----------|------|
| Windows | `folder-lens_0.1.0_x64-setup.exe`（NSIS）或 `folder-lens_0.1.0_x64_en-US.msi` |
| macOS（Apple Silicon） | `folder-lens_0.1.0_aarch64.dmg` |
| macOS（应用） | `folder-lens_aarch64.app.tar.gz` |
| Linux | `folder-lens_0.1.0_amd64.AppImage`、`folder-lens_0.1.0_amd64.deb`、`folder-lens-0.1.0-1.x86_64.rpm` |

## 构建与运行

要求：[Rust](https://rustup.rs)（stable）、[Node.js](https://nodejs.org) ≥ 18、Tauri 的[系统依赖](https://tauri.app/start/prerequisites/)。

```sh
npm install
npm run tauri dev     # 开发
npm run tauri build   # 发布构建（msi/nsis/deb/appimage/…）
```

## 测试

```sh
cd src-tauri && cargo test
```

覆盖范围：扫描和分组、取消扫描、不覆盖的移动、移动回滚、CSV/JSON 导出、预览（图片 / 文本 / 视频）。

## 技术细节

- **技术栈：** Tauri 2（Rust）+ Web 界面（原生 JS，无框架）。
- **主要 Rust 库：** `walkdir`、`serde`、`dunce`、`chrono`、`base64`、`tauri-plugin-dialog`、`tauri-plugin-opener`。
- **存储：** 本地存储，数据永远不会离开您的计算机。

---

## 支持项目

本项目利用业余时间创建和维护。如果 folder-lens 对您有用 — 请支持其发展：

- ⭐ **在 GitHub 上点星** — [PrEvAl85/folder-lens](https://github.com/PrEvAl85/folder-lens)
- 🐛 **错误报告和想法** — [Issues](https://github.com/PrEvAl85/folder-lens/issues)
- 💬 **分享** — 告诉可能用得上的人

**财务支持：**

- ☕ **Boosty** — https://boosty.to/pws/donate
- 🍩 **DonationAlerts** — https://www.donationalerts.com/r/photowithoutstudio

**加密货币：**

- USDT (TRC20)：`TRcWS42MhyFRGdGSc6LqTH8CdTy6pLUMn6`
- USDT (BEP20)：`0x0905134db34d8d54abf5b60a55406821ed7b8de0`
- BTC：`17hDrZL62DBpTjK6xNCGFFG682jN9PiVF1`
- TON：`UQCzoPJlYLHSoFGmRyh_-_ox1nOMCzx3LwG79xPR5pbjs3Aq`

感谢您使用 folder-lens！

---

## 许可证

本项目根据 **MIT 许可证**分发 — 一种宽松的自由软件许可证。您可以自由使用、复制、修改、合并、发布、分发、再许可和出售软件副本，也可以允许他人这样做，前提是包含下述版权声明于软件的所有副本或重要部分中。

本软件按「现状」提供，不提供任何明示或暗示的担保，包括但不限于适销性、特定用途适用性和非侵权性的保证。在任何情况下，作者均不对因使用本软件而产生的任何索赔、损害或其他责任负责。

完整许可证文本见 [LICENSE](LICENSE) 文件。
