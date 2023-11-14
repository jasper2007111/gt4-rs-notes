mod imp;

use adw::NavigationPage;
use glib::{clone, Object};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::window::Window;

glib::wrapper! {
    pub struct CreatePage(ObjectSubclass<imp::CreatePage>)
    @extends NavigationPage, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for CreatePage {
    fn default() -> Self {
        Self::new()
    }
}

impl CreatePage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn setup(&self, window: &Window) {
        // self.imp()
        //     .back_button
        //     .connect_clicked(clone!(@weak window=>move|_btn|{
        //         window.back();
        //     }));

        let text_view = self.imp().text_view.clone();
        self.imp()
            .save_button
            .connect_clicked(clone!(@weak window, @weak text_view=>move|_btn|{
                let buffer = text_view.buffer();
                let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
                if text.is_empty() {
                    return
                }
                buffer.set_text("");
                window.create_note(&text);
                window.back();
            }));
    }
}
