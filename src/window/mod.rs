mod imp;

use glib::{clone, Object};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, NoSelection, SignalListItemFactory};
use gtk::{prelude::*, ListItem};

use crate::create_page::CreatePage;
use crate::details_page::DetailsPage;
use crate::note_object::NoteObject;
use crate::note_row::NoteRow;

use rusqlite::Connection;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn setup(&self) {
        let imp = self.imp();

        let stack = imp.stack.clone();
        stack.set_transition_type(gtk::StackTransitionType::OverLeftRight);

        imp.add_button
            .connect_clicked(clone!(@weak stack,  @weak self as window=>move|_btn|{
                let page = stack.child_by_name("CreatePage");
                if let Some(p) = page {
                    stack.set_visible_child(&p);
                } else {
                    let create_page = CreatePage::new();
                    create_page.setup(&window);
                    stack.add_named(&create_page, Some("CreatePage"));
                    stack.set_visible_child(&create_page);
                }
            }));
    }

    pub fn back(&self) {
        let imp = self.imp();
        let stack = imp.stack.clone();

        let page = stack.child_by_name("HomePage");
        if let Some(p) = page {
            stack.set_visible_child(&p);
        }
    }

    fn notes(&self) -> gio::ListStore {
        // Get state
        self.imp()
            .notes
            .borrow()
            .clone()
            .expect("Could not get current notes.")
    }

    fn setup_notes(&self) {
        // Create new model
        let model = gio::ListStore::new::<NoteObject>();

        // Get state and set model
        self.imp().notes.replace(Some(model));

        let conn = Connection::open("./notos.db3");
        if let Ok(c) = conn {
            let _ = c.execute(
                "CREATE TABLE note (
                    id    INTEGER PRIMARY KEY,
                    content  TEXT NOT NULL,
                    create_at  INTEGER NOT NULL
                )",
                (), // empty list of parameters.
            );
            self.imp().sqlite_con.replace(Some(c));

            self.refresh_data();
        }

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.notes()));
        self.imp().list_view.set_model(Some(&selection_model));
    }

    fn refresh_data(&self) {
        let a = self.imp().sqlite_con.borrow();
        let mut stmt = a
            .as_ref()
            .clone()
            .expect("msg")
            .prepare("SELECT id, content, create_at FROM note ORDER by id DESC")
            .expect("msg");
        let note_iter = stmt
            .query_map([], |row| {
                Ok(NoteObject::new(row.get(0)?, row.get(1)?, Some(row.get(2)?)))
            })
            .expect("msg");

        self.notes().remove_all();
        for note in note_iter {
            // println!("Found note {:?}", note.unwrap());
            self.notes().append(&note.unwrap());
        }

        self.update_view();
    }

    pub fn create_note(&self, content: &str) {
        if content.is_empty() {
            return;
        }
        let note = NoteObject::new(0, content.to_string(), None);

        let a = self.imp().sqlite_con.borrow();
        let result = a.as_ref().clone().expect("msg").execute(
            "INSERT INTO note (content, create_at) VALUES (?1, ?2)",
            (note.content(), note.create()),
        );

        if result.is_ok() {
            print!("{}", result.unwrap());
            self.refresh_data();
        }
    }

    pub fn update_view(&self) {
        if self.notes().n_items() > 0 {
            self.imp().list_view.set_visible(true);
            self.imp().status_page.set_visible(false);
        } else {
            self.imp().list_view.set_visible(false);
            self.imp().status_page.set_visible(true);
        }
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `TaskRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `TaskRow`
            let task_row = NoteRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&task_row));
        });

        factory.connect_bind(move |_, list_item| {
            let note_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<NoteObject>()
                .expect("The item has to be an `NoteObject`.");

            let note_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<NoteRow>()
                .expect("The child has to be a `NoteRow`.");

            note_row.bind(&note_object);
        });

        factory.connect_unbind(move |_, list_item| {
            let note_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<NoteRow>()
                .expect("The child has to be a `NoteRow`.");

            note_row.unbind();
        });

        // Set the factory of the list view
        self.imp().list_view.set_factory(Some(&factory));
        let stack = self.imp().stack.clone();
        self.imp().list_view.connect_activate(
            clone!(@weak stack, @weak self as window=>move|list_view, position| {
                let model = list_view.model().unwrap();
                let note = model.item(position).and_downcast::<NoteObject>().unwrap();

                let page = stack.child_by_name("DetailsPage");
                if let Some(p) = page {
                    stack.remove(&p);
                }

                let details_page = DetailsPage::new();
                details_page.setup(&window, &note.content());
                stack.add_named(&details_page, Some("DetailsPage"));
                stack.set_visible_child(&details_page);
            }),
        );
    }
}
