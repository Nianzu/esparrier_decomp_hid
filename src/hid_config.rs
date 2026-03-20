pub struct HidConfig {
    pub manufacturer: &'static str,
    pub product: &'static str,
    pub serial_number: &'static str,
}

impl Default for HidConfig {
    fn default() -> Self {
        Self {
            manufacturer: "Unknown",
            product: "esp31 HID device",
            serial_number: "037777777777",
        }
    }
}
