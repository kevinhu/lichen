use crate::extract_links;

#[cfg(test)]
use pretty_assertions::assert_eq;

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
    assert_eq!(links, vec![] as Vec<String>);
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
    assert_eq!(links, vec!["https://example.com"]);
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
        vec!["https://example.com", "https://example.com/test"]
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

#[test]
fn test_links_with_spaces_in_url() {
    let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <a href="/test with spaces in url">Test</a>
            </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec!["https://example.com/test%20with%20spaces%20in%20url"]
    );
}

#[test]
fn test_links_with_special_characters_in_url() {
    let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <a href="/test-url-with-@-special-characters">Test</a>
            </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec!["https://example.com/test-url-with-@-special-characters"]
    );
}

#[test]
fn test_links_with_multiple_slashes() {
    let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <a href="//example.com">Example</a>
            </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(links, vec!["https://example.com/"]);
}

#[test]
fn test_links_with_empty_href() {
    let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <a href="">Empty Link</a>
            </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(links, vec!["https://example.com/"]);
}

#[test]
fn test_links_with_encoded_urls() {
    let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <a href="/search?q=%20">Search</a>
            </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(links, vec!["https://example.com/search?q=%20"]);
}

#[test]
fn test_links_with_subdomains() {
    let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <a href="https://sub.example.com">Example</a>
            </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(links, vec!["https://sub.example.com"]);
}

#[test]
fn test_large_html_document_with_various_link_types() {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Large HTML Document</title>
        </head>
        <body>
            <header>
                <nav>
                    <ul>
                        <li><a href="/">Home</a></li>
                        <li><a href="/about">About</a></li>
                        <li><a href="/contact">Contact</a></li>
                    </ul>
                </nav>
            </header>
            <main>
                <section>
                    <h1>Welcome to our website</h1>
                    <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
                    <p>Read more about us on our <a href="/about">about page</a>.</p>
                </section>
                <section>
                    <h2>Latest Blog Posts</h2>
                    <ul>
                        <li><a href="/blog/post-1">Blog Post 1</a></li>
                        <li><a href="/blog/post-2">Blog Post 2</a></li>
                        <li><a href="/blog/post-3">Blog Post 3</a></li>
                    </ul>
                </section>
                <section>
                    <h2>External Links</h2>
                    <ul>
                        <li><a href="https://example.com">Example</a></li>
                        <li><a href="https://example.org">Example Org</a></li>
                        <li><a href="https://example.net">Example Net</a></li>
                    </ul>
                </section>
            </main>
            <footer>
                <p>&copy; 2023 Our Company. All rights reserved.</p>
                <ul>
                    <li><a href="/terms">Terms of Service</a></li>
                    <li><a href="/privacy">Privacy Policy</a></li>
                </ul>
            </footer>
        </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com",
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/blog/post-1",
            "https://example.com/blog/post-2",
            "https://example.com/blog/post-3",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
            "https://example.net",
            "https://example.org",
        ]
    );
}

#[test]
fn test_large_html_document_with_nested_links() {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Large HTML Document with Nested Links</title>
        </head>
        <body>
            <nav>
                <ul>
                    <li>
                        <a href="/">Home</a>
                        <ul>
                            <li><a href="/home/section1">Section 1</a></li>
                            <li><a href="/home/section2">Section 2</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="/products">Products</a>
                        <ul>
                            <li>
                                <a href="/products/category1">Category 1</a>
                                <ul>
                                    <li><a href="/products/category1/item1">Item 1</a></li>
                                    <li><a href="/products/category1/item2">Item 2</a></li>
                                </ul>
                            </li>
                            <li>
                                <a href="/products/category2">Category 2</a>
                                <ul>
                                    <li><a href="/products/category2/item1">Item 1</a></li>
                                    <li><a href="/products/category2/item2">Item 2</a></li>
                                </ul>
                            </li>
                        </ul>
                    </li>
                    <li>
                        <a href="/services">Services</a>
                        <ul>
                            <li><a href="/services/service1">Service 1</a></li>
                            <li><a href="/services/service2">Service 2</a></li>
                        </ul>
                    </li>
                </ul>
            </nav>
            <main>
                <h1>Welcome to our website</h1>
                <p>Explore our <a href="/products">products</a> and <a href="/services">services</a>.</p>
            </main>
            <footer>
                <p>&copy; 2023 Our Company. All rights reserved.</p>
            </footer>
        </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/home/section1",
            "https://example.com/home/section2",
            "https://example.com/products",
            "https://example.com/products/category1",
            "https://example.com/products/category1/item1",
            "https://example.com/products/category1/item2",
            "https://example.com/products/category2",
            "https://example.com/products/category2/item1",
            "https://example.com/products/category2/item2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_table() {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Large HTML Document with Links in Table</title>
        </head>
        <body>
            <h1>Product Catalog</h1>
            <table>
                <thead>
                    <tr>
                        <th>Product</th>
                        <th>Category</th>
                        <th>Price</th>
                        <th>Details</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>Product 1</td>
                        <td><a href="/categories/category1">Category 1</a></td>
                        <td>$10.00</td>
                        <td><a href="/products/product1">View Details</a></td>
                    </tr>
                    <tr>
                        <td>Product 2</td>
                        <td><a href="/categories/category2">Category 2</a></td>
                        <td>$20.00</td>
                        <td><a href="/products/product2">View Details</a></td>
                    </tr>
                    <tr>
                        <td>Product 3</td>
                        <td><a href="/categories/category1">Category 1</a></td>
                        <td>$15.00</td>
                        <td><a href="/products/product3">View Details</a></td>
                    </tr>
                    <tr>
                        <td>Product 4</td>
                        <td><a href="/categories/category3">Category 3</a></td>
                        <td>$25.00</td>
                        <td><a href="/products/product4">View Details</a></td>
                    </tr>
                    <tr>
                        <td>Product 5</td>
                        <td><a href="/categories/category2">Category 2</a></td>
                        <td>$18.00</td>
                        <td><a href="/products/product5">View Details</a></td>
                    </tr>
                </tbody>
            </table>
        </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/categories/category1",
            "https://example.com/categories/category2",
            "https://example.com/categories/category3",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/products/product3",
            "https://example.com/products/product4",
            "https://example.com/products/product5",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_list() {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Large HTML Document with Links in List</title>
        </head>
        <body>
            <h1>Blog Posts</h1>
            <ul>
                <li>
                    <h2><a href="/blog/post1">Blog Post 1</a></h2>
                    <p>Published on <time datetime="2023-01-01">January 1, 2023</time></p>
                    <p>Category: <a href="/categories/category1">Category 1</a></p>
                    <p>Tags: <a href="/tags/tag1">Tag 1</a>, <a href="/tags/tag2">Tag 2</a></p>
                </li>
                <li>
                    <h2><a href="/blog/post2">Blog Post 2</a></h2>
                    <p>Published on <time datetime="2023-02-15">February 15, 2023</time></p>
                    <p>Category: <a href="/categories/category2">Category 2</a></p>
                    <p>Tags: <a href="/tags/tag2">Tag 2</a>, <a href="/tags/tag3">Tag 3</a></p>
                </li>
                <li>
                    <h2><a href="/blog/post3">Blog Post 3</a></h2>
                    <p>Published on <time datetime="2023-03-10">March 10, 2023</time></p>
                    <p>Category: <a href="/categories/category1">Category 1</a></p>
                    <p>Tags: <a href="/tags/tag1">Tag 1</a>, <a href="/tags/tag4">Tag 4</a></p>
                </li>
                <li>
                    <h2><a href="/blog/post4">Blog Post 4</a></h2>
                    <p>Published on <time datetime="2023-04-20">April 20, 2023</time></p>
                    <p>Category: <a href="/categories/category3">Category 3</a></p>
                    <p>Tags: <a href="/tags/tag3">Tag 3</a>, <a href="/tags/tag5">Tag 5</a></p>
                </li>
            </ul>
        </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/blog/post1",
            "https://example.com/blog/post2",
            "https://example.com/blog/post3",
            "https://example.com/blog/post4",
            "https://example.com/categories/category1",
            "https://example.com/categories/category2",
            "https://example.com/categories/category3",
            "https://example.com/tags/tag1",
            "https://example.com/tags/tag2",
            "https://example.com/tags/tag3",
            "https://example.com/tags/tag4",
            "https://example.com/tags/tag5",
        ]
    );
}

