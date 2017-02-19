#![allow(dead_code)]
#![allow(unused_variables)]

mod elmkelon;

#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate serde_json;



fn main() {
    env_logger::init().unwrap();
    let elmkelon = elmkelon::game::Game::create();
    elmkelon.run();
}


