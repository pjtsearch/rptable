use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum H3TabMsg {}

pub struct Model {
    text: String
}

#[widget]
impl Widget for H3 {
    fn model(text:String) -> Model {
        Model {text}
    }

    fn update(&mut self, _event: H3TabMsg) {}

    fn init_view(&mut self){
        add_class(self.h3.clone(),vec!["h3"]);
        use_css(CSS);
    }

    view! {
        #[name="h3"]
        gtk::Label{
            text:&self.model.text
        }
    }
}

static CSS:&[u8] = "
.h3 {
    font-size:22px;
}
".as_bytes();