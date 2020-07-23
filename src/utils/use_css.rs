use gtk::{CssProviderExt};

pub fn use_css(css:&'static [u8]){
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css).unwrap();
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}