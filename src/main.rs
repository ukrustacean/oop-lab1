mod action1;
mod action2;

use gtk::{gio, glib, prelude::*, Application, ApplicationWindow, Label};
use crate::action1::add_action1;
use crate::action2::add_action2;

const APP_ID: &str = "ua.ukrustacean.lab1";

static mut LABEL: Option<Label> = None;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(build_menu);
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("lab1")
        .default_width(600)
        .default_height(400)
        .show_menubar(true)
        .build();

    let label = Label::builder().label("50").build();
    unsafe { LABEL = Some(label.clone()) };
    
    window.set_child(Some(&label));
    window.present();
}

fn build_menu(app: &Application) {
    add_action1(app);
    add_action2(app);

    let menubar = gio::Menu::new();

    let file_menu = gio::Menu::new();
    file_menu.append(Some("New"), None);

    let action_menu = gio::Menu::new();
    action_menu.append(Some("Робота1"), Some("app.action1"));
    action_menu.append(Some("Робота2"), Some("app.action2"));

    menubar.append_submenu(Some("File"), &file_menu);
    menubar.append_submenu(Some("Action"), &action_menu);
    
    app.set_menubar(Some(&menubar));
}