mod imp;

use chrono::{DateTime, Local, Utc};
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct NoteObject(ObjectSubclass<imp::NoteObject>);
}

impl NoteObject {
    pub fn new(id: i64, content: String, create_option: Option<i64>) -> Self {
        let create = if let Some(c) = create_option {
            c
        } else {
            let local_time = Local::now();
            let utc_time = DateTime::<Utc>::from_naive_utc_and_offset(local_time.naive_utc(), Utc);
            utc_time.timestamp()
        };

        Object::builder()
            .property("id", id)
            .property("content", content)
            .property("create", create)
            .build()
    }
}

#[derive(Default)]
pub struct NoteData {
    pub id: i64,
    pub content: String,
    pub create: i64,
    pub update: Option<i64>,
}
