#[macro_use]
extern crate lazy_static;

mod inputs;
mod actions;
mod read_and_write;
mod calculations;
mod print_utils;
mod emojis;
mod prompts;

fn main() {
    inputs::run();
}
