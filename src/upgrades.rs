pub struct Upgrade {
    pub name: String,
    pub cost: i64,
}

impl Upgrade {
    pub fn new(name: &str, cost: i64) -> Self {
        Upgrade {
            name: name.to_string(),
            cost: cost,
        }
    }
}