use gtk::{gio, glib, prelude::*, Adjustment, Application, Button, FlowBox, Orientation, Scrollbar, Window};
use crate::LABEL;

pub fn add_action1(app: &Application) {
    let action1 = gio::ActionEntry::builder("action1")
        .activate(action1)
        .build();

    app.add_action_entries([action1]);
}

fn action1(app: &Application, _: &gio::SimpleAction, _: Option<&glib::Variant>) {
    let window = Window::builder()
        .application(app)
        .title("action1")
        .default_width(300)
        .default_height(300)
        .resizable(false)
        .build();

    let dialog_box = FlowBox::builder()
        .orientation(Orientation::Horizontal)
        .homogeneous(true)

        .build();

    let yes_button = Button::with_label("Так");
    let cancel_button = Button::with_label("Відміна");

    dialog_box.append(&yes_button);
    dialog_box.append(&cancel_button);

    let flowbox = FlowBox::builder()
        .orientation(Orientation::Vertical)
        .build();

    let label = gtk::Label::builder()
        .label("50")
        .build();

    let adjustment = Adjustment::builder()
        .lower(1.0)
        .upper(100.0)
        .value(50.0)
        .step_increment(1.0)
        .build();

    let scrollbar = Scrollbar::builder()
        .orientation(Orientation::Horizontal)
        .adjustment(&adjustment)
        .build();

    let label_ref = label.clone();
    scrollbar.connect_state_flags_changed(move |scrollbar, _| {
        let value = scrollbar.adjustment().value() as u32;
        label_ref.set_text(&value.to_string());
    });

    let scrollbar_ref = scrollbar.clone();
    let window_ref = window.clone();
    yes_button.connect_clicked(move |button| {
        let value = scrollbar_ref.adjustment().value() as u32;
        unsafe { LABEL.clone().expect("Label must be already set by the time value is updated").set_text(&value.to_string()) };
        window_ref.destroy()
    });

    let window_ref = window.clone();
    cancel_button.connect_clicked(move |button| { window_ref.destroy() });

    flowbox.append(&label);
    flowbox.append(&scrollbar);
    flowbox.append(&dialog_box);

    window.set_child(Some(&flowbox));
    window.present();
}