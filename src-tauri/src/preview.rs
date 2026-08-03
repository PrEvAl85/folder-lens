use base64::Engine;
use serde::Serialize;
use std::fs;
use std::io::Read;
use std::path::Path;
use tauri::AppHandle;
use tauri::Manager;

const MAX_IMAGE_BYTES: u64 = 10 * 1024 * 1024;
const MAX_TEXT_BYTES: u64 = 256 * 1024;
const MAX_PDF_BYTES: u64 = 50 * 1024 * 1024;

#[derive(Clone, Debug, Serialize)]
pub struct PreviewData {
    pub kind: String, // "image" | "text" | "video" | "audio" | "pdf" | "none"
    pub mime: String,
    pub data: String, // base64 (image) или текст
    pub path: String, // путь для asset-протокола (видео/аудио/pdf); пуст для остальных
    pub truncated: bool,
    pub note: String, // пояснение для kind == "none" или "truncated"
}

impl PreviewData {
    fn none(note: &str) -> Self {
        Self {
            kind: "none".into(),
            mime: String::new(),
            data: String::new(),
            path: String::new(),
            truncated: false,
            note: note.into(),
        }
    }

    fn image(mime: &str, bytes: &[u8]) -> Self {
        Self {
            kind: "image".into(),
            mime: mime.into(),
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            path: String::new(),
            truncated: false,
            note: String::new(),
        }
    }

    fn text(data: String, truncated: bool) -> Self {
        Self {
            kind: "text".into(),
            mime: "text/plain".into(),
            data,
            path: String::new(),
            truncated,
            note: String::new(),
        }
    }

    fn asset(kind: &str, mime: &str, path: &str) -> Self {
        Self {
            kind: kind.into(),
            mime: mime.into(),
            data: String::new(),
            path: path.into(),
            truncated: false,
            note: String::new(),
        }
    }
}

fn ext_of(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
}

fn image_mime(ext: &str) -> Option<&'static str> {
    match ext {
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        "webp" => Some("image/webp"),
        "svg" => Some("image/svg+xml"),
        "bmp" => Some("image/bmp"),
        "ico" => Some("image/x-icon"),
        _ => None,
    }
}

fn is_text_ext(ext: &str) -> bool {
    matches!(
        ext,
        "txt" | "md" | "markdown" | "log" | "csv" | "json" | "xml" | "yaml" | "yml" | "ini"
            | "cfg" | "conf" | "toml" | "html" | "htm" | "css" | "js" | "mjs" | "ts" | "tsx"
            | "jsx" | "py" | "rb" | "rs" | "c" | "h" | "cpp" | "hpp" | "java" | "go" | "sql"
            | "sh" | "bat" | "ps1" | "env" | "gitignore" | "dockerfile" | "gradle" | "lock"
    )
}

fn video_mime(ext: &str) -> Option<&'static str> {
    match ext {
        "mp4" | "m4v" => Some("video/mp4"),
        "webm" => Some("video/webm"),
        "ogg" | "ogv" => Some("video/ogg"),
        "mov" => Some("video/quicktime"),
        "avi" => Some("video/x-msvideo"),
        "mkv" => Some("video/x-matroska"),
        "wmv" => Some("video/x-ms-wmv"),
        "flv" => Some("video/x-flv"),
        "3gp" | "3g2" => Some("video/3gpp"),
        _ => None,
    }
}

/// Форматы видео, которые WebView2 умеет проигрывать нативно
/// (без внешнего ffmpeg). Прочие — конвертируются через ffmpeg при наличии.
fn is_native_video(ext: &str) -> bool {
    matches!(ext, "mp4" | "m4v" | "webm" | "ogg" | "ogv")
}

fn audio_mime(ext: &str) -> Option<&'static str> {
    match ext {
        "mp3" => Some("audio/mpeg"),
        "wav" => Some("audio/wav"),
        "m4a" => Some("audio/mp4"),
        "aac" => Some("audio/aac"),
        "ogg" | "oga" => Some("audio/ogg"),
        "opus" => Some("audio/opus"),
        "flac" => Some("audio/flac"),
        "wma" => Some("audio/x-ms-wma"),
        _ => None,
    }
}

