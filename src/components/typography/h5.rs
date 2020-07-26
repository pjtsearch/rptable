use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum H5TabMsg {}

pub struct Model {
    text: String
}

#[widget]
impl Widget for H5 {
    fn model(text:String) -> Model {
        Model {text}
    }

    fn update(&mut self, _event: H5TabMsg) {}

    fn init_view(&mut self){
        add_class(self.h5.clone(),vec!["h5"]);
        use_css(CSS);
    }

    view! {
        #[name="h5"]
        gtk::Label{
            text:&self.model.text
        }
    }
}

static CSS:&[u8] = "
.h5 {
    font-size:18px;
}
".as_bytes();

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use super::H5;

    #[test]
    fn h5() {
        let (_component, widgets) = relm::init_test::<H5>("1".to_string()).expect("init_test failed");
        let label = widgets.h5;

        assert_text!(label, "1");
    }
}