

use std::collections::HashMap;

pub type AppTable = HashMap<String, Appliance>;

/// An appliance that is connected to the pi
#[derive(Debug, Clone, RustcDecodable)]
pub struct Appliance {
    pub on: bool,
    pub pin: i32,
}
