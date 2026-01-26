#![no_std]

mod hid_report_writer;
mod synergy_hid;

pub mod constants;
pub use hid_report_writer::{HidReport, send_hid_report, start_hid_task};
pub use synergy_hid::{ASCII_2_HID, KeyboardReport, ReportType, SynergyHid};

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