fn is_office_ext(ext: &str) -> bool {
    matches!(ext, "docx" | "xlsx" | "pptx")
}

pub async fn preview_file(app: AppHandle, path: String) -> Result<PreviewData, String> {
    let res = tauri::async_runtime::spawn_blocking(move || preview_impl(Some(&app), &path))
        .await
        .map_err(|e| e.to_string())?;
    res
}

fn preview_impl(app: Option<&AppHandle>, path: &str) -> Result<PreviewData, String> {
    let meta = fs::metadata(path).map_err(|e| format!("не удалось открыть файл: {e}"))?;
    if !meta.is_file() {
        return Ok(PreviewData::none("Это не файл"));
    }

    let ext = ext_of(path);

    if let Some(mime) = image_mime(&ext) {
        if meta.len() > MAX_IMAGE_BYTES {
            return Ok(PreviewData::none("Файл слишком большой для предпросмотра"));
        }
        let bytes = fs::read(path).map_err(|e| format!("не удалось прочитать файл: {e}"))?;
        return Ok(PreviewData::image(mime, &bytes));
    }

    if ext == "psd" {
        return match crate::psd::render_psd(path) {
            Ok(png) => Ok(PreviewData::image("image/png", &png)),
            Err(msg) => Ok(PreviewData::none(&msg)),
        };
    }

    if let Some(mime) = audio_mime(&ext) {
        return Ok(PreviewData::asset("audio", mime, path));
    }

    if let Some(mime) = video_mime(&ext) {
        if is_native_video(&ext) {
            return Ok(PreviewData::asset("video", mime, path));
        }
        return match convert_with_ffmpeg(app, path) {
            Ok(Some((mime, tmp))) => Ok(PreviewData::asset("video", &mime, &tmp)),
            Ok(None) => Ok(PreviewData::none(
                "Установите FFmpeg для предпросмотра этого формата",
            )),
            Err(msg) => Ok(PreviewData::none(&msg)),
        };
    }

    if ext == "pdf" {
        if meta.len() > MAX_PDF_BYTES {
            return Ok(PreviewData::none("Файл слишком большой для предпросмотра"));
        }
        return Ok(PreviewData::asset("pdf", "application/pdf", path));
    }

    if is_office_ext(&ext) {
        return match crate::office::extract_office(path, &ext) {
            Ok(Some(text)) => {
                let truncated = text.len() as u64 > MAX_TEXT_BYTES;
                let data: String = text.chars().take(MAX_TEXT_BYTES as usize).collect();
                Ok(PreviewData::text(data, truncated))
            }
            Ok(None) => Ok(PreviewData::none(
                "Предпросмотр недоступен для этого типа файла",
            )),
            Err(msg) => Ok(PreviewData::none(&msg)),
        };
    }

    if is_text_ext(&ext) {
        let file = fs::File::open(path).map_err(|e| format!("не удалось открыть файл: {e}"))?;
        let mut buf = Vec::new();
        file.take(MAX_TEXT_BYTES)
            .read_to_end(&mut buf)
            .map_err(|e| format!("не удалось прочитать файл: {e}"))?;
        if buf.iter().take(8192).any(|&b| b == 0) {
            return Ok(PreviewData::none("Бинарный файл — предпросмотр недоступен"));
        }
        let truncated = meta.len() > MAX_TEXT_BYTES;
        return Ok(PreviewData::text(
            String::from_utf8_lossy(&buf).into_owned(),
            truncated,
        ));
    }

    Ok(PreviewData::none(
        "Предпросмотр недоступен для этого типа файла",
    ))
}

