//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use embedded_time::rate::Extensions;
use heapless::String;
use panic_probe as _;

// Get the driver for the display
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};

use ds323x::{DateTimeAccess, Ds323x, NaiveDate};

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio, pac,
    sio::Sio,
    watchdog::Watchdog,
    I2C,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();

    let mut lcd = HD44780::new_4bit(
        pins.gpio16.into_push_pull_output(),
        pins.gpio17.into_push_pull_output(),
        pins.gpio18.into_push_pull_output(),
        pins.gpio19.into_push_pull_output(),
        pins.gpio20.into_push_pull_output(),
        pins.gpio21.into_push_pull_output(),
        &mut delay,
    )
    .unwrap();

    // Configure two pins as being I²C
    let sda_pin = pins.gpio2.into_mode::<gpio::pin::FunctionI2C>();
    let scl_pin = pins.gpio3.into_mode::<gpio::pin::FunctionI2C>();

    // Create the I²C driver, using the two pre-configured pins.
    let i2c = I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock,
    );

    let mut rtc = Ds323x::new_ds3231(i2c);

    lcd.set_display_mode(
        DisplayMode {
            display: Display::On,
            cursor_visibility: Cursor::Invisible,
            cursor_blink: CursorBlink::Off,
        },
        &mut delay,
    )
    .unwrap();

    loop {
        // Create string for top line
        let mut top_line: String<24> = String::new();

        lcd.reset(&mut delay).unwrap();
        lcd.clear(&mut delay).unwrap();

        led_pin.set_high().unwrap();

        let dt = match rtc.datetime() {
            Err(_) => NaiveDate::from_ymd(1900, 1, 1).and_hms(00, 00, 00),
            Ok(res) => res,
        };

        core::write!(top_line, "{}", dt).unwrap();

        lcd.write_str(top_line.as_str(), &mut delay).ok().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
// End of file
