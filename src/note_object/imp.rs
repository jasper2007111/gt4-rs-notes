use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::NoteData;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::NoteObject)]
pub struct NoteObject {
    #[property(name = "id", get, set, type = i64, member = id)]
    #[property(name = "content", get, set, type = String, member = content)]
    #[property(name = "create", get, set, type = i64, member = create)]
    pub data: RefCell<NoteData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NoteObject {
    const NAME: &'static str = "NoteObject";
    type Type = super::NoteObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for NoteObject {}