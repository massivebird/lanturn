use colored::Colorize;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::{Frame, Terminal};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct App {
    sites: Arc<Mutex<Vec<Site>>>,
}

#[derive(Clone)]
struct Site {
    name: String,
    addr: String,
    status: Option<u16>,
}

impl Site {
    fn new(name: &str, addr: &str) -> Self {
        Self {
            name: name.to_string(),
            addr: addr.to_string(),
            status: None,
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

    std::thread::spawn(move || loop {
        update_cache(&Arc::clone(&sites));
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
    // let status_str = reqwest::blocking::get(site.addr).map_or_else(
    //     |_| "■".red(),
    //     |response| match response.status().as_u16() {
    //         200 => "■".green(),
    //         _ => "■".red(),
    //     },
    // );
}

fn update_cache(sites: &Arc<Mutex<Vec<Site>>>) {
    for (idx, site) in sites.lock().unwrap().iter().enumerate().cycle() {
        let status_code = reqwest::blocking::get(site.addr.clone())
            .unwrap()
            .status()
            .as_u16();

        sites.lock().unwrap().get_mut(idx).unwrap().status = Some(status_code);
    }
}
