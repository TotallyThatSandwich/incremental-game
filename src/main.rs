use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
//use std::thread::sleep;
use std::time::Duration;
use game_state::GameState;
use tabs::Tab;
use upgrades::{Buyable, UpgradeType, CostType};
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::io::{self};
use ui::ui_render;
mod game_state;
mod tabs;
mod upgrades;
mod ui;

fn main() -> Result<(), io::Error> {
    // Setup the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // Important variables
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut game_state = GameState::default();

    // App loop
    run_app(&mut terminal, &mut game_state)
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut GameState,
) -> io::Result<()> {
    
    let mut last_bytes_update = std::time::Instant::now();
    
    loop {
        // Draw the ui
        terminal.draw(|f: &mut Frame| ui_render(f, state))?;

        // Catch inputs
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            return Ok(());
                        }
                        KeyCode::Char('g') => {
                            state.change_bytes(1.0);
                        },

                        KeyCode::Char('-') => {state.change_bytes(1.0e10);},
                        KeyCode::Char('m') => { 
                            if state.buy_max().is_ok() {
                                let upgrade = state.upgrades.iter_mut().find(|x| x.id == state.cursor.selected().unwrap() as u16 && x.tab == state.active_tab).unwrap();

                                match upgrade.buyable_type {
                                    UpgradeType::BytesClicker => {},
                                    UpgradeType::BytesMultiplier => {},
                                    UpgradeType::ClickersMultiplyBytes => { if upgrade.is_max() && state.upgrades.iter().find(|&x| x.id == 3 && x.tab == 001).is_none() { state.new_upgrade(3, "Unlock Cores: Cores Are A Secondary Currancy, Cores Boost Your Byte Production", 001, 1_000_000.0, 1.15, 1.0, CostType::Bytes, UpgradeType::UnlockCore); }},
                                    UpgradeType::UnlockCore=> { if upgrade.is_max() && state.unlock_tabs == 001 {state.new_tab("Cores Shop"); state.cps = 5.0}},
                                    UpgradeType::CoreClicker => {}
                                };
                            }
                        }

                        KeyCode::Down => state.select_next(),
                        KeyCode::Up => state.select_prev(),
                        KeyCode::Right => state.next_tab(),
                        KeyCode::Left => state.prev_tab(),
                        KeyCode::Enter => { 
                            if state.buy_upgrade().is_ok() {
                                let upgrade = state.upgrades.iter_mut().find(|x| x.id == state.cursor.selected().unwrap() as u16 && x.tab == state.active_tab).unwrap();

                                match upgrade.buyable_type {
                                    UpgradeType::BytesClicker => {},
                                    UpgradeType::BytesMultiplier => {},
                                    UpgradeType::ClickersMultiplyBytes => { if upgrade.is_max() && state.upgrades.iter().find(|&x| x.id == 3 && x.tab == 001).is_none() { state.new_upgrade(3, "Unlock Cores: Cores Are A Secondary Currancy, Cores Boost Your Byte Production", 001, 1_000_000.0, 1.15, 1.0, CostType::Bytes, UpgradeType::UnlockCore); }},
                                    UpgradeType::UnlockCore=> { if upgrade.is_max() && state.unlock_tabs == 001 {state.new_tab("Cores Shop"); state.cps = 5.0}},
                                    UpgradeType::CoreClicker => {}
                                };
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let elapsed = std::time::Instant::now().duration_since(last_bytes_update);
        if elapsed >= std::time::Duration::from_secs(1) {
            let time_multiplier = elapsed.as_secs() as f64;
            let clickers_self_multiplier = state.upgrades.iter_mut().find(|x| x.tab == 001 && x.id == 2).unwrap().owned; 

            state.change_bytes((state.clicker * (state.bpc + (state.clicker * clickers_self_multiplier * 0.3))) * time_multiplier * (1.0 + (0.5 * state.cores)));
            
            state.change_cores(state.microprocessors * state.cps);

            last_bytes_update = std::time::Instant::now();
        }
    }
}