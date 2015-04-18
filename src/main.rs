extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::{Router};

fn main() {
    let mut router = Router::new();

    router.get("/", roothandler);
    router.get("/:query", roothandler);

    Iron::new(router).http("localhost:3000").unwrap();

    fn roothandler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");

        Ok(Response::with((status::Ok, *query)))
    }
}