#[test]
fn test_large_html_document_with_various_link_types_and_attributes() {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Large HTML Document with Various Link Types and Attributes</title>
        </head>
        <body>
            <header>
                <nav>
                    <ul>
                        <li><a href="/">Home</a></li>
                        <li><a href="/about" target="_blank">About</a></li>
                        <li><a href="/contact" rel="nofollow">Contact</a></li>
                    </ul>
                </nav>
            </header>
            <main>
                <section>
                    <h1>Welcome to our website</h1>
                    <p>Check out our <a href="/products" title="Products">products</a> and <a href="/services" class="highlight">services</a>.</p>
                    <p>Read more about us on our <a href="/about" target="_self">about page</a>.</p>
                </section>
                <section>
                    <h2>Latest Blog Posts</h2>
                    <ul>
                        <li><a href="/blog/post-1?id=1&category=news">Blog Post 1</a></li>
                        <li><a href="/blog/post-2#comments">Blog Post 2</a></li>
                        <li><a href="/blog/post-3?utm_source=newsletter&utm_medium=email&utm_campaign=summer_sale">Blog Post 3</a></li>
                    </ul>
                </section>
                <section>
                    <h2>External Links</h2>
                    <ul>
                        <li><a href="https://example.com" rel="noopener noreferrer">Example</a></li>
                        <li><a href="https://example.org" target="_top">Example Org</a></li>
                        <li><a href="https://example.net" ping="/tracking/link">Example Net</a></li>
                    </ul>
                </section>
            </main>
            <footer>
                <p>&copy; 2023 Our Company. All rights reserved.</p>
                <ul>
                    <li><a href="/terms" hreflang="en">Terms of Service</a></li>
                    <li><a href="/privacy" type="text/html">Privacy Policy</a></li>
                </ul>
            </footer>
        </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com",
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/blog/post-1?id=1&category=news",
            "https://example.com/blog/post-2#comments",
            "https://example.com/blog/post-3?utm_source=newsletter&utm_medium=email&utm_campaign=summer_sale",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
            "https://example.net",
            "https://example.org",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_different_contexts() {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Large HTML Document with Links in Different Contexts</title>
        </head>
        <body>
            <header>
                <h1><a href="/">Company Logo</a></h1>
                <nav>
                    <ul>
                        <li><a href="/products">Products</a></li>
                        <li><a href="/services">Services</a></li>
                        <li><a href="/about">About</a></li>
                        <li><a href="/contact">Contact</a></li>
                    </ul>
                </nav>
            </header>
            <main>
                <article>
                    <h2><a href="/blog/featured-post">Featured Blog Post</a></h2>
                    <p>Read our <a href="/blog/featured-post">featured blog post</a> to learn more about our latest product launch.</p>
                    <p>For more information, visit our <a href="/products/new-product">new product page</a>.</p>
                    <aside>
                        <h3>Related Posts</h3>
                        <ul>
                            <li><a href="/blog/related-post-1">Related Post 1</a></li>
                            <li><a href="/blog/related-post-2">Related Post 2</a></li>
                        </ul>
                    </aside>
                </article>
                <section>
                    <h2>Latest News</h2>
                    <ul>
                        <li><a href="/news/article-1">Article 1</a></li>
                        <li><a href="/news/article-2">Article 2</a></li>
                        <li><a href="/news/article-3">Article 3</a></li>
                    </ul>
                </section>
                <section>
                    <h2>Customer Testimonials</h2>
                    <blockquote>
                        <p>Great product! I highly recommend it.</p>
                        <cite><a href="/testimonials/john-doe">John Doe</a></cite>
                    </blockquote>
                    <blockquote>
                        <p>Excellent service! The team was very helpful.</p>
                        <cite><a href="/testimonials/jane-smith">Jane Smith</a></cite>
                    </blockquote>
                </section>
            </main>
            <footer>
                <div>
                    <h3>Connect with Us</h3>
                    <ul>
                        <li><a href="https://facebook.com/example">Facebook</a></li>
                        <li><a href="https://twitter.com/example">Twitter</a></li>
                        <li><a href="https://instagram.com/example">Instagram</a></li>
                    </ul>
                </div>
                <div>
                    <h3>Legal</h3>
                    <ul>
                        <li><a href="/terms-of-service">Terms of Service</a></li>
                        <li><a href="/privacy-policy">Privacy Policy</a></li>
                    </ul>
                </div>
                <div>
                    <h3>Contact Us</h3>
                    <ul>
                        <li><a href="/contact">Contact Form</a></li>
                        <li><a href="mailto:info@example.com">Email Us</a></li>
                        <li><a href="tel:+1234567890">Call Us</a></li>
                    </ul>
                </div>
            </footer>
        </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/blog/featured-post",
            "https://example.com/blog/related-post-1",
            "https://example.com/blog/related-post-2",
            "https://example.com/contact",
            "https://example.com/news/article-1",
            "https://example.com/news/article-2",
            "https://example.com/news/article-3",
            "https://example.com/privacy-policy",
            "https://example.com/products",
            "https://example.com/products/new-product",
            "https://example.com/services",
            "https://example.com/terms-of-service",
            "https://example.com/testimonials/jane-smith",
            "https://example.com/testimonials/john-doe",
            "https://facebook.com/example",
            "https://instagram.com/example",
            "https://twitter.com/example",
            "mailto:info@example.com",
            "tel:+1234567890",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_different_languages() {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Large HTML Document with Links in Different Languages</title>
        </head>
        <body>
            <header>
                <nav>
                    <ul>
                        <li><a href="/" hreflang="en">Home</a></li>
                        <li><a href="/fr" hreflang="fr">Accueil</a></li>
                        <li><a href="/es" hreflang="es">Inicio</a></li>
                        <li><a href="/de" hreflang="de">Startseite</a></li>
                    </ul>
                </nav>
            </header>
            <main>
                <section>
                    <h1>Welcome to our multilingual website</h1>
                    <p>
                        <span lang="en">Check out our <a href="/products">products</a> and <a href="/services">services</a>.</span>
                        <span lang="fr">Découvrez nos <a href="/fr/produits">produits</a> et <a href="/fr/services">services</a>.</span>
                        <span lang="es">Descubre nuestros <a href="/es/productos">productos</a> y <a href="/es/servicios">servicios</a>.</span>
                        <span lang="de">Entdecken Sie unsere <a href="/de/produkte">Produkte</a> und <a href="/de/dienstleistungen">Dienstleistungen</a>.</span>
                    </p>
                </section>
                <section>
                    <h2>Latest Blog Posts</h2>
                    <ul>
                        <li>
                            <span lang="en"><a href="/blog/post-1">Blog Post 1</a></span>
                            <span lang="fr"><a href="/fr/blog/article-1">Article 1</a></span>
                            <span lang="es"><a href="/es/blog/articulo-1">Artículo 1</a></span>
                            <span lang="de"><a href="/de/blog/beitrag-1">Beitrag 1</a></span>
                        </li>
                        <li>
                            <span lang="en"><a href="/blog/post-2">Blog Post 2</a></span>
                            <span lang="fr"><a href="/fr/blog/article-2">Article 2</a></span>
                            <span lang="es"><a href="/es/blog/articulo-2">Artículo 2</a></span>
                            <span lang="de"><a href="/de/blog/beitrag-2">Beitrag 2</a></span>
                        </li>
                    </ul>
                </section>
            </main>
            <footer>
                <div>
                    <h3>
                        <span lang="en">Contact Us</span>
                        <span lang="fr">Contactez-nous</span>
                        <span lang="es">Contáctenos</span>
                        <span lang="de">Kontaktieren Sie uns</span>
                    </h3>
                    <ul>
                        <li>
                            <span lang="en"><a href="/contact">Contact Form</a></span>
                            <span lang="fr"><a href="/fr/contact">Formulaire de contact</a></span>
                            <span lang="es"><a href="/es/contacto">Formulario de contacto</a></span>
                            <span lang="de"><a href="/de/kontakt">Kontaktformular</a></span>
                        </li>
                    </ul>
                </div>
            </footer>
        </body>
        </html>
    "#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/blog/post-1",
            "https://example.com/blog/post-2",
            "https://example.com/contact",
            "https://example.com/de",
            "https://example.com/de/blog/beitrag-1",
            "https://example.com/de/blog/beitrag-2",
            "https://example.com/de/dienstleistungen",
            "https://example.com/de/kontakt",
            "https://example.com/de/produkte",
            "https://example.com/es",
            "https://example.com/es/blog/articulo-1",
            "https://example.com/es/blog/articulo-2",
            "https://example.com/es/contacto",
            "https://example.com/es/productos",
            "https://example.com/es/servicios",
            "https://example.com/fr",
            "https://example.com/fr/blog/article-1",
            "https://example.com/fr/blog/article-2",
            "https://example.com/fr/contact",
            "https://example.com/fr/produits",
            "https://example.com/fr/services",
            "https://example.com/products",
            "https://example.com/services",
        ]
    );
}

