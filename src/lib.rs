#![no_std]

// Defines a mk_static macro.
// Macro is used to reserve memory at compile time
// https://docs.rs/static_cell/latest/static_cell/#:~:text=reserve%20memory%20at%20compile%20time%20for%20a%20value
// This can be done with Box, unsafe code, or std in less limited envoronments
#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub mod hid_report_writer;
pub mod keycodes;
