mod ascii_2_hid;
mod descriptors;
mod hid;
mod keycodes;

pub use ascii_2_hid::ASCII_2_HID;
use descriptors::COMPOSITE_REPORT_DESCRIPTOR;
pub use hid::KeyboardReport;
pub(super) use hid::*;
pub(crate) use keycodes::{KeyCode, synergy_mouse_button, synergy_to_hid};

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

    pub fn key_down<'a>(
        &mut self,
        key: u16,
        mask: u16,
        button: u16,
        report: &'a mut [u8],
    ) -> (ReportType, &'a [u8]) {
        debug!("Key down {key} {mask} {button}");
        self.server_buttons[button as usize] = key;
        let hid = synergy_to_hid(key);
        // debug!("Key Down {:#04x} -> Keycode: {:?}", key, hid);
        match hid {
            KeyCode::None => {
                if key != 0 {
                    warn!("Keycode {key} not found");
                }
                report[0] = ReportType::Keyboard as u8;
                report[1..9].copy_from_slice(&self.keyboard_report.clear());
                (ReportType::Keyboard, &report[0..9])
            }
            KeyCode::Key(key) => {
                report[0] = ReportType::Keyboard as u8;
                report[1..9].copy_from_slice(&self.keyboard_report.press(key));
                (ReportType::Keyboard, &report[0..9])
            }
            KeyCode::Consumer(key) => {
                report[0] = ReportType::Consumer as u8;
                report[1..3].copy_from_slice(&self.consumer_report.press(key));
                (ReportType::Consumer, &report[0..3])
            }
        }
    }

    pub fn key_up<'a>(
        &mut self,
        key: u16,
        mask: u16,
        button: u16,
        report: &'a mut [u8],
    ) -> (ReportType, &'a [u8]) {
        debug!("Key up {key} {mask} {button}");
        let key = self.server_buttons[button as usize];
        let hid = if self.server_buttons[button as usize] != 0 {
            // debug!("Key {key} up");
            self.server_buttons[button as usize] = 0;
            synergy_to_hid(key)
        } else if key == 0 {
            debug!("Key 0 up, clear all key down");
            KeyCode::None
        } else {
            // warn!("Key {key} up with no key down");
            KeyCode::None
        };
        // debug!("Key Down {:#04x} -> Keycode: {:?}", key, hid);
        match hid {
            KeyCode::None => {
                if key != 0 {
                    warn!("Keycode {key} not found");
                }
                report[0] = ReportType::Keyboard as u8;
                report[1..9].copy_from_slice(&self.keyboard_report.clear());
                (ReportType::Keyboard, &report[0..9])
            }
            KeyCode::Key(key) => {
                report[0] = ReportType::Keyboard as u8;
                report[1..9].copy_from_slice(&self.keyboard_report.release(key));
                (ReportType::Keyboard, &report[0..9])
            }
            KeyCode::Consumer(_key) => {
                report[0] = ReportType::Consumer as u8;
                report[1..3].copy_from_slice(&self.consumer_report.release());
                (ReportType::Consumer, &report[0..3])
            }
        }
    }

    pub fn set_cursor_position<'a>(
        &mut self,
        x: u16,
        y: u16,
        report: &'a mut [u8],
    ) -> (ReportType, &'a [u8]) {
        report[0] = ReportType::Mouse as u8;
        report[1..8].copy_from_slice(&self.mouse_report.move_to(x, y));
        (ReportType::Mouse, &report[..8])
    }

    pub fn mouse_down<'a>(&mut self, button: i8, report: &'a mut [u8]) -> (ReportType, &'a [u8]) {
        report[0] = ReportType::Mouse as u8;
        report[1..8].copy_from_slice(&self.mouse_report.mouse_down(synergy_mouse_button(button)));
        (ReportType::Mouse, &report[..8])
    }

    pub fn mouse_up<'a>(&mut self, button: i8, report: &'a mut [u8]) -> (ReportType, &'a [u8]) {
        report[0] = ReportType::Mouse as u8;
        report[1..8].copy_from_slice(&self.mouse_report.mouse_up(synergy_mouse_button(button)));
        (ReportType::Mouse, &report[..8])
    }

    pub fn mouse_scroll<'a>(
        &mut self,
        x: i16,
        y: i16,
        report: &'a mut [u8],
    ) -> (ReportType, &'a [u8]) {
        let x = (x as f32 / 120.0) as i16;
        let y = (y as f32 / 120.0) as i16;
        let mut x = x as i8;
        let mut y = y as i8;
        if self.flip_mouse_wheel {
            x = -x;
            y = -y;
        }
        report[0] = ReportType::Mouse as u8;
        report[1..8].copy_from_slice(&self.mouse_report.mouse_wheel(y, x));
        (ReportType::Mouse, &report[..8])
    }

    pub fn clear<'a>(
        &mut self,
        report_type: ReportType,
        report: &'a mut [u8],
    ) -> (ReportType, &'a [u8]) {
        match report_type {
            ReportType::Keyboard => {
                report[0] = ReportType::Keyboard as u8;
                report[1..9].copy_from_slice(&self.keyboard_report.clear());
                (ReportType::Keyboard, &report[0..9])
            }
            ReportType::Mouse => {
                report[0] = ReportType::Mouse as u8;
                report[1..8].copy_from_slice(&self.mouse_report.clear());
                (ReportType::Mouse, &report[..8])
            }
            ReportType::Consumer => {
                report[0] = ReportType::Consumer as u8;
                report[1..3].copy_from_slice(&self.consumer_report.clear());
                (ReportType::Consumer, &report[0..3])
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.keyboard_report.is_empty()
            && self.mouse_report.is_empty()
            && self.consumer_report.is_empty()
    }
}


