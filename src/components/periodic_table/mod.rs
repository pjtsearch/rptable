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