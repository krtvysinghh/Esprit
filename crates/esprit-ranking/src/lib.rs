use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Ranked<T> {
    pub score: f32,
    pub item: T,
}

pub fn sort<T>(mut items: Vec<Ranked<T>>) -> Vec<Ranked<T>> {
    items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
    items
}
