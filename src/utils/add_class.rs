use gtk::prelude::IsA;
use gtk::WidgetExt;
use gtk::StyleContextExt;

pub fn add_class<T:IsA<gtk::Widget>>(element:T,classes:Vec<&str>){
    classes.iter().for_each(|class|
        element.get_style_context().add_class(class)
    );
}