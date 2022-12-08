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

    let espressif_style = MonoTextStyleBuilder::new()   
        .font(&FONT_10X20)
        .text_color(RgbColor::CYAN)
        .build();

    //letter E

    for position_y in 94..154 {
        for position_x in 56..66 {
            Text::with_alignment("O", Point::new(position_x, position_y), espressif_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
        }
    }

    for y in (94..154).step_by(28) {
        for position_y in y..y+4 {
            for position_x in 66..116 {
                Text::with_alignment("O", Point::new(position_x, position_y), espressif_style,  Alignment::Center)
                .draw(&mut display)
                .unwrap();
            }
        }
    }

    //letter S

    for y in (94..154).step_by(28) {
        for position_y in y..y+5 {
            for position_x in 131..191 {
                Text::with_alignment("O", Point::new(position_x, position_y), espressif_style,  Alignment::Center)
                .draw(&mut display)
                .unwrap();
            }
        }
    }

    for position_y in 106..113 {
        for position_x in 131..141 {
            Text::with_alignment("O", Point::new(position_x, position_y), espressif_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
        }
    }

    for position_y in 136..143 {
        for position_x in 181..191 {
            Text::with_alignment("O", Point::new(position_x, position_y), espressif_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
        }
    }

    //letter P

    for position_y in 94..154 {
        for position_x in 206..216 {
            Text::with_alignment("O", Point::new(position_x, position_y), espressif_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
        }
    }

    for y in (94..126).step_by(28) {
        for position_y in y..y+3 {
            for position_x in 216..266 {
                Text::with_alignment("O", Point::new(position_x, position_y), espressif_style,  Alignment::Center)
                .draw(&mut display)
                .unwrap();
            }
        }
    }

    for position_y in 106..113 {
        for position_x in 256..266 {
            Text::with_alignment("O", Point::new(position_x, position_y), espressif_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
        }
    }

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

        Text::with_alignment("o", Point::new((x + 161.0) as i32, (y + 22.0) as i32), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    let mut i = 217;
    let mut j = 86;
    while(i > 125 && j > 35) {
        Text::with_alignment("-", Point::new(i, j), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
        i = i - 1;
        j = j - 1;
    }

    i = 105;
    j = 86;
    while(i < 199 && j > 35) {
        Text::with_alignment("-", Point::new(i, j), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
        i = i + 1;
        j = j - 1;
    }

    for k in 105..217 {
        Text::with_alignment("'", Point::new(k, 89), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }

    for k in 114..208 {
        Text::with_alignment("'", Point::new(k, 79), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
    }



    


    
    loop {}
}
