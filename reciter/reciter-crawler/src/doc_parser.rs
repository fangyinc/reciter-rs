#[cfg(test)]
mod tests {
    use scraper::Html;
    use scraper::Selector;

    #[test]
    fn test_parser_html() {
        let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;
        let document = Html::parse_document(html);
        let r = document.select(&Selector::parse("h1").unwrap()).next().unwrap();
        let element = r.value();
        let class = element.attr("class");
        println!("element: {:?}", element);
        assert_eq!("foo", class.unwrap());
    }
}