use scraper::{ElementRef, Selector};

pub trait ElementRefExtensions<'a> {
    fn select_string(&self, selector: &str) -> String;
}

impl<'a> ElementRefExtensions<'a> for ElementRef<'a> {
    fn select_string(&self, selector: &str) -> String {
        self.select(&Selector::parse(selector).unwrap())
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_string()
    }
}

pub trait HtmlExtensions {
    fn select_attr(&self, selector: &str, attr_name: &str) -> String;
}

impl HtmlExtensions for scraper::Html {
    fn select_attr(&self, selector: &str, attr_name: &str) -> String {
        self.select(&Selector::parse(selector).unwrap())
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .value()
            .attr(attr_name)
            .unwrap()
            .to_string()
    }
}