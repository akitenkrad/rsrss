use crate::sites::{Category, Html, Site, Text, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct TrendMicroSecurityNews {}

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
impl Site for TrendMicroSecurityNews {
    fn name(&self) -> String {
        return "Trend Micro Security News".to_string();
    }
    fn category(&self) -> Category {
        return Category::Security;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "http://feeds.trendmicro.com/jp/NewestMalware".to_string();
        let body = self.request(&url).await;
        let feeds = parsers::rss2::parse(&body).unwrap();
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                site: self.name(),
                title: feed.title,
                url: feed.link,
                description: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc2822(&feed.publish_date.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return Ok(articles);
    }
    async fn get_article_text(&self, url: &String) -> Result<(Html, Text), String> {
        let body = self.request(url).await;
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("section.TEArticle div.articleContainer").unwrap();
        let text = document
            .select(&selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("\n");
        let html = document
            .select(&selector)
            .next()
            .unwrap()
            .html()
            .to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}