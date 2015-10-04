use std::collections::HashMap;

pub trait TemplateWrapper<'a> {
    fn new(template_string: String) -> Self;
    fn parse(&'a mut self);
    fn render(&self, HashMap<String, String>) -> String;
}
