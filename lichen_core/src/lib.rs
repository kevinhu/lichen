use ahash::AHashSet;
use lol_html::{element, HtmlRewriter, Settings};
use std::error::Error;
use url::Url;

#[cfg(test)]
mod tests;

pub fn extract_links(html: &str, base_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut links: AHashSet<String> = AHashSet::new();

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("a[href]", |el| {
                let href = el.get_attribute("href");

                match href {
                    Some(href) => {
                        links.insert(href.to_string());
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

    let mut links = links
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
        .collect::<AHashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    links.sort();

    Ok(links)
}
