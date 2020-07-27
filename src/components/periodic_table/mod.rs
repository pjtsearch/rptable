pub mod element;
use relm::init;
use crate::components::periodic_table::element::{Element,ElementWidget};
use relm::Relm;
use relm_derive::{Msg, widget};
use relm::Widget;
use gtk::prelude::*;

pub struct PeriodicTableModel {
    elements: Vec<Element>,
    //HACK: need to store relm widget so that updates work. See https://github.com/antoyo/relm/issues/50#issuecomment-314931446
    element_widgets: Vec<relm::Component<ElementWidget>>
}

#[derive(Msg)]
pub enum PeriodicTableMsg {

}

#[widget]
impl Widget for PeriodicTable {
    fn model(_relm: &Relm<Self>, elements: Vec<Element>) -> PeriodicTableModel {
        PeriodicTableModel {elements,element_widgets:vec![]}
    }

    fn update(&mut self, _event: PeriodicTableMsg) {
        
    }


    fn init_view(&mut self){
        for element in self.model.elements.clone() {
            let relm_widget = init::<ElementWidget>(element.clone()).expect("Element");
            //HACK: need to store relm widget so that updates work. See https://github.com/antoyo/relm/issues/50#issuecomment-314931446
            self.model.element_widgets.push(relm_widget);
            let widget = self.model.element_widgets[self.model.element_widgets.len()-1].widget();
            self.table.clone().attach(
                widget,
                element.xpos as i32-1,
                element.ypos as i32-1,
                1,
                1
            );
            widget.show_all()
        }
    }

    view! {
        #[name="table"]
        gtk::Grid {
            hexpand:true,
            vexpand:true
        }
    }
}

#[cfg(test)]
mod tests {
    use gtk::LabelExt;
    use gtk_test::{assert_text};
    use gtk::ContainerExt;
    use glib::object::Cast;
    use super::PeriodicTable;
    use super::element::Element;

    #[test]
    fn ptable() {
        let element = Element { name: "Ununennium".to_string(), appearance: None, atomic_mass: 315.0, boil: Some(630.0), category: "unknown, but predicted to be an alkali metal".to_string(), density: Some(3.0), discovered_by: Some("GSI Helmholtz Centre for Heavy Ion Research".to_string()), melt: None, molar_heat: None, named_by: None, number: 119.0, period: 8.0, phase: "Solid".to_string(), source: "https://en.wikipedia.org/wiki/Ununennium".to_string(), spectral_img: None, summary: "Ununennium, also known as eka-francium or simply element 119, is the hypothetical chemical element with symbol Uue and atomic number 119. Ununennium and Uue are the temporary systematic IUPAC name and symbol respectively, until a permanent name is decided upon. In the periodic table of the elements, it is expected to be an s-block element, an alkali metal, and the first element in the eighth period.".to_string(), symbol: "Uue".to_string(), xpos: 1.0, ypos: 8.0, shells: vec![2.0, 8.0, 18.0, 32.0, 32.0, 18.0, 8.0, 1.0], electron_configuration: "1s2 2s2 2p6 3s2 3p6 4s2 3d10 4p6 5s2 4d10 5p6 6s2 4f14 5d10 6p6 7s2 5f14 6d10 7p6 8s1".to_string(), electron_configuration_semantic: "*[Uuo] 8s1".to_string(), electron_affinity: Some(63.87), electronegativity_pauling: None, ionization_energies: vec![] };
        let (_component, widgets) = relm::init_test::<PeriodicTable>(vec![element]).expect("init_test failed");
        let item = widgets.table.get_children()[0].clone().downcast::<gtk::Button>().expect("expected a gtk button for an element");
        let element = item.get_children()[0].clone().downcast::<gtk::Box>().expect("expected a gtk box for an element");
        let number = element.get_children()[0].clone().downcast::<gtk::Label>().expect("expected a gtk label for an element number");

        assert_text!(number, "119");
    }
}