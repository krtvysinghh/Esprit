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
                    snippet: None,
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

pub fn search_with_metadata(query: &str, limit: usize) -> Result<Vec<crate::SearchResult>> {
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

        let Some(path_value) = doc.get_first(path) else {
            continue;
        };

        let Some(path_string) = path_value.as_str() else {
            continue;
        };

        let snippet = doc
            .get_first(content)
            .and_then(|value| value.as_str())
            .map(|text| {
                let text = text.trim();
                let max = 240usize;
                if text.len() <= max {
                    text.to_string()
                } else {
                    format!("{}…", &text[..max])
                }
            });

        results.push(crate::SearchResult {
            path: std::path::PathBuf::from(path_string),
            score,
            snippet,
        });
    }

    Ok(results)
}

pub fn filtered_search(
    query: &str,
    filters: &crate::SearchFilters,
    limit: usize,
) -> Result<Vec<crate::SearchResult>> {
    let mut results = search_with_metadata(query, limit.min(1000))?;

    results.retain(|result| {
        let path = &result.path;

        if let Some(extension) = &filters.extension {
            let wanted = extension.trim_start_matches('.').to_ascii_lowercase();
            let actual = path
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();

            if actual != wanted {
                return false;
            }
        }

        if let Some(fragment) = &filters.path_contains {
            if !path
                .to_string_lossy()
                .to_ascii_lowercase()
                .contains(&fragment.to_ascii_lowercase())
            {
                return false;
            }
        }

        let Ok(metadata) = std::fs::metadata(path) else {
            return false;
        };

        let size = metadata.len();

        if let Some(min_size) = filters.min_size {
            if size < min_size {
                return false;
            }
        }

        if let Some(max_size) = filters.max_size {
            if size > max_size {
                return false;
            }
        }

        let modified = metadata.modified().ok();

        if let Some(after) = filters.modified_after {
            if modified.map(|value| value <= after).unwrap_or(true) {
                return false;
            }
        }

        if let Some(before) = filters.modified_before {
            if modified.map(|value| value >= before).unwrap_or(true) {
                return false;
            }
        }

        true
    });

    results.truncate(limit.min(1000));
    Ok(results)
}

struct SearchCache {
    entries: std::sync::Mutex<
        std::collections::HashMap<String, (std::time::Instant, Vec<crate::SearchResult>)>,
    >,
}

impl SearchCache {
    fn new() -> Self {
        Self {
            entries: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    fn get(&self, key: &str) -> Result<Option<Vec<crate::SearchResult>>> {
        let cache = self
            .entries
            .lock()
            .map_err(|_| anyhow::anyhow!("search cache lock poisoned"))?;

        Ok(cache.get(key).and_then(|(created, results)| {
            if created.elapsed() < std::time::Duration::from_secs(30) {
                Some(results.clone())
            } else {
                None
            }
        }))
    }

    fn insert(&self, key: String, results: Vec<crate::SearchResult>) -> Result<()> {
        let mut cache = self
            .entries
            .lock()
            .map_err(|_| anyhow::anyhow!("search cache lock poisoned"))?;

        if cache.len() >= 128 {
            if let Some(oldest) = cache
                .iter()
                .min_by_key(|(_, (created, _))| *created)
                .map(|(key, _)| key.clone())
            {
                cache.remove(&oldest);
            }
        }

        cache.insert(key, (std::time::Instant::now(), results));
        Ok(())
    }
}

static SEARCH_CACHE: std::sync::OnceLock<SearchCache> = std::sync::OnceLock::new();

pub fn cached_search(query: &str, limit: usize) -> Result<Vec<crate::SearchResult>> {
    let query = query.trim();

    if query.is_empty() {
        return Ok(Vec::new());
    }

    let limit = limit.min(1000);
    let key = format!("{query}:{limit}");

    let cache = SEARCH_CACHE.get_or_init(SearchCache::new);

    if let Some(results) = cache.get(&key)? {
        return Ok(results);
    }

    let results = search_with_metadata(query, limit)?;
    cache.insert(key, results.clone())?;

    Ok(results)
}

pub fn intelligent_search(query: &str, limit: usize) -> Result<Vec<crate::SearchResult>> {
    let query = query.trim();

    if query.is_empty() {
        return Ok(Vec::new());
    }

    let limit = limit.clamp(1, 1000);

    let mut results = cached_search(query, limit)?;

    if results.is_empty() {
        results = ranked_search(query, limit)?;
    }

    if results.len() > limit {
        results.truncate(limit);
    }

    Ok(results)
}

pub fn rebuild_search_index_for_workspace(root: impl AsRef<std::path::Path>) -> Result<()> {
    let root = root.as_ref().canonicalize()?;
    let dir = index_dir()?.join("workspace");

    std::fs::create_dir_all(&dir)?;

    let workspace_key = {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        root.to_string_lossy().hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    };

    let workspace_dir = dir.join(workspace_key);

    if workspace_dir.exists() {
        std::fs::remove_dir_all(&workspace_dir)?;
    }

    std::fs::create_dir_all(&workspace_dir)?;

    let (schema, fields) = build();
    let index = Index::create_in_dir(&workspace_dir, schema)?;
    let mut writer: tantivy::IndexWriter<TantivyDocument> = index.writer(50_000_000)?;

    for entry in walkdir::WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();

        if !entry.file_type().is_file() {
            continue;
        }

        if std::fs::File::open(path).is_err() {
            continue;
        }

        let content = extract(path);

        let filename = path.file_name().and_then(|v| v.to_str()).unwrap_or("");

        writer.add_document(doc!(
            fields.path => path.to_string_lossy().to_string(),
            fields.name => filename.to_string(),
            fields.content => content,
        ))?;
    }

    writer.commit()?;

    Ok(())
}

pub fn workspace_search(
    root: impl AsRef<std::path::Path>,
    query: &str,
    limit: usize,
) -> Result<Vec<crate::SearchResult>> {
    let root = root.as_ref().canonicalize()?;

    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let dir = index_dir()?.join("workspace");

    let workspace_key = {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        root.to_string_lossy().hash(&mut hasher);
        format!("{:016x}", hasher.finish())
    };

    let workspace_dir = dir.join(workspace_key);
    let index = Index::open_in_dir(workspace_dir)?;

    let schema = index.schema();
    let path_field = schema.get_field("path")?;
    let name_field = schema.get_field("name")?;
    let content_field = schema.get_field("content")?;

    let reader = index.reader()?;
    let searcher = reader.searcher();

    let parser = QueryParser::for_index(&index, vec![name_field, content_field]);

    let parsed = parser.parse_query(query)?;

    let docs = searcher.search(&parsed, &TopDocs::with_limit(limit.clamp(1, 1000)))?;

    let mut results = Vec::with_capacity(docs.len());

    for (score, addr) in docs {
        let document: TantivyDocument = searcher.doc(addr)?;

        let Some(value) = document.get_first(path_field) else {
            continue;
        };

        let Some(path_text) = value.as_str() else {
            continue;
        };

        let path = std::path::PathBuf::from(path_text);

        let Ok(canonical_path) = path.canonicalize() else {
            continue;
        };

        if canonical_path.starts_with(&root) {
            results.push(crate::SearchResult {
                path: canonical_path,
                score,
                snippet: None,
            });
        }
    }

    Ok(results)
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
