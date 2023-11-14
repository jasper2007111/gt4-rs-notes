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

struct CountQuery {
    count: i64,
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn setup(&self) {
        let imp = self.imp();

        self.imp().cur_page.replace(1);
        self.imp().total_page.replace(1);

        let stack = imp.stack.clone();
        // stack.set_transition_type(gtk::StackTransitionType::OverLeftRight);

        imp.add_button
            .connect_clicked(clone!(@weak stack,  @weak self as window=>move|_btn|{
                let create_page = CreatePage::new();
                create_page.setup(&window);
                stack.push(&create_page);
            }));

        self.imp().prev_button.connect_clicked(
            clone!(@weak stack,  @weak self as window=>move|_btn|{
                println!("prev_page");
                window.prev_page();
            }),
        );

        self.imp().next_button.connect_clicked(
            clone!(@weak stack,  @weak self as window=>move|_btn|{
                println!("next_button");
                window.next_page();
            }),
        );
    }

    fn prev_page(&self) {
        let cur_page = self.imp().cur_page.clone();
        let prev_page = cur_page.borrow().clone() - 1;
        if prev_page == 1 {
            self.imp().prev_button.set_sensitive(false); // 不可用
        }

        let total_page = self.imp().total_page.borrow().clone();
        if prev_page != total_page {
            self.imp().next_button.set_sensitive(true); // 可用
        }

        self.imp().cur_page.replace(prev_page);
        self.refresh_data();
    }

    fn next_page(&self) {
        let cur_page = self.imp().cur_page.clone();
        let next_page = cur_page.borrow().clone() + 1;
        let total_page = self.imp().total_page.borrow().clone();
        if next_page == total_page {
            self.imp().next_button.set_sensitive(false); // 不可用
        }
        if next_page != 1 {
            self.imp().prev_button.set_sensitive(true); // 可用
        }
        self.imp().cur_page.replace(next_page);
        self.refresh_data();
    }

    pub fn back(&self) {
        let imp = self.imp();
        let stack = imp.stack.clone();
        stack.pop();
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

        let conn = Connection::open("./notes.db3");
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

            self.create_test_data();
            self.refresh_data();
        }

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.notes()));
        self.imp().list_view.set_model(Some(&selection_model));
    }

    fn create_test_data(&self) {
        for i in 0..100 {
            if i % 2 == 0 {
                if i % 10 == 0 {
                    self.create_note(&format!("{}: 测试笔记", i));
                } else {
                    self.create_note(&format!("{}: Rust速度惊人且内存利用率极高。由于没有运行时和垃圾回收, 它能够胜任对性能要求特别高的服务, 可以在嵌入式设备上运行，还能轻松和其他语言集成。", i));
                }
            } else if i % 3 == 0 {
                self.create_note(&format!("{}: Rust拥有出色的文档、友好的编译器和清晰的错误提示信息, 还集成了一流的工具——包管理器和构建工具, 智能地自动补全和类型检验的多编辑器支持, 以及自动格式化代码等等。", i));
            } else {
                self.create_note(&format!("{}: 全世界已有数百家公司在生产环境中使用 Rust，以达到快速、跨平台、低资源占用的目的。很多著名且受欢迎的软件，例如 Firefox、 Dropbox 和 Cloudflare 都在使用 Rust。从初创公司到大型企业，从嵌入式设备到可扩展的 Web 服务，Rust 都完全合适。", i));
            }
        }
    }

    fn refresh_data(&self) {
        let cur_page = self.imp().cur_page.borrow().clone();
        // println!("refresh_data: {}", cur_page);
        let a = self.imp().sqlite_con.borrow();
        let sql = format!(
            "SELECT id, content, create_at FROM note ORDER by id DESC limit {}, 20",
            (cur_page - 1) * 20
        );
        let mut stmt = a.as_ref().clone().expect("msg").prepare(&sql).expect("msg");
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

        let count_sql = "SELECT count(id) FROM note";
        let mut stmt_2 = a
            .as_ref()
            .clone()
            .expect("msg")
            .prepare(count_sql)
            .expect("msg");
        let row_query = stmt_2.query_row([], |row| Ok(CountQuery { count: row.get(0)? }));

        let mut show = false;
        if row_query.is_ok() {
            let count = row_query.unwrap().count;
            let mut total_page = count / 20;
            if count % 20 > 0 {
                total_page += 1;
            }
            self.imp().total_page.replace(total_page);
            if count > 20 {
                show = true;
                self.imp()
                    .info_label
                    .set_label(&format!("{}/{}", cur_page, total_page));
            }
            if cur_page == 1 {
                self.imp().prev_button.set_sensitive(false);
                if total_page > 1 {
                    self.imp().next_button.set_sensitive(true);
                } else {
                    self.imp().next_button.set_sensitive(false);
                }
            }
        }
        self.imp().show_all_button.set_visible(show);

        self.update_view();
    }

    pub fn delete_note(&self, id: i64) {
        let a = self.imp().sqlite_con.borrow();
        let result = a
            .as_ref()
            .clone()
            .expect("msg")
            .execute(&format!("DELETE FROM note WHERE id={}", id), ());

        if result.is_ok() {
            self.back();
            self.imp().cur_page.replace(1);
            self.imp().total_page.replace(1);
            self.refresh_data();
        }
    }

    pub fn update_note(&self, note: &NoteObject) {
        let a = self.imp().sqlite_con.borrow();
        let result = a.as_ref().clone().expect("msg").execute(
            &format!(
                "UPDATE note SET content=\"{}\" WHERE id={}",
                note.content(),
                note.id()
            ),
            (),
        );

        if result.is_ok() {
            println!("更新成功");
        } else {
            println!("错误：{:?}", result.err());
        }
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
            self.imp().cur_page.replace(1);
            self.imp().total_page.replace(1);
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
                let mut note = model.item(position).and_downcast::<NoteObject>().unwrap();
                let details_page = DetailsPage::new();
                details_page.setup(&window, &mut note);
                stack.push(&details_page);
            }),
        );
    }
}
