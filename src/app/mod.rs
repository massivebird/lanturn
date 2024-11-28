use self::cli::{generate_matches, OutputFmt};
use self::selected_tab::SelectedTab;
use self::site::Site;
use std::path::Path;
use std::sync::{Arc, Mutex};
use yaml_rust2::Yaml;

pub mod cli;
pub mod selected_tab;
pub mod site;

pub struct App {
    pub sites: Arc<Mutex<Vec<Site>>>,
    pub output_fmt: OutputFmt,
    pub selected_tab: SelectedTab,
    selected_chart_site_idx: usize,
}

impl App {
    pub fn generate() -> Self {
        let matches: clap::ArgMatches = generate_matches();

        let home_dir = std::env::var("HOME").unwrap();
        let config_path = format!("{home_dir}/.config/lanturn/config.yaml");

        assert!(
            Path::new(&config_path).exists(),
            "Unable to locate config file at {}",
            config_path,
        );

        let Ok(config_contents) = std::fs::read_to_string(config_path.clone()) else {
            panic!("Unable to read config file at {}", config_path);
        };

        let Ok(yaml) = yaml_rust2::YamlLoader::load_from_str(&config_contents) else {
            panic!("Failed to parse config file at {} into yaml.", config_path)
        };

        let sites_yaml: &Yaml = &yaml[0]["sites"];

        let mut sites: Vec<Site> = Vec::new();

        // I don't know how to iterate over yaml::as_hash() without
        // unwrapping it, and that panics when unwrapping zero users.
        // So if there are no users, we exit this block.
        if sites_yaml.as_hash().is_none() {
            unimplemented!();
        };

        for (label, properties) in sites_yaml.as_hash().unwrap() {
            let Some(label) = label.as_str() else {
                panic!("Failed to process label: {label:?}");
            };

            let Some(name) = properties["name"].as_str() else {
                panic!("Failed to process field `name` for user labeled `{label}`");
            };

            let Some(url) = properties["url"].as_str() else {
                panic!("Failed to process field `url` for user labeled `{label}`");
            };

            sites.push(Site::new(name, url));
        }

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
