use self::{
    cli::generate_matches, connection::Connection, output_fmt::OutputFmt, selected_tab::SelectedTab,
};
use std::sync::{Arc, Mutex};

pub mod cli;
mod config;
pub mod connection;
pub mod output_fmt;
pub mod selected_tab;

pub use connection::Status;

#[derive(Default)]
pub struct App {
    pub connections: Arc<Mutex<Vec<Connection>>>,
    pub output_fmt: OutputFmt,

    pub paused: bool,

    /// The index of the connection displayed in the Log tab.
    pub selected_tab: SelectedTab,

    /// The index of the connection displayed in the Log tab.
    log_conn_idx: usize,

    /// Alternates with every wave of connection tests.
    /// Used for displaying update progress.
    pub clk: Arc<Mutex<bool>>,

    /// Indicates if the user has begun quitting the app.
    is_closing: bool,

    /// Clock interval between requests.
    pub interval: u32,
}

impl App {
    pub fn generate() -> eyre::Result<Self> {
        let matches: clap::ArgMatches = generate_matches();

        let conns = config::read_config()?;

        let output_fmt = match matches.get_one::<OutputFmt>("output_fmt") {
            Some(&fmt) => fmt,
            None => OutputFmt::default(),
        };

        let interval = matches.get_one::<u32>("interval").unwrap_or(&15);

        Ok(Self {
            connections: Arc::new(Mutex::new(conns)),
            output_fmt,
            interval: *interval,
            ..Default::default()
        })
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn prev_tab(&mut self) {
        self.selected_tab = self.selected_tab.prev();
    }

    /// Returns the connection currently selected in the Log tab.
    pub fn log_conn(&self) -> (usize, Connection) {
        (
            self.log_conn_idx,
            self.connections
                .lock()
                .unwrap()
                .get(self.log_conn_idx)
                .unwrap()
                .clone(),
        )
    }

    pub fn cycle_output_fmt(&mut self) {
        self.output_fmt = self.output_fmt.next();
    }

    pub fn next_log_conn(&mut self) {
        if self.log_conn_idx != self.connections.lock().unwrap().len() - 1 {
            self.log_conn_idx += 1;
        }
    }

    pub const fn prev_log_conn(&mut self) {
        self.log_conn_idx = self.log_conn_idx.saturating_sub(1);
    }

    pub const fn close(&mut self) {
        self.is_closing = true;
    }

    pub const fn is_closing(&self) -> bool {
        self.is_closing
    }
}
