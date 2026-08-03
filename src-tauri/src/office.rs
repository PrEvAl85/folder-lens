use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::Cursor;
use std::io::Read;

const MAX_OFFICE_BYTES: u64 = 20 * 1024 * 1024;

pub fn extract_docx(bytes: &[u8]) -> Result<String, String> {
    let xml = read_zip_entry(bytes, "word/document.xml")
        .ok_or_else(|| "не удалось прочитать document.xml")?;
    let text = xml_to_text(
        &xml,
        &[b"t".as_slice()][..],
        &[b"p".as_slice()][..],
        &[][..],
    );
    Ok(clean(&text))
}

pub fn extract_pptx(bytes: &[u8]) -> Result<String, String> {
    let archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|e| format!("не удалось открыть архив: {e}"))?;
    let mut names: Vec<String> = archive
        .file_names()
        .filter(|n| n.starts_with("ppt/slides/slide") && n.ends_with(".xml"))
        .map(|s| s.to_string())
        .collect();
    names.sort();

    let mut out = String::new();
    for (i, name) in names.iter().enumerate() {
        let xml = read_zip_entry(bytes, name).unwrap_or_default();
        let text = xml_to_text(&xml, &[b"t".as_slice()][..], &[b"p".as_slice()][..], &[b"sld".as_slice()][..]);
        if !text.trim().is_empty() {
            out.push_str(&format!("--- Slide {} ---\n", i + 1));
            out.push_str(&text);
            out.push('\n');
        }
    }
    Ok(clean(&out))
}

pub fn extract_xlsx(bytes: &[u8]) -> Result<String, String> {
    let archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|e| format!("не удалось открыть архив: {e}"))?;

    let shared = if let Some(xml) = read_zip_entry(bytes, "xl/sharedStrings.xml") {
        xml_to_text(&xml, &[b"t".as_slice()][..], &[b"si".as_slice()][..], &[][..])
    } else {
        String::new()
    };
    let shared: Vec<&str> = shared.lines().collect();

    let mut names: Vec<String> = archive
        .file_names()
        .filter(|n| n.starts_with("xl/worksheets/sheet") && n.ends_with(".xml"))
        .map(|s| s.to_string())
        .collect();
    names.sort();

    let mut out = String::new();
    for name in names {
        let xml = read_zip_entry(bytes, &name).unwrap_or_default();
        out.push_str(&xlsx_sheet_text(&xml, &shared));
    }
    Ok(clean(&out))
}

pub fn extract_office(path: &str, ext: &str) -> Result<Option<String>, String> {
    if !matches!(ext, "docx" | "pptx" | "xlsx") {
        return Ok(None);
    }
    let meta = std::fs::metadata(path).map_err(|e| format!("не удалось открыть файл: {e}"))?;
    if meta.len() > MAX_OFFICE_BYTES {
        return Err("Файл слишком большой для предпросмотра".into());
    }
    let bytes = std::fs::read(path).map_err(|e| format!("не удалось прочитать файл: {e}"))?;
    let text = match ext {
        "docx" => extract_docx(&bytes)?,
        "pptx" => extract_pptx(&bytes)?,
        "xlsx" => extract_xlsx(&bytes)?,
        _ => return Ok(None),
    };
    Ok(Some(text))
}

fn read_zip_entry(bytes: &[u8], name: &str) -> Option<String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).ok()?;
    let mut f = archive.by_name(name).ok()?;
    let mut s = String::new();
    f.read_to_string(&mut s).ok()?;
    Some(s)
}

