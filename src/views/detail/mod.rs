mod header;
use relm::Relm;
use crate::components::periodic_table::element::Element;
use gtk::Orientation::{Vertical};
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Component, Widget, init};
use header::Header;
use self::DetailWinMsg::*;



pub struct Model {
    header: Component<Header>,
    element:Element
}

#[derive(Msg)]
pub enum DetailWinMsg {
    Quit
}

#[widget]
impl Widget for DetailWin {
    fn model(_relm: &Relm<Self>, element: Element) -> Model {
        let header = init::<Header>(element.clone().name).expect("Header");

        Model {
            header,
            element
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