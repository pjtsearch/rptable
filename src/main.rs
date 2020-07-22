mod views;
mod components;
use crate::views::main::Win;
use relm::Widget;

fn main() {
    Win::run(()).unwrap();
}
