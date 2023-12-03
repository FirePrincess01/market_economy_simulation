//! vector that restructures its elements but never invalidates its indices

struct IndexedVector<T> {
    pub elements: Vec<T>,
    pub indices: usize,
}