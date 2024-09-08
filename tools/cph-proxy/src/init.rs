use std::thread::JoinHandle;

use crate::cfg::init_refresh_global_cfg;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() -> JoinHandle<()> {
    tracing_subscriber::registry().with(fmt::layer()).init();
    init_refresh_global_cfg()
}
