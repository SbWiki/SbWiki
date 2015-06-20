use templatewrapper::TemplateWrapper;

pub struct LiquidTemplate {
    template: String,
}

impl TemplateWrapper for LiquidTemplate {
    fn new() -> LiquidTemplate {
        LiquidTemplate {
            template: String::from("hello"),
        }
    }
}
