mod grid;
use crate::components::table::grid::TableGridModel;
use crate::components::table::grid::TableGrid;
use relm_derive::{Msg, widget};
use relm::Widget;
use gtk::prelude::*;
use gtk::Orientation::{Vertical};

#[derive(Msg)]
pub enum TableMsg {

}

#[widget]
impl Widget for Table {
    fn model() -> () {
        
    }

    fn update(&mut self, _event: TableMsg) {
        
    }

    view! {
        #[name="table"]
        gtk::Box {
            orientation: Vertical,
            #[name="top"]
            gtk::Box {
                #[name="s"]
                TableGrid<gtk::Label>(TableGridModel{
                    children: vec![
                        gtk::Label::new(Some("1")),
                        gtk::Label::new(Some("2")),
                        gtk::Label::new(Some("3")),
                    ],
                    columns:2,
                    rows:7,
                }),
                #[name="d"]
                TableGrid<gtk::Label>(TableGridModel{
                    children: vec![
                        gtk::Label::new(Some("1")),
                        gtk::Label::new(Some("2")),
                        gtk::Label::new(Some("3")),
                    ],
                    columns:10,
                    rows:7,
                }),
                #[name="p"]
                TableGrid<gtk::Label>(TableGridModel{
                    children: vec![
                        gtk::Label::new(Some("1")),
                        gtk::Label::new(Some("2")),
                        gtk::Label::new(Some("3")),
                    ],
                    columns:6,
                    rows:7,
                }),
            },
            #[name="bottom"]
            gtk::Box {
                #[name="f"]
                TableGrid<gtk::Label>(TableGridModel{
                    children: vec![
                        gtk::Label::new(Some("1")),
                        gtk::Label::new(Some("2")),
                        gtk::Label::new(Some("3")),
                    ],
                    columns:14,
                    rows:2,
                }),
            }
        }
    }
}