pub enum TriState {
    F,
    T,
    X,
}

struct Implicant {
    terms: Vec<TriState>,
}
