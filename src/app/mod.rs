use self::cli::{generate_matches, OutputFmt};
use ratatui::text::Line;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use strum::{Display, EnumIter, FromRepr};

pub mod cli;

pub const MAX_STATUSES: usize = 50;

#[derive(Clone)]
pub struct Site {
    pub name: String,
    pub addr: String,
    status_codes: VecDeque<Option<Result<u16, ()>>>,
}

impl Site {
    fn new(name: &str, addr: &str) -> Self {
        Self {
            name: name.to_string(),
            addr: addr.to_string(),
            status_codes: vec![None; MAX_STATUSES].into(),
        }
    }

    pub fn push_status_code(&mut self, code: Option<Result<u16, ()>>) {
        if self.status_codes.len() == MAX_STATUSES {
            self.status_codes.pop_back();
        }

        self.status_codes.push_front(code);
    }

    pub fn get_status_codes(&self) -> VecDeque<Option<Result<u16, ()>>> {
        self.status_codes.clone()
    }
}

#[derive(Copy, Clone, Display, FromRepr, EnumIter, PartialEq, Eq)]
pub enum SelectedTab {
    #[strum(to_string = "Live")]
    Live,
    #[strum(to_string = "Chart")]
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

    pub fn title(self) -> Line<'static> {
        format!("  {self}  ").into()
    }
}

pub struct App {
    pub sites: Arc<Mutex<Vec<Site>>>,
    pub output_fmt: OutputFmt,
    pub selected_tab: SelectedTab,
    selected_chart_site_idx: usize,
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
            selected_chart_site_idx: 0,
        }
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn prev_tab(&mut self) {
        self.selected_tab = self.selected_tab.prev();
    }

    pub const fn get_selected_chart_site_idx(&self) -> usize {
        self.selected_chart_site_idx
    }

    pub fn next_chart_site(&mut self) {
        if self.selected_chart_site_idx != self.sites.lock().unwrap().len() - 1 {
            self.selected_chart_site_idx += 1;
        }
    }

    pub fn prev_chart_site(&mut self) {
        self.selected_chart_site_idx = self.selected_chart_site_idx.saturating_sub(1);
    }
}
