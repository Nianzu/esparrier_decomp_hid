use crate::hid_config::HidConfig;
use core::{
    future::Future,
    sync::atomic::{AtomicBool, Ordering},
};
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
    once_lock::OnceLock,
};
use embassy_time::{Duration, with_timeout};
use embassy_usb::{
    class::hid::HidWriter,
    msos::{self, windows_version},
};
use esp_hal::{
    otg_fs::{Usb, asynch::Driver},
    system,
};
use log::{debug, info, warn};
type ReportWriter<'a, const N: usize> = HidWriter<'a, Driver<'a>, N>;

#[rustfmt::skip]
pub const COMPOSITE_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01,        // Usage Page (Generic Desktop Ctrls)
    0x09, 0x06,        // Usage (Keyboard)
    0xA1, 0x01,        // Collection (Application)
    0x85, 0x01,        //   Report ID (1)
    0x05, 0x07,        //   Usage Page (Keyboard/Keypad)
    0x19, 0xE0,        //   Usage Minimum (0xE0)
    0x29, 0xE7,        //   Usage Maximum (0xE7)
    0x15, 0x00,        //   Logical Minimum (0)
    0x25, 0x01,        //   Logical Maximum (1)
    0x95, 0x08,        //   Report Count (8)
    0x75, 0x01,        //   Report Size (1)
    0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x95, 0x01,        //   Report Count (1)
    0x75, 0x08,        //   Report Size (8)
    0x81, 0x01,        //   Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x07,        //   Usage Page (Keyboard/Keypad)
    0x19, 0x00,        //   Usage Minimum (0x00)
    0x29, 0xFF,        //   Usage Maximum (0xFF)
    0x15, 0x00,        //   Logical Minimum (0)
    0x25, 0xFF,        //   Logical Maximum (-1)
    0x95, 0x06,        //   Report Count (6)
    0x75, 0x08,        //   Report Size (8)
    0x81, 0x00,        //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x08,        //   Usage Page (LEDs)
    0x19, 0x01,        //   Usage Minimum (Num Lock)
    0x29, 0x05,        //   Usage Maximum (Kana)
    0x95, 0x05,        //   Report Count (5)
    0x75, 0x01,        //   Report Size (1)
    0x91, 0x02,        //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x95, 0x01,        //   Report Count (1)
    0x75, 0x03,        //   Report Size (3)
    0x91, 0x01,        //   Output (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0xC0,              // End Collection
    0x05, 0x01,        // Usage Page (Generic Desktop Controls)
    0x09, 0x02,        // Usage (Mouse)
    0xA1, 0x01,        // Collection (Application)
    0x85, 0x02,        //   Report ID (2)
    0x09, 0x01,        //   Usage (Pointer)
    0xA1, 0x00,        //   Collection (Physical)
    0x05, 0x09,        //     Usage Page (Button)
    0x19, 0x01,        //     Usage Minimum (0x01)
    0x29, 0x05,        //     Usage Maximum (0x05)
    0x15, 0x00,        //     Logical Minimum (0)
    0x25, 0x01,        //     Logical Maximum (1)
    0x95, 0x05,        //     Report Count (5)
    0x75, 0x01,        //     Report Size (1)
    0x81, 0x02,        //     Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x95, 0x01,        //     Report Count (1)
    0x75, 0x03,        //     Report Size (3)
    0x81, 0x01,        //     Input (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x01,        //     Usage Page (Generic Desktop Controls)
    0x09, 0x30,        //     Usage (X)
    0x09, 0x31,        //     Usage (Y)
    0x15, 0x00,        //     Logical Minimum (0)
    0x26, 0xFF, 0x7F,  //     Logical Maximum (32767)
    0x95, 0x02,        //     Report Count (2)
    0x75, 0x10,        //     Report Size (16)
    0x81, 0x02,        //     Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x09, 0x38,        //     Usage (Wheel)
    0x15, 0x81,        //     Logical Minimum (-127)
    0x25, 0x7F,        //     Logical Maximum (127)
    0x95, 0x01,        //     Report Count (1)
    0x75, 0x08,        //     Report Size (8)
    0x81, 0x06,        //     Input (Data,Var,Rel,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x0C,        //     Usage Page (Consumer)
    0x0A, 0x38, 0x02,  //     Usage (AC Pan)
    0x15, 0x81,        //     Logical Minimum (-127)
    0x25, 0x7F,        //     Logical Maximum (127)
    0x95, 0x01,        //     Report Count (1)
    0x75, 0x08,        //     Report Size (8)
    0x81, 0x06,        //     Input (Data,Var,Rel,No Wrap,Linear,Preferred State,No Null Position)
    0xC0,              //   End Collection
    0xC0,              // End Collection
    0x05, 0x0C,        // Usage Page (Consumer)
    0x09, 0x01,        // Usage (Consumer Control)
    0xA1, 0x01,        // Collection (Application)
    0x85, 0x03,        //   Report ID (3)
    0x15, 0x00,        //   Logical Minimum (0)
    0x26, 0xFF, 0x03,  //   Logical Maximum (1023)
    0x19, 0x00,        //   Usage Minimum (Unassigned)
    0x2A, 0xFF, 0x03,  //   Usage Maximum (0x03FF)
    0x95, 0x01,        //   Report Count (1)
    0x75, 0x10,        //   Report Size (16)
    0x81, 0x00,        //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0xC0,              // End Collection
];

