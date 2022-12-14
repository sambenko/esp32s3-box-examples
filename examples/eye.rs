#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::{ RgbColor, DrawTarget, Point },
    pixelcolor::Rgb565,
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyleBuilder,
    },
    text::{Alignment, Text},
    Drawable,
};

use core::f32::consts::PI;
use libm::{sin, cos};

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

use embedded_graphics_framebuf::FrameBuf;

use mipidsi::{ Orientation, ColorOrder };

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

    let mut data = [Rgb565::WHITE; 320 * 240];
    let mut fbuf = FrameBuf::new(&mut data, 320, 240);

    let mut vt;
    let mut x;
    let mut y;
    for i in 0..12400 {
        vt = i as f64 / (20.0 * PI as f64);
        if i < 8000 {
            x = (vt - 50.0) * sin(vt);
        } else {
            x = (vt + 20.0) * sin(vt);
        }
        y = (vt - 50.0) * cos(vt);
        if i < 8000 {
            Text::with_alignment("'", Point::new((x + 160.0) as i32, (y + 125.0) as i32), 
            MonoTextStyleBuilder::new().font(&FONT_10X20).text_color(RgbColor::BLACK).build(),  Alignment::Center)
                .draw(&mut display)
                .unwrap();
            
        } else {
            Text::with_alignment("|", Point::new((x + 160.0) as i32, (y + 125.0) as i32), 
            MonoTextStyleBuilder::new().font(&FONT_10X20).text_color(RgbColor::BLACK).build(),  Alignment::Center)
                .draw(&mut display)
                .unwrap();
        }
    }

    loop {}
}