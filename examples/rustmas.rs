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
    Rng,
    Delay,
};

use mipidsi::{ Orientation, ColorOrder };

#[allow(unused_imports)]
use esp_backtrace as _;

use xtensa_lx_rt::entry;

use embedded_graphics_framebuf::FrameBuf;

use examples_assets::{ hat, logo, ferris, tree, gift, gifts, snowflake };

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
    let dc = io.pins.gpio4;
    let bcklght = io.pins.gpio45;
    let rst = io.pins.gpio48;
    
    let mut backlight = bcklght.into_push_pull_output();
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

    let di = SPIInterfaceNoCS::new(spi, dc.into_push_pull_output());
    let reset = rst.into_push_pull_output();

    let mut delay = Delay::new(&clocks);

    let mut display = mipidsi::Builder::ili9342c_rgb565(di)
        .with_orientation(Orientation::PortraitInverted(false))
        .with_color_order(ColorOrder::Bgr)
        .init(&mut delay, Some(reset))
        .unwrap();

    let mut data = [Rgb565::BLACK; 320 * 240];
    let mut fbuf = FrameBuf::new(&mut data, 320, 240);
    let mut rng = Rng::new(peripherals.RNG);
    let mut x_values = [0u8; 10];
    let mut sizes = [0u8; 10];
    let mut num_buffer = [0u8; 1];

    rng.read(&mut x_values).unwrap();
    rng.read(&mut sizes).unwrap();
    let mut y_values = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let offsets = [0, 25, 50, 75, 100, 125, 150, 175, 200, 225];
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

            if main_counter > offsets[i] {
                snowflake(&mut fbuf, x_values[i] as i32, y_values[i], sizes[i] as u32);
                y_values[i] += 5;
            }

            if y_values[i] > 240 {
                y_values[i] = 0;
                rng.read(&mut num_buffer).unwrap();
                x_values[i] = num_buffer[0];
            }
        }

        display.draw_iter(fbuf.into_iter()).unwrap();

        #[allow(unused_must_use)] {
            fbuf.clear(Rgb565::BLACK);
        }

        main_counter += 5;

        if main_counter == 50000 {
            main_counter = 0;
        }
    }
}
