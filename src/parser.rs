use anyhow::Result;
use scraper::{Html, Selector};
use url::Url;

pub fn extract_links(body: &str, base: &Url) -> Vec<Url> {
    let document = Html::parse_document(body);
    let selector = Selector::parse("a[href]").unwrap();
    document.select(&selector)
        .filter_map(|elem| elem.value().attr("href"))
        .filter_map(|href| base.join(href).ok())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links() {
        let html = r#"<a href="/foo">Foo</a>"#;
        let base = Url::parse("https://example.com").unwrap();
        let links = extract_links(html, &base);
        assert_eq!(links, vec![Url::parse("https://example.com/foo").unwrap()]);
    }
}
