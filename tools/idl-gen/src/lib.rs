mod gen {
    include!(concat!(env!("OUT_DIR"), "/volo_gen.rs"));
}

use std::net::IpAddr;

pub use gen::volo_gen::*;
use volo::FastStr;

impl From<(IpAddr, u16)> for info::Host {
    fn from(val: (IpAddr, u16)) -> Self {
        info::Host {
            ip_addr: FastStr::new(val.0.to_string()),
            port: val.1 as u32,
        }
    }
}
