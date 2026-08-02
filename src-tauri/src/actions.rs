use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoveItem {
    pub src: String,
    pub dest_dir: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UndoItem {
    pub from: String,
    pub back_to: String,
}

#[derive(Serialize)]
pub struct MoveReport {
    pub moved: Vec<UndoItem>,
    pub errors: Vec<String>,
}

fn unique_dest(dir: &Path, src_name: &str) -> PathBuf {
    let name = Path::new(src_name);
    let stem = name.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let ext = name.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();

    let candidate = if ext.is_empty() {
        dir.join(stem.clone())
    } else {
        dir.join(format!("{stem}.{ext}"))
    };
    if !candidate.exists() {
        return candidate;
    }

    let mut i = 1;
    loop {
        let next = if ext.is_empty() {
            dir.join(format!("{stem} ({i})"))
        } else {
            dir.join(format!("{stem} ({i}).{ext}"))
        };
        if !next.exists() {
            return next;
        }
        i += 1;
    }
}

fn relocate(src: &Path, dest: &Path) -> Result<(), String> {
    match std::fs::rename(src, dest) {
        Ok(()) => Ok(()),
        Err(_) => {
            // Cross-device or permission fallback: copy + delete.
            std::fs::copy(src, dest).map_err(|e| format!("копирование: {e}"))?;
            std::fs::remove_file(src).map_err(|e| format!("удаление исходника: {e}"))?;
            Ok(())
        }
    }
}

fn apply(src_str: &str, dest_dir_str: &str) -> Result<(String, String), String> {
    let src = PathBuf::from(src_str);
    if !src.is_file() {
        return Err(format!("исходный файл не найден: {src_str}"));
    }

    let dest_dir = PathBuf::from(dest_dir_str);
    let src_name = src
        .file_name()
        .ok_or_else(|| format!("не удалось получить имя файла: {src_str}"))?
        .to_string_lossy()
        .to_string();

    if !dest_dir.is_dir() {
        std::fs::create_dir_all(&dest_dir).map_err(|e| format!("создание папки {dest_dir_str}: {e}"))?;
    }

    let dest = unique_dest(&dest_dir, &src_name);
    let same = dunce::canonicalize(&src).ok() == dunce::canonicalize(&dest).ok();
    if same {
        return Err(format!("файл уже в целевой папке: {src_str}"));
    }

    relocate(&src, &dest).map_err(|e| format!("{src_str} -> {}: {e}", dest.display()))?;

    Ok((dest.to_string_lossy().to_string(), src.to_string_lossy().to_string()))
}

pub fn move_files(items: &[MoveItem]) -> MoveReport {
    let mut moved = Vec::new();
    let mut errors = Vec::new();
    for item in items {
        match apply(&item.src, &item.dest_dir) {
            Ok((dest, original)) => moved.push(UndoItem {
                from: dest,
                back_to: original,
            }),
            Err(e) => errors.push(e),
        }
    }
    MoveReport { moved, errors }
}

pub fn undo_move(items: &[UndoItem]) -> MoveReport {
    let mut moved = Vec::new();
    let mut errors = Vec::new();
    for item in items {
        let dest_parent = Path::new(&item.back_to)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());
        match apply(&item.from, &dest_parent) {
            Ok((back_to, current)) => moved.push(UndoItem {
                from: back_to,
                back_to: current,
            }),
            Err(e) => errors.push(e),
        }
    }
    MoveReport { moved, errors }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp(name: &str) -> PathBuf {
        let base = std::env::temp_dir().join(format!("fl_act_{}_{}", std::process::id(), name));
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).unwrap();
        base
    }

    #[test]
    fn moves_file_and_keeps_name() {
        let base = temp("move");
        let src_dir = base.join("src");
        let dest_dir = base.join("dest");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("a.txt"), "x").unwrap();

        let report = move_files(&[MoveItem {
            src: src_dir.join("a.txt").to_string_lossy().to_string(),
            dest_dir: dest_dir.to_string_lossy().to_string(),
        }]);

        assert!(report.errors.is_empty());
        assert_eq!(report.moved.len(), 1);
        assert!(dest_dir.join("a.txt").is_file());
        assert!(!src_dir.join("a.txt").exists());
        fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn does_not_overwrite_existing() {
        let base = temp("overwrite");
        let src_dir = base.join("src");
        let dest_dir = base.join("dest");
        fs::create_dir_all(&src_dir).unwrap();
        fs::create_dir_all(&dest_dir).unwrap();
        fs::write(src_dir.join("a.txt"), "new").unwrap();
        fs::write(dest_dir.join("a.txt"), "old").unwrap();

        let report = move_files(&[MoveItem {
            src: src_dir.join("a.txt").to_string_lossy().to_string(),
            dest_dir: dest_dir.to_string_lossy().to_string(),
        }]);

        assert!(report.errors.is_empty());
        assert!(dest_dir.join("a (1).txt").is_file());
        assert!(dest_dir.join("a.txt").is_file());
        assert_eq!(fs::read_to_string(dest_dir.join("a.txt")).unwrap(), "old");
        fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn undo_restores_original_location() {
        let base = temp("undo");
        let src_dir = base.join("src");
        let dest_dir = base.join("dest");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("b.pdf"), "x").unwrap();

        let report = move_files(&[MoveItem {
            src: src_dir.join("b.pdf").to_string_lossy().to_string(),
            dest_dir: dest_dir.to_string_lossy().to_string(),
        }]);
        assert!(report.errors.is_empty());
        assert!(dest_dir.join("b.pdf").is_file());

        let undo = undo_move(&report.moved);
        assert!(undo.errors.is_empty());
        assert!(src_dir.join("b.pdf").is_file());
        assert!(!dest_dir.join("b.pdf").exists());
        fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn missing_source_reports_error() {
        let base = temp("missing");
        let report = move_files(&[MoveItem {
            src: base.join("nope.txt").to_string_lossy().to_string(),
            dest_dir: base.join("dest").to_string_lossy().to_string(),
        }]);
        assert_eq!(report.errors.len(), 1);
        assert!(report.moved.is_empty());
        fs::remove_dir_all(&base).ok();
    }
}
