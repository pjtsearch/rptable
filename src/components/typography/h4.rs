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

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use super::H4;

    #[test]
    fn h4() {
        let (_component, widgets) = relm::init_test::<H4>("1".to_string()).expect("init_test failed");
        let label = widgets.h4;

        assert_text!(label, "1");
    }
}