#![feature(generators, generator_trait, try_trait)]

use async_std;
use futures::pin_mut;
use futures::StreamExt;

async fn get(url: String) -> String {
    let result = surf::get(&url).recv_string().await;
    result.unwrap()
}

struct Nephilia {
    pub urls: Vec<String>,
}

impl Nephilia {
    fn new(urls: Vec<String>) -> Self {
        Nephilia { urls }
    }

    #[propane::generator]
    async fn crawl(self) -> String {
        let futures = self.urls.into_iter().map(|url| get(url));
        for result in futures {
            yield result.await;
        }
    }
}

#[async_std::main]
async fn main() {
    let spider = Nephilia::new(vec![
        String::from("http://quotes.toscrape.com/page/7/"),
        String::from("http://quotes.toscrape.com/page/8/"),
        String::from("http://quotes.toscrape.com/page/9/"),
    ]);
    let stream = spider.crawl();
    pin_mut!(stream);
    while let Some(quote) = stream.next().await {
        println!("got {}", quote);
    }
}
