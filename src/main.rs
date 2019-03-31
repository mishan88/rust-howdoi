use clap::{App, Arg};
use reqwest::{self, Client};
use regex::Regex;
use std::borrow::Cow;
use scraper::{Html, Selector};

fn create_url(s: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    let query = re.replace_all(s, "+");
    let url = "https://www.google.com/search?q=site:stackoverflow.com%20";
    format!("{}{}", url, Cow::Borrowed(&query))
}

#[test]
fn test_create_url() {
    let url = create_url("rust install");
    assert_eq!(url, "https://www.google.com/search?q=site:stackoverflow.com%20rust+install".to_owned());
}

fn get_text(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let res = client.get(url)
        .header("User-Agent", r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.86 Safari/537.36"#)
        .send()?
        .text()?;
    Ok(res)
}


fn extract_link_from_google(html: &str) -> Vec<String> {
    let document = Html::parse_fragment(html);
    let g_selector = Selector::parse(r#"[class='r']"#).unwrap();
    let a_selector = Selector::parse(r#"a"#).unwrap();
    let mut links = Vec::<String>::new();
    for g_element in document.select(&g_selector) {
        for element in g_element.select(&a_selector) {
            // remove /url?q=
            links.push(element.value().attr("href").unwrap().to_owned().replace("/url?q=", ""));
        }
    };
    links
}

fn get_answer(html: &str) -> Vec<String> {
    let document = Html::parse_fragment(html);
    let answer_selector = Selector::parse(r#"[id='answers']"#).unwrap();
    let code_selector = Selector::parse("code").unwrap();
    let mut answers = Vec::<String>::new();
    for answer_element in document.select(&answer_selector) {
        for code_element in answer_element.select(&code_selector) {
            answers.push(code_element.inner_html())
        }
    }
    answers
}


fn main() {
    let app = App::new("howdoi")
        .version("0.1.0")
        .author("mishan88 <mishanhideaki88@gmail.com>")
        .about("rust implementation howdoi")
        .arg(Arg::with_name("query")
            .short("q")
            .long("query")
            .help("the question to answer")
            .takes_value(true)
        )
        .arg(Arg::with_name("position")
            .short("p")
            .long("position")
            .help("select answer in specified position")
            .default_value("1")
        );
    let matches = app.get_matches();
    let search_query = matches.value_of("query").unwrap();
    let q = create_url(&search_query);
    let response = get_text(&q);
    let html = match response {
        Ok(n) => n,
        Err(err) => "None".to_string(),
    };
    let links = extract_link_from_google(&html);
    for link in &links[0..1] {
        let contents = get_text(link).unwrap();
        dbg!(get_answer(&contents));
    }
}
