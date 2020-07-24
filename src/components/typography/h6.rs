use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum H6TabMsg {}

pub struct Model {
    text: String
}

#[widget]
impl Widget for H6 {
    fn model(text:String) -> Model {
        Model {text}
    }

    fn update(&mut self, _event: H6TabMsg) {}

    fn init_view(&mut self){
        add_class(self.h6.clone(),vec!["h6"]);
        use_css(CSS);
    }

    view! {
        #[name="h6"]
        gtk::Label{
            text:&self.model.text
        }
    }
}

static CSS:&[u8] = "
.h6 {
    font-size:14px;
}
".as_bytes();