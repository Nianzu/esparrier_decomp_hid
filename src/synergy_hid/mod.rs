mod descriptors;
mod hid;
//mod keycodes;

use descriptors::COMPOSITE_REPORT_DESCRIPTOR;
pub use hid::KeyboardReport;
pub(super) use hid::*;
//pub(crate) use keycodes::KeyCode;

use log::{debug, warn};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReportType {
    Keyboard = 1,
    Mouse = 2,
    Consumer = 3,
}

impl ReportType {
    pub const fn get_report_size(&self) -> usize {
        match self {
            Self::Keyboard => 9,
            Self::Mouse => 8,
            Self::Consumer => 3,
        }
    }
    pub const fn get_max_report_size() -> usize {
        9
    }
}

#[derive(Debug)]
pub struct SynergyHid {
    flip_mouse_wheel: bool,
    server_buttons: [u16; 512],

    // Report 1
    keyboard_report: KeyboardReport,
    // Report 2
    mouse_report: AbsMouseReport,
    // Report 3
    consumer_report: ConsumerReport,
}

impl SynergyHid {
    pub fn new(flip_mouse_wheel: bool) -> Self {
        Self {
            flip_mouse_wheel,
            server_buttons: [0; 512],
            keyboard_report: KeyboardReport::default(),
            mouse_report: AbsMouseReport::default(),
            consumer_report: ConsumerReport::default(),
        }
    }

    pub const fn get_report_descriptor() -> (u8, &'static [u8]) {
        (
            ReportType::get_max_report_size() as u8,
            COMPOSITE_REPORT_DESCRIPTOR,
        )
    }
}
