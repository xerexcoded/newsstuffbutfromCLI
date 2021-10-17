mod theme;
use dotenv::dotenv;

use api::{Article, Country, Endpoint, NewsAPI};
use std::error::Error;
fn render_articles(artciles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top Headlines\n\n");
    for i in artciles {
        theme.print_text(&format!("`{}`", i.title()));
        theme.print_text(&format!("> *{}*", i.url()));
        theme.print_text("---");
    }
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {
    dotenv();

    let api_key = std::env::var("API_KEY")?;
    let mut api = NewsAPI::new(&api_key);
    api.endpoint(Endpoint::TopHeadlines).country(Country::Us);
    let api_response = api.fetch_async().await?;
    render_articles(&api_response.articles());

    Ok(())
}
