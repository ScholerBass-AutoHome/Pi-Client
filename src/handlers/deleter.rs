
extern crate "bodyparser" as bp;

use std::sync::{Arc, RwLock};

use iron::prelude::*;
use iron::status;
use iron::{Handler};

use apps::*;

pub struct Deleter {
    pub table: Arc<RwLock<AppTable>>,
}

impl Handler for Deleter {
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
            match table.remove(name) {
                Some(app) => {
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
