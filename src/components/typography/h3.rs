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

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use super::H3;

    #[test]
    fn h3() {
        let (_component, widgets) = relm::init_test::<H3>("1".to_string()).expect("init_test failed");
        let label = widgets.h3;

        assert_text!(label, "1");
    }
}