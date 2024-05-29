
import sys
import gi
gi.require_version('Gtk', '4.0')
gi.require_version('Adw', '1')
from gi.repository import Gtk,Gio,GObject,Gdk,GLib, Adw

import sqlite3


class NoteRow(Gtk.Box):
    __gtype_name__ = "NoteRow"
    def __init__(self, *args, **kwargs):
        super(NoteRow, self).__init__(orientation=Gtk.Orientation.VERTICAL, spacing=5)

    def setup(self):
        self.label = Gtk.Label.new()
        self.label.set_halign(Gtk.Align.START)
        self.label.props.wrap = True
        self.label.props.lines = 3
        self.label.props.wrap_mode = "word-char"
        self.label.props.ellipsize = "end"
        self.label.props.margin_top = 12
        self.label.props.margin_bottom = 12
        self.label.props.margin_start = 12
        self.label.props.margin_end = 12

        self.append(self.label)

        self.carete_label = Gtk.Label.new()
        self.carete_label.props.margin_top = 12
        self.carete_label.props.margin_bottom = 12
        self.carete_label.props.margin_start = 12
        self.carete_label.props.margin_end = 12
        self.carete_label.props.halign = "GTK_ALIGN_END"

        self.append(self.carete_label)
    
    def set_lable(self, str):
        self.label.props.label = str
        self.carete_label.props.label = "2024/05/28"

@Gtk.Template(filename='./resources/create_page.ui')        
class CreatePage(Adw.NavigationPage):
    __gtype_name__ = 'CreatePage'

@Gtk.Template(filename='./resources/window.ui')
class NotesWindow(Adw.ApplicationWindow):
    __gtype_name__ = 'NotesWindow'

    list_view_parent = Gtk.Template.Child("list_view_parent")
    status_page = Gtk.Template.Child("status_page")

    add_button =  Gtk.Template.Child("add_button")

    stack = Gtk.Template.Child("stack")
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        # GtkStringList is a list model that wraps an array of strings.
        # The objects in the model are of type [class`Gtk`.StringObject
        # and have a “string” property that can be used inside expressions.
        self.stringlist = Gtk.StringList.new(None) 
        
       
        self.add_button.connect("clicked",self.on_quit_button_clicked)
        
        # https://docs.gtk.org/gtk4/section-list-widget.html
        # self.listview = Gtk.ListView.new(self.single_selection_list_store,self.signal_factory)
        # self.listview.props.show_separators = True
        # sw.set_child(self.listview)
        
        new_con = sqlite3.connect("./notes.db3")
        new_cur = new_con.cursor()
        res = new_cur.execute("SELECT id, content, create_at FROM note ORDER by id DESC limit 1, 100")
        notes= res.fetchall()
        
        for item in notes:
            _, content, _ = item
            # print(content)
            self.stringlist.append(content)

        # self.listview.set_visible(True)
        self.status_page.set_visible(False)
        # for i in range(100):
        #     self.stringlist.append(str(i))


        print(len(self.stringlist))

        # allows marking each item in a model as either selected or not selected
        # by wrapping a model with one of the GTK models provided for this purposes,
        # such as GtkNoSelection or GtkSingleSelection.
        self.single_selection_list_store = Gtk.SingleSelection.new(self.stringlist)

                # when 'selected' property changed
        self.single_selection_list_store.connect("notify::selected",self.on_item_list_selected)
        
        # A GtkListItemFactory creates widgets for the items taken from a GListModel.
        # The GtkListItemFactory is tasked with creating widgets for items taken from the model when the views need
        # them and updating them as the items displayed by the view change.
        self.signal_factory = Gtk.SignalListItemFactory.new()


        # setup signal Emitted when a new listitem has been created and needs to be setup for use.
        self.signal_factory.connect("setup",self.on_setup)
        
        # bind signal emitted  when  [property`Gtk`.ListItem:item] has been set on a listitem and should be bound for use.
        self.signal_factory.connect("bind",self.on_object_bound_to_use) 


        self.listview = Gtk.ListView.new(self.single_selection_list_store,self.signal_factory)
        self.listview.props.show_separators = True
        self.list_view_parent.set_child(self.listview)

        self.listview.set_factory(self.signal_factory)
        self.listview.set_single_click_activate(self.single_selection_list_store)

    def on_quit_button_clicked(self,clicked_button):
        create_page = CreatePage()
        self.stack.push(create_page)

    def on_setup(self,signal_factory, list_item):
        print("Setup--> Prepering Widgets Start")
        # GtkListItem is used by list widgets to represent items in a [iface`Gio`.ListModel]
        # GtkListItem objects are managed by the list widget (with its factory) and cannot be created by applications,
        # but they need to be populated by application code. This is done by calling [method`Gtk`.ListItem.set_child].
        print(list_item)
        print(list_item.props.item) # item == None
    
        box = NoteRow()
        box.setup()

        list_item.set_child(box)
        print("Setup--> Prepering Widgets End\n")
        
    def on_object_bound_to_use(self,signal_factory, list_item):
        print("Bind--> modified Widgets Start") 
        item  = list_item.props.item # item == Gtk.StringObject (Gtk.StringObject wrapping str in Gtk.StringList)
        box = list_item.get_child()# label
        box.set_lable(item.props.string)
        print("Bind--> modified Widgets End\n") 
        

    def on_item_list_selected(self,single_selection_list_store,props):
        selected_item = single_selection_list_store.props.selected_item # Gtk.StringObject
        string_value  = selected_item.props.string
        position      = single_selection_list_store.get_selected()
        # print(f"Selected String   = {string_value}")
        # print(f"Selected Position = {position}")

        
    def on_add_button_clicked(self,button):
        self.stringlist.append("New")
        context = GLib.MainContext().default()
        while context.pending():
            context.iteration(True)
        self.listview.scroll_to(self.stringlist.get_n_items()-1,Gtk.ListScrollFlags.FOCUS | Gtk.ListScrollFlags.SELECT  ,None)
        
class MyApp(Adw.Application):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)

    def do_activate(self):
        active_window = self.props.active_window
        if active_window:
            active_window.present()
        else:
            self.win = NotesWindow(application=self)
            self.win.present()



app = MyApp(application_id="com.jasper.ji.gtk.py.notes",flags= Gio.ApplicationFlags.FLAGS_NONE)
app.run(sys.argv)