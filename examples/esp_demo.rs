#![no_std]
#![no_main]

use display_interface_spi::SPIInterfaceNoCS;

use embedded_graphics::{
    prelude::{RgbColor, Point, DrawTarget},
    pixelcolor::Rgb565,
    mono_font::{
        ascii::FONT_10X20,
        MonoTextStyleBuilder,
    },
    text::{Alignment, Text},
    Drawable,
};

use hal::{
    clock::{ClockControl, CpuClock},
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    IO,
    Delay,
};

use esp_backtrace as _;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let sclk = io.pins.gpio7;
    let mosi = io.pins.gpio6;

    let dc = io.pins.gpio4.into_push_pull_output();
    let mut backlight = io.pins.gpio45.into_push_pull_output();
    let reset = io.pins.gpio48.into_push_pull_output();

    let spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sclk,
        mosi,
        60u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    );

    let di = SPIInterfaceNoCS::new(spi, dc);
    delay.delay_ms(500u32);
    
    let mut display = match mipidsi::Builder::ili9342c_rgb565(di)
        .with_display_size(320, 240)
        .with_orientation(mipidsi::Orientation::PortraitInverted(false))
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .init(&mut delay, Some(reset)) {
        Ok(display) => display,
        Err(_) => {
            panic!("Display initialization failed");
        }
    };

    backlight.set_high().unwrap();

    display.clear(Rgb565::WHITE).unwrap();

    let default_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::BLACK)
        .build();
    
    let espressif_style = MonoTextStyleBuilder::new()   
        .font(&FONT_10X20)
        .text_color(RgbColor::CYAN)
        .build();

    for position_y in (25..240).step_by(28) {
        for position_x in 0..320 {
            Text::with_alignment("O", Point::new(position_x, position_y), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
        }
    }

    for position_x in (23..350).step_by(25) {
        for position_y in 0..250 {
            Text::with_alignment("O", Point::new(position_x, position_y), default_style,  Alignment::Center)
            .draw(&mut display)
            .unwrap();
        }
    }

    // letter E

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

    loop {}
}
