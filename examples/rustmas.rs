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
    primitives::{Triangle, Line, Rectangle, RoundedRectangle, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment, Circle}, 
    text::{Alignment, Text},
    Drawable,
};

use tinybmp::Bmp;

use esp32s3_hal::{
    clock::{ClockControl, CpuClock},
    pac::{Peripherals, debug_assist::core_1_area_pc::R, hmac::set_message_pad},
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
use esp_println::println;

#[allow(unused_imports)]
use esp_backtrace as _;

use xtensa_lx_rt::entry;

use embedded_graphics_framebuf::FrameBuf;


fn ferris<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let ferris_data = include_bytes!("../data/ferris.bmp");
    let ferris = Bmp::from_slice(ferris_data).unwrap();
    Image::new(&ferris, Point::new(87, 140)).draw(display);
    
}

fn hat<D>(fbuf: &mut D, pos_x: f64, pos_y: f64)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let default_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::WHITE)
        .build();

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

        Text::with_alignment("o", Point::new((x + pos_x) as i32, (y + pos_y) as i32), default_style,  Alignment::Center)
            .draw(fbuf);
    }

    Triangle::new(
        Point::new(pos_x as i32, (pos_y + 8.0) as i32),
        Point::new((pos_x - 32.0) as i32, (pos_y + 41.0) as i32),
        Point::new((pos_x + 35.0) as i32, (pos_y + 41.0) as i32),
    )
    .into_styled(PrimitiveStyle::with_fill(RgbColor::RED))
    .draw(fbuf);

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new((pos_x - 42.0) as i32, (pos_y + 42.0) as i32), Size::new(89, 18)),
        Size::new(10, 10),
    )
    .into_styled(PrimitiveStyle::with_fill(RgbColor::WHITE))
    .draw(fbuf);
    
}


fn logo<D>(fbuf: &mut D)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(19, 80), Size::new(95, 95)),
        Size::new(10, 10),
    )
    .into_styled(PrimitiveStyle::with_fill(RgbColor::RED))
    .draw(fbuf);

    let espressif_data = include_bytes!("../data/espressif.bmp");

    let logo = Bmp::from_slice(espressif_data).unwrap();

    Image::new(&logo, Point::new(30, 89)).draw(fbuf);
}

fn tree<D>(fbuf: &mut D)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {
    
    let tree_style = PrimitiveStyle::with_fill(RgbColor::GREEN);
    
    Triangle::new(
        Point::new(280, 5),
        Point::new(250, 75),
        Point::new(310, 75),
    )
    .into_styled(tree_style)
    .draw(fbuf);

    Triangle::new(
        Point::new(280, 35),
        Point::new(250, 135),
        Point::new(310, 135),
    )
    .into_styled(tree_style)
    .draw(fbuf);

    Triangle::new(
        Point::new(280, 95),
        Point::new(250, 195),
        Point::new(310, 195),
    )
    .into_styled(tree_style)
    .draw(fbuf);

    Rectangle::new(Point::new(275, 196), Size::new(15, 45))
    .into_styled(PrimitiveStyle::with_fill(Rgb565::new(58, 29, 0)))
    .draw(fbuf);    
}


fn gift<D>(fbuf: &mut D, pos_x: i32, pos_y: i32)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {
    
    let gift_data = include_bytes!("../data/gift.bmp");

    let gift = Bmp::from_slice(gift_data).unwrap();

    Image::new(&gift, Point::new(pos_x, pos_y)).draw(fbuf);
}

fn gifts<D>(fbuf: &mut D, pos_x: i32, pos_y: i32)
where
    D:DrawTarget<Color = Rgb565>+Dimensions {
    
    let gift_data = include_bytes!("../data/stack_of_gifts.bmp");

    let gifts = Bmp::from_slice(gift_data).unwrap();

    Image::new(&gifts, Point::new(pos_x, pos_y)).draw(fbuf);
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

    let mut rng = Rng::new(peripherals.RNG);
    let mut x_values = [0u8; 10];
    let mut sizes = [0u8; 10];
    let mut num_buffer = [0u8; 1];

    rng.read(&mut x_values).unwrap();
    rng.read(&mut sizes).unwrap();
    let mut y_values = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut offsets = [0, 25, 50, 75, 100, 125, 150, 175, 200, 225];
    let mut main_counter = 0;

    loop {
        hat(&mut fbuf, 64.0, 20.0);
        logo(&mut fbuf);

        ferris(&mut fbuf);
        hat(&mut fbuf, 166.0, 105.0);

        tree(&mut fbuf);
        gift(&mut fbuf, 250, 215);
        gifts(&mut fbuf, 290, 210);

        for i in 0..10 {

            if (main_counter > offsets[i]) {
                Circle::new(Point::new(x_values[i] as i32, y_values[i]), sizes[i] as u32 % 15 + 5)
                .into_styled(PrimitiveStyleBuilder::new().fill_color(RgbColor::WHITE).build())
                .draw(&mut fbuf);
                y_values[i] += 5;
            }

            if (y_values[i] > 240) {
                y_values[i] = 0;
                rng.read(&mut num_buffer).unwrap();
                x_values[i] = num_buffer[0];
            }
        }
        
        display.draw_iter(fbuf.into_iter()).unwrap();
        fbuf.clear(Rgb565::BLACK);
        main_counter += 5;

        if (main_counter == 50000) {
            main_counter = 0;
        }
    }
}
