mod header;

use crate::components::table::Elements;
use crate::components::table::Table;
use gtk::{Inhibit};
use gtk::Orientation::{Vertical};
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Component, Widget, init};
use std::collections::HashMap;

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
                Table(Elements{
                    s:vec!["H".to_string(),"Li".to_string(),"Be".to_string(),"Na".to_string(),"Mg".to_string(),"K".to_string(),"Ca".to_string(),"".to_string(),"Rb".to_string(),"Sr".to_string(),"Cs".to_string(),"Ba".to_string(),"Fr".to_string(),"Ra".to_string()],
                    p:vec![],
                    d:vec![],
                    f:vec![],
                })
            },

            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}