#[test]
fn test_large_html_document_with_deeply_nested_links() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Large HTML Document with Deeply Nested Links</title>
</head>
<body>
    <nav>
        <ul>
            <li>
                <a href="/">Home</a>
                <ul>
                    <li>
                        <a href="/about">About</a>
                        <ul>
                            <li>
                                <a href="/about/team">Our Team</a>
                                <ul>
                                    <li><a href="/about/team/john">John</a></li>
                                    <li><a href="/about/team/jane">Jane</a></li>
                                    <li><a href="/about/team/mike">Mike</a></li>
                                </ul>
                            </li>
                            <li>
                                <a href="/about/history">Our History</a>
                                <ul>
                                    <li><a href="/about/history/early-years">Early Years</a></li>
                                    <li><a href="/about/history/expansion">Expansion</a></li>
                                    <li><a href="/about/history/recent">Recent History</a></li>
                                </ul>
                            </li>
                        </ul>
                    </li>
                    <li>
                        <a href="/products">Products</a>
                        <ul>
                            <li>
                                <a href="/products/category1">Category 1</a>
                                <ul>
                                    <li><a href="/products/category1/product1">Product 1</a></li>
                                    <li><a href="/products/category1/product2">Product 2</a></li>
                                    <li><a href="/products/category1/product3">Product 3</a></li>
                                </ul>
                            </li>
                            <li>
                                <a href="/products/category2">Category 2</a>
                                <ul>
                                    <li><a href="/products/category2/product4">Product 4</a></li>
                                    <li><a href="/products/category2/product5">Product 5</a></li>
                                    <li><a href="/products/category2/product6">Product 6</a></li>
                                </ul>
                            </li>
                        </ul>
                    </li>
                    <li>
                        <a href="/services">Services</a>
                        <ul>
                            <li><a href="/services/service1">Service 1</a></li>
                            <li><a href="/services/service2">Service 2</a></li>
                            <li><a href="/services/service3">Service 3</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="/contact">Contact</a>
                        <ul>
                            <li><a href="/contact/form">Contact Form</a></li>
                            <li><a href="/contact/phone">Phone</a></li>
                            <li><a href="/contact/email">Email</a></li>
                        </ul>
                    </li>
                </ul>
            </li>
        </ul>
    </nav>
