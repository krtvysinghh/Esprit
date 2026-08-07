use anyhow::Result;

pub fn rebuild_search_index() -> Result<()> {
    println!("Rebuilding search index...");
    Ok(())
}

pub fn search(query: &str) -> Result<Vec<String>> {
    println!("Search: {query}");
    Ok(Vec::new())
}
