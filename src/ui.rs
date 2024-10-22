use crate::{GameState, CostType};
use ratatui::{
    layout::{Constraint, Layout},
    text::{Text, Span, Line},
    widgets::{
        Block, HighlightSpacing, List, Paragraph,
    },
    Frame,
};
use thousands::Separable;


//const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
//const NORMAL_ROW_BG: Color = SLATE.c950;
//const ALT_ROW_BG_COLOR: Color = SLATE.c900;
//const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
//const TEXT_FG_COLOR: Color = SLATE.c200;
//const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub fn ui_render(f: &mut Frame, state: &mut GameState) {
    
    use Constraint::{Fill, Length, Min};
    
    let mut left_lines = vec![];

    let bpc_equasion = (state.bpc + (state.clicker * state.upgrades.iter_mut().find(|x| x.tab == 001 && x.id == 2).unwrap().owned  * 0.3)) * state.clicker * (1.0 + (0.5 * state.cores));

    //let size = f.size();

    let vertical = Layout::vertical([Length(1), Min(0),Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(f.area());

    let horizontal = Layout::horizontal([Fill(1), Fill(3)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    //let title_block = Block::bordered().title(format!("{} * ( {} + ( {} * {} * 0.3 )) = {}", &state.clicker, &state.bpc, &state.clicker, &state.upgrades.iter().find(|&x| x.tab == 001 && x.id == 2).unwrap().owned, bpc_equasion));
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
                        CostType::Bytes => {
                            if i.max > 0.0 {
                                if i.owned >= i.max {
                                    tab_upgrade_list.push(format!("{} [{}/{} Owned]", i.name, i.owned.separate_with_commas(), i.max.separate_with_commas()))
                                } else {
                                    tab_upgrade_list.push(format!("{}", 
                                        if i.cost> 999_999_999.0 {
                                            format!("{} ({:.2e} Bytes) [{}/{} Owned]", i.name, i.cost.round(), i.owned.separate_with_commas(), i.max.separate_with_commas())
                                        } else {
                                            format!("{} ({} Bytes) [{}/{} Owned]", i.name, i.cost.round().separate_with_commas(), i.owned.separate_with_commas(), i.max.separate_with_commas())
                                        })
                                    );
                                }
                            } else {
                                if i.owned > 0.0 {
                                    tab_upgrade_list.push(format!("{}", 
                                        if i.cost> 999_999_999.0 {
                                            format!("{} ({:.2e} Bytes) [{} Owned]", i.name, i.cost.round(), i.owned.separate_with_commas())
                                        } else {
                                            format!("{} ({} Bytes) [{} Owned]", i.name, i.cost.round().separate_with_commas(), i.owned.separate_with_commas())
                                        })
                                    );
                                } else {
                                    tab_upgrade_list.push(format!("{}", 
                                        if i.cost> 999_999_999.0 {
                                            format!("{} ({:.2e} Bytes)", i.name, i.cost.round())
                                        } else {
                                            format!("{} ({} Bytes)", i.name, i.cost.round().separate_with_commas())
                                        })
                                    );
                                }
                            };
                        }
                        CostType::Cores => {
                            if i.max > 0.0 {
                                if i.owned >= i.max {
                                    tab_upgrade_list.push(format!("{} [{}/{} Owned]", i.name, i.owned.separate_with_commas(), i.max.separate_with_commas()))
                                } else {
                                    tab_upgrade_list.push(format!("{}", 
                                        if i.cost> 999_999_999.0 {
                                            format!("{} ({:.2e} Bytes) [{}/{} Owned]", i.name, i.cost.round(), i.owned.separate_with_commas(), i.max.separate_with_commas())
                                        } else {
                                            format!("{} ({} Bytes) [{}/{} Owned]", i.name, i.cost.round().separate_with_commas(), i.owned.separate_with_commas(), i.max.separate_with_commas())
                                        })
                                    );
                                }
                            } else {
                                if i.owned > 0.0 {
                                    tab_upgrade_list.push(format!("{}", 
                                        if i.cost> 999_999_999.0 {
                                            format!("{} ({:.2e} Cores) [{} Owned]", i.name, i.cost.round(), i.owned.separate_with_commas())
                                        } else {
                                            format!("{} ({} Cores) [{} Owned]", i.name, i.cost.round().separate_with_commas(), i.owned.separate_with_commas())
                                        })
                                    );
                                } else {
                                    tab_upgrade_list.push(format!("{}", 
                                        if i.cost> 999_999_999.0 {
                                            format!("{} ({:.2e} Cores)", i.name, i.cost.round())
                                        } else {
                                            format!("{} ({} Cores)", i.name, i.cost.round().separate_with_commas())
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

    left_lines.push(Line::from(Span::raw(format!("Bytes: {}", 
        if state.bytes > 999_999_999.0 {
            format!("{:.2e}", state.bytes.round())
        } else {
            format!("{}", state.bytes.round().separate_with_commas())
        })))
    );

    if state.clicker > 0.0 {
        left_lines.push(Line::from(Span::raw(format!("Clickers: {}", 
            if state.clicker > 999_999_999.0 {
                format!("{:.2e}", state.clicker)
            } else {
                format!("{}", state.clicker.separate_with_commas())
            })))
        );
    }

    if state.cores > 0.0 {
        left_lines.push(Line::from(Span::raw("")));
        
        left_lines.push(Line::from(Span::raw(format!("Cores: {}", 
            if state.cores > 999_999_999.0 {
                format!("{:.2e}", state.cores)
            } else {
                format!("{}", state.cores.separate_with_commas())
            })))
        );
    }
    if state.unlock_tabs >= 2 {
        if state.microprocessors > 0.0 && state.upgrades.iter_mut().find(|x| x.id == 3 as u16 && x.tab == 001).unwrap().is_max() {
            left_lines.push(Line::from(Span::raw(format!("microprocessor: {}", 
                if state.microprocessors > 999_999_999.0 {
                    format!("{:.2e}", state.microprocessors)
                } else {
                    format!("{}", state.microprocessors.separate_with_commas())
                })))
            );
        }
    }

    let text = Text::from(left_lines);
    let p = Paragraph::new(text).block(left_block);


    let status_block = Block::bordered().title(format!("GPS: {} | [↑↓] select upgrade | [ENTER] Buy Selected Upgrade | [M] Buy Max", 
        if bpc_equasion > 999_999_999.0 {
            format!("{:.2e}", bpc_equasion.round())
        } else {
            format!("{}", bpc_equasion.round().separate_with_commas())
        }
    ));

    f.render_widget(title_block, title_area);
    f.render_widget(status_block, status_area);
    f.render_widget(p, left_area);
}