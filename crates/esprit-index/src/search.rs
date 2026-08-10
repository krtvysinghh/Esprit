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
