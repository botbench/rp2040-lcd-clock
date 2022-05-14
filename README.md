# Simple RTC-based LCD clock

This uses a DS3231 RTC to get the time, and display it on the 2x16 LCD. Due to the nature of Rust on embedded, there is no memory dynamic memory allocation. This makes formatted strings a bit trickier. This project gets around that limitation by using heapless::String.

This project is based on the [RP2040 Project Template](https://github.com/rp-rs/rp2040-project-template).

## How to build
I've replaced the `probe-run` with a standard `gdb` runner, as that worked better with `openocd` and the [picoprobe](https://github.com/rp-rs/rp2040-project-template). A `gdb` commands file is there for convenience. A version of openocd with support for the RP2040 based `picoprobe` is required. 

Before running `cargo run` to build and run the application on the target board, start `openocd-pico -f interface/picoprobe.cfg -f target/rp2040.cfg` in another terminal, to connect to the `picoprobe`.
## License

The contents of this repository are dual-licensed under the _MIT OR Apache
2.0_ License. That means you can chose either the MIT licence or the
Apache-2.0 licence when you re-use this code. See `MIT` or `APACHE2.0` for more
information on each specific licence.

Any submissions to this project (e.g. as Pull Requests) must be made available
under these terms.