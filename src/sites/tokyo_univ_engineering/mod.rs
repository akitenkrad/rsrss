use crate::sites::{Site, WebArticle};
use chrono::DateTime;
use feed_parser::parsers;
pub struct TokyoUniversityEngineering {}

#[cfg(test)]
mod tests;

impl Site for TokyoUniversityEngineering {
    fn name(&self) -> String {
        return "Tokyo University Enginerring".to_string();
    }
    async fn get_articles(&self) -> Vec<WebArticle> {
        let client = reqwest::Client::new();
        let body = client
            .get("https://www.t.u-tokyo.ac.jp/press/rss.xml")
            .header(reqwest::header::USER_AGENT, self.user_agent())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let feeds = parsers::rss2::parse(&body).unwrap();
        let mut articles = Vec::new();
        for feed in feeds {
            articles.push(WebArticle {
                title: feed.title,
                url: feed.link,
                text: feed.description.unwrap_or("".to_string()),
                timestamp: DateTime::parse_from_rfc2822(&feed.publish_date.unwrap())
                    .unwrap()
                    .into(),
            });
        }
        return articles;
    }
    async fn get_article_text(&self, url: &String) -> String {
        let client = reqwest::Client::new();
        let body = client
            .get(url)
            .header(reqwest::header::USER_AGENT, self.user_agent())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let document = scraper::Html::parse_document(&body);
        let selector =
            scraper::Selector::parse("main div.ly_cont div.blog_title,div.bl_wysiwyg").unwrap();
        let mut text = String::new();
        for p in document.select(&selector) {
            text.push_str(&p.text().collect::<Vec<_>>().join("\n"));
            text.push_str("\n");
        }
        return self.trim_text(&text);
    }
}
