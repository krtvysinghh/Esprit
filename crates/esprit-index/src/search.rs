use crate::{all_files, content::extract, schema::build};
use anyhow::Result;
use directories::ProjectDirs;
use std::{fs, path::PathBuf};
use tantivy::{collector::TopDocs, doc, query::QueryParser, schema::Value, Index, TantivyDocument};

fn index_dir() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow::anyhow!("unable to determine data directory"))?;

    let dir = dirs.data_dir().join("tantivy");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn rebuild_search_index() -> Result<()> {
    let dir = index_dir()?;

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

    println!("Indexed {} files.", files.len());

    Ok(())
}

pub fn search(query: &str) -> Result<Vec<String>> {
    let index = Index::open_in_dir(index_dir()?)?;

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

        {
            if let Some(v) = doc.get_first(path) {
                if let Some(s) = v.as_str() {
                    out.push(s.to_string());
                }
            }
        }
    }

    Ok(out)
}
