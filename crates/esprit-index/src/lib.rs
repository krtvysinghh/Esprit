#![warn(missing_debug_implementations)]
#![forbid(unsafe_code)]
pub mod workspace;

pub mod graph;

mod content;
mod database;
mod indexer;
mod model;
mod query;
mod schema;
mod search;

pub use database::{delete_file, insert_file, rename_file, update_file};
pub use indexer::index;
pub use model::IndexedFile;
pub use query::{all_files, index_stats, IndexStats};
pub use search::{rebuild_search_index, search, search_all_workspaces};
