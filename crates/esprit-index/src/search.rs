use crate::{all_files, content::extract, schema::build};
use anyhow::Result;
use std::{fs, path::PathBuf};
use tantivy::{collector::TopDocs, doc, query::QueryParser, schema::Value, Index, TantivyDocument};

pub fn index_dir(hash: &str) -> Result<PathBuf> {
    let dir = crate::workspace::workspace_dir(hash)?.join("tantivy");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn rebuild_search_index() -> Result<()> {
    let hash = crate::workspace::get_workspace_hash();
    let dir = index_dir(&hash)?;

    if dir.exists() {
        let _ = fs::remove_dir_all(&dir);
    }
    fs::create_dir_all(&dir)?;

    let (schema, fields) = build();
    let index = Index::create_in_dir(&dir, schema)?;
    let mut writer = index.writer(50_000_000)?;

    let files = all_files()?;
    for file in &files {
        let filename = file.path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        writer.add_document(doc!(
            fields.path => file.path.to_string_lossy().to_string(),
            fields.name => filename.to_string(),
            fields.content => extract(&file.path),
        ))?;
    }
    writer.commit()?;
    println!("Indexed {} files in workspace.", files.len());
    Ok(())
}

pub fn search(query: &str) -> Result<Vec<String>> {
    let hash = crate::workspace::get_workspace_hash();
    search_in_dir(&index_dir(&hash)?, query)
}

pub fn search_all_workspaces(query: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    if let Ok(workspaces) = crate::workspace::all_workspaces() {
        for w in workspaces {
            let dir = w.join("tantivy");
            if dir.exists() {
                if let Ok(mut results) = search_in_dir(&dir, query) {
                    out.append(&mut results);
                }
            }
        }
    }
    // Deduplicate
    out.sort();
    out.dedup();
    Ok(out)
}

fn search_in_dir(dir: &PathBuf, query: &str) -> Result<Vec<String>> {
    let index = Index::open_in_dir(dir)?;
    let schema = index.schema();
    let path = schema.get_field("path")?;
    let name = schema.get_field("name")?;
    let content = schema.get_field("content")?;

    let reader = index.reader()?;
    let searcher = reader.searcher();
    let parser = QueryParser::for_index(&index, vec![name, content]);
    let q = parser.parse_query(query)?;
    let docs = searcher.search(&q, &TopDocs::with_limit(100))?;

    let mut out = Vec::new();
    for (_, addr) in docs {
        let doc: TantivyDocument = searcher.doc(addr)?;
        if let Some(v) = doc.get_first(path) {
            if let Some(s) = v.as_str() {
                out.push(s.to_string());
            }
        }
    }
    Ok(out)
}
