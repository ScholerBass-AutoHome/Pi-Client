extern crate iron;
extern crate router;
extern crate "bodyparser" as bp;
extern crate "rustc-serialize" as rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use iron::Url;

use router::Router;

use apps::Appliance;

mod apps;
mod handler;

fn handler(req: &mut Request) -> IronResult<Response> {
    let json = req.get::<bp::Json>();
    let query = req.extensions.find::<Router>().unwrap();

    Ok(Response::with( (status::Ok, "HAA") ))
}

fn main() {
    let mut table = apps::AppTable::new();

    table.insert(
        "lamp".to_string(),
        Appliance {
           on: false,
           pin: 0
        }
    );

    let mut router = Router::new();

    router.get("/apps/:app", handler);
    router.get("/apps/:app/toggle", handler);

    router.get("/get-app", handler);
    router.post("/set-app", handler);

    Iron::new(router).http("localhost:3000").unwrap();
}
