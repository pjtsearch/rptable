use crate::components::typography::field::Field;
use gtk::Orientation::Vertical;
use crate::components::periodic_table::element::Element;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum ElectronicTabMsg {

}

pub struct Model {
    element: Element
}

#[widget]
impl Widget for ElectronicTab {
    fn model(element:Element) -> Model {
        Model {element}
    }

    fn update(&mut self, _event: ElectronicTabMsg) {
        
    }

    view! {
        #[name="tab_electronic"]
        gtk::Box{
            margin_top:5,margin_bottom:5,margin_start:5,margin_end:5,
            orientation:Vertical,
            Field("Electron Configuration: ".to_string(),self.model.element.electron_configuration_semantic.to_string()),
            Field("Electronegativity: ".to_string(),self.model.element.electronegativity_pauling.as_ref().map_or("N/A".to_string(), f64::to_string)),
            Field("Electron Affinity: ".to_string(),self.model.element.electron_affinity.as_ref().map_or("N/A".to_string(), f64::to_string)),
            Field("Ionization Energies: ".to_string(),self.model.element.ionization_energies.iter().map(f64::to_string).collect::<Vec<String>>().join(", \n")),
        }
    }
}