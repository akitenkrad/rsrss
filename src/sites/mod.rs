use chrono::{DateTime, Local};
use futures::Future;
use regex::Regex;

pub enum Category {
    Blog,
    Organization,
    Security,
    News,
}

pub trait InfoItem {
    fn title(&self) -> String;
    fn url(&self) -> String;
    fn description(&self) -> String;
    fn timestamp(&self) -> DateTime<Local>;
}

#[derive(Debug)]
pub struct WebArticle {
    pub title: String,
    pub url: String,
    pub text: String,
    pub timestamp: DateTime<Local>,
}

impl InfoItem for WebArticle {
    fn title(&self) -> String {
        return self.title.clone();
    }

    fn url(&self) -> String {
        return self.url.clone();
    }

    fn description(&self) -> String {
        return self.text.clone();
    }

    fn timestamp(&self) -> DateTime<Local> {
        return self.timestamp.clone();
    }
}

pub trait Site {
    fn name(&self) -> String;
    fn category(&self) -> Category;
    fn get_articles(&self) -> impl Future<Output = Result<Vec<WebArticle>, String>> + Send;
    fn get_article_text(&self, url: &String)
        -> impl Future<Output = Result<String, String>> + Send;
    fn to_slack_message(&self, article: &WebArticle) -> String {
        return format!(
            "{}\n{}\n{}",
            article.title(),
            article.url(),
            article.description()
        );
    }
    fn trim_text(&self, text: &String) -> String {
        // let ptn = r#"\.?[0-9a-zA-Z\-]+\s*[0-9a-zA-Z:;="'\s\(\)\{\}!\?/,]+"#;
        // let re = Regex::new(ptn).unwrap();
        // let trimmed_text = re.replace_all(text, "").to_string();

        let re = Regex::new(r"\s\s+").unwrap();
        let trimmed_text = re.replace_all(text, "\n").to_string();
        return trimmed_text;
    }
    fn request(&self, url: &String) -> impl Future<Output = String> + Send {
        async move {
            let client = reqwest::Client::new();
            let body = client
                .get(url)
                .header(
                    reqwest::header::USER_AGENT,
                    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
                )
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            return body;
        }
    }
}

mod ai_it_now;
mod aws_security_blog;
mod business_insider_science;
mod business_insider_technology;
mod canon_malware_center;
mod codezine;
mod cookpad_techblog;
mod crowdstrike_blog;
mod cyberagent_techblog;
mod cybozu_blog;
mod dena_engineering_blog;
mod gigazine;
mod github_developers_blog;
mod gizmodo;
mod google_developers_blog;
mod gree_techblog;
mod gunosy_techblog;
mod hatena_bookmark_it;
mod hatena_developer_blog;
mod ipa_security_center;
mod itmedia_at_it;
mod itmedia_enterprise;
mod itmedia_general;
mod itmedia_marketing;
mod jpcert;
mod line_techblog;
mod macafee_security_news;
mod mercari_engineering_blog;
mod moneyforward_developers_blog;
mod motex;
mod nikkei_xtech;
mod qiita_blog;
mod retrieva_techblog;
mod sakura_internet_techblog;
mod sansan;
mod security_next;
mod sophos_news;
mod stockmark_news;
mod stockmark_techblog;
mod supership;
mod tokyo_univ_engineering;
mod trend_micro_security_advisories;
mod trend_micro_security_blog;
mod yahoo_japan_techblog;
mod yahoo_news_it;
mod yahoo_news_science;
mod zen_mu_tech;
mod zenn_topic_nlp;
mod zenn_trend;
