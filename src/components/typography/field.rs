use super::primary::Primary;
use gtk::Orientation::Horizontal;
use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum FieldMsg {}

pub struct Model {
    k: String,
    v: String
}

#[widget]
impl Widget for Field {
    fn model(value:(String,String)) -> Model {
        Model {k:value.0,v:value.1}
    }

    fn update(&mut self, _event: FieldMsg) {}

    fn init_view(&mut self){
        add_class(self.field.clone(),vec!["primary"]);
        use_css(CSS);
    }

    view! {
        #[name="field"]
        gtk::Box{
            margin_top:5,
            orientation:Horizontal,
            Primary(self.model.k.clone()),
            Primary(self.model.v.clone()),
        },
    }
}

static CSS:&[u8] = "
.field {
    
}
".as_bytes();