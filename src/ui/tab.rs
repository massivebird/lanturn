use crate::app::{App, connection::Address, output_fmt::OutputFmt};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, Paragraph, Wrap},
};

pub fn render_tab_live(f: &mut Frame, app: &App) {
    let mut list_items: Vec<Line<'_>> = Vec::new();

    for conn in app.connections.lock().unwrap().iter() {
        let color = conn
            .log()
            .front()
            .map_or(Color::Gray, |sts| sts.generate_color(&conn.addr));

        let url = conn.addr();

        let conn_output: Line<'_> = match app.output_fmt {
            OutputFmt::Bullet => Line::from(vec![
                Span::from(" 󰝤 ").style(color),
                Span::from(format!("{} ({})", conn.pretty_name(), url)),
            ]),
            OutputFmt::Line => Line::from(Span::from(format!(" {} ({})", conn.pretty_name(), url)))
                .style(
                    Style::new()
                        .bg(color)
                        .fg(if color == Color::DarkGray {
                            Color::DarkGray
                        } else {
                            Color::Black
                        })
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
        };

        list_items.push(conn_output);
    }

    let block = Block::bordered().title_bottom(" q: Quit | o: Cycle formats ");

    f.render_widget(
        List::new(list_items).block(block),
        Rect::new(0, 1, f.area().width, f.area().height),
    );
}

pub fn render_tab_log(f: &mut Frame, app: &App) {
    let (i, log_conn) = app.log_conn();

    let sidebar_width = 8;

    // Populate sidebar with conn summaries.
    let sidebar_txt: Vec<Line> = app
        .connections
        .lock()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(j, conn)| {
            let color = conn
                .log()
                .front()
                .map_or_else(|| Color::Gray, |s| s.generate_color(&conn.addr));

            let indicator = if j == i { "> " } else { "  " };

            Line::from(vec![
                Span::from(indicator),
                Span::from(format!("[{j:02}]"))
                    .style(Style::new().bg(color).fg(Color::Black).bold()),
            ])
        })
        .collect();

    f.render_widget(
        Paragraph::new(sidebar_txt).block(Block::bordered()),
        Rect::new(0, 1, sidebar_width, f.area().height - 1),
    );

    // String-ify the selected connection's status log.
    let log_txt: Vec<Line> = log_conn
        .log()
        .iter()
        .map(|status| {
            let desc = match (status.code(), &log_conn.addr) {
                (Ok(code), Address::Remote { .. }) => code.to_string(),
                (Ok(_), Address::Json { .. }) => status.msg().unwrap(),
                (Ok(ms), Address::Local { .. }) => format!("{ms} ms"),
                (Err(e), _) => e.clone(),
            };

            let color = status.generate_color(&log_conn.addr);

            let time = status.timestamp();

            // Make the latest status distinct.
            let color_pop = if chrono::Local::now()
                .signed_duration_since(time)
                .num_milliseconds()
                < 350
            {
                Span::styled("░░░░░░░", Style::new().bg(color).fg(Color::Black))
            } else {
                Span::styled("███████", Style::new().fg(color))
            };

            let guide = {
                let min_desc_len: usize = 16;
                let interval = 3;
                let id = status.id;

                let mid = if id > 0 && id.saturating_add(1) % interval == 0 {
                    "."
                } else {
                    " "
                }
                .repeat(min_desc_len.saturating_sub(desc.len()));

                format!(" {mid} ")
            };

            Line::from(vec![
                color_pop,
                Span::raw(" "),
                Span::raw(desc),
                Span::from(guide).fg(Color::Gray),
                Span::raw(time.to_string()),
            ])
        })
        .collect();

    f.render_widget(
        Paragraph::new(log_txt).block(Block::bordered()).wrap(Wrap { trim: true }),
        Rect::new(sidebar_width, 1, f.area().width, f.area().height - 1),
    );

    // Display info on selected conn.
    f.render_widget(
        Line::from(format!(
            " [{i:02}] {} ({}) ",
            log_conn.pretty_name(),
            log_conn.addr()
        )),
        Rect::new(sidebar_width + 1, 1, f.area().width, f.area().height - 1),
    );

    // Render some controls instructions.
    f.render_widget(
        Line::from(" q: Quit | j: Next connection | k: Previous connection "),
        Rect::new(1, f.area().height - 1, f.area().width, f.area().height - 1),
    );
}
