use crate::all_files;
use anyhow::Result;
use directories::ProjectDirs;
use std::fs;
use tantivy::{
    Index,
    collector::TopDocs,
    doc,
    query::QueryParser,
    schema::{STORED, Schema, TEXT, document::Value},
};

fn index_dir() -> Result<std::path::PathBuf> {
    let dirs = ProjectDirs::from("dev", "esprit", "esprit")
        .ok_or_else(|| anyhow::anyhow!("unable to determine data directory"))?;

    Ok(dirs.data_dir().join("tantivy"))
}

pub fn rebuild_search_index() -> Result<()> {
    let dir = index_dir()?;

    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }

    fs::create_dir_all(&dir)?;

    let mut builder = Schema::builder();

    let path = builder.add_text_field("path", TEXT | STORED);
    let name = builder.add_text_field("name", TEXT | STORED);

    let schema = builder.build();

    let index = Index::create_in_dir(&dir, schema.clone())?;

    let mut writer = index.writer(50_000_000)?;

    let files = all_files()?;

    for file in &files {
        let filename = file.path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        writer.add_document(doc!(
            path => file.path.to_string_lossy().to_string(),
            name => filename.to_string(),
        ))?;
    }

    writer.commit()?;

    println!("Indexed {} files.", files.len());

    Ok(())
}

pub fn search(query: &str) -> Result<Vec<String>> {
    let index = Index::open_in_dir(index_dir()?)?;

    let reader = index.reader()?;
    let searcher = reader.searcher();

    let schema = index.schema();

    let path = schema.get_field("path")?;
    let name = schema.get_field("name")?;

    let parser = QueryParser::for_index(&index, vec![name]);

    let query = parser.parse_query(query)?;

    let docs = searcher.search(&query, &TopDocs::with_limit(100))?;

    let mut results = Vec::new();

    for (_, addr) in docs {
        let doc = searcher.doc::<tantivy::TantivyDocument>(addr)?;

        if let Some(value) = doc.get_first(path) {
            if let Some(text) = value.as_str() {
                results.push(text.to_string());
            }
        }
    }

    Ok(results)
}
