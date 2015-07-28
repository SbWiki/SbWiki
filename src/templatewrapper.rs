use std::collections::HashMap;

pub trait TemplateWrapper {
    fn new(template_string: String) -> Self;
    fn parse(&self);
    fn render(&self, HashMap<String, String>) -> String;
}
