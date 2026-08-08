use tantivy::schema::{Field, Schema, SchemaBuilder, STORED, TEXT};

pub struct Fields {
    pub path: Field,
    pub name: Field,
    pub content: Field,
}

pub fn build() -> (Schema, Fields) {
    let mut builder = SchemaBuilder::default();

    let path = builder.add_text_field("path", TEXT | STORED);
    let name = builder.add_text_field("name", TEXT);
    let content = builder.add_text_field("content", TEXT);

    (builder.build(), Fields { path, name, content })
}
