use crate::utils::add_class::add_class;
use crate::utils::use_css::use_css;
use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

#[derive(Msg)]
pub enum SecondaryTabMsg {}

pub struct Model {
    text: String
}

#[widget]
impl Widget for Secondary {
    fn model(text:String) -> Model {
        Model {text}
    }

    fn update(&mut self, _event: SecondaryTabMsg) {}

    fn init_view(&mut self){
        add_class(self.secondary.clone(),vec!["secondary"]);
        use_css(CSS);
    }

    view! {
        #[name="secondary"]
        gtk::Label{
            text:&self.model.text
        }
    }
}

static CSS:&[u8] = "
.secondary {
    font-size:12px;
    opacity: 0.7;
}
".as_bytes();

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use super::Secondary;

    #[test]
    fn primary() {
        let (_component, widgets) = relm::init_test::<Secondary>("1".to_string()).expect("init_test failed");
        let label = widgets.secondary;

        assert_text!(label, "1");
    }
}