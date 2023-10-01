mod imp;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;

use glib::{clone, Object};
use crate::window::Window;

glib::wrapper! {
    pub struct DetailsPage(ObjectSubclass<imp::DetailsPage>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for DetailsPage {
    fn default() -> Self {
        Self::new()
    }
}

impl DetailsPage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setup(&self, window: &Window, content: &str) {
        self.imp().back_button.connect_clicked(clone!(@weak window=>move|_btn|{
            window.back();
        }));

        let buffer =  self.imp().text_view.buffer();
        buffer.set_text(content);
    }
}