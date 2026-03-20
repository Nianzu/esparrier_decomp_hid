#![no_std]

/// Defines a mk_static macro.
/// Macro is used to reserve memory at compile time
/// <https://docs.rs/static_cell/latest/static_cell/#:~:text=reserve%20memory%20at%20compile%20time%20for%20a%20value>
/// This can be done with Box, unsafe code, or std in less limited environments
#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write($val);
        x
    }};
}

/// Used to configure HID strings, to allow your device to be easily identified.
pub mod hid_config;
pub(crate) mod hid_report_writer;
/// Object used to send HID keyboard reports.
pub mod keyboard;
pub(crate) mod keyboard_report;
/// HID compatible keycodes to be used with keyboard object.
pub mod keycodes;
