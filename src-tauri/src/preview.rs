use base64::Engine;
use serde::Serialize;
use std::fs;
use std::io::Read;
use std::path::Path;

const MAX_IMAGE_BYTES: u64 = 10 * 1024 * 1024;
const MAX_TEXT_BYTES: u64 = 256 * 1024;

#[derive(Clone, Debug, Serialize)]
pub struct PreviewData {
    pub kind: String, // "image" | "text" | "none"
    pub mime: String,
    pub data: String, // base64 (image) или текст
    pub truncated: bool,
    pub note: String, // пояснение для kind == "none" или "truncated"
}

impl PreviewData {
    fn none(note: &str) -> Self {
        Self {
            kind: "none".into(),
            mime: String::new(),
            data: String::new(),
            truncated: false,
            note: note.into(),
        }
    }

    fn image(mime: &str, bytes: &[u8]) -> Self {
        Self {
            kind: "image".into(),
            mime: mime.into(),
            data: base64::engine::general_purpose::STANDARD.encode(bytes),
            truncated: false,
            note: String::new(),
        }
    }

    fn text(data: String, truncated: bool) -> Self {
        Self {
            kind: "text".into(),
            mime: "text/plain".into(),
            data,
            truncated,
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
        _ => None,
    }
}

pub fn preview_file(path: &str) -> Result<PreviewData, String> {
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

    if let Some(mime) = video_mime(&ext) {
        return Ok(PreviewData {
            kind: "video".into(),
            mime: mime.into(),
            data: String::new(),
            truncated: false,
            note: String::new(),
        });
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
        let res = preview_file(p.to_str().unwrap()).unwrap();
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
        let res = preview_file(p.to_str().unwrap()).unwrap();
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
        let res = preview_file(p.to_str().unwrap()).unwrap();
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
        let res = preview_file(p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "none");
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_video_returns_mime_without_reading_bytes() {
        let p = temp_path("clip.mp4");
        std::fs::write(&p, "fake-video-bytes").unwrap();
        let res = preview_file(p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "video");
        assert_eq!(res.mime, "video/mp4");
        assert!(res.data.is_empty());
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_unknown_ext_is_none() {
        let p = temp_path("archive.zip");
        std::fs::write(&p, "PK\x03\x04fake").unwrap();
        let res = preview_file(p.to_str().unwrap()).unwrap();
        assert_eq!(res.kind, "none");
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn preview_missing_file_errors() {
        let p = temp_path("nope.txt");
        let res = preview_file(p.to_str().unwrap());
        assert!(res.is_err());
    }
}

