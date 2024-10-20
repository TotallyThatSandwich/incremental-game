use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::thread::sleep;
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

                        KeyCode::Char('-') => {
                            state.change_bytes(1.0e10);
                        },
                        KeyCode::Char('m') => state.buy_max(),

                        KeyCode::Down => state.select_next(),
                        KeyCode::Up => state.select_prev(),
                        KeyCode::Right => state.next_tab(),
                        KeyCode::Left => state.prev_tab(),
                        KeyCode::Enter => state.buy_upgrade(),
                        _ => {}
                    }
                }
            }
        }

        let elapsed = std::time::Instant::now().duration_since(last_bytes_update);
        if elapsed >= std::time::Duration::from_secs(1) {
            let time_multiplier = elapsed.as_secs() as f64;
            let clickers_self_multiplier = state.upgrades.iter_mut().find(|x| x.tab == 001 && x.id == 2).unwrap().owned; 

            state.change_bytes(((state.clicker * (state.bpc + (state.clicker * clickers_self_multiplier * 0.3))) * time_multiplier));
            
            last_bytes_update = std::time::Instant::now();
        }
    }
}