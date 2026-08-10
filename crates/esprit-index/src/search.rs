use crate::{all_files, content::extract, schema::build};
use anyhow::Result;
use directories::ProjectDirs;
use std::{fs, path::PathBuf};
use tantivy::{
    collector::TopDocs,
    doc,
    query::{BooleanQuery, FuzzyTermQuery, Occur, Query, QueryParser},
    schema::Value,
    Index, TantivyDocument,
};

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

pub fn semantic_search(query: &str) -> Result<Vec<String>> {
    let query = query.trim();

    if query.is_empty() {
        return Ok(Vec::new());
    }

    let index = Index::open_in_dir(index_dir()?)?;
    let schema = index.schema();

    let path = schema.get_field("path")?;
    let name = schema.get_field("name")?;
    let content = schema.get_field("content")?;

    let reader = index.reader()?;
    let searcher = reader.searcher();

    let parser = QueryParser::for_index(&index, vec![name, content]);
    let exact = parser.parse_query(query)?;

    let mut fuzzy_queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();

    for token in query.split_whitespace() {
        let token = token.trim_matches(|c: char| !c.is_alphanumeric());

        if token.len() < 3 {
            continue;
        }

        let term = tantivy::Term::from_field_text(content, token);
        fuzzy_queries.push((Occur::Should, Box::new(FuzzyTermQuery::new(term, 1, true))));

        let term = tantivy::Term::from_field_text(name, token);
        fuzzy_queries.push((Occur::Should, Box::new(FuzzyTermQuery::new(term, 1, true))));
    }

    let semantic_query: Box<dyn Query> = if fuzzy_queries.is_empty() {
        exact
    } else {
        Box::new(BooleanQuery::new(vec![
            (Occur::Must, exact),
            (Occur::Should, Box::new(BooleanQuery::new(fuzzy_queries))),
        ]))
    };

    let docs = searcher.search(&semantic_query, &TopDocs::with_limit(100))?;

    let mut ranked = Vec::with_capacity(docs.len());

    for (score, addr) in docs {
        let doc: TantivyDocument = searcher.doc(addr)?;

        if let Some(value) = doc.get_first(path) {
            if let Some(path) = value.as_str() {
                let name_match = doc
                    .get_first(name)
                    .and_then(|v| v.as_str())
                    .map(|v| {
                        query
                            .split_whitespace()
                            .filter(|term| v.to_lowercase().contains(&term.to_lowercase()))
                            .count()
                    })
                    .unwrap_or(0);

                let content_match = doc
                    .get_first(content)
                    .and_then(|v| v.as_str())
                    .map(|v| {
                        query
                            .split_whitespace()
                            .filter(|term| v.to_lowercase().contains(&term.to_lowercase()))
                            .count()
                    })
                    .unwrap_or(0);

                let hybrid_score =
                    score + (name_match as f32 * 2.0) + (content_match as f32 * 0.25);

                ranked.push((hybrid_score, path.to_string()));
            }
        }
    }

    ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    Ok(ranked.into_iter().map(|(_, path)| path).collect())
}

pub fn sync_search_insert(path: impl AsRef<std::path::Path>) -> Result<()> {
    let path = path.as_ref();

    if !path.is_file() {
        return Ok(());
    }

    let dir = index_dir()?;
    let (schema, fields) = build();

    let index = if dir.join("meta.json").exists() {
        Index::open_in_dir(&dir)?
    } else {
        fs::create_dir_all(&dir)?;
        Index::create_in_dir(&dir, schema)?
    };

    let mut writer = index.writer::<TantivyDocument>(10_000_000)?;

    writer.delete_term(tantivy::Term::from_field_text(
        fields.path,
        &path.to_string_lossy(),
    ));

    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    writer.add_document(doc!(
        fields.path => path.to_string_lossy().to_string(),
        fields.name => filename.to_string(),
        fields.content => extract(path),
    ))?;

    writer.commit()?;
    Ok(())
}

pub fn sync_search_delete(path: impl AsRef<std::path::Path>) -> Result<()> {
    let dir = index_dir()?;

    if !dir.join("meta.json").exists() {
        return Ok(());
    }

    let index = Index::open_in_dir(&dir)?;
    let path_field = index.schema().get_field("path")?;
    let mut writer = index.writer::<TantivyDocument>(10_000_000)?;

    writer.delete_term(tantivy::Term::from_field_text(
        path_field,
        &path.as_ref().to_string_lossy(),
    ));

    writer.commit()?;
    Ok(())
}

pub fn sync_search_rename(
    old: impl AsRef<std::path::Path>,
    new: impl AsRef<std::path::Path>,
) -> Result<()> {
    sync_search_delete(old)?;
    sync_search_insert(new)?;
    Ok(())
}

pub fn ranked_search(query: &str, limit: usize) -> Result<Vec<crate::SearchResult>> {
    let query = query.trim();

    if query.is_empty() || limit == 0 {
        return Ok(Vec::new());
    }

    let index = Index::open_in_dir(index_dir()?)?;
    let schema = index.schema();

    let path = schema.get_field("path")?;
    let name = schema.get_field("name")?;
    let content = schema.get_field("content")?;

    let reader = index.reader()?;
    let searcher = reader.searcher();

    let parser = QueryParser::for_index(&index, vec![name, content]);
    let parsed = parser.parse_query(query)?;

    let docs = searcher.search(&parsed, &TopDocs::with_limit(limit.min(1000)))?;
    let mut results = Vec::with_capacity(docs.len());

    for (score, addr) in docs {
        let doc: TantivyDocument = searcher.doc(addr)?;

        if let Some(value) = doc.get_first(path) {
            if let Some(path) = value.as_str() {
                results.push(crate::SearchResult {
                    path: std::path::PathBuf::from(path),
                    score,
                });
            }
        }
    }

    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(results)
}

pub fn update_search_document(path: impl AsRef<std::path::Path>) -> Result<()> {
    use tantivy::IndexWriter;

    let path = path.as_ref();

    if !path.is_file() {
        return remove_search_document(path);
    }

    let index = Index::open_in_dir(index_dir()?)?;
    let schema = index.schema();

    let path_field = schema.get_field("path")?;
    let name_field = schema.get_field("name")?;
    let content_field = schema.get_field("content")?;

    let mut writer: IndexWriter<TantivyDocument> = index.writer(10_000_000)?;

    let path_string = path.to_string_lossy().to_string();
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    writer.delete_term(tantivy::Term::from_field_text(path_field, &path_string));

    writer.add_document(doc!(
        path_field => path_string,
        name_field => filename.to_string(),
        content_field => extract(path),
    ))?;

    writer.commit()?;

    Ok(())
}

pub fn remove_search_document(path: impl AsRef<std::path::Path>) -> Result<()> {
    use tantivy::IndexWriter;

    let index = Index::open_in_dir(index_dir()?)?;
    let schema = index.schema();
    let path_field = schema.get_field("path")?;

    let mut writer: IndexWriter<TantivyDocument> = index.writer(10_000_000)?;
    let path_string = path.as_ref().to_string_lossy().to_string();

    writer.delete_term(tantivy::Term::from_field_text(path_field, &path_string));
    writer.commit()?;

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
