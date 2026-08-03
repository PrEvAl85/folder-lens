const MAX_PSD_BYTES: u64 = 50 * 1024 * 1024;
const MAX_PSD_PIXELS: u64 = 40_000_000;

pub fn render_psd(path: &str) -> Result<Vec<u8>, String> {
    let meta = std::fs::metadata(path).map_err(|e| format!("не удалось открыть файл: {e}"))?;
    if meta.len() > MAX_PSD_BYTES {
        return Err("Файл слишком большой для предпросмотра".into());
    }
    let bytes = std::fs::read(path).map_err(|e| format!("не удалось прочитать файл: {e}"))?;
    let psd = psd::Psd::from_bytes(&bytes).map_err(|e| format!("не удалось разобрать PSD: {e}"))?;

    let (w, h) = (u64::from(psd.width()), u64::from(psd.height()));
    if w * h > MAX_PSD_PIXELS {
        return Err("Изображение слишком большое для предпросмотра".into());
    }

    let rgba = psd.rgba();
    let mut out = Vec::with_capacity(rgba.len());
    {
        let mut enc = png::Encoder::new(&mut out, psd.width(), psd.height());
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        let mut writer = enc.write_header().map_err(|e| format!("ошибка PNG: {e}"))?;
        writer
            .write_image_data(&rgba)
            .map_err(|e| format!("ошибка PNG: {e}"))?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path(name: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("fl_psd_{unique}_{name}"))
    }

    #[test]
    fn render_psd_invalid_file_errors() {
        let p = temp_path("fake.psd");
        std::fs::write(&p, "not-a-psd").unwrap();
        let res = render_psd(p.to_str().unwrap());
        assert!(res.is_err());
        std::fs::remove_file(&p).ok();
    }

    #[test]
    fn render_psd_missing_file_errors() {
        let p = temp_path("nope.psd");
        assert!(render_psd(p.to_str().unwrap()).is_err());
    }
}
