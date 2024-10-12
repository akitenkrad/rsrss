use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct Sansan {}

#[cfg(test)]
mod tests;

impl Site for Sansan {
    fn name(&self) -> String {
        return "Sansan".to_string();
    }
    async fn get_articles(&self) -> Vec<WebArticle> {
        let body = reqwest::get("https://buildersbox.corp-sansan.com/feed")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let feeds = parsers::atom::parse(&body).unwrap();
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                title: feed.title,
                url: feed.link,
                text: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc3339(&feed.publish_date.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return articles;
    }
    async fn get_article_text(&self, url: &String) -> String {
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        let document = scraper::Html::parse_document(&body);
        let selector =
            scraper::Selector::parse("#main article div.entry-inner div.entry-content").unwrap();
        if let Some(article) = document.select(&selector).next() {
            let text = article.text().collect::<Vec<_>>().join("\n");
            return self.trim_text(&text);
        } else {
            return "NO CONTENT".to_string();
        }
    }
}
