use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum H1TabMsg {}

pub struct Model {
    text: String
}

#[widget]
impl Widget for H1 {
    fn model(text:String) -> Model {
        Model {text}
    }

    fn update(&mut self, _event: H1TabMsg) {}

    fn init_view(&mut self){
        add_class(self.h1.clone(),vec!["h1"]);
        use_css(CSS);
    }

    view! {
        #[name="h1"]
        gtk::Label{
            text:&self.model.text
        }
    }
}

static CSS:&[u8] = "
.h1 {
    font-size:30px;
}
".as_bytes();

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use super::H1;

    #[test]
    fn h1() {
        let (_component, widgets) = relm::init_test::<H1>("1".to_string()).expect("init_test failed");
        let label = widgets.h1;

        assert_text!(label, "1");
    }
}