</body>
</html>
"#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/about/history",
            "https://example.com/about/history/early-years",
            "https://example.com/about/history/expansion",
            "https://example.com/about/history/recent",
            "https://example.com/about/team",
            "https://example.com/about/team/jane",
            "https://example.com/about/team/john",
            "https://example.com/about/team/mike",
            "https://example.com/contact",
            "https://example.com/contact/email",
            "https://example.com/contact/form",
            "https://example.com/contact/phone",
            "https://example.com/products",
            "https://example.com/products/category1",
            "https://example.com/products/category1/product1",
            "https://example.com/products/category1/product2",
            "https://example.com/products/category1/product3",
            "https://example.com/products/category2",
            "https://example.com/products/category2/product4",
            "https://example.com/products/category2/product5",
            "https://example.com/products/category2/product6",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/services/service3",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_comments_and_scripts() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Large HTML Document with Links in Comments and Scripts</title>
</head>
<body>
    <header>
        <nav>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/contact">Contact</a></li>
            </ul>
        </nav>
    </header>
    <main>
        <section>
            <h1>Welcome to our website</h1>
            <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
        </section>
        <section>
            <h2>Latest Blog Posts</h2>
            <ul>
                <li><a href="/blog/post1">Blog Post 1</a></li>
                <li><a href="/blog/post2">Blog Post 2</a></li>
                <li><a href="/blog/post3">Blog Post 3</a></li>
            </ul>
        </section>
    </main>
    <footer>
        <p>&copy; 2023 Our Company. All rights reserved.</p>
        <ul>
            <li><a href="/terms">Terms of Service</a></li>
            <li><a href="/privacy">Privacy Policy</a></li>
        </ul>
    </footer>

    <!-- Links in comments should not be extracted -->
    <!-- <a href="/comment-link1">Comment Link 1</a> -->
    <!-- <a href="/comment-link2">Comment Link 2</a> -->

    <script>
        // Links in scripts should not be extracted
        const scriptLink1 = '<a href="/script-link1">Script Link 1</a>';
        const scriptLink2 = '<a href="/script-link2">Script Link 2</a>';
    </script>
</body>
</html>
"#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/blog/post1",
            "https://example.com/blog/post2",
            "https://example.com/blog/post3",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_different_encodings() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Large HTML Document with Links in Different Encodings</title>
</head>
<body>
    <header>
        <nav>
            <ul>
                <li><a href="/">Home</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/contact">Contact</a></li>
            </ul>
        </nav>
    </header>
    <main>
        <section>
            <h1>Welcome to our website</h1>
            <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
            <p>Here are some links with different encodings:</p>
            <ul>
                <li><a href="/link-with-spaces">Link with Spaces</a></li>
                <li><a href="/link-with-special-chars!@#$%^&*()">Link with Special Characters</a></li>
                <li><a href="/link-with-unicode-chars-✓🌍🚀">Link with Unicode Characters</a></li>
                <li><a href="/link-with-encoded-chars%21%40%23%24%25%5E%26%2A%28%29">Link with Encoded Characters</a></li>
            </ul>
        </section>
    </main>
    <footer>
        <p>&copy; 2023 Our Company. All rights reserved.</p>
        <ul>
            <li><a href="/terms">Terms of Service</a></li>
            <li><a href="/privacy">Privacy Policy</a></li>
        </ul>
    </footer>
</body>
</html>
"#;

    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/link-with-encoded-chars%21%40%23%24%25%5E%26%2A%28%29",
            "https://example.com/link-with-spaces",
            "https://example.com/link-with-special-chars!@#$%^&*()",
            "https://example.com/link-with-unicode-chars-%E2%9C%93%F0%9F%8C%8D%F0%9F%9A%80",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_complex_structure() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Complex Structure</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li>
 <a href="/products">Products</a>
 <ul>
 <li><a href="/products/category1">Category 1</a></li>
 <li><a href="/products/category2">Category 2</a></li>
 <li><a href="/products/category3">Category 3</a></li>
 </ul>
 </li>
 <li><a href="/services">Services</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/featured-products">featured products</a> and <a href="/latest-news">latest news</a>.</p>
 <div>
 <h2>Categories</h2>
 <ul>
 <li><a href="/categories/electronics">Electronics</a></li>
 <li><a href="/categories/clothing">Clothing</a></li>
 <li><a href="/categories/home-appliances">Home Appliances</a></li>
 <li><a href="/categories/sports">Sports</a></li>
 </ul>
 </div>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 <li><a href="/products/product3">Product 3</a></li>
 <li><a href="/products/product4">Product 4</a></li>
 </ul>
 </div>
 <div>
 <h2>Latest News</h2>
 <ul>
 <li><a href="/news/article1">Article 1</a></li>
 <li><a href="/news/article2">Article 2</a></li>
 <li><a href="/news/article3">Article 3</a></li>
 <li><a href="/news/article4">Article 4</a></li>
 </ul>
 </div>
 </section>
 <section>
 <h2>About Us</h2>
 <p>Learn more about our <a href="/company-history">company history</a> and <a href="/team">team</a>.</p>
 <div>
 <h3>Our Mission</h3>
 <p>Our mission is to provide high-quality products and excellent customer service.</p>
 </div>
 <div>
 <h3>Our Vision</h3>
 <p>Our vision is to become the leading company in our industry.</p>
 </div>
 </section>
 <section>
 <h2>Contact Us</h2>
 <p>Get in touch with us via our <a href="/contact-form">contact form</a> or <a href="/store-locator">find a store near you</a>.</p>
 <div>
 <h3>Customer Support</h3>
 <ul>
 <li><a href="/support/faq">FAQ</a></li>
 <li><a href="/support/shipping-info">Shipping Information</a></li>
 <li><a href="/support/returns-exchanges">Returns &amp; Exchanges</a></li>
 </ul>
 </div>
 <div>
 <h3>Connect with Us</h3>
 <ul>
 <li><a href="https://www.facebook.com/example">Facebook</a></li>
 <li><a href="https://www.twitter.com/example">Twitter</a></li>
 <li><a href="https://www.instagram.com/example">Instagram</a></li>
 </ul>
 </div>
 </section>
 </main>
 <footer>
 <div>
 <h4>Quick Links</h4>
 <ul>
 <li><a href="/sitemap">Sitemap</a></li>
 <li><a href="/privacy-policy">Privacy Policy</a></li>
 <li><a href="/terms-of-service">Terms of Service</a></li>
 </ul>
 </div>
 <div>
 <h4>Newsletter</h4>
 <p>Subscribe to our <a href="/newsletter">newsletter</a> for the latest updates and promotions.</p>
 </div>
 <div>
 <p>&copy; 2023 Example Company. All rights reserved.</p>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/categories/clothing",
            "https://example.com/categories/electronics",
            "https://example.com/categories/home-appliances",
            "https://example.com/categories/sports",
            "https://example.com/company-history",
            "https://example.com/contact",
            "https://example.com/contact-form",
            "https://example.com/featured-products",
            "https://example.com/latest-news",
            "https://example.com/news/article1",
            "https://example.com/news/article2",
            "https://example.com/news/article3",
            "https://example.com/news/article4",
            "https://example.com/newsletter",
            "https://example.com/privacy-policy",
            "https://example.com/products",
            "https://example.com/products/category1",
            "https://example.com/products/category2",
            "https://example.com/products/category3",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/products/product3",
            "https://example.com/products/product4",
            "https://example.com/services",
            "https://example.com/sitemap",
            "https://example.com/store-locator",
            "https://example.com/support/faq",
            "https://example.com/support/returns-exchanges",
            "https://example.com/support/shipping-info",
            "https://example.com/team",
            "https://example.com/terms-of-service",
            "https://www.facebook.com/example",
            "https://www.instagram.com/example",
            "https://www.twitter.com/example",
        ]
    );
}

