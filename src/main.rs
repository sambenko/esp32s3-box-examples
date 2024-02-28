#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_display_interface_spi_dma::display_interface_spi_dma;

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
    dma::DmaPriority,
    gdma::Gdma,
    peripherals::Peripherals,
    prelude::*,
    gpio::{GpioPin, Output},
    spi::{
        master::{prelude::*, Spi},  
        SpiMode,
        FullDuplexMode,
    },
    IO,
    Delay,
};

use esp_bsp::{lcd_gpios, BoardType, DisplayConfig, define_display_type};

use esp_println::println;
use static_cell::make_static;
use esp_backtrace as _;

type BoardDisplay = define_display_type!(BoardType::ESP32S3Box);

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let dma = Gdma::new(peripherals.DMA);
    let dma_channel = dma.channel0;
    
    let descriptors = make_static!([0u32; 8 * 3]);
    let rx_descriptors = make_static!([0u32; 8 * 3]);

    let (lcd_sclk, lcd_mosi, lcd_cs, lcd_miso, lcd_dc, mut lcd_backlight, lcd_reset) = lcd_gpios!(BoardType::ESP32S3Box, io);

    let spi = Spi::new(
        peripherals.SPI2,
        40u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    ).with_pins(
        Some(lcd_sclk),
        Some(lcd_mosi),
        Some(lcd_miso),
        Some(lcd_cs),
    ).with_dma(dma_channel.configure(
        false,
        &mut *descriptors,
        &mut *rx_descriptors,
        DmaPriority::Priority0,
    ));

    let di = display_interface_spi_dma::new_no_cs(2 * 256 * 192, spi, lcd_dc);
    let display_config = DisplayConfig::for_board(BoardType::ESP32S3Box);
    let mut display: BoardDisplay = match mipidsi::Builder::ili9342c_rgb565(di)
        .with_display_size(display_config.h_res, display_config.v_res)
        .with_orientation(mipidsi::Orientation::PortraitInverted(false))
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .init(&mut delay, Some(lcd_reset)) {
        Ok(display) => display,
        Err(_) => {
            panic!("Display initialization failed");
        }
    };

    lcd_backlight.set_high().unwrap();

    display.clear(Rgb565::WHITE).unwrap();

    Text::with_alignment("HELLO WORLD!", Point::new(160, 120), MonoTextStyleBuilder::new().font(&FONT_10X20).text_color(RgbColor::BLACK).build(),  Alignment::Center)
        .draw(&mut display)
        .unwrap();
    
    println!("Hello World!");

    loop {}
}
                    