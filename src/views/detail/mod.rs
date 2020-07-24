mod header;
use gtk::Orientation::{Vertical};
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Component, Widget, init};
use header::Header;
use self::DetailWinMsg::*;



pub struct Model {
    header: Component<Header>
}

#[derive(Msg)]
pub enum DetailWinMsg {
    Quit
}

#[widget]
impl Widget for DetailWin {
    fn model() -> Model {
        let header = init::<Header>("Detail".to_string()).expect("Header");

        Model {
            header
        }
    }

    fn update(&mut self, event: DetailWinMsg) {
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
            },
            delete_event(_, _) => (Quit, Inhibit(false))
        }
    }
}