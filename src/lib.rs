#![no_std]

mod clipboard;
mod config;
mod hid_report_writer;
mod synergy_hid;

pub mod constants;
pub use clipboard::{button_task, set_clipboard};
pub use config::{AppConfig, ConfigStore};
pub use hid_report_writer::{HidReport, send_hid_report, start_hid_task};
pub use synergy_hid::{ReportType, SynergyHid, ASCII_2_HID, KeyboardReport};

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
