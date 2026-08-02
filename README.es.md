# folder-lens — Inventario de carpetas

[English](README.md) | [Русский](README.ru.md) | [Deutsch](README.de.md) | [Español](README.es.md)

Una herramienta ligera de escritorio para el **inventario de carpetas**: escaneo completo, agrupación de archivos por tipo, mover archivos y exportar. Desarrollado con Rust + Tauri 2, se instala en un solo archivo (~10 MB), multiplataforma (Windows / macOS / Linux).

Útil cuando los archivos están «enterrados» en cadenas profundas de subcarpetas: una lista plana con rutas muestra al instante dónde está todo.

## Captura de pantalla

![folder-lens — inventario de carpetas](https://github.com/PrEvAl85/folder-lens/releases/download/v0.1.0/Screenshot_1.png)

## Características

- **Escanear una carpeta** — recorrido recursivo con barra de progreso y botón de detener.
- **Agrupar por tipo** — todos los archivos agrupados por extensión: cantidad y tamaño total por tipo.
- **Buscar y ordenar** — filtro instantáneo por nombre, ordenar por tamaño / cantidad / nombre.
- **Lista de archivos** — cada tipo se expande en una lista completa de archivos con rutas.
- **Vista previa** — al hacer clic en un archivo se muestra su contenido a la derecha: imágenes, archivos de texto y vídeo.
- **Mover archivos** — marque los archivos y muévalos a la carpeta elegida; los duplicados no se sobrescriben (reciben el sufijo `(1)`); el último movimiento se puede deshacer.
- **Exportar** — el inventario se exporta a CSV (con BOM, separador `;`) y JSON.
- **Carpetas vacías** — se muestran por separado, un clic las abre en Explorer/Finder.
- **Interfaz personalizable** — la barra lateral y el panel de vista previa son redimensionables con el ratón, los tamaños se recuerdan.

## Instalación

Descargue el instalador para su plataforma desde la página de [Releases](https://github.com/PrEvAl85/folder-lens/releases):

| Plataforma | Archivo |
|-----------|------|
| Windows | `folder-lens_0.1.0_x64-setup.exe` (NSIS) o `folder-lens_0.1.0_x64_en-US.msi` |
| macOS (Apple Silicon) | `folder-lens_0.1.0_aarch64.dmg` |
| macOS (aplicación) | `folder-lens_aarch64.app.tar.gz` |
| Linux | `folder-lens_0.1.0_amd64.AppImage`, `folder-lens_0.1.0_amd64.deb`, `folder-lens-0.1.0-1.x86_64.rpm` |

## Compilación y ejecución

Requisitos: [Rust](https://rustup.rs) (stable), [Node.js](https://nodejs.org) ≥ 18, [dependencias del sistema](https://tauri.app/start/prerequisites/) de Tauri.

```sh
npm install
npm run tauri dev     # desarrollo
npm run tauri build   # compilación de lanzamiento (msi/nsis/deb/appimage/…)
```

## Pruebas

```sh
cd src-tauri && cargo test
```

Cobertura: escaneo y agrupación, cancelación de escaneo, mover sin sobrescribir, deshacer movimiento, exportación CSV/JSON, vista previa (imágenes / texto / vídeo).

## Detalles técnicos

- **Stack:** Tauri 2 (Rust) + interfaz web (Vanilla JS, sin frameworks).
- **Crates de Rust principales:** `walkdir`, `serde`, `dunce`, `chrono`, `base64`, `tauri-plugin-dialog`, `tauri-plugin-opener`.
- **Almacenamiento:** local, los datos nunca salen de su ordenador.

---

## Apoyar el proyecto

Este proyecto se crea y mantiene en el tiempo libre. Si folder-lens le resulta útil — apoye su desarrollo:

- ⭐ **Estrella en GitHub** — [PrEvAl85/folder-lens](https://github.com/PrEvAl85/folder-lens)
- 🐛 **Informes de errores e ideas** — [Issues](https://github.com/PrEvAl85/folder-lens/issues)
- 💬 **Comparta** — cuéntelo a otras personas a quienes pueda resultar útil

**Apoyo económico:**

- ☕ **Boosty** — https://boosty.to/pws/donate
- 🍩 **DonationAlerts** — https://www.donationalerts.com/r/photowithoutstudio

**Criptomoneda:**

- USDT (TRC20): `TRcWS42MhyFRGdGSc6LqTH8CdTy6pLUMn6`
- USDT (BEP20): `0x0905134db34d8d54abf5b60a55406821ed7b8de0`
- BTC: `17hDrZL62DBpTjK6xNCGFFG682jN9PiVF1`
- TON: `UQCzoPJlYLHSoFGmRyh_-_ox1nOMCzx3LwG79xPR5pbjs3Aq`

¡Gracias por usar folder-lens!

---

## Licencia

El proyecto se distribuye bajo la **licencia MIT** — una licencia permisiva de software libre. Usted es libre de usar, copiar, modificar, fusionar, publicar, distribuir, sublicenciar y vender copias del software, así como de permitir que otros lo hagan, siempre que el aviso de copyright que figura a continuación se incluya en todas las copias o partes sustanciales del software.

El software se proporciona «tal cual», sin garantía de ningún tipo, expresa o implícita, incluidas, entre otras, las garantías de comerciabilidad, idoneidad para un fin determinado y no infracción. En ningún caso los autores serán responsables de reclamaciones, daños u otras responsabilidades derivadas del uso del software.

El texto completo de la licencia se encuentra en el archivo [LICENSE](LICENSE).
