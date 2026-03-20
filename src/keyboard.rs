use crate::hid_report_writer;
use crate::{
    hid_report_writer::{HidReport, send_hid_report},
    keyboard_report::KeyboardReport,
};
use embassy_executor::Spawner;
use esp_hal::otg_fs::Usb;

pub struct Keyboard {
    keyboard_report: KeyboardReport,
}

impl Keyboard {
    pub fn new(spawner: Spawner, usb: Usb<'static>) -> Self {
        hid_report_writer::start_hid_task(spawner, usb);
        Self {
            keyboard_report: KeyboardReport::default(),
        }
    }
    pub async fn press(&mut self, keycode: u8) {
        let report = self.keyboard_report.press(keycode);
        send_hid_report(HidReport::keyboard(report)).await;
    }
    pub async fn release(&mut self, keycode: u8) {
        let report = self.keyboard_report.release(keycode);
        send_hid_report(HidReport::keyboard(report)).await;
    }
}
