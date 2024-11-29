use crate::app::{output_fmt::OutputFmt, selected_tab::SelectedTab, site::MAX_STATUSES, App};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, List, Tabs},
    Frame,
};
use strum::IntoEnumIterator;

pub fn ui(f: &mut Frame, app: &App) {
    // Render tabs at the top.

    let titles = SelectedTab::iter().map(SelectedTab::title);

    let tabs = Tabs::new(titles).select(app.selected_tab as usize);

    f.render_widget(tabs, Rect::new(0, 0, f.area().width, f.area().height));

    // Render contents of selected tab.
    match app.selected_tab {
        SelectedTab::Live => render_tab_live(f, app),
        SelectedTab::Chart => render_tab_chart(f, app),
    }
}

fn render_tab_live(f: &mut Frame, app: &App) {
    let sites = app.sites.lock().unwrap().clone();

    let mut list_items: Vec<Line<'_>> = Vec::new();

    for site in &sites {
        // Compute online status color.
        // Green is OK, red is bad, etc.
        let status_color = {
            if site.get_status_codes()[0].is_none() {
                Color::Gray // Requests have not been sent yet.
            } else {
                match site.get_status_codes()[0].as_ref() {
                    Some(Ok(status_code)) => match status_code {
                        200 => Color::Green,
                        _ => Color::Yellow,
                    },
                    _ => Color::Red,
                }
            }
        };

        let site_output: Line<'_> = match app.output_fmt {
            OutputFmt::Bullet => Line::from(vec![
                Span::from(" ■ ").style(status_color),
                Span::from(format!("{} ({})", site.name.clone(), site.url)),
            ]),
            OutputFmt::Line => {
                Line::from(Span::from(format!(" {} ({})", site.name.clone(), site.url))).style(
                    Style::new()
                        .bg(status_color)
                        .fg(if status_color == Color::DarkGray {
                            Color::DarkGray
                        } else {
                            Color::Black
                        })
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                )
            }
        };

        list_items.push(site_output);
    }

    let block = Block::bordered().title_bottom(" q: Quit ");

    f.render_widget(
        List::new(list_items).block(block),
        Rect::new(0, 1, f.area().width, f.area().height),
    );
}

fn render_tab_chart(f: &mut Frame, app: &App) {
    let idx = app.get_selected_chart_site_idx();

    let site = app.sites.lock().unwrap().get(idx).unwrap().clone();

    let statuses = site.get_status_codes();

    let bars: Vec<Bar> = statuses
        .iter()
        .enumerate()
        .take(MAX_STATUSES)
        .map(|(idx, s)| {
            let val = s
                .as_ref()
                .map_or(u64::MIN, |s| s.map_or(1, |s| if s == 200 { 3 } else { 2 }));

            let color = match val {
                1 => Color::Red,
                3 => Color::Green,
                _ => Color::Yellow,
            };

            let bar_style = Style::new().fg(color);

            Bar::default()
                .value(val)
                .style(bar_style)
                .text_value(String::new())
                .label(if idx == 0 {
                    Line::from("Now")
                } else {
                    Line::from("")
                })
                .value_style(bar_style.reversed())
        })
        .collect();

    let block = Block::bordered().title_bottom(" q: Quit | j: Next site | k: Previous site ");

    let barchart = BarChart::default()
        .block(block)
        .bar_gap(0)
        .bar_width(3)
        .max(3)
        .data(BarGroup::default().bars(&bars));

    let info = Line::from(format!(
        " Selected site: [{idx:02}] {} ({}) ",
        site.name, site.url
    ));

    f.render_widget(
        barchart,
        Rect::new(0, 1, f.area().width, f.area().height - 1),
    );

    f.render_widget(info, Rect::new(2, 1, f.area().width, f.area().height - 1))
}
