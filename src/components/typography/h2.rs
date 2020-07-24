use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum H2TabMsg {

}

pub struct Model {
    text: String
}

#[widget]
impl Widget for H2 {
    fn model(text:String) -> Model {
        Model {text}
    }

    fn update(&mut self, _event: H2TabMsg) {
        
    }

    fn init_view(&mut self){
        add_class(self.h2.clone(),vec!["h2"]);
        use_css(include_bytes!("h2.css"));
    }

    view! {
        #[name="h2"]
        gtk::Label{
            text:&self.model.text
        }
    }
}