use relm::Update;
use relm::Relm;
use relm_derive::{Msg};
use relm::Widget;
use gtk::prelude::*;
use gtk::Orientation::{Vertical,Horizontal};

pub struct TableGridModel <T: IsA<gtk::Widget>> {
    pub rows: usize,
    pub columns: usize,
    pub children: Vec<T>
}

#[derive(Msg)]
pub enum TableGridMsg {

}

pub struct TableGrid<T: IsA<gtk::Widget>> {
    model: TableGridModel<T>,
    root: gtk::Box,
}

impl <T: IsA<gtk::Widget>>Update for TableGrid<T>  {
    // Specify the TableGridModel used for this widget.
    type Model = TableGridModel<T>;
    // Specify the TableGridModel parameter used to init the TableGridModel.
    type ModelParam = TableGridModel<T>;
    // Specify the type of the messages sent to the update function.
    type Msg = TableGridMsg;

    // Return the initial TableGridModel.
    fn model(_: &Relm<Self>, model: TableGridModel<T>) -> TableGridModel<T> {
        model
    }

    // The TableGridModel may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, _event: TableGridMsg) {
       
    }
}

impl <T: IsA<gtk::Widget>> Widget for TableGrid<T> {
    // Specify the type of the root widget.
    type Root = gtk::Box;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    // Create the widgets.
    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        let root = gtk::Box::new(Horizontal,1);
        for _ in 0..model.columns{
            root.add(&gtk::Box::new(Vertical,1));
        }
        model.children.iter().enumerate().for_each(|(i,child)|{
            root.get_children()[i/model.rows].clone().downcast::<gtk::Box>().unwrap().add(child);
        });

        root.show_all();

        TableGrid {
            model,
            root,
        }
    }
}