fn xml_to_text(xml: &str, text_tags: &[&[u8]], para_tags: &[&[u8]], block_tags: &[&[u8]]) -> String {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut out = String::new();
    let mut in_text = false;
    let mut buf = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                if text_tags.iter().any(|t| *t == e.local_name().as_ref()) {
                    in_text = true;
                    buf.clear();
                }
            }
            Ok(Event::Empty(e)) => {
                if para_tags.iter().any(|t| *t == e.local_name().as_ref()) {
                    out.push('\n');
                }
            }
            Ok(Event::Text(t)) => {
                if in_text {
                    if let Ok(s) = t.unescape() {
                        buf.push_str(&s);
                    }
                }
            }
            Ok(Event::End(e)) => {
                let name = e.local_name();
                if text_tags.iter().any(|t| *t == name.as_ref()) {
                    in_text = false;
                    out.push_str(buf.trim());
                    out.push(' ');
                } else if para_tags.iter().any(|t| *t == name.as_ref()) {
                    out.push('\n');
                } else if block_tags.iter().any(|t| *t == name.as_ref()) {
                    out.push('\n');
                    out.push('\n');
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    out
}

fn xlsx_sheet_text(xml: &str, shared: &[&str]) -> String {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut out = String::new();
    let mut in_v = false;
    let mut in_inline_t = false;
    let mut cell_type = String::new();
    let mut cell_value = String::new();
    let mut inline = String::new();
    let mut in_row = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                match e.local_name().as_ref() {
                    b"c" => {
                        cell_type = e
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"t")
                            .and_then(|a| a.decode_and_unescape_value(reader.decoder()).ok())
                            .unwrap_or_default()
                            .to_string();
                        cell_value.clear();
                        inline.clear();
                    }
                    b"v" => in_v = true,
                    b"is" | b"t" => in_inline_t = true,
                    b"row" => in_row = true,
                    _ => {}
                }
            }
            Ok(Event::Text(t)) => {
                if in_v {
                    if let Ok(s) = t.unescape() {
                        cell_value.push_str(&s);
                    }
                } else if in_inline_t {
                    if let Ok(s) = t.unescape() {
                        inline.push_str(&s);
                    }
                }
            }
            Ok(Event::Empty(e)) => {
                if e.local_name().as_ref() == b"br" && in_row {
                    cell_value.push('\n');
                }
            }
            Ok(Event::End(e)) => {
                match e.local_name().as_ref() {
                    b"v" => in_v = false,
                    b"t" | b"is" => in_inline_t = false,
                    b"c" => {
                        let val = if !inline.is_empty() {
                            inline.clone()
                        } else if cell_type == "s" {
                            cell_value
                                .trim()
                                .parse::<usize>()
                                .ok()
                                .and_then(|i| shared.get(i).copied())
                                .unwrap_or_default()
                                .to_string()
                        } else {
                            cell_value.clone()
                        };
                        if !val.trim().is_empty() {
                            out.push_str(val.trim());
                            out.push('\t');
                        }
                    }
                    b"row" => {
                        if in_row && out.ends_with('\t') {
                            out.push('\n');
                        }
                        in_row = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    out
}

fn clean(s: &str) -> String {
    let mut lines: Vec<&str> = s.lines().map(str::trim).collect();
    while lines.first().is_some_and(|l| l.is_empty()) {
        lines.remove(0);
    }
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    fn make_docx_bytes() -> Vec<u8> {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>Hello</w:t></w:r><w:r><w:t> world</w:t></w:r></w:p>
    <w:p><w:r><w:t>Second paragraph</w:t></w:r></w:p>
  </w:body>
</w:document>"#;
        let mut buf = Cursor::new(Vec::new());
        {
            let mut zw = zip::ZipWriter::new(&mut buf);
            zw.start_file("word/document.xml", SimpleFileOptions::default())
                .unwrap();
            zw.write_all(xml.as_bytes()).unwrap();
            zw.finish().unwrap();
        }
        buf.into_inner()
    }

    #[test]
    fn extract_docx_reads_paragraphs() {
        let bytes = make_docx_bytes();
        let text = extract_docx(&bytes).unwrap();
        assert!(text.contains("Hello world"));
        assert!(text.contains("Second paragraph"));
    }

    #[test]
    fn extract_docx_invalid_zip_errors() {
        assert!(extract_docx(b"PK\x03\x04garbage").is_err());
    }

    #[test]
    fn extract_office_unknown_ext_is_none() {
        assert!(extract_office("x", "doc").unwrap().is_none());
    }

    #[test]
    fn extract_office_missing_file_errors() {
        assert!(extract_office("nope", "docx").is_err());
    }

    #[test]
    fn xml_to_text_extracts_t_and_paragraphs() {
        let text = xml_to_text(
            r#"<w:p><w:t>A</w:t></w:p><w:p><w:t>B</w:t></w:p>"#,
            &[b"t".as_slice()][..],
            &[b"p".as_slice()][..],
            &[][..],
        );
        assert!(text.contains('A'));
        assert!(text.contains('B'));
    }
}
