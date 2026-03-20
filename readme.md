# ESP32 HID keyboard 

A stripped down version of the HID implementation from https://github.com/windoze/esparrier, designed to be a drop-in library for making esp32 based keyboards in rust.

```
cargo install espup --locked
cargo install cargo-espflash --locked
cargo install cargo-espmonitor
espup install
echo 'source $HOME/export-esp.sh' >> ~/.bashrc
source ~/.bashrc
cargo build --example single_key
```

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