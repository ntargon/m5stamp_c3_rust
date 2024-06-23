use esp_idf_hal::peripherals::Peripherals;
use smart_leds::SmartLedsWrite;
use std::thread;
use std::time::Duration;
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGB8};

const NUM_PIXELS: usize = 1;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let led_pin = peripherals.pins.gpio2;
    let channel = peripherals.rmt.channel0;

    let mut ws2812 = LedPixelEsp32Rmt::<RGB8, LedPixelColorGrbw32>::new(channel, led_pin).unwrap();

    thread::spawn(|| {
        for i in 1..100 {
            // やあ！立ち上げたスレッドから数字{}だよ！
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    thread::spawn(move || {
        let mut i = 0;
        loop {
            let r = 0;
            let g = (i % 256) as u8;
            let b = 0;

            let pixels = std::iter::repeat(RGB8::from((r, g, b))).take(NUM_PIXELS);
            ws2812.write(pixels).unwrap();
            i += 1;

            thread::sleep(Duration::from_millis(10));
        }
    });

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
