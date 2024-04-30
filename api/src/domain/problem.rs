pub struct Problem {
    id: String,
    name: String,
}

impl Problem {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    pub fn reconstruct(id: String, name: String) -> Self {
        Self { id, name }
    }
}
