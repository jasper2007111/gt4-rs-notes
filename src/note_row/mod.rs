mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;

use chrono::prelude::*;

use crate::note_object::NoteObject;

glib::wrapper! {
    pub struct NoteRow(ObjectSubclass<imp::NoteRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for NoteRow {
    fn default() -> Self {
        Self::new()
    }
}

impl NoteRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, task_object: &NoteObject) {
        // Get state
        let content_label = self.imp().content_label.get();
        let create_label = self.imp().create_label.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind `task_object.content` to `task_row.content_label.label`
        let content_label_binding = task_object
            .bind_property("content", &content_label, "label")
            .sync_create()
            .build();
        // Save binding
        bindings.push(content_label_binding);

        let create_label_binding = task_object
            .bind_property("create", &create_label, "label")
            .sync_create()
            .transform_to(|_a, b: i64| {
                // Create a NaiveDateTime from the timestamp
                let naive = NaiveDateTime::from_timestamp_opt(b, 0);

                // Create a normal DateTime from the NaiveDateTime
                let datetime: DateTime<Utc> =
                    DateTime::from_naive_utc_and_offset(naive.unwrap(), Utc);

                // Format the datetime how you want
                let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
                Some(newdate.to_string())
            })
            .build();
        // Save binding
        bindings.push(create_label_binding);
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
