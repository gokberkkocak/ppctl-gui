use eframe::egui::{self};

use core::fmt::Display;
use core::panic;
use std::error::Error;
use std::process::Command;

const POWER_SAVER: &str = "power-saver";
const BALANCED: &str = "balanced";
const PERFORMANCE: &str = "performance";

const PPCTL_CMD: &str = "powerprofilesctl";

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

impl From<u8> for PowerProfile {
    fn from(value: u8) -> Self {
        match value {
            0 => PowerProfile::PowerSaver,
            1 => PowerProfile::Balanced,
            2 => PowerProfile::Performance,
            _ => panic!("cannot")
        }
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
    fn to_u8(&self) -> u8 {
        match self {
            PowerProfile::PowerSaver => 0,
            PowerProfile::Balanced => 1,
            PowerProfile::Performance => 2,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // gui options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([200.0, 35.0]),
        ..Default::default()
    };

    // State
    let mut state = PowerProfile::init();
    let mut state_u8 = state.to_u8();

    eframe::run_simple_native("ppctl-gui", options, move |ctx, _frame| {
        let cp = egui::CentralPanel::default();
        cp.show(ctx, |ui| {
            let slider =
                egui::Slider::new(&mut state_u8, 0..=2).custom_formatter(|s, _| match s as u64 {
                    0 => "power-saver".to_owned(),
                    1 => "balanced".to_owned(),
                    2 => "performance".to_owned(),
                    _ => "".to_owned(),
                });
            ui.add(slider);
            if state.to_u8() != state_u8 {
                state = state_u8.into();
                let _output = Command::new(PPCTL_CMD)
                    .args(["set", &format!("{}", state)])
                    .output()
                    .expect("failed to execute process");
                println!("Profile changed: {}", state);
            }
        });
    })?;
    Ok(())
}