#[derive(Debug)]
pub enum HidReport {
    Keyboard([u8; 9]),
}

impl HidReport {
    pub fn keyboard(data: [u8; 8]) -> Self {
        Self::Keyboard([
            1, data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ])
    }
}

pub type HidReportSender = Sender<'static, CriticalSectionRawMutex, HidReport, 32>;

type HidReportChannel = Channel<CriticalSectionRawMutex, HidReport, 32>;
type HidReportReceiver = Receiver<'static, CriticalSectionRawMutex, HidReport, 32>;

trait HidReportWriter {
    fn write_report(&mut self, report: HidReport) -> impl Future<Output = ()>;
}

struct UsbHidReportWriter<'a> {
    hid_report_writer: ReportWriter<'a, 9>,
    polling_interval: u8,
}

impl<'a> UsbHidReportWriter<'a> {
    pub fn new(hid_report_writer: ReportWriter<'a, 9>) -> Self {
        Self {
            hid_report_writer,
            polling_interval: 5,
        }
    }
}

impl HidReportWriter for UsbHidReportWriter<'_> {
    async fn write_report(&mut self, report: HidReport) {
        debug!("Sending report: {report:?}");
        let data: &[u8] = match &report {
            HidReport::Keyboard(data) => data,
        };
        // Assuming 10 * polling_interval is enough time for the host to poll the device, but not too short or too long.
        let timeout = Duration::from_millis((self.polling_interval as u64 * 10).clamp(100, 200));
        if with_timeout(timeout, async {
            self.hid_report_writer
                .write(data)
                .await
                .inspect_err(|e| {
                    warn!("Error writing HID report: {e:?}");
                })
                .ok();
        })
        .await
        .is_err()
        {
            // This can happen if the device is writing the report while unplugged.
            // Some board doesn't really support `self_powered` because the VBUS pin
            // in USB-OTG port is not solely powered by the host, or, it has not
            // configured a GPIO pin to monitor the VBUS voltage, which doesn't meet
            // the standard of USB self-powered device.
            // In this case, the board cannot detect the unplugging event even the
            // function is already implemented by the underlying OTG driver. And if a
            // report is being sent while the device is unplugged, the USB stack will
            // be stalled.
            // Above scenario may happen if the device is plugged into a USB hub which
            // supplies power to the device even if the host is disconnected or powered
            // off.
            // There is no way we can resume the USB stack, so we just panic here, and
            // the watchdog will reset the board.
            // @see https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32s3/api-reference/peripherals/usb_device.html#self-powered-device
            warn!("Timeout writing HID report, resetting the system.");
            system::software_reset()
        }
    }
}

#[embassy_executor::task]
async fn start_hid_report_writer(writer: ReportWriter<'static, 9>, receiver: HidReportReceiver) {
    let mut writer = UsbHidReportWriter::new(writer);
    loop {
        let report = receiver.receive().await;
        writer.write_report(report).await;
    }
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl embassy_usb::Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Device {}", if enabled { "enabled" } else { "disabled" });
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {addr}");
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}

