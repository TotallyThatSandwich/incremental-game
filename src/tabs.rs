pub struct Tab {
    pub title: String,
    pub index: u16,
}

impl Tab {
    pub fn new(title: &str, index: u16) -> Self {
        Tab {
            title: title.to_string(),
            index,
        }
    }
}