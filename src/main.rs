use std::error::Error;
use serde::Deserialize;
use colour::{ cyan, magenta};

#[derive(Deserialize, Debug)]

struct Articles {
    articles: Vec<Article>
}
#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String
}

fn get_articles (url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()? ;
    let articles: Articles = serde_json::from_str(&response)? ;

    Ok(articles)

}

fn render_articles (articles: &Articles) {
    for a in &articles.articles {
        magenta!("> {}\n", a.title);
        cyan!("> {}\n\n", a.url);
    }
}

fn main()-> Result<(), Box<dyn Error>> {
    let url = "https://newsapi.org/v2/everything?q=apple&from=2022-06-07&to=2022-06-07&sortBy=popularity&apiKey=39a9e0e134ae47e2b93d451951c81981";
    let articles: Articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}

