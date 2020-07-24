use crate::components::typography::field::Field;
use gtk::Orientation::Vertical;
use crate::components::periodic_table::element::Element;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum ThermalTabMsg {

}

pub struct Model {
    element: Element
}

#[widget]
impl Widget for ThermalTab {
    fn model(element:Element) -> Model {
        Model {element}
    }

    fn update(&mut self, _event: ThermalTabMsg) {
        
    }

    view! {
        #[name="tab_thermal"]
        gtk::Box{
            margin_top:5,margin_bottom:5,margin_start:5,margin_end:5,
            orientation:Vertical,
            #[name="name"]
            Field("Phase: ".to_string(),self.model.element.phase.to_string()),
            Field("Melting Point: ".to_string(),self.model.element.melt.as_ref().map_or("N/A".to_string(), f64::to_string)),
            Field("Boiling Point: ".to_string(),self.model.element.boil.as_ref().map_or("N/A".to_string(), f64::to_string)),
            Field("Molar Heat: ".to_string(),self.model.element.molar_heat.as_ref().map_or("N/A".to_string(), f64::to_string)),
        }
    }
}