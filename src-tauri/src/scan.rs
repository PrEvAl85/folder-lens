use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

#[derive(Clone, Serialize)]
pub struct FileInfo {
    pub path: String,
    pub rel_path: String,
    pub name: String,
    pub size: u64,
    pub modified_ms: i64,
    pub extension: String,
}

#[derive(Clone, Serialize)]
pub struct TypeGroup {
    pub extension: String,
    pub count: usize,
    pub total_size: u64,
    pub files: Vec<FileInfo>,
}

#[derive(Serialize)]
pub struct ScanResult {
    pub root: String,
    pub groups: Vec<TypeGroup>,
    pub empty_dirs: Vec<String>,
    pub total_files: usize,
    pub total_size: u64,
    pub total_dirs: usize,
    pub cancelled: bool,
    pub scan_errors: Vec<String>,
}

fn modified_ms(t: Option<SystemTime>) -> i64 {
    match t {
        Some(t) => match t.duration_since(UNIX_EPOCH) {
            Ok(d) => d.as_millis() as i64,
            Err(_) => 0,
        },
        None => 0,
    }
}

fn extension_of(name: &str) -> String {
    Path::new(name)
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default()
}

fn rel_str(path: &Path, root: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path);
    let parts: Vec<String> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();
    parts.join("/")
}

