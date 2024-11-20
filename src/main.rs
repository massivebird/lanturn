struct Site {
    name: &str,
    url: &str,
}

fn main() {
    let urls = vec![
        "github.com",
    ];

    for url in urls {
        let status = request_site(url);
        println!("")
    }
}

fn request_site(url: &str) -> reqwest::StatusCode {
    reqwest::blocking::get("https://github.com")
        .map(|s| s.status())
        .unwrap()
}
