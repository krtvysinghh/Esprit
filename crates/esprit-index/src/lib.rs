mod content;
mod database;
mod indexer;
mod model;
mod query;
mod schema;
mod search;

pub use database::{delete_file, insert_file, rename_file, update_file, IndexDatabase};
pub use indexer::index;
pub use model::{FileRelation, IndexHealth, IndexedFile, SearchFilters, SearchResult};
pub use query::{all_files, file_relations, files_in_workspace, index_health, recover_index};
pub use search::{
    cached_search, filtered_search, intelligent_search, ranked_search, rebuild_search_index,
    remove_search_document, search, search_with_metadata, semantic_search, sync_search_delete,
    sync_search_insert, sync_search_rename, update_search_document,
};

pub fn verify_database_integrity() -> anyhow::Result<()> {
    IndexDatabase::open()?.verify_integrity()
}
