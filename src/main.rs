use self::{
    app::{
        App, Status,
        connection::{Address, Connection},
        selected_tab::SelectedTab,
    },
    ui::ui,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

mod app;
mod ui;

fn main() -> eyre::Result<()> {
    let mut app = App::generate()?;

    // Set up terminal.
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it.
    let ui_refresh_rate = Duration::from_millis(100);
    let res = start_app(&mut terminal, ui_refresh_rate, &mut app);

    // App is quitting!
    // Restore terminal and environment to normal.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn start_app<B: Backend>(
    terminal: &mut Terminal<B>,
    ui_refresh_rate: Duration,
    app: &mut App,
) -> io::Result<()> {
    let conns = Arc::clone(&app.connections);
    let clk = app.clk.clone();
    let interval = app.interval;

    thread::spawn(move || {
        loop {
            let num_conns = conns.lock().unwrap().len();

            for idx in 0..num_conns {
                let conns = Arc::clone(&conns);

                thread::spawn(move || {
                    test_conn(&conns, idx);
                });
            }

            *clk.lock().unwrap() ^= true;

            thread::sleep(Duration::from_secs(interval.into()));
        }
    });

    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        let timeout = ui_refresh_rate.saturating_sub(last_tick.elapsed());

        if crossterm::event::poll(timeout)? {
            handle_events(app)?;
        }

        if app.is_closing() {
            return Ok(());
        }

        if last_tick.elapsed() >= ui_refresh_rate {
            last_tick = Instant::now();
        }
    }
}

/// Handles user input.
fn handle_events(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q' | 'Q') => app.close(),
            KeyCode::Char('l') => app.next_tab(),
            KeyCode::Char('h') => app.prev_tab(),
            KeyCode::Char('o') if app.selected_tab == SelectedTab::Summary => {
                app.cycle_output_fmt();
            }
            KeyCode::Char('j') if app.selected_tab == SelectedTab::Log => {
                app.next_log_conn();
            }
            KeyCode::Char('k') if app.selected_tab == SelectedTab::Log => {
                app.prev_log_conn();
            }
            _ => (),
        }
    }

    Ok(())
}

fn test_conn(conns: &Arc<Mutex<Vec<Connection>>>, idx: usize) {
    let addr = conns.lock().unwrap().get(idx).unwrap().addr.clone();

    let code: app::Status = match addr {
        Address::Remote { url } => {
            let client = reqwest::blocking::Client::new()
                .get(url)
                .timeout(Duration::from_secs(3));

            match client.send() {
                Ok(response) => Ok(response.status().as_u16()).into(),
                Err(e) => Err(e.to_string()).into(),
            }
        }

        Address::Local { ip } => match ping::new(ip).timeout(Duration::from_secs(1)).send() {
            Ok(ping_res) => Ok(ping_res.rtt.subsec_millis().try_into().unwrap()).into(),
            Err(e) => Err(e.to_string()).into(),
        },

        Address::Json {
            url,
            field,
            ok,
            warn,
            alert,
        } => {
            let client = reqwest::blocking::Client::new()
                .get(url)
                .timeout(Duration::from_secs(3));

            match client.send() {
                Ok(json) => {
                    let mut json = json::parse(&json.text().unwrap()).unwrap();

                    // Parse individual json keys
                    for u in field.split(&['.', '[']) {
                        if let Some(i) = u.find(']') {
                            // Numeric indexing from an array
                            json = json[u[..i].parse::<usize>().unwrap()].clone();
                        } else {
                            // String key
                            json = json[u].clone();
                        }
                    }

                    let val = json.to_string();

                    let code = if val == alert {
                        400
                    } else if val == warn {
                        300
                    } else if val == ok {
                        200
                    } else {
                        100
                    };

                    Status::new(Ok(code), 0).set_msg(val)
                }
                Err(e) => Err(e.to_string()).into(),
            }
        }
    };

    conns
        .lock()
        .unwrap()
        .get_mut(idx)
        .unwrap()
        .push_status(code);
}
