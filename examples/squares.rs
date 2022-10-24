#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::RgbColor,
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyleBuilder, MonoTextStyle,
    },
    prelude::Point,
    text::{Alignment, Text},
    Drawable,
};

use esp32s3_hal::{
    clock::ClockControl,
    pac::Peripherals,
    prelude::*,
    spi::{self, Spi},
    timer::TimerGroup,
    Rtc,
    IO,
    Delay,
};

use mipidsi::{DisplayOptions, Display, models::ILI9342CRgb565};

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

    let white_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::WHITE)
        .build();

    let mut start_point: i32 = 10;
    let mut edge_x: i32 = 310;
    let mut edge_y: i32 = 230;

    loop {
        while start_point != 140 && edge_x != 140 && edge_y != 70 {
            for x in start_point..edge_x {
                Text::with_alignment("o", Point::new(x, start_point), default_style,  Alignment::Center)
                    .draw(&mut display)
                    .unwrap();
            }
        
            for y in start_point..edge_y {
                Text::with_alignment("o", Point::new(edge_x, y), default_style,  Alignment::Center)
                    .draw(&mut display)
                    .unwrap();
            }
        
            for x in (start_point..edge_x).rev() {
                Text::with_alignment("o", Point::new(x, edge_y), default_style,  Alignment::Center)
                    .draw(&mut display)
                    .unwrap();
            }
        
            for y in (start_point..edge_y).rev() {
                Text::with_alignment("o", Point::new(start_point, y), default_style,  Alignment::Center)
                    .draw(&mut display)
                    .unwrap();
            }
            start_point += 10;
            edge_x -= 10;
            edge_y -= 10;
        }
    
        while start_point != 0 && edge_x != 320 && edge_y != 240 {
            for x in start_point..edge_x {
                Text::with_alignment("o", Point::new(x, start_point), white_style,  Alignment::Center)
                    .draw(&mut display)
                    .unwrap();
            }
        
            for y in start_point..edge_y {
                Text::with_alignment("o", Point::new(edge_x, y), white_style,  Alignment::Center)
                    .draw(&mut display)
                    .unwrap();
            }
        
            for x in (start_point..edge_x).rev() {
                Text::with_alignment("o", Point::new(x, edge_y), white_style,  Alignment::Center)
                    .draw(&mut display)
                    .unwrap();
            }
        
            for y in (start_point..edge_y).rev() {
                Text::with_alignment("o", Point::new(start_point, y), white_style,  Alignment::Center)
                    .draw(&mut display)
                    .unwrap();
            }
            start_point -= 5;
            edge_x += 5;
            edge_y += 5;
        }
    }
}