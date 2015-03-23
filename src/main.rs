
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

use handlers::{Toggler, Creator, Deleter};
use apps::Appliance;

#[macro_export]
macro_rules! json_body {
    ($req:ident, $t:ty) => (match $req.get::<bp::Struct<$t>>() {
        Ok(Some(body)) => body,
        _ => return Ok(Response::with(status::BadRequest)),
    })
}

mod apps;
mod handlers;

fn main() {
    let mut table = apps::AppTable::new();

    table.insert(
        "lamp".to_string(),
        Appliance {
           on: false,
           pin: 0,
        }
    );

    let rc_table = Arc::new(RwLock::new(table));
    let mut router = Router::new();

    router.post("/toggle-apps", Toggler {
        table: rc_table.clone(),
    });
    router.post("/delete-apps", Deleter {
        table: rc_table.clone()    
    });
    router.post("/create-apps", Creator {
        table: rc_table.clone() 
    });

    Iron::new(router).http("autohome.local:3000").map(|_| {
        println!("Server is live");
    });
}
