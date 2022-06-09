mod theme;

use std::error::Error;
use newsapi::{Articles, get_articles};
use dotenv::dotenv;

fn render_articles (articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");

    for a in &articles.articles {
        theme.print_text(&format!("`{}`", a.title));
        theme.print_text(&format!("> {}", a.url));
        theme.print_text("---");
    }
}

fn main()-> Result<(), Box<dyn Error>> {
    dotenv()?;
    let api_key: String = std::env::var("API_KEY")?;

    let url = "https://newsapi.org/v2/everything?q=apple&from=2022-06-07&to=2022-06-07&sortBy=popularity&apiKey=";
    let url_fmt = format!("{}{}", url, api_key );

    let articles: Articles = get_articles(&url_fmt)?;

    render_articles(&articles);

    Ok(())
}