fn convert_with_ffmpeg(
    app: Option<&AppHandle>,
    path: &str,
) -> Result<Option<(String, String)>, String> {
    if !crate::media::ffmpeg_available() {
        return Ok(None);
    }
    let tmp = crate::media::convert_video(path)?;
    if let Some(app) = app {
        app.asset_protocol_scope()
            .allow_file(&tmp)
            .map_err(|e| format!("не удалось открыть доступ к файлу: {e}"))?;
    }
    Ok(Some(("video/mp4".to_string(), tmp.to_string_lossy().into_owned())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("fl_preview_{unique}_{name}"))
    }

    #[test]
    fn preview_image_returns_base64() {
        let p = temp_path("img.png");
        let raw: Vec<u8> = vec![0x89, b'P', b'N', b'G', 1, 2, 3];
        std::fs::write(&p, &raw).unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "image");
        assert_eq!(res.mime, "image/png");
        assert_eq!(
            res.data,
            base64::engine::general_purpose::STANDARD.encode(&raw)
        );
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_text_returns_content_and_flags_truncation() {
        let p = temp_path("note.md");
        std::fs::write(&p, "hello\nworld").unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "text");
        assert_eq!(res.data, "hello\nworld");
        assert!(!res.truncated);
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_text_truncates_long_files() {
        let p = temp_path("big.log");
        let mut f = std::fs::File::create(&p).unwrap();
        let chunk = "x".repeat(4096);
        for _ in 0..(MAX_TEXT_BYTES as usize / 4096 + 1) {
            f.write_all(chunk.as_bytes()).unwrap();
        }
        f.flush().unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "text");
        assert!(res.truncated);
        assert!(res.data.len() as u64 <= MAX_TEXT_BYTES);
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_binary_with_null_bytes_is_none() {
        let p = temp_path("data.bin");
        let raw: Vec<u8> = vec![0u8, 1, 2, 0, 3, 255];
        std::fs::write(&p, &raw).unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "none");
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_video_returns_mime_without_reading_bytes() {
        let p = temp_path("clip.mp4");
        std::fs::write(&p, "fake-video-bytes").unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "video");
        assert_eq!(res.mime, "video/mp4");
        assert!(res.data.is_empty());
        assert_eq!(res.path, p.to_str().unwrap());
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_audio_returns_mime_without_reading_bytes() {
        for (name, mime) in [("song.mp3", "audio/mpeg"), ("clip.wav", "audio/wav")] {
            let p = temp_path(name);
            std::fs::write(&p, "fake-audio").unwrap();
            let res = preview_impl(None, p.to_str().unwrap()).unwrap();
            assert_eq!(res.kind, "audio");
            assert_eq!(res.mime, mime);
            std::fs::remove_file(&p).ok();
        }
    }

    #[test]
    fn preview_pdf_returns_asset_kind() {
        let p = temp_path("doc.pdf");
        std::fs::write(&p, "%PDF-1.4 fake").unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "pdf");
        assert_eq!(res.mime, "application/pdf");
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_psd_invalid_is_none() {
        let p = temp_path("layer.psd");
        std::fs::write(&p, "not-a-psd").unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "none");
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_docx_extracts_text() {
        let p = temp_path("report.docx");
        let xml = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>Office text</w:t></w:r></w:p></w:body></w:document>"#;
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut zw = zip::ZipWriter::new(&mut buf);
            zw.start_file(
                "word/document.xml",
                zip::write::SimpleFileOptions::default(),
            )
            .unwrap();
            zw.write_all(xml.as_bytes()).unwrap();
            zw.finish().unwrap();
        }
        std::fs::write(&p, buf.into_inner()).unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "text");
        assert!(res.data.contains("Office text"));
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_legacy_doc_is_none() {
        let p = temp_path("old.doc");
        std::fs::write(&p, "legacy binary").unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "none");
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_unknown_ext_is_none() {
        let p = temp_path("archive.zip");
        std::fs::write(&p, "PK\x03\x04fake").unwrap();
        let res = preview_impl(None, p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "none");
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_missing_file_errors() {
        let p = temp_path("nope.txt");
        let res = preview_impl(None, p.to_str().unwrap());
        assert!(res.is_err());
    }

    #[test]
    fn video_mime_recognizes_3gp() {
        assert_eq!(video_mime("3gp"), Some("video/3gpp"));
        assert_eq!(video_mime("mov"), Some("video/quicktime"));
        assert_eq!(video_mime("avi"), Some("video/x-msvideo"));
    }
}
