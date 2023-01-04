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

use esp_println::println;

#[allow(unused_imports)]
use esp_backtrace as _;

use xtensa_lx_rt::entry;

use embedded_graphics_framebuf::FrameBuf;

use rustmas_assets;

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
        rustmas_assets::hat(&mut fbuf, 64.0, 20.0);
        rustmas_assets::logo(&mut fbuf);

        rustmas_assets::ferris(&mut fbuf);
        rustmas_assets::hat(&mut fbuf, 166.0, 105.0);

        rustmas_assets::tree(&mut fbuf);
        rustmas_assets::gift(&mut fbuf, 250, 215);
        rustmas_assets::gifts(&mut fbuf, 290, 210);

        for i in 0..10 {

            if (main_counter > offsets[i]) {
                rustmas_assets::snowflake(&mut fbuf, x_values[i] as i32, y_values[i], sizes[i] as u32);
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
