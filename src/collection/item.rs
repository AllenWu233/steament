#[derive(Clone)]
// Single option
pub struct Item {
    pub content: String,
    pub checked: bool,
}

impl Item {
    pub fn new(content: String) -> Self {
        Item {
            content,
            checked: false,
        }
    }
}
