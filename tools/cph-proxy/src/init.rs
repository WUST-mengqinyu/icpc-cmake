use super::cfg::GLOBAL_CFG;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub(super) fn init() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    let _cfg = GLOBAL_CFG.clone(); // init config
}
