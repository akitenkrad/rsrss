use crate::sites::{Category, Html, Site, Text, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct IPASecurityCenter {}

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
impl Site for IPASecurityCenter {
    fn name(&self) -> String {
        return "IPA Security Center".to_string();
    }
    fn category(&self) -> Category {
        return Category::Security;
    }
    async fn get_articles(&self) -> Result<Vec<WebArticle>, String> {
        let url = "https://www.ipa.go.jp/security/rss/alert.rdf".to_string();
        let body = self.request(&url).await;
        let feeds = parsers::rss1::parse(&body).unwrap();
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                site: self.name(),
                title: feed.title,
                url: feed.link,
                description: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc3339(&feed.date.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return Ok(articles);
    }
    async fn get_article_text(&self, url: &String) -> Result<(Html, Text), String> {
        let body = self.request(url).await;
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse(
            "div.news-detail main h1.ttl,h2.ttl,p.article-txt,span.list__item__txt",
        )
        .unwrap();
        let mut text = String::new();
        for p in document.select(&selector) {
            text.push_str(&p.text().collect::<Vec<_>>().join("\n"));
            text.push_str("\n");
        }
        let html = document
            .select(&selector)
            .next()
            .unwrap()
            .html()
            .to_string();
        return Ok((self.trim_text(&html), self.trim_text(&text)));
    }
}
