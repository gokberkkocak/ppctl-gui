use gtk4::{self, glib, Adjustment, Application, ApplicationWindow, StateFlags};
use gtk4::{prelude::*, Scale};

const APP_ID: &str = "org.gtk_rs.MainEventLoop1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Build slider
    let slider = Scale::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .digits(0)
        .can_focus(false)
        .adjustment(
            &Adjustment::builder()
                .value(1.0)
                .lower(0.0)
                .upper(2.0)
                .step_increment(1.0)
                .page_increment(0.0)
                .page_size(0.0)
                .build(),
        )
        .round_digits(0)
        .build();
    slider.set_value(1.0);
    slider.add_mark(0.0, gtk4::PositionType::Bottom, Some("pp"));
    slider.add_mark(1.0, gtk4::PositionType::Bottom, Some("b"));
    slider.add_mark(2.0, gtk4::PositionType::Bottom, Some("per"));

    slider.connect_state_flags_changed(|s, _state| {
        println!("{}", s.value())
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&slider)
        .build();
    window.set_default_size(200, 35);
    
    // Present window
    window.present();
}
