use const_env::env_item;
use const_str::{parse, split};

const VERSION_SEGMENTS: [&str; 3] = split!(env!("CARGO_PKG_VERSION"), ".");
pub const VERSION_MAJOR: u8 = parse!(VERSION_SEGMENTS[0], u8);
pub const VERSION_MINOR: u8 = parse!(VERSION_SEGMENTS[1], u8);
pub const VERSION_PATCH: u8 = parse!(VERSION_SEGMENTS[2], u8);
        pub const MODEL_ID: u8 = 6;
const LED_INDICATOR_FLAG: u8 = 0b0000_0000;
const SMARTLED_INDICATOR_FLAG: u8 = 0b0000_0000;
const GRAPHICS_INDICATOR_FLAG: u8 = 0b0000_0000;
const CLIPBOARD_FLAG: u8 = 0b1000_0000;

const OTA_FLAG: u8 = 0b0000_0000;

pub const FEATURE_FLAGS: u8 = LED_INDICATOR_FLAG
    | SMARTLED_INDICATOR_FLAG
    | GRAPHICS_INDICATOR_FLAG
    | CLIPBOARD_FLAG
    | OTA_FLAG;

        pub const LED_PIN: u8 = 21;

#[env_item]
pub const MAX_CLIPBOARD_SIZE: usize = 1024;

// Default config settings
#[env_item]
pub const WIFI_SSID: &str = "my-ssid";
#[env_item]
pub const WIFI_PASSWORD: &str = "my-password";
#[env_item]
pub const BARRIER_SERVER: &str = "192.168.100.200:24800";
#[env_item]
pub const SCREEN_NAME: &str = "my-screen";
#[env_item]
pub const SCREEN_WIDTH: u16 = 1920;
#[env_item]
pub const SCREEN_HEIGHT: u16 = 1080;
#[env_item]
pub const JIGGLE_INTERVAL: u16 = 60;
#[env_item]
pub const POLLING_RATE: u16 = 200;
#[env_item]
pub const REVERSED_WHEEL: bool = false;

#[env_item]
pub const USB_VID: u16 = 0x0d0a;
#[env_item]
pub const USB_PID: u16 = 0xc0de;
#[env_item]
pub const USB_MANUFACTURER: &str = "0d0a.com";
#[env_item]
pub const USB_PRODUCT: &str = "Esparrier KVM";
#[env_item]
pub const USB_SERIAL_NUMBER: &str = "88888888";

pub const DEVICE_INTERFACE_GUIDS: &[&str] = &["{4d36e96c-e325-11ce-bfc1-08002be10318}"];

