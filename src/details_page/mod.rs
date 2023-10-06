mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::{note_object::NoteObject, window::Window};
use glib::{clone, Object};

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

    pub fn setup(&self, window: &Window, note: &mut NoteObject) {
        self.imp()
            .back_button
            .connect_clicked(clone!(@weak window=>move|_btn|{
                window.back();
            }));

        let id = note.id().clone();
        self.imp()
            .delete_button
            .connect_clicked(clone!(@weak window=>move|_btn|{
                let info_dialog =  gtk::Dialog::with_buttons(Some("删除笔记"), Some(&window), gtk::DialogFlags::MODAL, &[("取消", gtk::ResponseType::Cancel), ("确定", gtk::ResponseType::Ok)]);
                info_dialog.connect_response(move|a, b|{
                    if b == gtk::ResponseType::Ok {
                        window.delete_note(id);
                    }
                    a.close();
                });
                info_dialog.show();
            }));

        let buffer = self.imp().text_view.buffer();
        buffer.set_text(&note.content());

        let save_btn = self.imp().save_button.clone();
        let cancel_btn = self.imp().cancel_button.clone();
        let edit_button = self.imp().edit_button.clone();
        let text_view = self.imp().text_view.clone();

        self.imp().edit_button.connect_clicked(
            clone!(@weak window, @weak save_btn, @weak cancel_btn, @weak text_view=>move|btn|{
                btn.set_visible(false);
                save_btn.set_visible(true);
                cancel_btn.set_visible(true);
                text_view.set_editable(true);
                text_view.set_can_focus(true);
            }),
        );

        self.imp().cancel_button.connect_clicked(
            clone!(@weak window, @weak save_btn, @weak edit_button, @weak text_view=>move|_btn|{
                window.back();
            }),
        );

        let note_clone = note.clone();
        self.imp()
            .save_button
            .connect_clicked(clone!(@weak window, @weak text_view=>move|_btn|{
                let buffer = text_view.buffer();
                let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
                if text.is_empty() {
                    return
                }

                buffer.set_text("");
                if text == note_clone.content() {
                    window.back();
                } else {
                    note_clone.set_content(text.clone());
                    window.update_note(&note_clone);
                    window.back();
                }
            }));
    }
}
