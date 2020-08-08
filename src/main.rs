#![feature(generators, generator_trait, try_trait)]
use async_std;
use futures::pin_mut;
use futures::StreamExt;

async fn get(url: &str) -> String {
    let result = surf::get(url).recv_string().await;
    result.unwrap()
}

#[propane::generator]
async fn get_articles(urls: Vec<&str>) -> String {
    let futures = urls.into_iter().map(|url| get(url));
    for result in futures {
        yield result.await;
    }
}

#[async_std::main]
async fn main() {
    let quotes_stream = get_articles(vec![
        "http://quotes.toscrape.com/page/7/",
        "http://quotes.toscrape.com/page/8/",
        "http://quotes.toscrape.com/page/9/",
    ]);
    pin_mut!(quotes_stream);
    while let Some(quote) = quotes_stream.next().await {
        println!("got {}", quote);
    }
}
