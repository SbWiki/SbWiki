#![feature(collections)]
#![feature(convert)]

extern crate toml;
extern crate iron;
extern crate router;

use std::fs::File;
use std::io::Read;

//use iron::Iron;
use iron::prelude::*;
use iron::headers::*;
use iron::{status, headers};

//use core::str::Str;

use router::{Router};

#[derive(Clone)]
struct SbWikiServer {
    listenaddr: String,
    wikipath: String,
    wikifrontpage: String,
}

impl SbWikiServer {
    //TODO: pass config object instead of bunch of argument
    pub fn new(cfgfile: &'static str) -> SbWikiServer {
        let listenaddr    = String::from_str("localhost:31337");
        let wikipath      = String::from_str("/wiki");
        let wikifrontpage = String::from_str("FrontPage");

        //loading toml configure file
        let mut fd = File::open(cfgfile).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        fd.read_to_end(&mut buf);
        let data = String::from_utf8(buf).unwrap();
        let mut parser = toml::Parser::new(data.as_str());

        match parser.parse() {
            Some(value) => println!("Value from config file! : {:?}", value),
            None => println!("No data!"),
        } // temporal routine of parsing config file
        
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
        //TODO: fetch it from toml config.
        let wpath = self.wikipath.clone();
        let listenaddr = self.listenaddr.clone();

        let cloned = self.clone();
        router.get(wpath.as_str(), 
                   move |req: &mut Request| -> IronResult<Response>
                   {cloned.wikiredirect(req)});
        
        let cloned = self.clone();
        router.get(wikidocument.as_str(),
                   move |req: &mut Request| -> IronResult<Response>
                   {cloned.wikihandler(req)});
 
        let cloned = self.clone();
        router.get("/",
                   move |req: &mut Request| -> IronResult<Response>
                   {cloned.roothandler(req)});
        
        let cloned = self.clone();
        router.get("/:query",
                   move |req: &mut Request| -> IronResult<Response>
                   {cloned.roothandler(req)});
        
        Iron::new(router).http(
            listenaddr.as_str()).unwrap();
    }
    
    fn roothandler(&self, req: &mut Request)
                   -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");

        Ok(Response::with((status::Ok, *query)))
    }
    
    fn wikiredirect(&self, req: &mut Request)
                    -> IronResult<Response> {
        let mut url = req.url.path.clone();
        let mut header = Headers::new();
        let mut newpath = String::new();

        //TODO: customizable root page.
        url.push(self.wikifrontpage.clone());

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

    fn wikihandler(&self, req: &mut Request) 
                   -> IronResult<Response> {
        let ref requrl = req.url.clone();
        let mut urlpath = requrl.path.clone();
        
        for i in urlpath.iter() {
            println!("{}", i);
        }

        Ok(Response::with((status::Ok, "Watch console.")))
    }
}

fn main() {
    //TODO: make it can fetch config file name from command line argument.
    let sbwiki: SbWikiServer = SbWikiServer::new("config.toml");

    sbwiki.open(false);
}
