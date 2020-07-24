mod header;
mod tabs;
use crate::views::detail::tabs::thermal::ThermalTab;
use tabs::general::GeneralTab;
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

    fn init_view(&mut self) {
        // self.window.set_default_size(650,550);
    }

    view! {
        #[name="window"]
        gtk::Window {
            titlebar: Some(self.model.header.widget()),
            property_height_request: 600,
            property_width_request: 500,

            #[name="app"]
            gtk::Box {
                orientation: Vertical,
                #[name="tabs"]
                gtk::Notebook {
                    #[name="tab_general"]
                    gtk::ScrolledWindow{
                        vexpand:true,
                        child: {tab_label: Some("General")},
                        GeneralTab(self.model.element.clone())
                    },
                    #[name="tab_thermal"]
                    gtk::ScrolledWindow{
                        vexpand:true,
                        child: {tab_label: Some("Thermal")},
                        ThermalTab(self.model.element.clone())
                    },

                },
            },
            delete_event(_, _) => (Quit, Inhibit(false))
        }
    }
}