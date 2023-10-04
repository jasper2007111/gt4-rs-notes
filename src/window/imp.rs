use std::cell::RefCell;

use adw::StatusPage;
use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Button, CompositeTemplate, ListView, Stack};

use rusqlite::Connection;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jasper/ji/gtk/rs/notes/window.ui")]
pub struct Window {
    #[template_child]
    pub stack: TemplateChild<Stack>,

    #[template_child]
    pub add_button: TemplateChild<Button>,

    #[template_child]
    pub list_view: TemplateChild<ListView>,

    #[template_child]
    pub status_page: TemplateChild<StatusPage>,

    // 数据
    pub notes: RefCell<Option<gio::ListStore>>,

    // Sqlite Connection
    pub sqlite_con: RefCell<Option<Connection>>
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "NotesWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        let obj = self.obj();
        obj.setup();

        obj.setup_notes();
        obj.setup_factory();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

// Trait shared by all AdwApplicationWindowImpl windows
impl adw::subclass::prelude::AdwApplicationWindowImpl for Window {}
