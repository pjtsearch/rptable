use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum PrimaryTabMsg {}

pub struct Model {
    text: String
}

#[widget]
impl Widget for Primary {
    fn model(text:String) -> Model {
        Model {text}
    }

    fn update(&mut self, _event: PrimaryTabMsg) {}

    fn init_view(&mut self){
        add_class(self.primary.clone(),vec!["primary"]);
        use_css(CSS);
    }

    view! {
        #[name="primary"]
        gtk::Label{
            line_wrap:true,
            text:&self.model.text
        }
    }
}

static CSS:&[u8] = "
.primary {
    
}
".as_bytes();