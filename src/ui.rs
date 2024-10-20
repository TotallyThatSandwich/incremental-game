use crate::{GameState, CostType};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Text, Span, Line},
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


//const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
//const NORMAL_ROW_BG: Color = SLATE.c950;
//const ALT_ROW_BG_COLOR: Color = SLATE.c900;
//const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
//const TEXT_FG_COLOR: Color = SLATE.c200;
//const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub fn ui_render(f: &mut Frame, state: &mut GameState) {
    
    use Constraint::{Fill, Length, Min};
    
    let mut left_lines = vec![];

    let gpc_equasion = (state.gpc + (state.clicker * state.upgrades.iter_mut().find(|x| x.tab == 001 && x.id == 2).unwrap().owned  * 0.3)) * state.clicker;

    let size = f.size();

    let vertical = Layout::vertical([Length(1), Min(0),Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(f.area());

    let horizontal = Layout::horizontal([Fill(1), Fill(2)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    //let title_block = Block::bordered().title(format!("{} * ( {} + ( {} * {} * 0.3 )) = {}", &state.clicker, &state.gpc, &state.clicker, &state.upgrades.iter().find(|&x| x.tab == 001 && x.id == 2).unwrap().owned, gpc_equasion));
    let title_block = Block::bordered().title("Clicker Game");
    let left_block = Block::bordered().title("Statistics");
    let right_block = Block::bordered();

    for i in &state.tabs {
        if i.index == state.active_tab {
            let right_block = right_block.clone().title(i.title.clone());

            let mut tab_upgrade_list = Vec::new();
            for i in &state.upgrades {
                if i.tab == state.active_tab {
                    match i.cost_type {
                        CostType::Gold => {
                            if i.max > 0.0 {
                                if i.owned >= i.max {
                                    tab_upgrade_list.push(format!("{} [{}/{} Owned]", i.name, i.owned, i.max))
                                } else {
                                    tab_upgrade_list.push(format!("{}", 
                                        if i.cost> 999_999_999.0 {
                                            format!("{} ({:.2e} Gold) [{}/{} Owned]", i.name, i.cost.round(), i.owned, i.max)
                                        } else {
                                            format!("{} ({} Gold) [{}/{} Owned]", i.name, i.cost.round(), i.owned, i.max)
                                        })
                                    );
                                }
                            } else {
                                if i.owned > 0.0 {
                                    tab_upgrade_list.push(format!("{}", 
                                        if i.cost> 999_999_999.0 {
                                            format!("{} ({:.2e} Gold) [{} Owned]", i.name, i.cost.round(), i.owned)
                                        } else {
                                            format!("{} ({} Gold) [{} Owned]", i.name, i.cost.round(), i.owned)
                                        })
                                    );
                                } else {
                                    tab_upgrade_list.push(format!("{}", 
                                        if i.cost> 999_999_999.0 {
                                            format!("{} ({:.2e} Gold)", i.name, i.cost.round())
                                        } else {
                                            format!("{} ({} Gold)", i.name, i.cost.round())
                                        })
                                    );
                                }
                            };
                        }
                    }
                        
                }
            }

            let list = List::new(tab_upgrade_list)
                .block(right_block)
                .highlight_symbol(">")
                .highlight_spacing(HighlightSpacing::Always);

            f.render_stateful_widget(list, right_area, &mut state.cursor);
        }
    }

    left_lines.push(Line::from(Span::raw(format!("Gold: {}", 
        if state.gold > 999_999_999.0 {
            format!("{:.2e}", state.gold.round())
        } else {
            format!("{}", state.gold.round())
        })))
    );

    if state.clicker > 0.0 {
        left_lines.push(Line::from(Span::raw(format!("Clickers: {}", 
            if state.clicker > 999_999_999.0 {
                format!("{:.2e}", state.clicker)
            } else {
                format!("{}", state.clicker)
            })))
        );
    }

    let text = Text::from(left_lines);
    let p = Paragraph::new(text).block(left_block);


    let status_block = Block::bordered().title(format!("GPS: {} | [↑↓] select upgrade | [ENTER] Buy Selected Upgrade | [M] Buy Max", 
        if gpc_equasion > 999_999_999.0 {
            format!("{:.2e}", gpc_equasion.round())
        } else {
            format!("{}", gpc_equasion.round())
        }
    ));

    f.render_widget(title_block, title_area);
    f.render_widget(status_block, status_area);
    f.render_widget(p, left_area);
}