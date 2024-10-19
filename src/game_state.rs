use ratatui::text::{Span, Line};
use ratatui::widgets::ListState;
use crate::Tab;
use crate::Upgrade;

pub struct GameState {
    pub tabs: Vec<Tab>,
    pub active_tab: u16,
    pub cursor: ListState,
    pub gold: i64,
    
}

impl Default for GameState {
    fn default() -> Self {

        let upgrades = vec![
         Upgrade::new("Upgrade 1", 10),
         Upgrade::new("Upgrade 2", 20),
         Upgrade::new("Upgrade 3", 30)
        ];

        GameState {
            tabs: vec![Tab::new("Tab 1", upgrades)],
            active_tab: 0,
            gold: 0,
            cursor: ListState::default(),
        }
    }
}

impl GameState {
    pub fn change_gold(&mut self, amount: i64) {
        self.gold += amount;
        if self.gold < 0 {
            self.gold = 0;
        }
    }

    pub fn reset_gold(&mut self) {
        self.gold = 0;
    }

    pub fn change_tab(&mut self, tab: u16) {
        self.active_tab = tab;
    }

    pub fn select_next(&mut self) {
        self.cursor.select_next();
    }
    pub fn select_previous(&mut self) {
        self.cursor.select_previous();
    }
}