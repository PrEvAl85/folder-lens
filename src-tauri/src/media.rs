use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

static FFMPEG_AVAILABLE: OnceLock<bool> = OnceLock::new();

pub fn ffmpeg_available() -> bool {
    *FFMPEG_AVAILABLE.get_or_init(|| {
        Command::new("ffmpeg")
            .arg("-version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    })
}

fn temp_dir() -> PathBuf {
    std::env::temp_dir().join("folder-lens")
}

fn tmp_path(src: &str) -> PathBuf {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    src.hash(&mut h);
    let len = std::fs::metadata(src).map(|m| m.len()).unwrap_or(0);
    len.hash(&mut h);
    temp_dir().join(format!("preview_{:016x}.mp4", h.finish()))
}

/// Конвертирует видео в MP4 (H.264/AAC) для проигрывания в WebView2.
/// Сначала быстрый remux `-c copy`, при неудаче — полное перекодирование.
pub fn convert_video(src: &str) -> Result<PathBuf, String> {
    std::fs::create_dir_all(temp_dir()).map_err(|e| format!("не удалось создать temp: {e}"))?;
    let dst = tmp_path(src);
    if dst.exists() {
        return Ok(dst);
    }

    let remux = run_ffmpeg(&[
        "-y",
        "-i",
        src,
        "-c",
        "copy",
        "-movflags",
        "+faststart",
        dst.to_str().unwrap(),
    ]);
    if remux {
        return Ok(dst);
    }

    let _ = std::fs::remove_file(&dst);
    let encode = run_ffmpeg(&[
        "-y",
        "-i",
        src,
        "-c:v",
        "libx264",
        "-preset",
        "ultrafast",
        "-c:a",
        "aac",
        "-movflags",
        "+faststart",
        dst.to_str().unwrap(),
    ]);
    if encode {
        return Ok(dst);
    }

    let _ = std::fs::remove_file(&dst);
    Err("не удалось конвертировать видео".into())
}

fn run_ffmpeg(args: &[&str]) -> bool {
    Command::new("ffmpeg")
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp_path_is_stable_per_input() {
        let a = tmp_path(r"C:\some\clip.avi");
        let b = tmp_path(r"C:\some\clip.avi");
        assert_eq!(a, b);
    }

    #[test]
    fn tmp_path_differs_per_input() {
        let a = tmp_path(r"C:\some\clip1.avi");
        let b = tmp_path(r"C:\some\clip2.avi");
        assert_ne!(a, b);
    }
}
