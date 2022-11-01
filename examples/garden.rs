#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::RgbColor,
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyleBuilder,
    },
    prelude::Point,
    text::{Alignment, Text},
    Drawable,
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

use mipidsi::DisplayOptions;

use core::f32::consts::PI;
use libm::{sin, cos};

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

    let display_options = DisplayOptions {
        orientation: mipidsi::Orientation::PortraitInverted(false),
        ..Default::default()
    };

    let mut display = mipidsi::Display::ili9342c_rgb565(di, core::prelude::v1::Some(reset), display_options);
    display.init(&mut delay).unwrap();

    let default_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::BLACK)
        .build();

    let mut n = 6.0;
    let mut d = 71.0;    
    let mut a;
    let mut r;
    let mut x;
    let mut y;

    for t in 0..361 {
        a = t as f64 * d * (PI as f64 / 60.0);
        r = 30.0 * sin(n * a);
        x = r * cos(a);
        y = r * sin(a);

        Text::with_alignment("o", Point::new((x + 35.0) as i32, (y + 180.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }
    let pos_x = 1;
    for pos_y in 0..60 {
        Text::with_alignment("|", Point::new(pos_x + 34, pos_y + 180), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    n = 7.0;
    d = 19.0;
    for t in 0..700 {
        a = t as f64 * d * (PI as f64 / 300.0);
        r = 30.0 * sin(n * a);
        x = r * cos(a);
        y = r * sin(a);

        Text::with_alignment("o", Point::new((x + 90.0) as i32, (y + 140.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    for pos_y in 0..100 {
        Text::with_alignment("|", Point::new(pos_x + 89, pos_y + 140), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    n = 2.0;
    d = 39.0;
    for t in 0..500 {
        a = t as f64 * d * (PI as f64 / 150.0);
        r = 30.0 * sin(n * a);
        x = r * cos(a);
        y = r * sin(a);

        Text::with_alignment("S", Point::new((x + 140.0) as i32, (y + 190.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    for pos_y in 0..50 {
        Text::with_alignment("|", Point::new(pos_x + 139, pos_y + 190), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    n = 8.0;
    d = 27.0;
    for t in 0..1000 {
        a = t as f64 * d * (PI as f64 / 230.0);
        r = 30.0 * sin(n * a);
        x = r * cos(a);
        y = r * sin(a);

        Text::with_alignment("o", Point::new((x + 243.0) as i32, (y + 200.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }
    for pos_y in 0..85 {
        Text::with_alignment("|", Point::new(pos_x + 242, pos_y + 200), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    n = 5.0;
    d = 97.0;
    for t in 0..700 {
        a = t as f64 * d * (PI as f64 / 150.0);
        r = 30.0 * sin(n * a);
        x = r * cos(a);
        y = r * sin(a);

        Text::with_alignment("o", Point::new((x + 290.0) as i32, (y + 155.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }
    for pos_y in 0..85 {
        Text::with_alignment("|", Point::new(pos_x + 289, pos_y + 155), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    n = 6.0;
    d = 71.0;
    for t in 0..2500 {
        a = t as f64 * d * (PI as f64 / 1200.0);
        r = 80.0 * sin(n * a);
        x = r * cos(a);
        y = r * sin(a);

        Text::with_alignment("o", Point::new((x + 200.0) as i32, (y + 90.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    for pos_y in 0..100 {
        Text::with_alignment("|", Point::new(pos_x + 199, pos_y + 140), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    

    loop {}
}
