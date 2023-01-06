#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::{RgbColor, DrawTarget},
    pixelcolor::Rgb565,
};

use esp32s3_hal::{
    clock::{ClockControl, CpuClock},
    pac::Peripherals,
    prelude::*,
    spi,
    timer::TimerGroup,
    Rtc,
    IO,
    Delay,
};

use mipidsi::{ Orientation, ColorOrder };

#[allow(unused_imports)]
use esp_backtrace as _;

use xtensa_lx_rt::entry;

use examples_assets::print_squares;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();

    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let sclk = io.pins.gpio7;
    let mosi = io.pins.gpio6;
    
    let mut backlight = io.pins.gpio45.into_push_pull_output();
    backlight.set_high().unwrap();

    let spi = spi::Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sclk,
        mosi,
        4u32.MHz(),
        spi::SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let di = SPIInterfaceNoCS::new(spi, io.pins.gpio4.into_push_pull_output());
    let reset = io.pins.gpio48.into_push_pull_output();

    let mut delay = Delay::new(&clocks);

    let mut display = mipidsi::Builder::ili9342c_rgb565(di)
        .with_orientation(Orientation::PortraitInverted(false))
        .with_color_order(ColorOrder::Rgb)
        .init(&mut delay, core::prelude::v1::Some(reset))
    .unwrap();

    display.clear(Rgb565::WHITE).unwrap();

    loop {
        //prints black squares
        print_squares(&mut display, "o", RgbColor::BLACK, 10, 310, 230, 1, [140, 140, 70]);

        //white squares clear the board
        print_squares(&mut display, "o", RgbColor::WHITE, 140, 180, 100, -1, [0, 320, 240]);
    }
}