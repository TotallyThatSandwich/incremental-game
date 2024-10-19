use crate::GameState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Span, Line},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style, Stylize,
    },
    widgets::{
        Block, Borders, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    Frame,
};


const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub fn ui_render(f: &mut Frame, state: &mut GameState) {
    use Constraint::{Fill, Length, Min};
    
    let size = f.size();

    let vertical = Layout::vertical([Length(1), Min(0),Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(f.area());

    let horizontal = Layout::horizontal([Fill(1), Fill(2)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    let title_block = Block::bordered().title(state.gold.to_string());
    let status_block = Block::bordered().title("Status Bar");
    let left_block = Block::bordered().title("Left");
    let right_block = Block::bordered().title("Right");

    let mut tab_upgrade_list = Vec::new();
    for i in state.tabs[state.active_tab as usize].content.iter() {
        tab_upgrade_list.push(i.name.clone() + " (" + &i.cost.to_string() + ")");
    }

    let list = List::new(tab_upgrade_list)
        .block(right_block)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    f.render_widget(title_block, title_area);
    f.render_widget(status_block, status_area);
    f.render_widget(left_block, left_area);

    f.render_stateful_widget(list, right_area, &mut state.cursor);
}