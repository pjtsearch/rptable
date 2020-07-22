mod views;
use crate::views::main::Win;
use relm::Widget;

fn main() {
    Win::run(()).unwrap();
}
