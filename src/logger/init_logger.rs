use simplelog::{Config, LevelFilter, SimpleLogger};

pub fn init_logger() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
}
