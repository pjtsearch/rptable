mod header;

use crate::components::table::Table;
use gtk::{Inhibit};
use gtk::Orientation::{Vertical};
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Component, Widget, init};

use header::Header;
use self::WinMsg::*;



pub struct Model {
    header: Component<Header>,
}

#[derive(Msg)]
pub enum WinMsg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        let header = init::<Header>("Periodic Table".to_string()).expect("Header");

        Model {
            header
        }
    }

    fn update(&mut self, event: WinMsg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            titlebar: Some(self.model.header.widget()),

            #[name="app"]
            gtk::Box {
                orientation: Vertical,
                Table
            },

            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}