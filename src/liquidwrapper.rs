extern crate liquid;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::default::Default;

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

impl TemplateWrapper for LiquidTemplate {
    fn new(template_string: String) -> LiquidTemplate {
        let mut options : LiquidOptions = Default::default();

        let template : Box<Renderable> =
            Box::new(parse(&template_string, options).unwrap());

        LiquidTemplate {
            template_string: template_string,
            liquid_template: template,
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

