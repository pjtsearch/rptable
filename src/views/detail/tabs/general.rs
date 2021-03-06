use crate::components::typography::field::Field;
use crate::components::typography::h1::H1;
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
            H1(self.model.element.symbol.clone()),
            H2(self.model.element.name.clone()),
            Field("Discovered By: ".to_string(),self.model.element.discovered_by.clone().unwrap_or("N/A".to_string())),
            Primary(self.model.element.summary.clone()),
            Field("Atomic Number: ".to_string(),self.model.element.number.to_string()),
            Field("Atomic Mass: ".to_string(),self.model.element.atomic_mass.to_string()),
            Field("Electronegativity: ".to_string(),self.model.element.electronegativity_pauling.as_ref().map_or("N/A".to_string(), f64::to_string)),
            Field("Electron Affinity: ".to_string(),self.model.element.electron_affinity.as_ref().map_or("N/A".to_string(), f64::to_string)),
        }
    }
}