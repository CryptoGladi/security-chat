const LEVEL: log::Level = log::Level::Info;

pub fn init_logger() {
    simple_logger::init_with_level(LEVEL).unwrap();
}
