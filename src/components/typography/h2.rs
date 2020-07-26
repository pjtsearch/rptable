use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum H2TabMsg {}

pub struct Model {
    text: String
}

#[widget]
impl Widget for H2 {
    fn model(text:String) -> Model {
        Model {text}
    }

    fn update(&mut self, _event: H2TabMsg) {}

    fn init_view(&mut self){
        add_class(self.h2.clone(),vec!["h2"]);
        use_css(CSS);
    }

    view! {
        #[name="h2"]
        gtk::Label{
            text:&self.model.text
        }
    }
}

static CSS:&[u8] = "
.h2 {
    font-size:25px;
}
".as_bytes();

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use super::H2;

    #[test]
    fn h2() {
        let (_component, widgets) = relm::init_test::<H2>("1".to_string()).expect("init_test failed");
        let label = widgets.h2;

        assert_text!(label, "1");
    }
}