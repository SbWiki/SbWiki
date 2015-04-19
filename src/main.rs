#![feature(collections)]

extern crate toml;
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::headers::*;
use iron::{status, headers};

use router::{Router};


fn main() {
    let mut router = Router::new();

    //TODO: fetch it from toml config.
    router.get("/wiki", wikiredirect);
    router.get("/wiki/*", wikihandler);
    
    router.get("/", roothandler);
    router.get("/:query", roothandler);

    Iron::new(router).http("localhost:3000").unwrap();

    fn roothandler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");

        Ok(Response::with((status::Ok, *query)))
    }

    fn wikiredirect(req: &mut Request) -> IronResult<Response> {
        let mut url = req.url.path.clone();
        let mut header = Headers::new();
        let mut newpath = String::new();

        //TODO: customizable root page.
        url.push(String::from_str("FrontPage"));

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

    fn wikihandler(req: &mut Request) -> IronResult<Response> {
        let ref requrl = req.url.clone();
        let mut urlpath = requrl.path.clone();
        
        for i in urlpath.iter() {
            println!("{}\n", i);
        }

        Ok(Response::with((status::Ok, "Watch console.")))
    }
}