static HID_REPORT_SENDER: OnceLock<HidReportSender> = OnceLock::new();

pub async fn send_hid_report(report: HidReport) {
    HID_REPORT_SENDER.get().await.send(report).await;
}

pub fn start_hid_task(spawner: Spawner, usb: Usb<'static>, hid_config: HidConfig) {
    let ep_out_buffer = mk_static!([u8; 1024], [0u8; 1024]);
    let config = esp_hal::otg_fs::asynch::Config::default();
    let driver = esp_hal::otg_fs::asynch::Driver::new(usb, ep_out_buffer, config);
    let mut config = embassy_usb::Config::new(0x0d0a, 0xc0de);
    config.manufacturer = Some(hid_config.manufacturer);
    config.product = Some(hid_config.product);
    // TODO: MacOs doesn't like these settings, why? Not sure about the last 2 but the 1st one is definitely the issue.
    // config.device_class = 0x03; // HID
    // config.device_sub_class = 0x01; // Boot Interface Subclass
    // config.device_protocol = 0x01; // Keyboard
    config.device_class = 0xEF; // Miscellaneous Device
    config.device_sub_class = 0x02; // Common Class
    config.device_protocol = 0x01; // Interface Association Descriptor
    config.composite_with_iads = true;
    config.serial_number = Some(hid_config.serial_number);
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Create embassy-usb DeviceBuilder using the driver and config.
    let config_descriptor_buf = mk_static!([u8; 256], [0u8; 256]);
    let bos_descriptor_buf = mk_static!([u8; 256], [0u8; 256]);
    let msos_descriptor_buf = mk_static!([u8; 256], [0u8; 256]);
    let control_buf = mk_static!([u8; 256], [0u8; 256]);
    let device_handler = mk_static!(MyDeviceHandler, MyDeviceHandler::new());

    let mut builder = embassy_usb::Builder::new(
        driver,
        config,
        config_descriptor_buf,
        bos_descriptor_buf,
        msos_descriptor_buf,
        control_buf,
    );

    builder.handler(device_handler);
    builder.msos_descriptor(windows_version::WIN8_1, 0);

    // Initialize the USB peripheral
    let hid_dev_state = mk_static!(
        embassy_usb::class::hid::State<'static>,
        embassy_usb::class::hid::State::new()
    );

    // Create classes on the builder.
    let config = embassy_usb::class::hid::Config {
        report_descriptor: COMPOSITE_REPORT_DESCRIPTOR,
        request_handler: None,
        poll_ms: 5,
        max_packet_size: 64,
    };

    let hid_dev = HidWriter::<'_, esp_hal::otg_fs::asynch::Driver<'_>, 9>::new(
        &mut builder,
        hid_dev_state,
        config,
    );
    // Add a vendor-specific function (class 0xFF), and corresponding interface,
    // that uses our custom handler.
    let mut function = builder.function(0xFF, 0x0D, 0x0A);
    function.msos_feature(msos::CompatibleIdFeatureDescriptor::new("WINUSB", ""));
    function.msos_feature(msos::RegistryPropertyFeatureDescriptor::new(
        "DeviceInterfaceGUIDs",
        msos::PropertyData::RegMultiSz(&["{4d36e96c-e325-11ce-bfc1-08002be10318}"]),
    ));
    drop(function);

    // // Run the USB device.
    spawner.must_spawn(usb_task(builder));

    let hid_channel = mk_static!(HidReportChannel, HidReportChannel::new());
    let hid_receiver = hid_channel.receiver();
    let hid_sender = hid_channel.sender();
    spawner.must_spawn(start_hid_report_writer(hid_dev, hid_receiver));

    HID_REPORT_SENDER.init(hid_sender).ok();
}

#[embassy_executor::task]
async fn usb_task(builder: embassy_usb::Builder<'static, Driver<'static>>) {
    // I highly doubt there are some kind of race conditions inside of the OTG_FS driver.
    // M5Atom S3 cannot start the USB peripheral without a delay, but S3 Lite can.
    embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;
    // Build the builder.
    let mut usb = builder.build();
    usb.run().await;
}
