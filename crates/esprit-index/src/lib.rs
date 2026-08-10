mod content;
mod database;
mod indexer;
mod model;
mod query;
mod schema;
mod search;

pub use database::{delete_file, insert_file, rename_file, update_file, IndexDatabase};
pub use indexer::index;
pub use model::{IndexedFile, SearchResult};
pub use query::all_files;
pub use search::{
    ranked_search, rebuild_search_index, search, semantic_search, sync_search_delete,
    sync_search_insert, sync_search_rename,
};
