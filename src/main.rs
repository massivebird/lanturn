use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    style::Color,
    text::{Line, Span},
    widgets::List,
    Frame, Terminal,
};
use std::{
    io,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

struct App {
    sites: Arc<Mutex<Vec<Site>>>,
}

#[derive(Clone)]
struct Site {
    name: String,
    addr: String,
    status_code: Option<Result<u16, ()>>,
}

impl Site {
    fn new(name: &str, addr: &str) -> Self {
        Self {
            name: name.to_string(),
            addr: addr.to_string(),
            status_code: None,
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
            fetch_site(&Arc::clone(&sites), idx);
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
    let sites = {
        let guard = app.sites.lock().unwrap();
        guard.clone()
    };

    let mut list_items: Vec<Line<'_>> = Vec::new();

    for site in &sites {
        // Computing the color reflective of online status.
        // Green is OK, red is bad, etc.
        let color = {
            if site.status_code.is_none() {
                Color::Red
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

        list_items.push(
            vec![
                Span::from(" ■ ").style(color),
                Span::from(site.name.clone()),
            ]
            .into(),
        );
    }

    f.render_widget(
        List::new(list_items),
        Rect::new(0, 0, f.area().width, f.area().height),
    );
}

fn fetch_site(sites: &Arc<Mutex<Vec<Site>>>, idx: usize) {
    let client = reqwest::blocking::Client::new()
        .get(sites.lock().unwrap().get(idx).unwrap().addr.clone())
        .timeout(Duration::from_secs(3));

    let mut guard = sites.lock().unwrap();

    guard.get_mut(idx).unwrap().status_code = client.send().map_or_else(
        |_| Some(Err(())),
        |response| Some(Ok(response.status().as_u16())),
    );
}
