use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::Orientation::Vertical;
use relm::Relm;
use relm_derive::{Msg, widget};
use relm::Widget;
use gtk::prelude::*;
use serde_derive::{Deserialize,Serialize};
use self::ElementMsg::*;
use crate::views::detail::DetailWin;

pub struct ElementModel {
    name:String,
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
            name:element.name,
            symbol:element.symbol,
            atomic_mass:format!("{:.2}",element.atomic_mass),
        }
    }

    fn update(&mut self, event: ElementMsg) {
        match event {
            OpenDetail => DetailWin::run(()).clone().unwrap()
        }
    }

    fn init_view(&mut self){
        add_class(self.element.clone(),vec!["element"]);
        add_class(self.name.clone(),vec!["name","secondary"]);
        add_class(self.mass.clone(),vec!["mass","secondary"]);
        add_class(self.symbol.clone(),vec!["symbol"]);
        use_css(include_bytes!("element.css"));
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
                #[name="name"]
                gtk::Label {
                    text:&self.model.name
                },
                #[name="symbol"]
                gtk::Label {
                    text:&self.model.symbol
                },  
                #[name="mass"]
                gtk::Label {
                    text:&self.model.atomic_mass
                },  
            }          
        }
    }
}