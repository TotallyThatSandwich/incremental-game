pub enum UpgradeType {
    BytesClicker,
    BytesMultiplier,
    ClickersMultiplyBytes,
    UnlockCore,
    CoreClicker,
}

pub enum CostType {
    Bytes,
    Cores,
}

pub struct Buyable {
    pub id: u16,
    pub name: String,
    pub tab: u16,
    pub cost: f64,
    pub cost_type: CostType,
    cost_multiplier: f64,
    pub buyable_type: UpgradeType,
    pub max: f64,
    pub owned: f64,
}

impl Buyable {
    pub fn new(id: u16, name: &str, tab: u16, cost: f64, cost_multiplier: f64, max: f64, cost_type: CostType, buyable_type: UpgradeType) -> Self {
        Buyable {
            id,
            name: name.to_string(),
            tab,
            cost,
            cost_type,
            max,
            owned: 0.0,
            cost_multiplier,
            buyable_type,
        }
        

    }

    pub fn buy(&mut self) {
        self.cost = self.cost * self.cost_multiplier;
        self.owned += 1.0;
    }

    pub fn is_max(&mut self) -> bool {
        if self.owned == self.max && self.max != 0.0{
            return true;
        } else {
            return false;
        }


    }
}