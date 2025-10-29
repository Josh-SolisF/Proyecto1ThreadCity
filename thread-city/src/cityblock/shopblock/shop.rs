pub struct Shop {
    pub(crate) id: usize,
    pub(crate) name: String,
}

impl Shop {
    pub fn new(id: usize, name: String) -> Shop {
        Self {
            id,
            name
        }
    }
}