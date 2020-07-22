use gtk::prelude::*;
use relm_derive::{Msg, widget};
use relm::{Widget};

use self::HeaderMsg::*;
#[derive(Msg)]
pub enum HeaderMsg {
    Menu
}

pub struct Model {
    title: String
}

#[widget]
impl Widget for Header {
    fn model(title:String) -> Model {
        Model {title}
    }

    fn update(&mut self, event: HeaderMsg) {
        match event {
            Menu => println!("Menu"),
        }
    }

    view! {
        #[name="titlebar"]
        gtk::HeaderBar {
            title: Some(&self.model.title),
            show_close_button: true,

            #[name="menu_button"]
            gtk::Button {
                clicked => Menu,
                label: "Menu",
            },
        }
    }
}