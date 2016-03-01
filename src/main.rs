extern crate toml;
extern crate iron;
extern crate router;
extern crate liquid;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::convert::From;
use std::borrow::Borrow;

//use iron::Iron;
use iron::prelude::*;
use iron::headers::{Header, HeaderFormat, Headers, Location, ContentType};
use iron::{status, headers};

//use core::str::Str;

use router::{Router};

use liquid::Context;
use liquid::Renderable;
use liquid::Value;

use liquidwrapper::LiquidTemplate;
use templatewrapper::TemplateWrapper;

mod templatewrapper;
mod liquidwrapper;

struct SbWikiServer {
    listenaddr: String,
    wikipath: String,
    wikifrontpage: String,
}

impl SbWikiServer {
    //TODO: pass config object instead of bunch of argument
    pub fn new(cfgfile: &String) -> SbWikiServer {
        let mut listenaddr = String::new();
        let wikipath       = String::new();
        let wikifrontpage  = String::from("FrontPage");

        //loading toml configure file
        let mut confs = String::new();
        File::open(cfgfile).unwrap().read_to_string(&mut confs);
        let mut parser = toml::Parser::new(confs.borrow() as &str);
        
        //add portnum to address
        let root = parser.parse().unwrap();
        let port = root["server"].lookup("port").unwrap();
        let host = root["server"].lookup("host").unwrap();

        let portstr = format!("{}:{}",
                              host.as_str().unwrap(),
                              port.as_integer().unwrap());

        listenaddr.push_str(portstr.borrow());

        //get base path from toml config
        let wikipath = root["wiki"].lookup("basepath").unwrap();
        let wikipath = format!("{}", wikipath.as_str().unwrap());
        
        println!("{}", listenaddr);

        SbWikiServer {
            listenaddr: listenaddr,
            wikipath: wikipath,
            wikifrontpage: wikifrontpage,
        }
    }

    pub fn open(self, isTLS: bool) {
        let mut wikidocument = self.wikipath.clone();
        wikidocument.push_str("/*");

        let mut router = Router::new();
        let wpath      = self.wikipath.clone();
        let listenaddr = self.listenaddr.clone();

        router.get(wpath.borrow() as &str, 
                   move |req: &mut Request| -> IronResult<Response>
                   {SbWikiServer::wikiredirect(&self, req)});
        
        router.get(wikidocument.borrow() as &str,
                   move |req: &mut Request| -> IronResult<Response>
                   {SbWikiServer::wikihandler(req)});
 
        router.get("/",
                   move |req: &mut Request| -> IronResult<Response>
                   {SbWikiServer::roothandler(req)});
        
        router.get("/:query",
                   move |req: &mut Request| -> IronResult<Response>
                   {SbWikiServer::roothandler(req)});
        
        Iron::new(router).http(
            listenaddr.borrow() as &str).unwrap();
    }
    
    fn roothandler(req: &mut Request)
                   -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");

        Ok(Response::with((status::Ok, *query)))
    }
    
    fn wikiredirect(wiki: &SbWikiServer, req: &mut Request)
                    -> IronResult<Response> {
        let mut url     = req.url.path.clone();
        let mut header  = Headers::new();
        let mut newpath = String::new();

        //TODO: customizable root page.
        url.push(wiki.wikifrontpage.clone());

        for i in url.iter() {
            newpath.push_str("/");
            newpath.push_str(i);
        }

        println!("{}", newpath);
        header.set(Location(newpath));

        let mut resp: Response = Response::with((status::MovedPermanently));
        resp.headers = header;
        
        Ok(resp)
    }

    fn wikihandler(req: &mut Request) 
                   -> IronResult<Response> {
        let ref requrl  = req.url.clone();
        let mut header  = Headers::new();
        let mut urlpath = requrl.path.clone();
        
        for i in urlpath.iter() {
            println!("{}", i);
        }

        //Liquid testing code
        let mut replace_table: HashMap<String, String> = HashMap::new();
        replace_table.insert(String::from("message"),
                             String::from("Hello, liquid!"));
        let mut con = Context::new();
        for (key, value) in replace_table {
            con.set_val(&key, Value::Str(value));
        }
        let mut template_str = String::from("hello.liquid");
        let template = LiquidTemplate::new(template_str);
        let page = template.render(&mut con).unwrap();

        header.set(
            ContentType(
                iron::mime::Mime(
                    iron::mime::TopLevel::Text,
                    iron::mime::SubLevel::Html,
                    vec![])
            )
        );
        
        let mut resp: Response = Response::with((status::Ok, page));
        resp.headers = header;

        Ok(resp)
    }
}

fn main() {
    //load config file with command line arguments
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        panic!("Usage: {} config.toml", argv[0]);
    }
    let sbwiki: SbWikiServer = SbWikiServer::new(&argv[1]);

    sbwiki.open(false);
}

