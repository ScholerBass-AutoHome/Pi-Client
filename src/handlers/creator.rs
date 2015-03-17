
extern crate "bodyparser" as bp;

use std::sync::{Arc, RwLock};

use iron::prelude::*;
use iron::status;
use iron::{Handler};

use apps::*;

#[derive(Debug, Clone, RustcDecodable)]
pub struct JsonAppliance {
    name: String,
    pin: i32,
}

pub struct Creator {
    pub table: Arc<RwLock<AppTable>>,
}

impl Handler for Creator {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let app_requests = match req.get::<bp::Struct<Vec<JsonAppliance>>>() {
            Ok(Some(body)) => body,
            _ => {
                return Ok(Response::with(status::BadRequest));
            }
        };

        let mut table = self.table.write().unwrap();
        let mut result = String::new();

        for app_req in app_requests {
            let appliance = Appliance {
                pin: app_req.pin ,
                on: false,
            };

            result.push_str(&format!("{}: {:?}\n", app_req.name, appliance));

            table.insert(app_req.name, appliance); 
        }
        
        Ok(Response::with((status::Ok, &result[..])))
    }
}
