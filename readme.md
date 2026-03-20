# ESP32 HID keyboard 

A stripped down version of the HID implementation from https://github.com/windoze/esparrier, designed to be a drop-in library for making esp32 based keyboards in rust.

```bash
cargo install espup --locked
cargo install cargo-espflash --locked
cargo install cargo-espmonitor
espup install
echo 'source $HOME/export-esp.sh' >> ~/.bashrc
source ~/.bashrc
cargo build --example single_key
```
## Example
See [examples/single_key.rs](examples/single_key.rs) for a full example implementation. 

```rust
#[main]
async fn main(spawner: Spawner) {

    // ... esp32 boilerplate omitted

    let usb = Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);
    let config = HidConfig::default();
    let mut keyboard = Keyboard::new(spawner, usb, config);

    let config = InputConfig::default().with_pull(Pull::Down);
    let button = Input::new(peripherals.GPIO2, config);
    loop {
        if button.is_high() {
            keyboard.press(keycodes::HID_KEY_C).await;
        } else {
            keyboard.release(keycodes::HID_KEY_C).await;
        }
        Timer::after(Duration::from_millis(5)).await;
    }
}
```

## Usage
- Install espup and configure your environment. [incend1um](https://incend1um.pages.dev/posts/rust-esp32/) has a great post on this if you are confused.
```
cargo install espup
espup install
cat ~/export-esp.sh >> ~/.bashrc
```

- Generate your new project with esp-generate. Inside the esp-generate tool, select the features your project will need. This packages uses embassy, so you must enable unstable-hal and embassy. When you are done, save and generate the project.
```
esp-generate --chip your-chip your-project-name
```
    - For example:
```
esp-generate --chip esp32s3 keyboard_project
```

- Add this project as a dependency.
```
cargo add esp32_hid
```
    - Note: you may have to increase your rust-version in Cargo.toml if it is below 1.89.

- Ready to go! Start a build with `cargo build`, and when you are ready to flash a device use `cargo run`.
Note: you may have to add `default-features = false` to esp-println if you get a compiler error. This seems like an issue with the esp-generate template.


## Credits

This library is derived from [esparrier](https://github.com/windoze/esparrier) by [徐辰](https://github.com/windoze), used under the MIT license.

## License
MIT License

Copyright (c) 2024 Chen Xu

Copyright (c) 2026 Nico Zucca

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
