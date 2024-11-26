use crate::app::{cli::OutputFmt, App};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::List,
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    let sites = app.sites.lock().unwrap().clone();

    let mut list_items: Vec<Line<'_>> = Vec::new();

    for site in &sites {
        // Computing the color reflective of online status.
        // Green is OK, red is bad, etc.
        let status_color = {
            if site.status_code.is_none() {
                Color::Gray // Requests have not been sent yet.
            } else {
                match site.status_code.as_ref() {
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
