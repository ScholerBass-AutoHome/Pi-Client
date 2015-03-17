
extern crate "bodyparser" as bp;

use std::sync::{Arc, RwLock};

use iron::prelude::*;
use iron::status;
use iron::{Handler};

use apps::*;

pub struct Deleter {
    table: Arc<RwLock<AppTable>>,
}