pub fn scan_folder(
    root: &str,
    cancel: Option<Arc<AtomicBool>>,
    mut on_progress: Option<Box<dyn FnMut(usize) + Send + Sync>>,
) -> ScanResult {
    let root_path = Path::new(root).to_path_buf();
    let mut groups: Vec<TypeGroup> = Vec::new();
    let mut all_dirs: HashSet<std::path::PathBuf> = HashSet::new();
    let mut dirs_with_children: HashSet<std::path::PathBuf> = HashSet::new();
    let mut total_files = 0usize;
    let mut total_size = 0u64;
    let mut total_dirs = 0usize;
    let mut cancelled = false;
    let mut scan_errors: Vec<String> = Vec::new();
    let mut index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut progress_counter = 0usize;

    if !root_path.is_dir() {
        return ScanResult {
            root: root.to_string(),
            groups: Vec::new(),
            empty_dirs: Vec::new(),
            total_files: 0,
            total_size: 0,
            total_dirs: 0,
            cancelled: false,
            scan_errors: vec![format!("Папка не найдена или недоступна: {root}")],
        };
    }

    for entry in WalkDir::new(&root_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Some(c) = &cancel {
            if c.load(Ordering::Relaxed) {
                cancelled = true;
                break;
            }
        }

        let file_type = entry.file_type();
        let path = entry.path();
        let is_dir = file_type.is_dir();

        if is_dir {
            total_dirs += 1;
            if let Some(parent) = path.parent() {
                dirs_with_children.insert(parent.to_path_buf());
            }
            all_dirs.insert(path.to_path_buf());
        } else if file_type.is_file() {
            let parent = path.parent().map(|p| p.to_path_buf());
            if let Some(p) = parent {
                dirs_with_children.insert(p);
            }

            let metadata = match std::fs::metadata(path) {
                Ok(m) => m,
                Err(err) => {
                    scan_errors.push(format!("{}: {err}", path.display()));
                    continue;
                }
            };

            let name = entry.file_name().to_string_lossy().to_string();
            let rel = rel_str(path, &root_path);
            let size = metadata.len();
            let ext = extension_of(&name);

            let file_info = FileInfo {
                path: dunce::canonicalize(path)
                    .unwrap_or_else(|_| path.to_path_buf())
                    .to_string_lossy()
                    .to_string(),
                rel_path: rel,
                name,
                size,
                modified_ms: modified_ms(metadata.modified().ok()),
                extension: ext.clone(),
            };

            let idx = match index.get(&ext) {
                Some(i) => *i,
                None => {
                    groups.push(TypeGroup {
                        extension: ext.clone(),
                        count: 0,
                        total_size: 0,
                        files: Vec::new(),
                    });
                    let i = groups.len() - 1;
                    index.insert(ext, i);
                    i
                }
            };

            groups[idx].count += 1;
            groups[idx].total_size += size;
            groups[idx].files.push(file_info);
            total_files += 1;
            total_size += size;

            progress_counter += 1;
            if progress_counter % 512 == 0 {
                if let Some(cb) = &mut on_progress {
                    cb(progress_counter);
                }
            }
        }
    }

    let mut empty_dirs_vec: Vec<String> = all_dirs
        .iter()
        .filter(|d| !dirs_with_children.contains(*d))
        .map(|d| rel_str(d, &root_path))
        .collect();
    empty_dirs_vec.sort();

    groups.sort_by(|a, b| {
        b.count
            .cmp(&a.count)
            .then_with(|| a.extension.cmp(&b.extension))
    });
    for g in groups.iter_mut() {
        g.files
            .sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
    }

    if let Some(cb) = &mut on_progress {
        cb(total_files);
    }

    ScanResult {
        root: root.to_string(),
        groups,
        empty_dirs: empty_dirs_vec,
        total_files,
        total_size,
        total_dirs,
        cancelled,
        scan_errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrder};

    fn make_tree(base: &Path) {
        fs::create_dir_all(base.join("a/b/c")).unwrap();
        fs::create_dir_all(base.join("a/b/empty1")).unwrap();
        fs::create_dir_all(base.join("a/empty2/deep")).unwrap();
        fs::write(base.join("a/b/c/doc.txt"), "hello").unwrap();
        fs::write(base.join("a/b/c/photo.JPG"), "img").unwrap();
        fs::write(base.join("a/note.md"), "# title").unwrap();
        fs::write(base.join("top.txt"), "top").unwrap();
        fs::write(base.join("noext"), "no extension").unwrap();
    }

    #[test]
    fn groups_by_extension_case_insensitive() {
        let base = std::env::temp_dir().join(format!("fl_scan_{}", std::process::id()));
        fs::remove_dir_all(&base).ok();
        make_tree(&base);

        let result = scan_folder(base.to_str().unwrap(), None, None);

        fs::remove_dir_all(&base).ok();

        assert_eq!(result.total_files, 5);
        assert_eq!(result.total_dirs, 7);

        let exts: Vec<&str> = result.groups.iter().map(|g| g.extension.as_str()).collect();
        assert!(exts.contains(&"txt"));
        assert!(exts.contains(&"jpg"));
        assert!(exts.contains(&"md"));
        assert!(exts.contains(&""));

        let txt = result.groups.iter().find(|g| g.extension == "txt").unwrap();
        assert_eq!(txt.count, 2);

        let mut empty: Vec<&str> = result.empty_dirs.iter().map(|s| s.as_str()).collect();
        empty.sort();
        assert_eq!(
            empty,
            vec!["a/b/empty1", "a/empty2/deep"]
        );
    }

    #[test]
    fn cancel_stops_scan() {
        let base = std::env::temp_dir().join(format!("fl_cancel_{}", std::process::id()));
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).unwrap();
        for i in 0..2000 {
            fs::write(base.join(format!("f{i}.txt")), "x").unwrap();
        }

        let cancel = Arc::new(AtomicBool::new(true));
        let result = scan_folder(base.to_str().unwrap(), Some(cancel), None);

        fs::remove_dir_all(&base).ok();
        assert!(result.cancelled);
        assert!(result.total_files < 2000);
    }

    #[test]
    fn not_found_returns_error() {
        let result = scan_folder("Z:\\__no_such_dir__", None, None);
        assert!(!result.scan_errors.is_empty());
        assert_eq!(result.total_files, 0);
    }

    #[test]
    fn progress_callback_fires() {
        let base = std::env::temp_dir().join(format!("fl_prog_{}", std::process::id()));
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).unwrap();
        for i in 0..1200 {
            fs::write(base.join(format!("f{i}.dat")), "x").unwrap();
        }

        let calls = Arc::new(AtomicUsize::new(0));
        let calls_cb = calls.clone();
        let result = scan_folder(
            base.to_str().unwrap(),
            None,
            Some(Box::new(move |_| {
                calls_cb.fetch_add(1, AtomicOrder::Relaxed);
            })),
        );

        fs::remove_dir_all(&base).ok();
        assert!(calls.load(AtomicOrder::Relaxed) >= 2);
        assert_eq!(result.total_files, 1200);
    }
}
