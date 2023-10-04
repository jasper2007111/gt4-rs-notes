use std::cell::RefCell;

use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/jasper/ji/gtk/rs/notes/note_row.ui")]
pub struct NoteRow {
    #[template_child]
    pub content_label: TemplateChild<Label>,

    #[template_child]
    pub create_label: TemplateChild<Label>,

    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NoteRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NoteRow";
    type Type = super::NoteRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for NoteRow {}

// Trait shared by all widgets
impl WidgetImpl for NoteRow {}

// Trait shared by all boxes
impl BoxImpl for NoteRow {}
