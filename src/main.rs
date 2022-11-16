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

fn main() -> Result<(), std::convert::Infallible> {

    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("ESP32-S3-BOX", &output_settings);

    display.clear(Rgb565::WHITE)?;
    let espressif_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(RgbColor::BLACK)
        .build();

    Text::with_alignment("HELLO WORLD!", Point::new(160, 120), espressif_style,  Alignment::Center)
        .draw(&mut display)
        .unwrap();

    'running: loop {
        window.update(&display);
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
    }
}
