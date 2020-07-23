mod grid;
use relm::Relm;
use std::collections::HashMap;
use crate::components::table::grid::TableGridModel;
use crate::components::table::grid::TableGrid;
use relm_derive::{Msg, widget};
use relm::Widget;
use gtk::prelude::*;
use gtk::Orientation::{Vertical};

pub struct TableModel {
    elements: Elements
}

pub struct Elements {
    pub s: Vec<String>,
    pub p: Vec<String>,
    pub d: Vec<String>,
    pub f: Vec<String>
}

#[derive(Msg)]
pub enum TableMsg {

}

#[widget]
impl Widget for Table {
    fn model(relm: &Relm<Self>, elements: Elements) -> TableModel {
        TableModel {elements}
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
                    children: self.model.elements.s.iter().map(|e|gtk::Label::new(Some(e))).collect(),
                    columns:2,
                    rows:7,
                }),
                #[name="d"]
                TableGrid<gtk::Label>(TableGridModel{
                    children: self.model.elements.d.iter().map(|e|gtk::Label::new(Some(e))).collect(),
                    columns:10,
                    rows:7,
                }),
                #[name="p"]
                TableGrid<gtk::Label>(TableGridModel{
                    children:  self.model.elements.p.iter().map(|e|gtk::Label::new(Some(e))).collect(),
                    columns:6,
                    rows:7,
                }),
            },
            #[name="bottom"]
            gtk::Box {
                #[name="f"]
                TableGrid<gtk::Label>(TableGridModel{
                    children: self.model.elements.f.iter().map(|e|gtk::Label::new(Some(e))).collect(),
                    columns:14,
                    rows:2,
                }),
            }
        }
    }
}