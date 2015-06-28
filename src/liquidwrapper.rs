extern crate liquid;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use liquid::Context;
use liquid::Renderable;
use liquid::Value;
use liquid::LiquidOptions;
use liquid::parse;

use templatewrapper::TemplateWrapper;

pub struct LiquidTemplate {
    template_string: String,
    liquid_template: Box<Renderable>,
}

impl<'a> TemplateWrapper for LiquidTemplate {
    fn new(template_string: String) -> LiquidTemplate {
        let mut options = Default::default();
        let template = parse(&template_string, &mut options).unwrap();
        
        LiquidTemplate {
            template_string: template_string,
            liquid_template: Box::new(template),
        }
    }
    
    fn render(&self, data: HashMap<String, String>) -> String {
        let mut con = Context::new();
        
        for (key, value) in data {
            con.set_val(&key, Value::Str(value));
        }

        return self.liquid_template.render(&mut con).unwrap();
    }
}

