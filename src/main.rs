use colored::Colorize;

struct Site<'a> {
    name: &'a str,
    url: &'a str,
}

impl<'a> Site<'a> {
    const fn new(name: &'a str, url: &'a str) -> Self {
        Self { name, url }
    }
}

fn main() {
    let sites = vec![
        Site::new("GitHub", "https://github.com"),
        Site::new("Google", "https://google.com"),
        Site::new("Steam", "https://steampowered.com"),
    ];

    for Site { name, url } in sites {
        let status_str = reqwest::blocking::get(url).map_or_else(
            |_| "■".red(),
            |response| match response.status().as_u16() {
                200 => "■".green(),
                _ => "■".red(),
            },
        );

        println!("{status_str} {name}");
    }
}
