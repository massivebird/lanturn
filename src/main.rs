use self::{
    app::{selected_tab::SelectedTab, site::Site, App},
    ui::ui,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

mod app;
mod ui;

fn main() -> io::Result<()> {
    let mut app = App::generate();

    // Set up terminal.
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it.
    let ui_refresh_rate = Duration::from_millis(200);

    let res = commence_application(&mut terminal, ui_refresh_rate, &mut app);

    // Restore terminal.
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

fn commence_application<B: Backend>(
    terminal: &mut Terminal<B>,
    ui_refresh_rate: Duration,
    app: &mut App,
) -> io::Result<()> {
    let sites = Arc::clone(&app.sites);

    thread::spawn(move || loop {
        let num_sites = sites.lock().unwrap().len();

        for idx in 0..num_sites {
            let sites = Arc::clone(&sites);

            thread::spawn(move || {
                fetch_site(&sites, idx);
            });
        }

        thread::sleep(Duration::from_secs(5));
    });

    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

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

fn handle_events(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => app.close(),
            KeyCode::Char('l') => app.next_tab(),
            KeyCode::Char('h') => app.prev_tab(),
            KeyCode::Char('j') if app.selected_tab == SelectedTab::Chart => {
                app.next_chart_site();
            }
            KeyCode::Char('k') if app.selected_tab == SelectedTab::Chart => {
                app.prev_chart_site();
            }
            _ => (),
        }
    }

    Ok(())
}

fn fetch_site(sites: &Arc<Mutex<Vec<Site>>>, idx: usize) {
    let client = reqwest::blocking::Client::new()
        .get(sites.lock().unwrap().get(idx).unwrap().url.clone())
        .timeout(Duration::from_secs(3));

    let status_code = client.send().map_or_else(
        |_| Some(Err(())),
        |response| Some(Ok(response.status().as_u16())),
    );

    sites
        .lock()
        .unwrap()
        .get_mut(idx)
        .unwrap()
        .push_status_code(status_code);
}
