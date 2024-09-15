use gtk::{gio, glib, prelude::*, Application, Button, FlowBox, Orientation, Window};

pub fn add_action2(app: &Application) {
    let action2 = gio::ActionEntry::builder("action2")
        .activate(action2)
        .build();

    app.add_action_entries([action2]);
}

static mut WINDOWS: [Option<Window>; 2] = [None, None];

fn action2(app: &Application, _: &gio::SimpleAction, _: Option<&glib::Variant>) {
    create_first_window(app);
}

fn create_first_window(app: &Application) {
    let window = Window::builder()
        .application(app)
        .title("action2 - window1")
        .default_width(300)
        .default_height(300)
        .build();

    let flow_box = FlowBox::builder().orientation(Orientation::Horizontal).build();
    let next_button = Button::builder().label("Далі >").build();
    let cancel_button = Button::builder().label("Відміна").build();

    flow_box.append(&next_button);
    flow_box.append(&cancel_button);
    
    let app_ref = app.clone();
    let window_ref = window.clone();
    next_button.connect_clicked(move |_| {
        create_second_window(&app_ref);
        unsafe { WINDOWS[0] = None; }
        window_ref.destroy();
    });

    let window_ref = window.clone();
    cancel_button.connect_clicked(move |_| {
        unsafe { WINDOWS[0] = None; }
        window_ref.destroy();
    });

    window.set_child(Some(&flow_box));
    unsafe { WINDOWS[0] = Some(window.clone()); }
    window.present();
}

fn create_second_window(app: &Application) {
    let window = Window::builder()
        .application(app)
        .title("action2 - window2")
        .default_width(300)
        .default_height(300)
        .build();

    let flow_box = FlowBox::builder().orientation(Orientation::Horizontal).build();
    let next_button = Button::builder().label("< Назад").build();
    let yes_button = Button::builder().label("Так").build();
    let cancel_button = Button::builder().label("Відміна").build();

    flow_box.append(&next_button);
    flow_box.append(&yes_button);
    flow_box.append(&cancel_button);

    let app_ref = app.clone();
    let window_ref = window.clone();
    next_button.connect_clicked(move |_| {
        create_first_window(&app_ref);
        unsafe { WINDOWS[1] = None; }
        window_ref.destroy();
    });

    let window_ref = window.clone();
    yes_button.connect_clicked(move |_| {
        unsafe { WINDOWS[1] = None; }
        window_ref.destroy();
    });
    
    let window_ref = window.clone();
    cancel_button.connect_clicked(move |_| {
        unsafe { WINDOWS[1] = None; }
        window_ref.destroy();
    });
    
    window.set_child(Some(&flow_box));
    unsafe { WINDOWS[1] = Some(window.clone()); }
    window.present();
}