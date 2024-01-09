#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::{ RgbColor, DrawTarget },
    pixelcolor::Rgb565,
};

use embedded_graphics_framebuf::FrameBuf;

use hal::{
    clock::{ClockControl, CpuClock},
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    IO,
    Delay,
};

use examples_assets::{ flower, stem };

use esp_backtrace as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let sclk = io.pins.gpio7;
    let mosi = io.pins.gpio6;
    let cs = io.pins.gpio5;
    let miso = io.pins.gpio2;

    let dc = io.pins.gpio4.into_push_pull_output();
    let mut backlight = io.pins.gpio45.into_push_pull_output();
    let reset = io.pins.gpio48.into_push_pull_output();

    let spi = Spi::new(
        peripherals.SPI2,
        40u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    ).with_pins(
        Some(sclk),
        Some(mosi),
        Some(miso),
        Some(cs),
    );

    let di = SPIInterfaceNoCS::new(spi, dc);
    delay.delay_ms(500u32);
    
    let mut display = match mipidsi::Builder::ili9342c_rgb565(di)
        .with_display_size(320, 240)
        .with_orientation(mipidsi::Orientation::PortraitInverted(false))
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .init(&mut delay, Some(reset)) {
        Ok(display) => display,
        Err(_) => {
            panic!("Display initialization failed");
        }
    };

    backlight.set_high().unwrap();

    display.clear(Rgb565::WHITE).unwrap();

    let mut data = [Rgb565::WHITE; 320 * 240];
    let mut fbuf = FrameBuf::new(&mut data, 320, 240);

    flower(&mut fbuf, 6.0, 71.0, 60.0, 30.0, 120, 35.0, 180.0);
    stem(&mut fbuf, 35, 180, 240);

    flower(&mut fbuf, 7.0, 19.0, 110.0, 30.0, 250, 90.0, 140.0);
    stem(&mut fbuf, 90, 140, 240);

    flower(&mut fbuf, 2.0, 39.0, 150.0, 30.0, 100, 140.0, 190.0);
    stem(&mut fbuf, 140, 190, 240);

    flower(&mut fbuf, 8.0, 27.0, 230.0, 30.0, 250, 243.0, 200.0);
    stem(&mut fbuf, 246, 200, 240);

    flower(&mut fbuf, 5.0, 69.0, 125.0, 30.0, 200, 290.0, 155.0);
    stem(&mut fbuf, 290, 155, 240);

    flower(&mut fbuf, 6.0, 71.0, 1200.0, 80.0, 1900, 200.0, 90.0);
    stem(&mut fbuf, 200, 130, 240);

    display.draw_iter(fbuf.into_iter()).unwrap();
    
    loop {}
}
