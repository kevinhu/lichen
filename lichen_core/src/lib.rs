use lol_html::{element, HtmlRewriter, Settings};
use url::Url;
use std::error::Error;

pub fn extract_links(html: &str, base_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut links = vec![];

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("a[href]", |el| {
                let href = el.get_attribute("href");

                match href {
                    Some(href) => {
                        links.push(href);
                    }
                    None => {}
                }

                Ok(())
            })],
            ..Settings::default()
        },
        |_: &[u8]| {},
    );

    rewriter.write(html.as_bytes())?;
    rewriter.end()?;

    let base_url = Url::parse(base_url)?;

    let links = links
        .into_iter()
        .filter_map(|link| {
            let parsed_link = Url::parse(&link);

            match parsed_link {
                Ok(parsed_link) => {
                    if parsed_link.host_str().is_none() {
                        let joined_url = base_url.clone().join(&link);
                        match joined_url {
                            Ok(joined_url) => Some(joined_url.to_string()),
                            Err(_) => None,
                        }
                    } else {
                        Some(link)
                    }
                }
                Err(_) => {
                    let new_url = base_url.clone().join(&link);
                    match new_url {
                        Ok(new_url) => Some(new_url.to_string()),
                        Err(_) => None,
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(links)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_html() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec![] as Vec<String>);
    }

    #[test]
    fn test_absolute_links() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="https://example.com">Example</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec!["https://example.com"]);
    }

    #[test]
    fn test_missing_href() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a>Example</a>
                    <a href="/test">Example</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec!["https://example.com/test"]);
    }

    #[test]
    fn test_no_backslash() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a>Example</a>
                    <a href="google">Example</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec!["https://example.com/google"]);
    }

    #[test]
    fn test_invalid_base() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="/test">Example</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "invalid_url");
        assert!(links.is_err());
    }

    #[test]
    fn test_relative_links() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="/test">Test</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec!["https://example.com/test"]);
    }

    #[test]
    fn test_relative_parent() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="../test">Test</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com/what").unwrap();
        assert_eq!(links, vec!["https://example.com/test"]);
    }

    #[test]
    fn test_http_absolute() {
        let html = r#"
            <html>
                <body>
                    <a href="http://example.com">Test</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec!["http://example.com"]);
    }

    #[test]
    fn test_http_relative() {
        let html = r#"
            <html>
                <body>
                    <a href="/test">Test</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "http://example.com").unwrap();
        assert_eq!(links, vec!["http://example.com/test"]);
    }

    #[test]
    fn test_both() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="https://example.com">Example</a>
                    <a href="/test">Test</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(
            links,
            vec!["https://example.com", "https://example.com/test"]
        );
    }

    #[test]
    fn test_no_links() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <p>No links here</p>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec![]as Vec<String>);
    }

    #[test]
    fn test_multiple_same_links() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="https://example.com">Example</a>
                    <a href="https://example.com">Example</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec!["https://example.com", "https://example.com"]);
    }

    #[test]
    fn test_links_in_nested_tags() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <div>
                        <a href="/test">Test</a>
                        <span>
                            <a href="https://example.com">Example</a>
                        </span>
                    </div>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(
            links,
            vec!["https://example.com/test", "https://example.com"]
        );
    }

    #[test]
    fn test_links_with_fragments() {
        let html = r##"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="#section">Section</a>
                </body>
            </html>
        "##;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec!["https://example.com/#section"]);
    }

    #[test]
    fn test_links_with_fragments_outside_root() {
        let html = r##"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="#section">Section</a>
                </body>
            </html>
        "##;

        let links = extract_links(html, "https://example.com/test").unwrap();
        assert_eq!(links, vec!["https://example.com/test#section"]);
    }

    #[test]
    fn test_links_with_query_parameters() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="/search?q=test">Search</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com").unwrap();
        assert_eq!(links, vec!["https://example.com/search?q=test"]);
    }

    #[test]
    fn test_port_in_base_url() {
        let html = r#"
            <html>
                <head>
                    <title>Test</title>
                </head>
                <body>
                    <a href="/test">Test</a>
                </body>
            </html>
        "#;

        let links = extract_links(html, "https://example.com:8080").unwrap();
        assert_eq!(links, vec!["https://example.com:8080/test"]);
    }
}
