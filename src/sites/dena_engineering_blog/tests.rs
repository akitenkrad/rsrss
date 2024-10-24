use super::*;

#[test]
fn test_dena_engineering_blog() {
    let site = DeNAEngineeringBlog {};
    let articles = tokio_test::block_on(site.get_articles());
    if let Ok(articles) = articles {
        assert!(articles.len() > 0);

        for article in articles.iter() {
            assert!(article.url.starts_with("https://engineering.dena.com/blog"));
        }

        let article = articles.get(0).unwrap();
        println!("Article: {:?}", article);
        let html_and_text = tokio_test::block_on(site.get_article_text(&article.url));
        match html_and_text {
            Ok(html_and_text) => {
                let (html, text) = html_and_text;
                println!("HTML: {}", html);
                println!("Text: {}", text);
                assert!(html.len() > 0);
                assert!(text.len() > 0);
            }
            Err(e) => {
                println!("Error: {}", e);
                assert!(false);
            }
        }
    } else {
        assert!(false);
    }
}
