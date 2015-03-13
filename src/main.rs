extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use iron::Url;

use router::Router;

use apps::Appliance;

mod apps;
mod handler;

fn handler(req: &mut Request) -> IronResult<Response> {
    let query = req.extensions.get::<Router>().unwrap();
    let found = query.find("app");

    Ok(Response::with( (status::Ok, format!("{:?}", found)) ))
}

fn main() {
    let mut table = apps::AppTable::new();

    table.insert(
        "lamp",
        Appliance {

        }
    );

    let mut router = Router::new();

    router.get("/apps/:app", handler);
    router.get("/apps/:app/toggle", Toggler(

    Iron::new(router).http("localhost:3000").unwrap();
}
