use anyhow::Result;
use petgraph::graph::DiGraph;
use rusqlite::params;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Add an edge to the graph in SQLite
pub fn add_edge(source: &str, target: &str, kind: &str) -> Result<()> {
    let conn = crate::database::open_database()?;
    conn.execute(
        "INSERT OR IGNORE INTO graph_edges (source, target, kind) VALUES (?1, ?2, ?3)",
        params![source, target, kind],
    )?;
    Ok(())
}

/// Parse dependencies from known file types
pub fn extract_dependencies(path: &Path) -> Result<()> {
    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let parent = path.parent().unwrap_or_else(|| Path::new("")).to_string_lossy();

    match file_name {
        "Cargo.toml" => {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(parsed) = content.parse::<toml::Value>() {
                    if let Some(deps) = parsed.get("dependencies").and_then(|d| d.as_table()) {
                        for (dep_name, _) in deps {
                            let _ = add_edge(&parent, dep_name, "rust_dependency");
                        }
                    }
                    if let Some(deps) = parsed.get("dev-dependencies").and_then(|d| d.as_table()) {
                        for (dep_name, _) in deps {
                            let _ = add_edge(&parent, dep_name, "rust_dev_dependency");
                        }
                    }
                }
            }
        }
        "package.json" => {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(deps) = json.get("dependencies").and_then(|d| d.as_object()) {
                        for (dep_name, _) in deps {
                            let _ = add_edge(&parent, dep_name, "js_dependency");
                        }
                    }
                    if let Some(deps) = json.get("devDependencies").and_then(|d| d.as_object()) {
                        for (dep_name, _) in deps {
                            let _ = add_edge(&parent, dep_name, "js_dev_dependency");
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

pub struct GraphData {
    pub graph: DiGraph<String, String>,
}

/// Reconstruct the full directed graph from the database
pub fn build_graph() -> Result<GraphData> {
    let conn = crate::database::open_database()?;
    let mut stmt = conn.prepare("SELECT source, target, kind FROM graph_edges")?;
    let edges = stmt.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, String>(2)?,
        ))
    })?;

    let mut graph = DiGraph::new();
    let mut nodes = HashMap::new();

    for edge in edges.filter_map(Result::ok) {
        let (src, tgt, kind) = edge;
        
        let src_idx = *nodes.entry(src.clone()).or_insert_with(|| graph.add_node(src));
        let tgt_idx = *nodes.entry(tgt.clone()).or_insert_with(|| graph.add_node(tgt));
        
        graph.add_edge(src_idx, tgt_idx, kind);
    }
    Ok(GraphData { graph })
}

/// Convert the graph to Mermaid JS markdown
pub fn to_mermaid(gd: &GraphData) -> String {
    let mut out = String::from("```mermaid\ngraph TD;\n");
    for edge in gd.graph.edge_indices() {
        if let Some((src_idx, tgt_idx)) = gd.graph.edge_endpoints(edge) {
            let src = &gd.graph[src_idx];
            let tgt = &gd.graph[tgt_idx];
            let kind = &gd.graph[edge];
            
            let s = src.replace("-", "_").replace(".", "_").replace("/", "_");
            let t = tgt.replace("-", "_").replace(".", "_").replace("/", "_");
            let lbl = kind.replace("_", " ");
            
            out.push_str(&format!("    {s}[\"{src}\"] -->|\"{lbl}\"| {t}[\"{tgt}\"];\n"));
        }
    }
    out.push_str("```\n");
    out
}
