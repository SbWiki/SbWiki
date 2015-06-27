extern crate liquid;

use std::fs::File;
use std::io::Read;

use templatewrapper::TemplateWrapper;

pub struct LiquidTemplate {
    template: String,
}

impl TemplateWrapper for LiquidTemplate {
    fn new(template_file: String) -> LiquidTemplate {

        let mut template = String::new();
        File::open(template_file).unwrap().read_to_string(&mut template);
        
        LiquidTemplate {
            template: template,
        }
    }
}

impl liquid::Renderable for LiquidTemplate {
    fn render(&self, context: &mut liquid::Context) -> Option<String> {
        let mut options : liquid::LiquidOptions = Default::default();
        let parsed = liquid::parse(&self.template, &mut options).unwrap();
        parsed.render(context)
    }
}


