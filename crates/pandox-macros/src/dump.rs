use std::fs;
use std::path::{Path, PathBuf};

use crate::render::RenderedBlock;

pub fn dump_full_rsx(source: &Path, blocks: &[RenderedBlock]) {
    let dump_root = match std::env::var("PANDOX_DUMP_RSX") {
        Ok(path) if !path.trim().is_empty() => PathBuf::from(path),
        _ => return,
    };

    let mut body = String::from("rsx! {\n");
    let mut saw_content = false;

    for block in blocks.iter().filter_map(|b| b.rsx.as_deref()) {
        if block.trim().is_empty() {
            continue;
        }
        body.push_str(block);
        if !block.ends_with('\n') {
            body.push('\n');
        }
        body.push('\n');
        saw_content = true;
    }

    if !saw_content {
        return;
    }

    if fs::create_dir_all(&dump_root).is_err() {
        return;
    }

    body.push_str("}\n");

    let cleaned = source
        .strip_prefix(Path::new(crate::MODULES_ROOT))
        .unwrap_or(source);
    let label = sanitize_dump_label(cleaned);
    let file_name = format!("{label}.rsx");
    let out_path = dump_root.join(file_name);
    let _ = fs::write(out_path, body);
}

pub fn markdown_slug(args: &crate::args::MarkdownArgs, markdown_path: &Path) -> String {
    if let Some(slug) = &args.slug {
        return sanitize_slug(slug);
    }

    let mut candidate = markdown_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("content");

    if candidate == "index" {
        if let Some(parent) = markdown_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|name| name.to_str())
        {
            candidate = parent;
        }
    }

    sanitize_slug(candidate)
}

pub fn sanitize_slug(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        let mapped = match ch {
            'a'..='z' | '0'..='9' => Some(ch),
            'A'..='Z' => Some(ch.to_ascii_lowercase()),
            '-' | '_' => Some(ch),
            _ => None,
        };

        if let Some(ch) = mapped {
            out.push(ch);
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }

    let trimmed = out.trim_matches('_');
    if trimmed.is_empty() {
        "content".into()
    } else {
        trimmed.to_string()
    }
}

fn sanitize_dump_label(path: &Path) -> String {
    let raw = path.to_string_lossy();
    let mut buf = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '0'..='9' | 'a'..='z' | 'A'..='Z' => buf.push(ch),
            _ => buf.push('_'),
        }
    }
    if buf.is_empty() { "_".into() } else { buf }
}