#[test]
fn test_huge_html_document_with_invalid_and_malformed_links() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Invalid and Malformed Links</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 <li><a href="invalid-link">Invalid Link</a></li>
 <li><a href="http://example.com">HTTP Link</a></li>
 <li><a href="ftp://example.com">FTP Link</a></li>
 <li><a href="mailto:info@example.com">Email Link</a></li>
 <li><a href="javascript:void(0)">JavaScript Link</a></li>
 <li><a href="\#section1">Fragment Link</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <p>Here are some invalid and malformed links:</p>
 <ul>
 <li><a href="/valid-link">Valid Link</a></li>
 <li><a href="/invalid-link-with-spaces ">Invalid Link with Spaces</a></li>
 <li><a href="/invalid-link-with-<brackets>">Invalid Link with Brackets</a></li>
 <li><a href="/invalid-link-with-'quotes'">Invalid Link with Quotes</a></li>
 <li><a href="/invalid-link-with-"double-quotes"">Invalid Link with Double Quotes</a></li>
 <li><a href="/invalid-link-with-&ampersand&">Invalid Link with Ampersand</a></li>
 <li><a>Missing Href Attribute</a></li>
 <li><a href="">Empty Href Attribute</a></li>
 <li><a href="/relative-link-without-quotes">Relative Link without Quotes</a></li>
 <li><a href='/relative-link-with-single-quotes'>Relative Link with Single Quotes</a></li>
 <li><a href=`/relative-link-with-backticks`>Relative Link with Backticks</a></li>
 </ul>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Example Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "ftp://example.com",
            "http://example.com",
            "https://example.com/",
            "https://example.com/#section1",
            "https://example.com/%60/relative-link-with-backticks%60",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/invalid-link",
            "https://example.com/invalid-link-with-",
            "https://example.com/invalid-link-with-%3Cbrackets%3E",
            "https://example.com/invalid-link-with-&ampersand&",
            "https://example.com/invalid-link-with-'quotes'",
            "https://example.com/invalid-link-with-spaces",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/relative-link-with-single-quotes",
            "https://example.com/relative-link-without-quotes",
            "https://example.com/services",
            "https://example.com/terms",
            "https://example.com/valid-link",
            "javascript:void(0)",
            "mailto:info@example.com"
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_different_cases() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Different Cases</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a HREF="/about">About</a></li>
 <li><A Href="/contact">Contact</A></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <A HREF="/services">services</A>.</p>
 <p>Here are some links in different cases:</p>
 <ul>
 <li><a href="/lowercase-link">Lowercase Link</a></li>
 <li><a href="/UPPERCASE-LINK">Uppercase Link</a></li>
 <li><a href="/MixedCase-Link">MixedCase Link</a></li>
 <li><a href="/link-with-UPPERCASE-path">Link with Uppercase Path</a></li>
 <li><a HREF="/link-with-UPPERCASE-tag">Link with Uppercase Tag</a></li>
 <li><A Href="/link-with-MixedCase-tag">Link with MixedCase Tag</A></li>
 </ul>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Example Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><A HREF="/privacy">Privacy Policy</A></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/MixedCase-Link",
            "https://example.com/UPPERCASE-LINK",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/link-with-MixedCase-tag",
            "https://example.com/link-with-UPPERCASE-path",
            "https://example.com/link-with-UPPERCASE-tag",
            "https://example.com/lowercase-link",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_svg() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Large HTML Document with Links in SVG</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
 <a href="/svg-link1">
 <circle cx="50" cy="50" r="40" fill="blue" />
 <text x="50" y="50" text-anchor="middle" fill="white">SVG Link 1</text>
 </a>
 <a href="/svg-link2">
 <rect x="10" y="60" width="80" height="30" fill="red" />
 <text x="50" y="80" text-anchor="middle" fill="white">SVG Link 2</text>
 </a>
 </svg>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/svg-link1",
            "https://example.com/svg-link2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_data_attributes() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Large HTML Document with Links in Data Attributes</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <ul>
 <li><a href="/link1" data-external-link="/external-link1">Link 1</a></li>
 <li><a href="/link2" data-related-link="/related-link2">Link 2</a></li>
 <li><a href="/link3" data-image-link="/image-link3">Link 3</a></li>
 </ul>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/link1",
            "https://example.com/link2",
            "https://example.com/link3",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_html_entities() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Large HTML Document with Links in HTML Entities</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <p>Here are some links with HTML entities:</p>
 <ul>
 <li><a href="/?param1=value1&amp;param2=value2">Link with &amp;amp;</a></li>
 <li><a href="/greater-than-&gt;-symbol">Link with &amp;gt;</a></li>
 <li><a href="/less-than-&lt;-symbol">Link with &amp;lt;</a></li>
 <li><a href="/single-quote-&apos;-symbol">Link with &amp;apos;</a></li>
 <li><a href="/double-quote-&quot;-symbol">Link with &amp;quot;</a></li>
 </ul>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/?param1=value1&amp;param2=value2",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/double-quote-&quot;-symbol",
            "https://example.com/greater-than-&gt;-symbol",
            "https://example.com/less-than-&lt;-symbol",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/single-quote-&apos;-symbol",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_different_protocols() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Large HTML Document with Links in Different Protocols</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <p>Here are some links with different protocols:</p>
 <ul>
 <li><a href="http://example.com">HTTP Link</a></li>
 <li><a href="https://example.com">HTTPS Link</a></li>
 <li><a href="ftp://example.com">FTP Link</a></li>
 <li><a href="mailto:info@example.com">Email Link</a></li>
 <li><a href="tel:+1234567890">Phone Link</a></li>
 <li><a href="sms:+1234567890">SMS Link</a></li>
 <li><a href="file:///path/to/file">File Link</a></li>
 <li><a href="data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==">Data URL Link</a></li>
 <li><a href="javascript:alert('Hello, World!')">JavaScript Link</a></li>
 </ul>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==",
            "file:///path/to/file",
            "ftp://example.com",
            "http://example.com",
            "https://example.com",
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
            "javascript:alert('Hello, World!')",
            "mailto:info@example.com",
            "sms:+1234567890",
            "tel:+1234567890",
        ]
    );
}

