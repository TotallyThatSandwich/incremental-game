use ratatui::text::{Span, Line};
use ratatui::widgets::ListState;
use crate::{Tab, Buyable, UpgradeType, CostType};

pub struct GameState {
    pub gold: f64,

    pub clicker: f64,
    pub gps: f64,
    pub altered_gps: f64,
    pub tabs: Vec<Tab>,
    pub active_tab: u16,
    pub unlock_tabs: u16,
    pub cursor: ListState,
    pub upgrades: Vec<Buyable>,
    
}

impl Default for GameState {
    fn default() -> Self {

        let buyables = vec![
            Buyable::new(0, "Clicker: Produces Gold For You", 001, 10.0, 1.15, 0.0, CostType::Gold, UpgradeType::GoldClicker),
            Buyable::new(1, "Click Multiplier: clickers produce 1.25x as much gold", 001, 100.0, 2.0, 0.0, CostType::Gold, UpgradeType::GoldMultiplier),
            Buyable::new(2, "Clickers Muliply: Clickers are more effective the more you own", 001, 10000.0, 2.0, 5.0, CostType::Gold, UpgradeType::ClickersMultiplyGold),
        ];

        GameState {
            tabs: vec![Tab::new("Gold Shop", 001)],
            active_tab: 001,
            unlock_tabs: 001,
            gold: 0.0,
            gps: 1.0,
            altered_gps: 1.0,
            clicker: 1.0,
            cursor: ListState::default(),
            upgrades: buyables
        }
    }
}

impl GameState {

    pub fn update(&mut self) {
        let multiplier = self.upgrades.iter_mut().find(|x| x.tab == 001 && x.id == 2).unwrap().owned; 

        self.altered_gps = self.gps + (self.clicker * multiplier * 0.3);
    }

    pub fn new_upgrade(&mut self, id: u16, name: &str, tab: u16, cost: f64, cost_multiplier: f64, max: f64, cost_type: CostType, buyable_type: UpgradeType) {
        self.upgrades.push(Buyable::new(id, name, tab, cost, cost_multiplier, max, cost_type, buyable_type));
    }

    pub fn change_gold(&mut self, amount: f64) {
        self.gold += amount;
        if self.gold < 0.0 as f64 {
            self.gold = 0.0 as f64;
        }
    }

    pub fn reset_gold(&mut self) {
        self.gold = 0.0;
    }

    pub fn next_tab(&mut self) {
        if self.active_tab >= self.unlock_tabs {
            return;
        }
        self.active_tab += 1;
    }

    pub fn prev_tab(&mut self) {
        if self.active_tab <= 1 {
            return;
        }

        self.active_tab -= 1;
    }

    pub fn select_next(&mut self) {
        self.cursor.select_next();
    }

    pub fn select_prev(&mut self) {
        self.cursor.select_previous();
    }

    pub fn buy_upgrade(&mut self) {
        if self.cursor.selected().is_some() {
            let selected_upgrade = self.upgrades.iter_mut().find(|x| x.id == self.cursor.selected().unwrap() as u16 && x.tab == self.active_tab).unwrap(); 

            match &selected_upgrade.cost_type {
                CostType::Gold => {
                    if &self.gold >= &selected_upgrade.cost {
                        if &selected_upgrade.owned >= &selected_upgrade.max && selected_upgrade.max > 0.0 as f64 {
                            return;
                        }
                        self.gold -= &selected_upgrade.cost;
                        match &selected_upgrade.buyable_type {
                            UpgradeType::GoldClicker => self.clicker += 1.0,
                            UpgradeType::GoldMultiplier => self.gps *= 1.25,
                            UpgradeType::ClickersMultiplyGold => return,
                        }
                        selected_upgrade.buy();
                    }
                }
            }
        }
    }

    pub fn buy_max(&mut self) {
        if self.cursor.selected().is_some() {
            let selected_upgrade = self.upgrades.iter_mut().find(|x| x.id == self.cursor.selected().unwrap() as u16 && x.tab == self.active_tab).unwrap(); 
            
            match &selected_upgrade.cost_type {
                CostType::Gold => {
                    while &self.gold >= &selected_upgrade.cost {
                        if &selected_upgrade.owned >= &selected_upgrade.max && selected_upgrade.max > 0.0 as f64 {
                            return;
                        }
                        self.gold -= &selected_upgrade.cost;
                        match &selected_upgrade.buyable_type {
                            UpgradeType::GoldClicker => self.clicker += 1.0,
                            UpgradeType::GoldMultiplier => self.gps *= 1.25,
                            UpgradeType::ClickersMultiplyGold => {},
                        }
                        selected_upgrade.buy();
                    }
                }
            }
        }
    }
}