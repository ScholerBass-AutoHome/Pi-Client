extern crate iron;
extern crate router;
extern crate "bodyparser" as bp;
extern crate "rustc-serialize" as rustc_serialize;

use std::sync::{Arc, RwLock};

use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use iron::Url;
use router::Router;

use handlers::Toggler;
use apps::Appliance;

mod apps;
mod handlers;

fn handler(req: &mut Request) -> IronResult<Response> {
    println!("Request received:");

    let json = req.get::<bp::Json>();
    
    println!("{:?}", json);
    let query = req.extensions.find::<Router>().unwrap();

    Ok(Response::with(status::Ok))
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

    let rc_table = Arc::new(RwLock::new(table));
    let mut router = Router::new();

    router.post("/toggle-apps", Toggler {
        table: rc_table.clone()
    });
    router.post("/delete-apps", handler);
    router.post("/create-apps", handler);

    Iron::new(router).http("localhost:3000").unwrap();
    println!("Server is live");
}
