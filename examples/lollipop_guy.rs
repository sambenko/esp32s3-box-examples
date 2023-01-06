#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::{ RgbColor, DrawTarget },
    pixelcolor::Rgb565,
};

use esp32s3_hal::{
    clock::ClockControl,
    pac::Peripherals,
    prelude::*,
    spi,
    timer::TimerGroup,
    Rtc,
    IO,
    Delay,
};

use mipidsi::{ Orientation, ColorOrder };

use examples_assets::{body_part, hand};

#[allow(unused_imports)]
use esp_backtrace as _;

use xtensa_lx_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();

    wdt0.disable();
    wdt1.disable();

    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let sclk = io.pins.gpio7;
    let mosi = io.pins.gpio6;

    let spi = spi::Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sclk,
        mosi,
        4u32.MHz(),
        spi::SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let mut backlight = io.pins.gpio45.into_push_pull_output();
    backlight.set_high().unwrap();

    let reset = io.pins.gpio48.into_push_pull_output();

    let di = SPIInterfaceNoCS::new(spi, io.pins.gpio4.into_push_pull_output());

    let mut display = mipidsi::Builder::ili9342c_rgb565(di)
        .with_orientation(Orientation::PortraitInverted(false))
        .with_color_order(ColorOrder::Rgb)
        .init(&mut delay, core::prelude::v1::Some(reset))
    .unwrap();

    display.clear(Rgb565::WHITE).unwrap();

    //main body
    body_part(&mut display, "'", [40.0, 50.0, 220.0, 200.0], 6400, -1, 1);

    //head
    body_part(&mut display, "'", [60.0, 50.0, 220.0, 60.0], 6400, 1, -1);

    //eyes
    body_part(&mut display, "'", [20.0, 15.0, 200.0, 60.0], 1300, -1, -1);
    body_part(&mut display, "'", [20.0, 15.0, 240.0, 60.0], 1300, -1, -1);

    //hand
    hand(&mut display, 125, 175);

    //lollipop
    body_part(&mut display, "'", [30.0, 30.0, 110.0, 110.0], 3300, -1, -1);
    
    loop {}
}