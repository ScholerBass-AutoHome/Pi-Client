
use std::sync::{Arc, RwLock};

use iron::prelude::*;
use iron::{Handler};

use apps::*;

pub struct Toggler {
    table: Arc<RwLock<AppTable>>,
}

impl Handler for Toggler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let table = self.table.write().unwrap();
        let app_name           
    }
}
