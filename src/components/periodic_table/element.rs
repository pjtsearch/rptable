use crate::components::typography::primary::Primary;
use crate::components::typography::secondary::Secondary;
use gtk::Orientation::Vertical;
use relm::Relm;
use relm_derive::{Msg, widget};
use relm::Widget;
use gtk::prelude::*;
use serde_derive::{Deserialize,Serialize};
use self::ElementMsg::*;
use crate::views::detail::DetailWin;

pub struct ElementModel {
    element:Element,
    number:String,
    symbol:String,
    atomic_mass:String
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Element {
    pub name:String,
    pub appearance:Option<String>,
    pub atomic_mass:f64,
    pub boil:Option<f64>,
    pub category:String,
    pub density:Option<f64>,
    pub discovered_by:Option<String>,
    pub melt:Option<f64>,
    pub molar_heat:Option<f64>,
    pub named_by:Option<String>,
    pub number:f32,
    pub period:f32,
    pub phase:String,
    pub source:String,
    pub spectral_img:Option<String>,
    pub summary:String,
    pub symbol:String,
    pub xpos:f32,
    pub ypos:f32,
    pub shells:Vec<f32>,
    pub electron_configuration:String,
    pub electron_configuration_semantic:String,
    pub electron_affinity:Option<f64>,
    pub electronegativity_pauling:Option<f64>,
    pub ionization_energies:Vec<f64>
}

#[derive(Msg,Debug)]
pub enum ElementMsg {
    OpenDetail
}

#[widget]
impl Widget for ElementWidget {
    fn model(_relm: &Relm<Self>, element: Element) -> ElementModel {
        ElementModel {
            element:element.clone(),
            number:element.number.to_string(),
            symbol:element.symbol,
            atomic_mass:format!("{:.2}",element.atomic_mass),
        }
    }

    fn update(&mut self, event: ElementMsg) {
        match event {
            OpenDetail => DetailWin::run(self.model.element.clone()).clone().unwrap()
        }
    }

    view! {
        #[name="element"]
        gtk::Button {
            hexpand:true,
            vexpand:true,
            margin_top:3,
            margin_bottom:3,
            margin_start:3,
            margin_end:3,
            clicked => OpenDetail,
            gtk::Box{
                orientation:Vertical,
                #[name="number"]
                Secondary(self.model.number.clone()),
                #[name="symbol"]
                Primary(self.model.symbol.clone()),
                #[name="mass"]
                Secondary(self.model.atomic_mass.clone())
            }          
        }
    }
}

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use super::{ElementWidget,Element};

    #[test]
    fn element() {
        let element = Element { name: "Ununennium".to_string(), appearance: None, atomic_mass: 315.0, boil: Some(630.0), category: "unknown, but predicted to be an alkali metal".to_string(), density: Some(3.0), discovered_by: Some("GSI Helmholtz Centre for Heavy Ion Research".to_string()), melt: None, molar_heat: None, named_by: None, number: 119.0, period: 8.0, phase: "Solid".to_string(), source: "https://en.wikipedia.org/wiki/Ununennium".to_string(), spectral_img: None, summary: "Ununennium, also known as eka-francium or simply element 119, is the hypothetical chemical element with symbol Uue and atomic number 119. Ununennium and Uue are the temporary systematic IUPAC name and symbol respectively, until a permanent name is decided upon. In the periodic table of the elements, it is expected to be an s-block element, an alkali metal, and the first element in the eighth period.".to_string(), symbol: "Uue".to_string(), xpos: 1.0, ypos: 8.0, shells: vec![2.0, 8.0, 18.0, 32.0, 32.0, 18.0, 8.0, 1.0], electron_configuration: "1s2 2s2 2p6 3s2 3p6 4s2 3d10 4p6 5s2 4d10 5p6 6s2 4f14 5d10 6p6 7s2 5f14 6d10 7p6 8s1".to_string(), electron_configuration_semantic: "*[Uuo] 8s1".to_string(), electron_affinity: Some(63.87), electronegativity_pauling: None, ionization_energies: vec![] };
        let (_component, widgets) = relm::init_test::<ElementWidget>(element).expect("init_test failed");
        let number = widgets.number.widget();
        let symbol = widgets.symbol.widget();
        let mass = widgets.mass.widget();

        assert_text!(number, "119");
        assert_text!(symbol, "Uue");
        assert_text!(mass, "315.00");
    }
}