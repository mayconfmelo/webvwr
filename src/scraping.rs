
pub fn get_title(url: String) -> str {
    use scraper::{Html, Selector};

    let response = reqwest::blocking::get("https://mayconfmelo-test.netlify.app/")
        .unwrap()
        .text()
        .unwrap();

    let doc = Html::parse_document(&response);
    let title_selector = Selector::parse("head title").unwrap();
    let title = doc.select(&title_selector).next().unwrap();

    title.inner_html().trim()
}

pub fn get_favicon(url: String) {
    use site_icons::SiteIcons;
}

/*
fn main() {
    let response = reqwest::blocking::get("https://mayconfmelo-test.netlify.app/")
        .unwrap()
        .text()
        .unwrap();

    // parse the HTML document
    let doc = Html::parse_document(&response);

    // select the elements with titleline class
    let title_selector = Selector::parse("head title").unwrap();
    let title = doc.select(&title_selector).next().unwrap();
    println!("{}", title.inner_html().trim());

    let mut icons = SiteIcons::new();
    // scrape the icons from a url
    let entries = icons.load_website("https://mayconfmelo-test.netlify.app/", false).await?;

    // entries are sorted from highest to lowest resolution
    for icon in entries {
        println!("{:?}", icon)
}
*/