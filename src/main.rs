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
use log::{debug, error, info, warn};

#[allow(unused_imports)]
use esparrier::constants::*;

use esparrier::{
    mk_static, 
    start_hid_task, HidReport, send_hid_report, ASCII_2_HID, KeyboardReport,
};

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
    start_hid_task(spawner, usb);

    info!("Ready to send keypresses");
    let byte = 0x41;
    let [k, m] = ASCII_2_HID[byte as usize];
    let mut report = KeyboardReport::default();
   loop {
       send_hid_report(HidReport::keyboard(report.press(k))).await;
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

