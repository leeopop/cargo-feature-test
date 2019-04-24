#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    simple_logger::init().unwrap();
    info!("This must not be shown.");
}
