mod views;
mod components;
mod utils;
use crate::views::main::Win;
use relm::Widget;

fn main() {
    Win::run(()).unwrap();
}
