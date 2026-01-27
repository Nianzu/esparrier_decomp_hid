#![no_std]
#![no_main]

extern crate alloc;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    otg_fs::Usb,
    peripherals::TIMG1,
    timer::timg::{MwdtStage, MwdtStageAction, TimerGroup, Wdt},
};
use esp_println::println;
use esp_rtos::main;
use log::{info};
use esparrier::mk_static;

mod keycodes;
mod hid_report_writer;
#[derive(Debug, Default)]
pub struct KeyboardReport {
    modifier: u8,
    keycode: [u8; 6],
}

impl KeyboardReport {
    pub fn press(&mut self, key: u8) -> [u8; 8] {
        match self.get_modifier(key) {
            Some(modifier) => self.modifier |= modifier,
            None => {
                // Don't add the same key twice
                for i in 0..6 {
                    if self.keycode[i] == key {
                        return self.send();
                    }
                }

                let mut found = false;
                for i in 0..6 {
                    if self.keycode[i] == 0 {
                        self.keycode[i] = key;
                        found = true;
                        break;
                    }
                }
                if !found {
                    // roll over the first key
                    for i in 1..6 {
                        self.keycode.swap(i - 1, i);
                    }
                    self.keycode[6 - 1] = key;
                }
            }
        }
        self.send()
    }

    pub fn release(&mut self, key: u8) -> [u8; 8] {
        match self.get_modifier(key) {
            Some(modifier) => self.modifier &= !modifier,
            None => {
                for i in 0..6 {
                    if self.keycode[i] == key {
                        self.keycode[i] = 0;
                        break;
                    }
                }
                // Compact the keycode array
                let mut pos = 0;
                for i in 0..6 {
                    if self.keycode[i] != 0 {
                        self.keycode.swap(i, pos);
                        pos += 1;
                    }
                }
            }
        }
        self.send()
    }

    pub fn clear(&mut self) -> [u8; 8] {
        self.modifier = 0;
        self.keycode = [0; 6];
        self.send()
    }

    pub fn is_empty(&self) -> bool {
        self.modifier == 0 && self.keycode.iter().all(|&x| x == 0)
    }

    fn send(&self) -> [u8; 8] {
        let mut report = [0u8; 8];
        report[0] = self.modifier;
        report[1] = 0;
        report[2..(6 + 2)].copy_from_slice(&self.keycode);
        report
    }

    fn get_modifier(&self, key: u8) -> Option<u8> {
        match key {
            0xE0 => Some(0x01), // Left Control
            0xE1 => Some(0x02), // Left Shift
            0xE2 => Some(0x04), // Left Alt
            0xE3 => Some(0x08), // Left GUI
            0xE4 => Some(0x10), // Right Control
            0xE5 => Some(0x20), // Right Shift
            0xE6 => Some(0x40), // Right Alt
            0xE7 => Some(0x80), // Right GUI
            _ => None,
        }
    }
}

 
// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    println!(
        "Firmware version: {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    esp_alloc::heap_allocator!(size: 160 * 1024);

    // Setup Embassy
    // let systimer = SystemTimer::new(peripherals.SYSTIMER);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    // Setup watchdog on TIMG1, which is by default disabled by the bootloader
    let wdt1 = mk_static!(Wdt<TIMG1>, TimerGroup::new(peripherals.TIMG1).wdt);
    wdt1.set_timeout(MwdtStage::Stage0, esp_hal::time::Duration::from_secs(1));
    wdt1.set_stage_action(MwdtStage::Stage0, MwdtStageAction::ResetSystem);
    wdt1.enable();
    wdt1.feed();

    // Start watchdog task
    spawner.must_spawn(watchdog_task(wdt1));

    // Setup HID task
    let usb = Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);
    hid_report_writer::start_hid_task(spawner, usb);

    info!("Ready to send keypresses");
    let mut report = KeyboardReport::default();
    loop {
        hid_report_writer::send_hid_report(hid_report_writer::HidReport::keyboard(report.press(keycodes::HID_KEY_B))).await;
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn watchdog_task(watchdog: &'static mut Wdt<TIMG1<'static>>) {
    loop {
        watchdog.feed();
        Timer::after(Duration::from_millis(500)).await;
    }
}
