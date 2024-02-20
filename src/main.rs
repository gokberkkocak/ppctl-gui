use gtk4::glib::property::PropertySet;
use gtk4::{self, glib, Adjustment, Application, ApplicationWindow};
use gtk4::{prelude::*, Scale};

use core::fmt::Display;
use std::cell::RefCell;
use std::process::Command;
use std::rc::Rc;

const APP_ID: &str = "com.goksh.ppctl_gui";

const POWER_SAVER: &str = "power-saver";
const BALANCED: &str = "balanced";
const PERFORMANCE: &str = "performance";

const PPCTL_CMD: &str = "powerprofilesctl";

#[derive(Clone, Copy)]
enum PowerProfile {
    PowerSaver,
    Balanced,
    Performance,
}

impl From<String> for PowerProfile {
    fn from(value: String) -> Self {
        match value.trim() {
            POWER_SAVER => PowerProfile::PowerSaver,
            BALANCED => PowerProfile::Balanced,
            PERFORMANCE => PowerProfile::Performance,
            _ => panic!("cannot"),
        }
    }
}

impl From<f64> for PowerProfile {
    fn from(x: f64) -> Self {
        match x {
            x if (-0.1..=0.1).contains(&x) => PowerProfile::PowerSaver,
            x if (0.9..=1.1).contains(&x) => PowerProfile::Balanced,
            x if (1.9..=2.1).contains(&x) => PowerProfile::Performance,
            _ => panic!("cannot"),
        }
    }
}

impl Default for PowerProfile {
    fn default() -> Self {
        PowerProfile::init()
    }
}

impl Display for PowerProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerProfile::PowerSaver => write!(f, "{}", POWER_SAVER),
            PowerProfile::Balanced => write!(f, "{}", BALANCED),
            PowerProfile::Performance => write!(f, "{}", PERFORMANCE),
        }
    }
}

impl PowerProfile {
    fn init() -> Self {
        let output = Command::new(PPCTL_CMD)
            .args(["get"])
            .output()
            .expect("failed to execute process");
        let stdout = String::from_utf8(output.stdout).expect("cannot");
        stdout.into()
    }
    fn to_f64(self) -> f64 {
        match self {
            PowerProfile::PowerSaver => 0.0,
            PowerProfile::Balanced => 1.0,
            PowerProfile::Performance => 2.0,
        }
    }
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
        .margin_start(40)
        .margin_end(40)
        .build();
    // init slider value
    let power_profile: PowerProfile = PowerProfile::init();
    slider.set_value(power_profile.to_f64());
    let state = Rc::new(RefCell::new(power_profile));
    // Add slider marks
    slider.add_mark(0.0, gtk4::PositionType::Bottom, Some(POWER_SAVER));
    slider.add_mark(1.0, gtk4::PositionType::Bottom, Some(BALANCED));
    slider.add_mark(2.0, gtk4::PositionType::Bottom, Some(PERFORMANCE));

    // capture value change via state change
    slider.connect_state_flags_changed(move |slider, _state| {
        let state_as_f64 = state.as_ref().borrow().to_f64();
        if state_as_f64 != slider.value() {
            state.set(slider.value().into());
            let _output = Command::new(PPCTL_CMD)
                .args(["set", &format!("{}", state.as_ref().borrow())])
                .output()
                .expect("failed to execute process");
            println!("Profile changed: {}", state.as_ref().borrow());
        }
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Power Profile Daemon GUI")
        .child(&slider)
        .build();
    window.set_default_size(200, 50);

    // Escape key handle
    let event_controller = gtk4::EventControllerKey::new();
    event_controller.connect_key_pressed(|_, key, _, _| {
        match key {
            gtk4::gdk::Key::Escape => {
                std::process::exit(0);
            }
            _ => (),
        }
        glib::Propagation::Proceed
    });
    window.add_controller(event_controller);

    // Present window
    window.present();
}

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
