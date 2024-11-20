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
    let sites = vec![Site::new("GitHub", "github.com")];

    for Site { name, url } in sites {
        let status = check_connection(url);
        println!("{status}: {name}");
    }
}

fn check_connection(url: &str) -> reqwest::StatusCode {
    reqwest::blocking::get("https://github.com")
        .map(|s| s.status())
        .unwrap()
}
