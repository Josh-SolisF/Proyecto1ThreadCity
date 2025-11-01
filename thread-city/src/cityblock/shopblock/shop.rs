#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shop {
    pub(crate) name: String,
}

impl Shop {
    pub fn new(name: String) -> Shop {
        Self {
            name
        }
    }
}