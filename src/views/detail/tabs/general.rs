use gtk::Orientation::Vertical;
use crate::components::typography::primary::Primary;
use crate::components::typography::h2::H2;
use crate::components::periodic_table::element::Element;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum GeneralTabMsg {

}

pub struct Model {
    element: Element
}

#[widget]
impl Widget for GeneralTab {
    fn model(element:Element) -> Model {
        Model {element}
    }

    fn update(&mut self, _event: GeneralTabMsg) {
        
    }

    view! {
        #[name="tab_general"]
        gtk::Box{
            margin_top:5,margin_bottom:5,margin_start:5,margin_end:5,
            orientation:Vertical,
            #[name="name"]
            H2(self.model.element.name.clone()),
            Primary(self.model.element.summary.clone())
        }
    }
}