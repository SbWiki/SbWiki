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