#[test]
fn test_large_html_document_with_links_in_different_cases() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Large HTML Document with Links in Different Cases</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <p>Here are some links in different cases:</p>
 <ul>
 <li><a href="/lowercase-link">lowercase link</a></li>
 <li><a href="/UPPERCASE-LINK">UPPERCASE LINK</a></li>
 <li><a href="/CamelCaseLink">CamelCaseLink</a></li>
 <li><a href="/snake_case_link">snake_case_link</a></li>
 <li><a href="/kebab-case-link">kebab-case-link</a></li>
 <li><a href="/MixedCase-WithHyphens_AndUnderscores">MixedCase-WithHyphens_AndUnderscores</a></li>
 </ul>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/CamelCaseLink",
            "https://example.com/MixedCase-WithHyphens_AndUnderscores",
            "https://example.com/UPPERCASE-LINK",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/kebab-case-link",
            "https://example.com/lowercase-link",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/snake_case_link",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_different_encodings() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Different Encodings</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <p>Here are some links with different encodings:</p>
 <ul>
 <li><a href="/link-with-spaces">Link with Spaces</a></li>
 <li><a href="/link-with-special-chars!@#$%^&*()">Link with Special Characters</a></li>
 <li><a href="/link-with-unicode-chars-✓🌍🚀">Link with Unicode Characters</a></li>
 <li><a href="/link-with-encoded-chars%21%40%23%24%25%5E%26%2A%28%29">Link with Encoded Characters</a></li>
 <li><a href="/link-with-mixed-encodings-Ā Ē Ī Ō Ū ā ē ī ō ū">Link with Mixed Encodings</a></li>
 <li><a href="/link-with-punycode-xn--80ak6aa92e">Link with Punycode</a></li>
 <li><a href="/link-with-url-encoded-chars%C4%80%C4%92%C4%AA%C5%8C%C5%AA%C4%81%C4%93%C4%AB%C5%8D%C5%AB">Link with URL Encoded Characters</a></li>
 </ul>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
