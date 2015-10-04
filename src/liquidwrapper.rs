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

pub struct LiquidTemplate<'a, 'b> {
    template_string: String,
    liquid_template: Option<Box<Renderable + 'a>>,
    liquid_options : LiquidOptions<'b>,
}

impl<'a, 'b> TemplateWrapper<'a> for LiquidTemplate<'a, 'b> {
    fn new(template_string: String) -> LiquidTemplate<'a, 'b> {
        let mut options : LiquidOptions = Default::default();

        LiquidTemplate {
            template_string: template_string,
            liquid_options : options,
            liquid_template: None,
        }
    }

    fn parse(&'a mut self) {
        self.liquid_template = Some(
            Box::new(
                    liquid::parse(&self.template_string, &mut self.liquid_options).unwrap()
                )
            );
    }

    fn render(&self, data: HashMap<String, String>) -> String {
        let mut con = Context::new();
        
        for (key, value) in data {
            con.set_val(&key, Value::Str(value));
        }

        return match self.liquid_template {
            Some(ref tpl) => tpl.render(&mut con).unwrap(),
            None => panic!("pan!")
        }
    }
}

