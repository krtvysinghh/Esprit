use tantivy::schema::*;
use tantivy::Index;
use std::path::Path;

pub fn init_index(index_path: &Path) -> tantivy::Result<Index> {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("id", STRING | STORED);
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    
    if Index::exists(index_path).unwrap_or(false) {
        Index::open_in_dir(index_path)
    } else {
        Index::create_in_dir(index_path, schema)
    }
}