links,
vec![
"https://example.com/",
"https://example.com/about",
"https://example.com/contact",
"https://example.com/link-with-encoded-chars%21%40%23%24%25%5E%26%2A%28%29",
"https://example.com/link-with-mixed-encodings-%C4%80%20%C4%92%20%C4%AA%20%C5%8C%20%C5%AA%20%C4%81%20%C4%93%20%C4%AB%20%C5%8D%20%C5%AB",
"https://example.com/link-with-punycode-xn--80ak6aa92e",
"https://example.com/link-with-spaces",
"https://example.com/link-with-special-chars!@#$%^&*()",
"https://example.com/link-with-unicode-chars-%E2%9C%93%F0%9F%8C%8D%F0%9F%9A%80",
"https://example.com/link-with-url-encoded-chars%C4%80%C4%92%C4%AA%C5%8C%C5%AA%C4%81%C4%93%C4%AB%C5%8D%C5%AB",
"https://example.com/privacy",
"https://example.com/products",
"https://example.com/services",
"https://example.com/terms",
 ]
 );
}
#[test]
fn test_huge_html_document_with_links_in_different_contexts() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Different Contexts</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <p>Here are some links in different contexts:</p>
 <ul>
 <li><a href="/link-in-paragraph">Link in Paragraph</a></li>
 <li><a href="/link-in-list-item">Link in List Item</a></li>
 <li><a href="/link-in-table-cell">Link in Table Cell</a></li>
 <li><a href="/link-in-blockquote">Link in Blockquote</a></li>
 <li><a href="/link-in-details-summary">Link in Details Summary</a></li>
 <li><a href="/link-in-figure-caption">Link in Figure Caption</a></li>
 <li><a href="/link-in-form-label">Link in Form Label</a></li>
 </ul>
 <table>
 <tr>
 <td><a href="/link-in-table-cell">Link in Table Cell</a></td>
 </tr>
 </table>
 <blockquote>
 <p><a href="/link-in-blockquote">Link in Blockquote</a></p>
 </blockquote>
 <details>
 <summary><a href="/link-in-details-summary">Link in Details Summary</a></summary>
 <p>Details content</p>
 </details>
 <figure>
 <img src="/image.jpg" alt="Image">
 <figcaption><a href="/link-in-figure-caption">Link in Figure Caption</a></figcaption>
 </figure>
 <form>
 <label for="input"><a href="/link-in-form-label">Link in Form Label</a></label>
 <input type="text" id="input">
 </form>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/link-in-blockquote",
            "https://example.com/link-in-details-summary",
            "https://example.com/link-in-figure-caption",
            "https://example.com/link-in-form-label",
            "https://example.com/link-in-list-item",
            "https://example.com/link-in-paragraph",
            "https://example.com/link-in-table-cell",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
        ]
    );
}
#[test]
fn test_huge_html_document_with_links_in_different_languages() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Different Languages</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our multilingual website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <p>Here are some links in different languages:</p>
 <ul>
 <li><a href="/link-in-english">Link in English</a></li>
 <li><a href="/link-in-french">Lien en français</a></li>
 <li><a href="/link-in-spanish">Enlace en español</a></li>
 <li><a href="/link-in-german">Link auf Deutsch</a></li>
 <li><a href="/link-in-italian">Link in italiano</a></li>
 <li><a href="/link-in-portuguese">Link em português</a></li>
 <li><a href="/link-in-russian">Ссылка на русском</a></li>
 <li><a href="/link-in-chinese">中文链接</a></li>
 <li><a href="/link-in-japanese">日本語のリンク</a></li>
 <li><a href="/link-in-korean">한국어 링크</a></li>
 </ul>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/link-in-chinese",
            "https://example.com/link-in-english",
            "https://example.com/link-in-french",
            "https://example.com/link-in-german",
            "https://example.com/link-in-italian",
            "https://example.com/link-in-japanese",
            "https://example.com/link-in-korean",
            "https://example.com/link-in-portuguese",
            "https://example.com/link-in-russian",
            "https://example.com/link-in-spanish",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
        ]
    );
}
#[test]
fn test_huge_html_document_with_links_in_metadata() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Metadata</title>
 <link rel="canonical" href="https://example.com/canonical-url">
 <link rel="alternate" href="https://example.com/alternate-url" hreflang="en">
 <link rel="stylesheet" href="/styles.css">
 <link rel="icon" href="/favicon.ico">
 <link rel="manifest" href="/manifest.json">
 <link rel="preload" href="/preload-resource" as="script">
 <link rel="prefetch" href="/prefetch-resource">
 <link rel="preconnect" href="https://example.com">
 <link rel="dns-prefetch" href="//example.com">
 <link rel="modulepreload" href="/module.js">
 <link rel="pingback" href="https://example.com/pingback">
 <link rel="search" href="/search" type="application/opensearchdescription+xml" title="Search">
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 </section>
 </main>
 <footer>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/services",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_nested_elements() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Nested Elements</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li>
 <a href="/">
 <span>
 <strong>Home</strong>
 <em>Page</em>
 </span>
 </a>
 </li>
 <li>
 <a href="/about">
 <div>
 <p>About</p>
 <p>Us</p>
 </div>
 </a>
 </li>
 <li>
 <a href="/contact">
 <span>
 <span>Contact</span>
 <span>Information</span>
 </span>
 </a>
 </li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products"><span><em>amazing</em> products</span></a> and <a href="/services"><strong>top-notch</strong> services</a>.</p>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li>
 <a href="/products/product1">
 <div>
 <h3>Product 1</h3>
 <p>Description of Product 1</p>
 </div>
 </a>
 </li>
 <li>
 <a href="/products/product2">
 <div>
 <h3>Product 2</h3>
 <p>Description of Product 2</p>
 </div>
 </a>
 </li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li>
 <a href="/services/service1">
 <span>
 <strong>Service 1</strong>
 <em>Description</em>
 </span>
 </a>
 </li>
 <li>
 <a href="/services/service2">
 <span>
 <strong>Service 2</strong>
 <em>Description</em>
 </span>
 </a>
 </li>
 </ul>
 </div>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li>
 <a href="/terms">
 <span>Terms of Service</span>
 </a>
 </li>
 <li>
 <a href="/privacy">
 <span>Privacy Policy</span>
 </a>
 </li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_non_standard_attributes() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Non-Standard Attributes</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a data-url="/">Home</a></li>
 <li><a data-link="/about">About</a></li>
 <li><a data-href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a data-product-link="/products">products</a> and <a data-service-url="/services">services</a>.</p>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a data-item-url="/products/product1">Product 1</a></li>
 <li><a data-item-link="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a data-service-href="/services/service1">Service 1</a></li>
 <li><a data-service-link="/services/service2">Service 2</a></li>
 </ul>
 </div>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a data-terms-url="/terms">Terms of Service</a></li>
 <li><a data-privacy-link="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(links, Vec::<&str>::new());
}

