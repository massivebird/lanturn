use crate::app::{cli::OutputFmt, App, SelectedTab};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Axis, Chart, List},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    match app.selected_tab {
        SelectedTab::Live => render_tab_live(f, app),
        SelectedTab::Chart => render_tab_chart(f, app),
    }
}

fn render_tab_live(f: &mut Frame, app: &App) {
    let sites = app.sites.lock().unwrap().clone();

    let mut list_items: Vec<Line<'_>> = Vec::new();

    for site in &sites {
        // Computing the color reflective of online status.
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
                Span::from(site.name.clone()),
            ]),
            OutputFmt::Line => Line::from(Span::from(format!(" {}", site.name.clone()))).style(
                Style::new()
                    .bg(status_color)
                    .fg(if status_color == Color::DarkGray {
                        Color::DarkGray
                    } else {
                        Color::Black
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        };

        list_items.push(site_output);
    }

    f.render_widget(
        List::new(list_items),
        Rect::new(0, 0, f.area().width, f.area().height),
    );
}

fn render_tab_chart(f: &mut Frame, app: &App) {
    let statuses = app.sites.as_ref().lock().unwrap()[0].get_status_codes().clone();

    let len: usize = app.sites.as_ref().lock().unwrap()[0].get_status_len();

    let mut data: [(f64, f64); len] = [(f64::MIN, f64::MIN); len];

    for idx in 0..len {
        let val = match statuses.get(idx) {
            Some(s) => 1.,
            None => f64::MIN,
        };

        data[idx] = (idx as f64, val);
    }

    let dataset = ratatui::widgets::Dataset::default()
        .data(&data)
        .marker(ratatui::symbols::Marker::Braille);

    let chart = Chart::new(vec![dataset])
        .block(ratatui::widgets::Block::bordered())
        .x_axis(Axis::default().bounds([0., len as f64]))
        .y_axis(Axis::default().bounds([-2., 2.]));

    f.render_widget(chart, f.area())
}
