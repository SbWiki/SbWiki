#![feature(collections)]

extern crate toml;
extern crate iron;
extern crate router;
extern crate core;

//use iron::Iron;
use iron::prelude::*;
use iron::headers::*;
use iron::{status, headers};

//use core::str::Str;

use router::{Router};

struct SbWikiServer<'sbwikisrv> {
    listenaddr: String,
    wikipath: String,
    wikifrontpage: String,
}

impl SbWikiServer {
    //TODO: pass config object instead of bunch of argument
    pub fn new(listenaddr: String, wikipath: String,
           wikifrontpage: String) -> SbWikiServer {
        SbWikiServer {
            listenaddr: listenaddr,
            wikipath: wikipath,
            wikifrontpage: wikifrontpage,
        }
    }

    pub fn open(&'sbwikisrv self, isTLS: bool) {
        let mut wikidocument = self.wikipath.clone();
        wikidocument.push_str("/*");

        let mut router = Router::new();
        //TODO: fetch it from toml config.
        
        router.get(self.wikipath.as_str(), 
                   |req: &mut Request| -> IronResult<Response>
                   {self.wikiredirect(req)});
        
        router.get(wikidocument.as_str(),
                   |req: &mut Request| -> IronResult<Response>
                   {self.wikihandler(req)});
    
        router.get("/",
                   |req: &mut Request| -> IronResult<Response>
                   {self.roothandler(req)});/*
        router.get("/:query",
                        |req: &mut Request| &self.roothandler(req));
        */
        //let mut iron = Iron::new(router);
        //iron.http(self.listenaddr).unwrap();

        //self.iron = iron;

        Iron::new(router).http(
            self.listenaddr.clone().as_str()).unwrap();
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
        url.push(self.wikifrontpage);

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
            println!("{}\n", i);
        }

        Ok(Response::with((status::Ok, "Watch console.")))
    }
}

fn main() {
    let sbwiki: SbWikiServer = SbWikiServer::new(
        String::from_str("localhost:3000"),
        String::from_str("/wiki"),
        String::from_str("FrontPage"));

    sbwiki.open(false);
}
