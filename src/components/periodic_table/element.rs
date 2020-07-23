use relm::Relm;
use relm_derive::{Msg, widget};
use relm::Widget;
use gtk::prelude::*;
use serde_derive::{Deserialize,Serialize};

pub struct ElementModel {
    element: Element
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Element {
    pub name:String,
    pub symbol:String,
    pub xpos:f32,
    pub ypos:f32
}

#[derive(Msg)]
pub enum ElementMsg {

}

#[widget]
impl Widget for ElementWidget {
    fn model(_relm: &Relm<Self>, element: Element) -> ElementModel {
        ElementModel {element}
    }

    fn update(&mut self, _event: ElementMsg) {
        
    }

    view! {
        #[name="table"]
        gtk::Button {
            hexpand:true,
            vexpand:true,
            margin_top:3,
            margin_bottom:3,
            margin_start:3,
            margin_end:3,
            gtk::Box{
                // gtk::Label {
                //     text:&self.model.element.name
                // },
                gtk::Label {
                    text:&self.model.element.symbol
                },  
            }          
        }
    }
}