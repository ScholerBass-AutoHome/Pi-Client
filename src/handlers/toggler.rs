
extern crate "bodyparser" as bp;

use std::sync::{Arc, RwLock};

use iron::prelude::*;
use iron::status;
use iron::{Handler};

use apps::*;

pub struct Toggler {
    pub table: Arc<RwLock<AppTable>>,
}

impl Handler for Toggler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let names = match req.get::<bp::Struct<Vec<String>>>() {
            Ok(Some(body)) => body,
            _ => {
                return Ok(Response::with(status::BadRequest));
            }
        };

        let mut table = self.table.write().unwrap();
        let mut result = String::new();

        for name in names.iter() {
            match table.get_mut(name) {
                Some(app) => {
                    app.on = !app.on;
                    result.push_str(&format!("{}: {:?}\n", name, app)[..]);
                },
                None => {
                    return Ok(Response::with(status::BadRequest));
                }
            }
        }
        
        Ok(Response::with((status::Ok, &result[..])))
    }
}
