use std::cell::RefCell;

use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, TextView, Button};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/jasper/ji/notes/create_page.ui")]
pub struct CreatePage {
    #[template_child]
    pub back_button: TemplateChild<Button>,

    #[template_child]
    pub text_view: TemplateChild<TextView>,

    #[template_child]
    pub save_button: TemplateChild<Button>,

    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CreatePage {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "CreatePage";
    type Type = super::CreatePage;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for CreatePage {}

// Trait shared by all widgets
impl WidgetImpl for CreatePage {}

// Trait shared by all boxes
impl BoxImpl for CreatePage {}
