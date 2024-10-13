use crate::sites::{Category, Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct MacAfeeSecurityNews {}

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
impl Site for MacAfeeSecurityNews {
    fn name(&self) -> String {
        return "MacAfee Security News".to_string();
    }
    fn category(&self) -> Category {
        return Category::Security;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://www.mcafee.com/blogs/ja-jp/mcafee-news/feed/".to_string();
        let body = self.request(&url).await;
        let feeds = parsers::rss2::parse(&body).unwrap();
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                site: self.name(),
                title: feed.title,
                url: feed.link,
                text: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc2822(&feed.publish_date.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return Ok(articles);
    }
    async fn get_article_text(&self, url: &String) -> Result<String, String> {
        let body = self.request(url).await;
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("div.the_content h1,h2,h3,p").unwrap();
        let mut text = String::new();
        for p in document.select(&selector) {
            text.push_str(&p.text().collect::<Vec<_>>().join("\n"));
            text.push_str("\n");
        }
        return Ok(self.trim_text(&text));
    }
}
