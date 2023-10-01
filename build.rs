fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "gtk4-rs-notes.gresource",
    );
}
