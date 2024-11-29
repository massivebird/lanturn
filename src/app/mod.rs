use self::{cli::generate_matches, output_fmt::OutputFmt, selected_tab::SelectedTab, site::Site};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use yaml_rust2::Yaml;

pub mod cli;
pub mod output_fmt;
pub mod selected_tab;
pub mod site;

#[derive(Default)]
pub struct App {
    pub sites: Arc<Mutex<Vec<Site>>>,
    pub output_fmt: OutputFmt,
    pub selected_tab: SelectedTab,
    selected_chart_site_idx: usize,
}

impl App {
    pub fn generate() -> Self {
        let matches: clap::ArgMatches = generate_matches();

        let sites = Self::read_sites_from_file();

        let output_fmt = match matches.get_one::<OutputFmt>("output_fmt") {
            Some(&fmt) => fmt,
            None => OutputFmt::default(),
        };

        Self {
            sites: Arc::new(Mutex::new(sites)),
            output_fmt,
            ..Default::default()
        }
    }

    fn read_sites_from_file() -> Vec<Site> {
        let home_dir = std::env::var("HOME").unwrap();
        let config_path = format!("{home_dir}/.config/lanturn/config.yaml");

        assert!(
            Path::new(&config_path).exists(),
            "Unable to locate config file at {config_path}",
        );

        let Ok(config_contents) = std::fs::read_to_string(config_path.clone()) else {
            panic!("Unable to read config file at {config_path}");
        };

        let Ok(yaml) = yaml_rust2::YamlLoader::load_from_str(&config_contents) else {
            panic!("Failed to parse config file at {config_path} into yaml.")
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

        sites
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
