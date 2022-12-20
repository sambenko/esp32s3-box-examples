#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::{RgbColor, Point, Primitive, Size, DrawTarget, Dimensions, DrawTargetExt},
    image::Image,
    pixelcolor::Rgb565,
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyleBuilder, MonoTextStyle,
    },
    primitives::{Triangle, Line, Rectangle, RoundedRectangle, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment}, 
    text::{Alignment, Text},
    Drawable,
};

use tinybmp::Bmp;

use esp32s3_hal::{
    clock::{ClockControl, CpuClock},
    pac::{Peripherals, debug_assist::core_1_area_pc::R},
    prelude::*,
    spi,
    timer::TimerGroup,
    Rtc,
    IO,
    Rng,
    Delay, gpio::Gpio38, ehal::can::Error,
};

use mipidsi::{DisplayOptions, Display};

use core::f32::consts::PI;
use libm::{sin, cos};

#[allow(unused_imports)]
use esp_backtrace as _;

use xtensa_lx_rt::entry;

use embedded_graphics_framebuf::FrameBuf;
//use spooky_core::spritebuf::SpriteBuf;


fn ferris<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let ferris_data = include_bytes!("../data/ferris.bmp");
    let ferris = Bmp::from_slice(ferris_data).unwrap();
    Image::new(&ferris, Point::new(97, 140)).draw(display);
    
}


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

    let mut data = [Rgb565::BLACK; 320 * 240];
    let mut fbuf = FrameBuf::new(&mut data, 320, 240);
    //let mut sbuf = SpriteBuf::new(fbuf);

    display.draw_iter(fbuf.into_iter()).unwrap();

    let default_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::WHITE)
        .build();

    //christmas hat

    let n = 6.0;
    let d = 71.0;    
    let mut a;
    let mut r;
    let mut x;
    let mut y;
    
    for t in 0..361 {
        a = t as f64 * d * (PI as f64 / 18.0);
        r = 10.0 * sin(n * a);
        x = r * cos(a);
        y = r * sin(a);

        Text::with_alignment("o", Point::new((x + 74.0) as i32, (y + 20.0) as i32), default_style,  Alignment::Center)
            .draw(&mut fbuf)
            .unwrap();
    }

    display.draw_iter(fbuf.into_iter()).unwrap();

    let hat_style = PrimitiveStyleBuilder::new()
        .fill_color(RgbColor::RED)
        .build();
    
    let cushion_style = PrimitiveStyleBuilder::new()
        .fill_color(RgbColor::WHITE)
        .build();

    Triangle::new(
        Point::new(74, 28),
        Point::new(42, 61),
        Point::new(109, 61),
    )
    .into_styled(hat_style)
    .draw(&mut fbuf)
    .unwrap();

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(32, 62), Size::new(89, 18)),
        Size::new(10, 10),
    )
    .into_styled(cushion_style)
    .draw(&mut fbuf)
    .unwrap();

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(29, 80), Size::new(95, 95)),
        Size::new(10, 10),
    )
    .into_styled(hat_style)
    .draw(&mut fbuf)
    .unwrap();
    
    let espressif_data = include_bytes!("../data/espressif.bmp");

    let logo = Bmp::from_slice(espressif_data).unwrap();

    Image::new(&logo, Point::new(40, 89)).draw(&mut fbuf).unwrap();

    ferris(&mut fbuf);
    
    display.draw_iter(fbuf.into_iter()).unwrap();

    loop {}
}
