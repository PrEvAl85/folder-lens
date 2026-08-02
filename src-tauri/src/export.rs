use serde::Deserialize;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
pub struct ExportRow {
    pub rel_path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub modified_ms: i64,
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains(';') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub fn write_csv(dest: &str, rows: &[ExportRow], delimiter: &str) -> Result<(), String> {
    let mut out = String::from("\u{feff}"); // UTF-8 BOM for Excel
    out.push_str("Относительный путь");
    out.push_str(delimiter);
    out.push_str("Имя файла");
    out.push_str(delimiter);
    out.push_str("Расширение");
    out.push_str(delimiter);
    out.push_str("Размер (байт)");
    out.push_str(delimiter);
    out.push_str("Дата изменения (ISO)\n");

    for r in rows {
        out.push_str(&csv_escape(&r.rel_path));
        out.push_str(delimiter);
        out.push_str(&csv_escape(&r.name));
        out.push_str(delimiter);
        out.push_str(&csv_escape(&r.extension));
        out.push_str(delimiter);
        out.push_str(&r.size.to_string());
        out.push_str(delimiter);
        out.push_str(&iso_time(r.modified_ms));
        out.push('\n');
    }

    write_file(dest, out.as_bytes())
}

pub fn write_json(dest: &str, rows: &[ExportRow]) -> Result<(), String> {
    let payload: Vec<serde_json::Value> = rows
        .iter()
        .map(|r| {
            serde_json::json!({
                "path": r.rel_path,
                "name": r.name,
                "extension": r.extension,
                "size": r.size,
                "modified": iso_time(r.modified_ms),
            })
        })
        .collect();
    let json = serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?;
    write_file(dest, json.as_bytes())
}

fn iso_time(ms: i64) -> String {
    let secs = (ms / 1000) as i64;
    let nanos = ((ms % 1000).clamp(0, 999) * 1_000_000) as u32;
    match chrono::DateTime::from_timestamp(secs, nanos) {
        Some(dt) => dt.to_rfc3339(),
        None => String::new(),
    }
}

fn write_file(dest: &str, data: &[u8]) -> Result<(), String> {
    let path = Path::new(dest);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.is_dir() {
            std::fs::create_dir_all(parent).map_err(|e| format!("создание папки: {e}"))?;
        }
    }
    std::fs::write(path, data).map_err(|e| format!("запись {dest}: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn csv_has_bom_and_header() {
        let base = std::env::temp_dir().join(format!("fl_exp_{}", std::process::id()));
        fs::create_dir_all(&base).unwrap();
        let dest = base.join("out.csv");
        let rows = vec![ExportRow {
            rel_path: "a;b/c.txt".into(),
            name: "c.txt".into(),
            extension: "txt".into(),
            size: 5,
            modified_ms: 1_700_000_000_000,
        }];
        write_csv(dest.to_str().unwrap(), &rows, ";").unwrap();

        let content = fs::read_to_string(&dest).unwrap();
        assert!(content.starts_with('\u{feff}'));
        assert!(content.contains("Относительный путь"));
        assert!(content.contains("\"a;b/c.txt\""));
        fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn json_valid() {
        let base = std::env::temp_dir().join(format!("fl_expj_{}", std::process::id()));
        fs::create_dir_all(&base).unwrap();
        let dest = base.join("out.json");
        let rows = vec![ExportRow {
            rel_path: "x/y.png".into(),
            name: "y.png".into(),
            extension: "png".into(),
            size: 42,
            modified_ms: 0,
        }];
        write_json(dest.to_str().unwrap(), &rows).unwrap();
        let content = fs::read_to_string(&dest).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed[0]["name"], "y.png");
        fs::remove_dir_all(&base).ok();
    }
}
