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
            valign:gtk::Align::Start,
            text:&self.model.text
        }
    }
}

static CSS:&[u8] = "
.primary {
    
}
".as_bytes();

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use super::Primary;

    #[test]
    fn primary() {
        let (_component, widgets) = relm::init_test::<Primary>("1".to_string()).expect("init_test failed");
        let label = widgets.primary;

        assert_text!(label, "1");
    }
}