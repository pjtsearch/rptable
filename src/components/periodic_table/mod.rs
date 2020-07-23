use relm::Relm;
use relm_derive::{Msg, widget};
use relm::Widget;
use gtk::prelude::*;
use serde_derive::{Deserialize,Serialize};

pub struct PeriodicTableModel {
    elements: Vec<Element>
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Element {
    name:String,
    symbol:String,
    xpos:f32,
    ypos:f32
}

#[derive(Msg)]
pub enum PeriodicTableMsg {

}

#[widget]
impl Widget for PeriodicTable {
    fn model(_relm: &Relm<Self>, elements: Vec<Element>) -> PeriodicTableModel {
        PeriodicTableModel {elements}
    }

    fn update(&mut self, _event: PeriodicTableMsg) {
        
    }


    fn init_view(&mut self){
        for element in self.model.elements.clone() {
            let btn = gtk::Button::with_label(&element.symbol);
            btn.set_margin_top(3);
            btn.set_margin_bottom(3);
            btn.set_margin_start(3);
            btn.set_margin_end(3);
            btn.set_hexpand(true);
            btn.set_vexpand(true);
            self.table.clone().attach(
                &btn,
                element.xpos as i32-1,
                element.ypos as i32-1,
                1,
                1
            );
            btn.show_all()
        }
    }

    view! {
        #[name="table"]
        gtk::Grid {
            hexpand:true,
            vexpand:true
        }
    }
}