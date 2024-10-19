use crate::Upgrade;

pub struct Tab {
    pub title: String,
    pub content: Vec<Upgrade>,
}

impl Tab {
    pub fn new(title: &str, content: Vec<Upgrade>) -> Self {
        Tab {
            title: title.to_string(),
            content,
        }
    }

    pub fn add_upgrade(&mut self, name: &str, cost: i64) {
        self.content.push(Upgrade::new(name, cost));
    }
}