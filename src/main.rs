// #![no_std]
// #![no_main]

// use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Sector},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

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

// use esp32s3_hal::{
//     clock::ClockControl,
//     pac::Peripherals,
//     prelude::*,
//     spi,
//     timer::TimerGroup,
//     Rtc,
//     IO,
//     Delay,
// };

// use esp_println::println;

// use mipidsi::DisplayOptions;

// #[allow(unused_imports)]
// use esp_backtrace as _;

// use xtensa_lx_rt::entry;

// #[entry]
fn main() -> Result<(), std::convert::Infallible>  {
    // let peripherals = Peripherals::take().unwrap();
    // let mut system = peripherals.SYSTEM.split();
    // let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    // let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    // let mut wdt0 = timer_group0.wdt;
    // let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    // let mut wdt1 = timer_group1.wdt;

    // rtc.rwdt.disable();

    // wdt0.disable();
    // wdt1.disable();

    // let mut delay = Delay::new(&clocks);

    // let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // let sclk = io.pins.gpio7;
    // let mosi = io.pins.gpio6;

    // let spi = spi::Spi::new_no_cs_no_miso(
    //     peripherals.SPI2,
    //     sclk,
    //     mosi,
    //     4u32.MHz(),
    //     spi::SpiMode::Mode0,
    //     &mut system.peripheral_clock_control,
    //     &clocks,
    // );

    // let mut backlight = io.pins.gpio45.into_push_pull_output();
    // backlight.set_high().unwrap();

    // let reset = io.pins.gpio48.into_push_pull_output();

    // let di = SPIInterfaceNoCS::new(spi, io.pins.gpio4.into_push_pull_output());

    // let display_options = DisplayOptions {
    //     orientation: mipidsi::Orientation::PortraitInverted(false),
    //     ..Default::default()
    // };

    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("ESP32-S3-BOX", &output_settings);
    // let mut display = mipidsi::Display::ili9342c_rgb565(di, core::prelude::v1::Some(reset), display_options);
    // display.init(&mut delay).unwrap();

    display.clear(Rgb565::WHITE)?;
    let espressif_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::BLACK)
        .build();

    Text::with_alignment("HELLO WORLD!", Point::new(160, 120), espressif_style,  Alignment::Center)
        .draw(&mut display)
        .unwrap();
    
    // println!("Hello World!");
    

    'running: loop {
        window.update(&display);
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
    }
}
