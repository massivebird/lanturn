use std::sync::{Arc, Mutex};

use self::cli::{generate_matches, OutputFmt};

pub mod cli;

#[derive(Clone)]
pub struct Site {
    pub name: String,
    pub addr: String,
    pub status_code: Option<Result<u16, ()>>,
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

pub struct App {
    pub sites: Arc<Mutex<Vec<Site>>>,
    pub output_fmt: OutputFmt,
}

impl App {
    pub fn generate() -> Self {
        let matches: clap::ArgMatches = generate_matches();

        let sites = vec![
            Site::new("GitHub", "https://github.com"),
            Site::new("Google", "https://google.com"),
            Site::new("Steam", "https://steampowered.com"),
        ];

        let output_fmt = match matches.get_one::<String>("output_fmt").unwrap().as_str() {
            "bullet" => OutputFmt::Bullet,
            "line" => OutputFmt::Line,
            _ => unreachable!(),
        };

        Self {
            sites: Arc::new(Mutex::new(sites)),
            output_fmt,
        }
    }
}
