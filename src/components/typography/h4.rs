use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum H4TabMsg {}

pub struct Model {
    text: String
}

#[widget]
impl Widget for H4 {
    fn model(text:String) -> Model {
        Model {text}
    }

    fn update(&mut self, _event: H4TabMsg) {}

    fn init_view(&mut self){
        add_class(self.h4.clone(),vec!["h4"]);
        use_css(CSS);
    }

    view! {
        #[name="h4"]
        gtk::Label{
            text:&self.model.text
        }
    }
}

static CSS:&[u8] = "
.h4 {
    font-size:20px;
}
".as_bytes();