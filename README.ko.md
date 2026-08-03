# folder-lens — 폴더 인벤토리

[English](README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [简体中文](README.zh.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

**폴더 인벤토리**를 위한 가벼운 데스크톱 도구: 전체 스캔, 파일 유형별 그룹화, 파일 이동, 내보내기. Rust + Tauri 2로 개발되었으며 단일 파일(~10 MB)로 설치되고 크로스 플랫폼(Windows / macOS / Linux)을 지원합니다.

파일이 깊은 하위 폴더 사슬에 '묻혀' 있을 때 유용합니다. 경로가 포함된 평평한 목록에서 모든 것이 어디에 있는지 즉시 확인할 수 있습니다.

## 스크린샷

![folder-lens — 폴더 인벤토리](https://github.com/PrEvAl85/folder-lens/releases/download/v0.3.0/Screenshot_1.png)

## 기능

- **폴더 스캔** — 진행 표시줄과 중지 버튼이 있는 재귀적 탐색.
- **유형별 그룹화** — 모든 파일을 확장자별로 그룹화: 유형별 개수와 총 크기.
- **검색 및 정렬** — 이름별 즉시 필터, 크기 / 개수 / 이름별 정렬.
- **파일 목록** — 각 유형이 경로가 포함된 전체 파일 목록으로 펼쳐집니다.
- **미리 보기** — 파일을 클릭하면 오른쪽에 내용이 표시됩니다: 이미지, 텍스트 파일, 오디오, PDF, Office 문서(docx/xlsx/pptx), PSD, 비디오.
- **파일 이동** — 파일에 체크 표시하고 선택한 폴더로 이동; 중복 파일은 덮어쓰지 않습니다(`(1)` 접미사가 붙습니다); 마지막 이동은 취소할 수 있습니다.
- **내보내기** — 인벤토리를 CSV(BOM 포함, 구분자 `;`) 및 JSON으로 내보냅니다.
- **빈 폴더** — 별도로 표시되며 클릭하면 Explorer/Finder에서 열립니다.
- **사용자 지정 가능한 UI** — 사이드바와 미리 보기 패널은 마우스로 크기를 조절할 수 있으며 크기가 기억됩니다.

## 설치

[Releases](https://github.com/PrEvAl85/folder-lens/releases) 페이지에서 플랫폼에 맞는 설치 프로그램을 다운로드하세요:

| 플랫폼 | 파일 |
|-----------|------|
| Windows | `folder-lens_0.3.0_x64-setup.exe`(NSIS) 또는 `folder-lens_0.3.0_x64_en-US.msi` |
| macOS(Apple Silicon) | `folder-lens_0.3.0_aarch64.dmg` |
| macOS(앱) | `folder-lens_aarch64.app.tar.gz` |
| Linux | `folder-lens_0.3.0_amd64.AppImage`, `folder-lens_0.3.0_amd64.deb`, `folder-lens-0.3.0-1.x86_64.rpm` |

> **Windows SmartScreen(서명되지 않은 EXE).** Windows 빌드는 디지털 인증서로 서명되지 않았으므로(코드 서명은 유료), Windows가 첫 실행 시 'Windows에서 PC를 보호했습니다'를 표시할 수 있습니다. 이는 정상입니다 — 파일은 안전합니다:
>
> - **'추가 정보' → '그래도 실행'** 을 클릭하세요(1회).
> - 또는 '인터넷에서 다운로드' 표시를 제거하세요: 파일을 마우스 오른쪽 클릭 → **속성** → **'차단 해제'** 체크 → 확인.
> - 또는 PowerShell에서 `Unblock-File folder-lens_0.3.0_x64-setup.exe`를 실행하세요.

## 빌드 및 실행

요구 사항: [Rust](https://rustup.rs)(stable), [Node.js](https://nodejs.org) ≥ 18, Tauri의 [시스템 종속성](https://tauri.app/start/prerequisites/).

```sh
npm install
npm run tauri dev     # 개발
npm run tauri build   # 릴리스 빌드(msi/nsis/deb/appimage/…)
```

## 테스트

```sh
cd src-tauri && cargo test
```

범위: 스캔 및 그룹화, 스캔 취소, 덮어쓰지 않는 이동, 이동 롤백, CSV/JSON 내보내기, 미리 보기(이미지 / 텍스트 / 오디오 / PDF / Office / PSD / 비디오).

## 기술 세부 정보

- **스택:** Tauri 2(Rust) + 웹 UI(프레임워크 없는 Vanilla JS).
- **주요 Rust 크레이트:** `walkdir`, `serde`, `dunce`, `chrono`, `base64`, `zip`, `quick-xml`, `psd`, `png`, `tauri-plugin-dialog`, `tauri-plugin-opener`.
- **저장:** 로컬. 데이터가 컴퓨터를 떠나지 않습니다.

---

## 프로젝트 지원하기

이 프로젝트는 여가 시간에 만들어지고 관리됩니다. folder-lens가 유용하다면 개발을 지원해 주세요:

- ⭐ **GitHub에서 스타** — [PrEvAl85/folder-lens](https://github.com/PrEvAl85/folder-lens)
- 🐛 **버그 신고 및 아이디어** — [Issues](https://github.com/PrEvAl85/folder-lens/issues)
- 💬 **공유** — 유용할 사람들에게 알려주세요

**재정 지원:**

- ☕ **Boosty** — https://boosty.to/pws/donate
- 🍩 **DonationAlerts** — https://www.donationalerts.com/r/photowithoutstudio

**암호화폐:**

- USDT (TRC20): `TRcWS42MhyFRGdGSc6LqTH8CdTy6pLUMn6`
- USDT (BEP20): `0x0905134db34d8d54abf5b60a55406821ed7b8de0`
- BTC: `17hDrZL62DBpTjK6xNCGFFG682jN9PiVF1`
- TON: `UQCzoPJlYLHSoFGmRyh_-_ox1nOMCzx3LwG79xPR5pbjs3Aq`

folder-lens를 사용해 주셔서 감사합니다!

---

## 라이선스

이 프로젝트는 **MIT 라이선스**(허용적인 자유 소프트웨어 라이선스)로 배포됩니다. 아래 저작권 고지가 소프트웨어의 모든 사본 또는 상당 부분에 포함되는 한, 소프트웨어 사본을 자유롭게 사용, 복사, 수정, 병합, 게시, 배포, 서브라이선스, 판매할 수 있으며 다른 사람이 그렇게 하도록 허용할 수 있습니다.

본 소프트웨어는 상품성, 특정 목적에의 적합성 및 비침해성에 대한 묵시적 보증을 포함하되 이에 국한되지 않는 명시적 또는 묵시적 보증 없이 "있는 그대로" 제공됩니다. 저자는 어떠한 경우에도 본 소프트웨어의 사용으로 인해 발생하는 청구, 손해 또는 기타 책임에 대해 책임을 지지 않습니다.

전체 라이선스 전문은 [LICENSE](LICENSE) 파일에 있습니다.
