const LEVEL: log::Level = log::Level::Debug;

pub fn init_logger() {
    simple_logger::init_with_level(LEVEL).unwrap();
}
