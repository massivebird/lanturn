use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::Span;
use ratatui::{Frame, Terminal};
use reqwest::blocking::Response;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct App {
    sites: Arc<Mutex<Vec<Site>>>,
}

struct Site {
    name: String,
    addr: String,
    latest_response: Option<Result<Response, ()>>,
}

impl Site {
    fn new(name: &str, addr: &str) -> Self {
        Self {
            name: name.to_string(),
            addr: addr.to_string(),
            latest_response: None,
        }
    }
}

fn main() -> io::Result<()> {
    // Set up terminal.
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it.
    let ui_refresh_rate = Duration::from_millis(200);
    let sites = vec![
        Site::new("GitHub", "https://github.com"),
        Site::new("Google", "https://google.com"),
        Site::new("Steam", "https://steampowered.com"),
    ];

    let app = App {
        sites: Arc::new(Mutex::new(sites)),
    };

    let res = commence_application(&mut terminal, ui_refresh_rate, &app);

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
    app: &App,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    let sites = Arc::clone(&app.sites);
    let num_sites = sites.lock().unwrap().len();

    std::thread::spawn(move || loop {
        for idx in (0..num_sites).cycle() {
            update_cache(&Arc::clone(&sites), idx);
        }
    });

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = ui_refresh_rate.saturating_sub(last_tick.elapsed());
        // Polls for the remaining time until the next scheduled tick.
        // Maintains a consistent tick schedule while checking for input!
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= ui_refresh_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let guard = app.sites.lock().unwrap();
    for (idx, site) in guard.iter().enumerate() {
        let mut _span: Span = Span::from("■");
        let mut span: Span = Span::from(site.name.clone());

        if site.latest_response.is_none() {
            span = span.gray();
        } else {
            match site.latest_response.as_ref() {
                Some(Ok(response)) => {
                    match response.status().as_u16() {
                        200 => span = span.green(),
                        _ => span = span.yellow(),
                    };
                }
                _ => span = span.red(),
            };
        }

        f.render_widget(
            span,
            Rect::new(0, idx as u16, f.area().width, f.area().height),
        );
    }
}

fn update_cache(sites: &Arc<Mutex<Vec<Site>>>, idx: usize) {
    let addr = sites.lock().unwrap().get(idx).unwrap().addr.clone();

    let response =
        reqwest::blocking::get(addr).map_or_else(|_| Some(Err(())), |response| Some(Ok(response)));

    sites.lock().unwrap().get_mut(idx).unwrap().latest_response = response;
}
