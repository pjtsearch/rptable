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

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use super::H6;

    #[test]
    fn h6() {
        let (_component, widgets) = relm::init_test::<H6>("1".to_string()).expect("init_test failed");
        let label = widgets.h6;

        assert_text!(label, "1");
    }
}