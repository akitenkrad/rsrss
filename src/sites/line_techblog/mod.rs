use crate::sites::{Category, Html, Site, Text, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
use scraper::Selector;
pub struct LineTechBlog {}

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
impl Site for LineTechBlog {
    fn name(&self) -> String {
        return "LINE Engineering Blog".to_string();
    }
    fn category(&self) -> Category {
        return Category::Blog;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://techblog.lycorp.co.jp/ja/feed/index.xml".to_string();
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
        let doc = scraper::Html::parse_document(&body);
        let sel = Selector::parse("main div.content").unwrap();
        let text = doc.select(&sel).next().unwrap().text().collect();
        let html = doc.select(&sel).next().unwrap().html().to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
