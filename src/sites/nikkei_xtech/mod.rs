use crate::sites::{Category, Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct NikkeiXTech {}

#[cfg(test)]
mod tests;

impl Site for NikkeiXTech {
    fn name(&self) -> String {
        return "Nikkei XTech".to_string();
    }
    fn category(&self) -> Category {
        return Category::News;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://xtech.nikkei.com/rss/index.rdf".to_string();
        let body = self.request(&url).await;
        let feeds = parsers::rss1::parse(&body).unwrap();
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                title: feed.title,
                url: feed.link,
                text: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc3339(&feed.date.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return Ok(articles);
    }
    async fn get_article_text(&self, url: &String) -> Result<String, String> {
        let body = self.request(url).await;
        let document = scraper::Html::parse_document(&body);
        let selector =
            scraper::Selector::parse("main article div.p-article div.p-article_body").unwrap();
        let article = document.select(&selector).next().unwrap();
        let text = article.text().collect::<Vec<_>>().join("\n");
        return Ok(self.trim_text(&text));
    }
}