#[test]
fn test_huge_html_document_with_links_in_javascript() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in JavaScript</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a href="/services/service1">Service 1</a></li>
 <li><a href="/services/service2">Service 2</a></li>
 </ul>
 </div>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
 <script>
 // Links in JavaScript should not be extracted
 const jsLink1 = '<a href="/js-link1">JavaScript Link 1</a>';
 const jsLink2 = '<a href="/js-link2">JavaScript Link 2</a>';
 const jsLinks = [
 '<a href="/js-link3">JavaScript Link 3</a>',
 '<a href="/js-link4">JavaScript Link 4</a>',
 ];
 const jsLinksHtml = `
 <a href="/js-link5">JavaScript Link 5</a>
 <a href="/js-link6">JavaScript Link 6</a>
 `;
 </script>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_css() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in CSS</title>
 <style>
 /* Links in CSS should not be extracted */
 a[href="/css-link1"] {
 color: blue;
 }
 a[href='/css-link2'] {
 text-decoration: none;
 }
 a[href="/css-link3"]:hover {
 color: red;
 }
 a[href="/css-link4"]:active {
 font-weight: bold;
 }
 a[href="/css-link5"]:visited {
 color: purple;
 }
 </style>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a href="/services/service1">Service 1</a></li>
 <li><a href="/services/service2">Service 2</a></li>
 </ul>
 </div>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_html_comments() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in HTML Comments</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <!-- Links in HTML comments should not be extracted -->
 <!-- <a href="/comment-link1">Comment Link 1</a> -->
 <!-- <a href="/comment-link2">Comment Link 2</a> -->
 <!--
 <a href="/comment-link3">Comment Link 3</a>
 <a href="/comment-link4">Comment Link 4</a>
 -->
 <!-- <a href="/comment-link5">
 Comment Link 5
 </a> -->
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a href="/services/service1">Service 1</a></li>
 <li><a href="/services/service2">Service 2</a></li>
 </ul>
 </div>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_invalid_html() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Invalid HTML</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <!-- Invalid HTML should not affect link extraction -->
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a href="/services/service1">Service 1</a></li>
 <li><a href="/services/service2">Service 2</a></li>
 </ul>
 </div>
 <p>Some invalid HTML follows:</p>
 <a href="/invalid-link1">Invalid Link 1
 <a href="/invalid-link2">Invalid Link 2</a>
 <a href="/invalid-link3">Invalid Link 3
 <p>More invalid HTML:</p>
 <ul>
 <li><a href="/invalid-link4">Invalid Link 4
 <li><a href="/invalid-link5">Invalid Link 5</a>
 </ul>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/invalid-link1",
            "https://example.com/invalid-link2",
            "https://example.com/invalid-link3",
            "https://example.com/invalid-link4",
            "https://example.com/invalid-link5",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_xml_namespaces() {
    let html = r#"
<!DOCTYPE html>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml" xmlns:og="http://ogp.me/ns#" xmlns:fb="http://www.facebook.com/2008/fbml">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in XML Namespaces</title>
 <link rel="canonical" href="https://example.com/canonical-url">
 <meta property="og:url" content="https://example.com/og-url">
 <meta property="og:image" content="https://example.com/og-image.jpg">
 <meta property="og:image:secure_url" content="https://example.com/og-image-secure.jpg">
 <meta property="og:image:url" content="https://example.com/og-image-url.jpg">
 <meta property="fb:app_id" content="1234567890">
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a href="/services/service1">Service 1</a></li>
 <li><a href="/services/service2">Service 2</a></li>
 </ul>
 </div>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_svg_and_math_ml() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in SVG and MathML</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a href="/services/service1">Service 1</a></li>
 <li><a href="/services/service2">Service 2</a></li>
 </ul>
 </div>
 <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
 <a href="/svg-link1">
 <circle cx="50" cy="50" r="40" fill="red" />
 <text x="50" y="50" text-anchor="middle" fill="white">SVG Link 1</text>
 </a>
 </svg>
 <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
 <a href="/svg-link2">
 <rect x="10" y="10" width="80" height="80" fill="blue" />
 <text x="50" y="50" text-anchor="middle" fill="white">SVG Link 2</text>
 </a>
 </svg>
 <math xmlns="http://www.w3.org/1998/Math/MathML">
 <mi><a href="/mathml-link1">x</a></mi>
 <mo>+</mo>
 <mi><a href="/mathml-link2">y</a></mi>
 <mo>=</mo>
 <mn><a href="/mathml-link3">1</a></mn>
 </math>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/mathml-link1",
            "https://example.com/mathml-link2",
            "https://example.com/mathml-link3",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/svg-link1",
            "https://example.com/svg-link2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_html_entities_and_cdata() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in HTML Entities and CDATA</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a href="/services/service1">Service 1</a></li>
 <li><a href="/services/service2">Service 2</a></li>
 </ul>
 </div>
 <p>Links with HTML entities: <a href="/html-entity-link1">HTML Entity Link 1 &amp; &lt; &gt;</a></p>
 <p>Links with CDATA: <![CDATA[<a href="/cdata-link1">CDATA Link 1</a>]]></p>
 <script>
 // <![CDATA[
 var link = '<a href="/cdata-link2">CDATA Link 2</a>';
 // ]]>
 </script>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/html-entity-link1",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_doctype_and_processing_instructions() {
    let html = r#"
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd" [
 <!ENTITY link1 "<a href='/entity-link1'>Entity Link 1</a>">
 <!ENTITY link2 "<a href='/entity-link2'>Entity Link 2</a>">
]>
<?xml-stylesheet type="text/css" href="/style.css"?>
<?php echo '<a href="/php-link">PHP Link</a>'; ?>
<html lang="en" xmlns="http://www.w3.org/1999/xhtml">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Doctype and Processing Instructions</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a href="/services/service1">Service 1</a></li>
 <li><a href="/services/service2">Service 2</a></li>
 </ul>
 </div>
 <p>&link1;</p>
 <p>&link2;</p>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/terms",
        ]
    );
}

#[test]
fn test_huge_html_document_with_links_in_conditional_comments_and_noscript() {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
 <meta charset="UTF-8">
 <title>Huge HTML Document with Links in Conditional Comments and Noscript</title>
</head>
<body>
 <header>
 <nav>
 <ul>
 <li><a href="/">Home</a></li>
 <li><a href="/about">About</a></li>
 <li><a href="/contact">Contact</a></li>
 </ul>
 </nav>
 </header>
 <main>
 <section>
 <h1>Welcome to our website</h1>
 <p>Check out our <a href="/products">products</a> and <a href="/services">services</a>.</p>
 <div>
 <h2>Featured Products</h2>
 <ul>
 <li><a href="/products/product1">Product 1</a></li>
 <li><a href="/products/product2">Product 2</a></li>
 </ul>
 </div>
 <div>
 <h2>Popular Services</h2>
 <ul>
 <li><a href="/services/service1">Service 1</a></li>
 <li><a href="/services/service2">Service 2</a></li>
 </ul>
 </div>
 <!--[if IE]>
 <p>You are using Internet Explorer.</p>
 <![endif]-->
 <!--[if !IE]><!-->
 <p>You are not using Internet Explorer.</p>
 <!--<![endif]-->
 <!--[if IE 7]>
 <p>You are using Internet Explorer 7.</p>
 <a href="/ie7-link">IE7 Link</a>
 <![endif]-->
 <!--[if IE 8]>
 <p>You are using Internet Explorer 8.</p>
 <a href="/ie8-link">IE8 Link</a>
 <![endif]-->
 <!--[if IE 9]>
 <p>You are using Internet Explorer 9.</p>
 <a href="/ie9-link">IE9 Link</a>
 <![endif]-->
 <noscript>
 <p>JavaScript is disabled.</p>
 <a href="/noscript-link">Noscript Link</a>
 </noscript>
 </section>
 </main>
 <footer>
 <div>
 <p>&copy; 2023 Our Company. All rights reserved.</p>
 <ul>
 <li><a href="/terms">Terms of Service</a></li>
 <li><a href="/privacy">Privacy Policy</a></li>
 </ul>
 </div>
 </footer>
</body>
</html>
"#;
    let links = extract_links(html, "https://example.com").unwrap();
    assert_eq!(
        links,
        vec![
            "https://example.com/",
            "https://example.com/about",
            "https://example.com/contact",
            "https://example.com/privacy",
            "https://example.com/products",
            "https://example.com/products/product1",
            "https://example.com/products/product2",
            "https://example.com/services",
            "https://example.com/services/service1",
            "https://example.com/services/service2",
            "https://example.com/terms",
        ]
    );
}
