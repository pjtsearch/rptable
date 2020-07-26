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

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use gtk::ContainerExt;
    use glib::object::Cast;
    use super::Field;

    #[test]
    fn field() {
        let (_component, widgets) = relm::init_test::<Field>(("1".to_string(),"2".to_string())).expect("init_test failed");
        let label1 = widgets.field.get_children()[0].clone().downcast::<gtk::Label>().expect("child must be a label");
        let label2 = widgets.field.get_children()[1].clone().downcast::<gtk::Label>().expect("child must be a label");

        assert_text!(label1, "1");
        assert_text!(label2, "2");
    }
}