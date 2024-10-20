use ratatui::text::{Span, Line};
use ratatui::widgets::ListState;
use crate::{Tab, Buyable, UpgradeType, CostType};

pub struct GameState {
    pub bytes: f64,

    pub clicker: f64,
    pub bpc: f64,
    pub tabs: Vec<Tab>,
    pub active_tab: u16,
    pub unlock_tabs: u16,
    pub cursor: ListState,
    pub upgrades: Vec<Buyable>,
    
}

impl Default for GameState {
    fn default() -> Self {

        let buyables = vec![
            Buyable::new(0, "Clicker: Produces Bytes For You", 001, 10.0, 1.15, 0.0, CostType::Bytes, UpgradeType::BytesClicker),
            Buyable::new(1, "Click Production Mulitplier: clickers produce 1.25x as many bytes", 001, 100.0, 2.0, 0.0, CostType::Bytes, UpgradeType::BytesMultiplier),
            Buyable::new(2, "Clickers Muliply: Clickers are more effective the more you own", 001, 10000.0, 2.0, 5.0, CostType::Bytes, UpgradeType::ClickersMultiplyBytes),
        ];

        GameState {
            tabs: vec![Tab::new("Byte Shop", 001)],
            active_tab: 001,
            unlock_tabs: 001,
            bytes: 0.0,
            bpc: 1.0,
            clicker: 1.0,
            cursor: ListState::default(),
            upgrades: buyables
        }
    }
}

impl GameState {

    pub fn new_upgrade(&mut self, id: u16, name: &str, tab: u16, cost: f64, cost_multiplier: f64, max: f64, cost_type: CostType, buyable_type: UpgradeType) {
        self.upgrades.push(Buyable::new(id, name, tab, cost, cost_multiplier, max, cost_type, buyable_type));
    }

    pub fn new_tab(&mut self, title: &str) {
        self.unlock_tabs += 1;
        self.tabs.push(Tab::new(title, self.unlock_tabs));
    }

    pub fn change_bytes(&mut self, amount: f64) {
        self.bytes += amount;
        if self.bytes < 0.0 as f64 {
            self.bytes = 0.0 as f64;
        }
    }

    pub fn reset_bytes(&mut self) {
        self.bytes = 0.0;
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
                CostType::Bytes => {
                    if &self.bytes >= &selected_upgrade.cost {
                        if &selected_upgrade.owned >= &selected_upgrade.max && selected_upgrade.max > 0.0 as f64 {
                            return;
                        }
                        self.bytes -= &selected_upgrade.cost;
                        match &selected_upgrade.buyable_type {
                            UpgradeType::BytesClicker => {self.clicker += 1.0; selected_upgrade.buy();},
                            UpgradeType::BytesMultiplier => {self.bpc *= 1.25; selected_upgrade.buy();},
                            UpgradeType::ClickersMultiplyBytes => {selected_upgrade.buy(); if selected_upgrade.owned >= 5.0 { self.new_upgrade(3, "Unlock Cores: Cores Are A Secondary Currancy, Cores Boost Your Byte Production", 001, 1_000_000.0, 1.15, 1.0, CostType::Bytes, UpgradeType::UnlockCore)} },
                            UpgradeType::UnlockCore => {self.new_tab("Cores");},
                        }
                    }
                }
            }
        }
    }

    pub fn buy_max(&mut self) {
        if self.cursor.selected().is_some() {
            let selected_upgrade = self.upgrades.iter_mut().find(|x| x.id == self.cursor.selected().unwrap() as u16 && x.tab == self.active_tab).unwrap(); 
            
            match &selected_upgrade.cost_type {
                CostType::Bytes => {
                    while &self.bytes >= &selected_upgrade.cost {
                        if &selected_upgrade.owned >= &selected_upgrade.max && selected_upgrade.max > 0.0 as f64 {
                            return;
                        }
                        self.bytes -= &selected_upgrade.cost;
                        match &selected_upgrade.buyable_type {
                            UpgradeType::BytesClicker => {self.clicker += 1.0; selected_upgrade.buy();},
                            UpgradeType::BytesMultiplier => {self.bpc *= 1.25; selected_upgrade.buy();},
                            UpgradeType::ClickersMultiplyBytes => {selected_upgrade.buy(); if selected_upgrade.owned >= 5.0 { self.new_upgrade(3, "Unlock Cores: Cores Are A Secondary Currancy, Cores Boost Your Byte Production", 001, 1_000_000.0, 1.15, 1.0, CostType::Bytes, UpgradeType::UnlockCore)} },
                            UpgradeType::UnlockCore => {self.new_tab("Cores");},
                        }
                    }
                }
            }
        }
    }
}