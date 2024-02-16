use eframe::egui::{self, Key};

use core::fmt::Display;
use core::panic;
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
    fn to_u8(&self) -> u8 {
        match self {
            PowerProfile::PowerSaver => 0,
            PowerProfile::Balanced => 1,
            PowerProfile::Performance => 2,
        }
    }
}

#[derive(Default)]
struct PPCtlGui {
    state: PowerProfile,
}

impl eframe::App for PPCtlGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let cp = egui::CentralPanel::default();
        let mut state_u8 = self.state.to_u8();
        cp.show(ctx, |ui| {
            let slider =
                egui::Slider::new(&mut state_u8, 0..=2).custom_formatter(|s, _| match s as u64 {
                    0 => POWER_SAVER.to_owned(),
                    1 => BALANCED.to_owned(),
                    2 => PERFORMANCE.to_owned(),
                    _ => "".to_owned(),
                });
            ui.add(slider);
            if self.state.to_u8() != state_u8 {
                self.state = state_u8.into();
                let _output = Command::new(PPCTL_CMD)
                    .args(["set", &format!("{}", self.state)])
                    .output()
                    .expect("failed to execute process");
                println!("Profile changed: {}", self.state);
            }
            if ctx.input(|i| i.key_pressed(Key::Escape)) {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    // options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id(env!("CARGO_CRATE_NAME"))
            .with_inner_size([200.0, 35.0]),
        ..Default::default()
    };
    eframe::run_native(
        env!("CARGO_CRATE_NAME"),
        options,
        Box::new(|_cc| Box::<PPCtlGui>::default()),
    )
}