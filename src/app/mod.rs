use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use strum::FromRepr;

use self::cli::{generate_matches, OutputFmt};

pub mod cli;

pub const LEN: usize = 50;

#[derive(Clone)]
pub struct Site {
    pub name: String,
    pub addr: String,
    status_codes: VecDeque<Option<Result<u16, ()>>>,
    status_len: usize,
}

impl Site {
    fn new(name: &str, addr: &str) -> Self {
        Self {
            name: name.to_string(),
            addr: addr.to_string(),
            status_codes: vec![None; 10].into(),
            status_len: 50,
        }
    }

    pub fn push_status_code(&mut self, code: Option<Result<u16, ()>>) {
        if self.status_codes.len() == self.status_len {
            self.status_codes.pop_back();
        }

        self.status_codes.push_front(code);
    }

    pub fn get_status_codes(&self) -> VecDeque<Option<Result<u16, ()>>> {
        self.status_codes.clone()
    }

    pub const fn get_status_len(&self) -> usize {
        self.status_len
    }
}

#[derive(Copy, Clone, FromRepr)]
pub enum SelectedTab {
    Live,
    Chart,
}

impl SelectedTab {
    fn next(self) -> Self {
        let current_idx: usize = self as usize;
        let next_idx: usize = current_idx.saturating_add(1);
        Self::from_repr(next_idx).unwrap_or(self)
    }

    fn prev(self) -> Self {
        let current_idx: usize = self as usize;
        let prev_idx: usize = current_idx.saturating_sub(1);
        Self::from_repr(prev_idx).unwrap_or(self)
    }
}

pub struct App {
    pub sites: Arc<Mutex<Vec<Site>>>,
    pub output_fmt: OutputFmt,
    pub selected_tab: SelectedTab,
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
            selected_tab: SelectedTab::Live,
        }
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn prev_tab(&mut self) {
        self.selected_tab = self.selected_tab.prev();
    }
}
